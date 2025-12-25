use super::super::result_manager::FuzzySearchResultItem;
use super::super::types::{FuzzyCondition, ValueType};
use super::manager::{BPLUS_TREE_ORDER, PAGE_SIZE};
use crate::core::DRIVER_MANAGER;
use crate::wuwa::PageStatusBitmap;
use anyhow::{anyhow, Result};
use bplustree::BPlusTreeSet;
use log::{debug, log_enabled, warn, Level};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

/// 批量读取：最大合并间隙（4KB，即1页大小）
/// 当两个地址之间的间隙小于此值时，合并为同一批次
const BATCH_MAX_GAP: u64 = 4096;

/// 批量读取：单批次大小上限（64KB）
const BATCH_MAX_SIZE: usize = 64 * 1024;

/// 进度更新：每处理多少批次更新一次进度
const PROGRESS_UPDATE_BATCH_SIZE: usize = 1;

/// 地址批次 - 表示一段连续或接近连续的内存区域
#[derive(Debug)]
struct AddressBatch {
    start_addr: u64,          // 批次起始地址
    total_size: usize,        // 需要读取的总大小（包含间隙）
    items: Vec<BatchItemRef>, // 包含的地址引用
}

/// 批次内的地址引用
#[derive(Debug, Clone, Copy)]
struct BatchItemRef {
    offset: usize,     // 在批次缓冲区中的偏移
    item_index: usize, // 在原始 items 数组中的索引
    value_size: usize, // 值大小
}

impl AddressBatch {
    /// 创建新批次（包含单个地址）
    fn new(start_addr: u64, size: usize, index: usize) -> Self {
        Self {
            start_addr,
            total_size: size,
            items: vec![BatchItemRef {
                offset: 0,
                item_index: index,
                value_size: size,
            }],
        }
    }
}

/// 模糊搜索初始扫描
/// 记录指定内存区域内所有地址的当前值
/// 使用 BPlusTreeSet 存储结果，保持有序且支持高效删除
///
/// # 参数
/// * `value_type` - 要搜索的值类型
/// * `start` - 区域起始地址
/// * `end` - 区域结束地址
/// * `chunk_size` - 每次读取的块大小
/// * `processed_counter` - 已处理计数器（可选）
/// * `total_found_counter` - 找到总数计数器（可选）
/// * `check_cancelled` - 取消检查闭包（可选）
///
/// # 返回
/// 返回所有成功读取的地址及其值（有序）
pub(crate) fn fuzzy_initial_scan<F>(
    value_type: ValueType,
    start: u64,
    end: u64,
    chunk_size: usize,
    processed_counter: Option<&Arc<AtomicUsize>>,
    total_found_counter: Option<&Arc<AtomicUsize>>,
    check_cancelled: Option<&F>,
) -> Result<BPlusTreeSet<FuzzySearchResultItem>>
where
    F: Fn() -> bool,
{
    let driver_manager = DRIVER_MANAGER.read().map_err(|_| anyhow!("Failed to acquire DriverManager lock"))?;

    let element_size = value_type.size();
    let page_size = *PAGE_SIZE;

    let mut results = BPlusTreeSet::new(BPLUS_TREE_ORDER);

    let mut read_success = 0usize;
    let mut read_failed = 0usize;

    let mut current = start & !(*PAGE_SIZE as u64 - 1); // 页对齐
    let mut chunk_buffer = vec![0u8; chunk_size];

    while current < end {
        // Check cancellation at each chunk
        if let Some(check_fn) = check_cancelled {
            if check_fn() {
                if log_enabled!(Level::Debug) {
                    debug!("Fuzzy initial scan cancelled, returning {} results", results.len());
                }
                return Ok(results);
            }
        }

        let chunk_end = (current + chunk_size as u64).min(end);
        let chunk_len = (chunk_end - current) as usize;

        let mut page_status = PageStatusBitmap::new(chunk_len, current as usize);

        let read_result = driver_manager.read_memory_unified(current, &mut chunk_buffer[..chunk_len], Some(&mut page_status));

        match read_result {
            Ok(_) => {
                let success_pages = page_status.success_count();
                if success_pages > 0 {
                    read_success += 1;

                    // 使用 rayon 并行处理 buffer，收集到临时 Vec
                    let chunk_results = scan_buffer_parallel(
                        &chunk_buffer[..chunk_len],
                        current,
                        start,
                        end,
                        element_size,
                        value_type,
                        page_size,
                        &page_status,
                    );

                    // 批量插入到 BPlusTreeSet
                    for item in chunk_results {
                        results.insert(item);
                    }
                } else {
                    read_failed += 1;
                }
            },
            Err(error) => {
                if log_enabled!(Level::Debug) {
                    warn!("Failed to read memory at 0x{:X} - 0x{:X}, err: {:?}", current, chunk_end, error);
                }
                read_failed += 1;
            },
        }

        // 更新计数器
        if let Some(counter) = processed_counter {
            counter.fetch_add(chunk_len, Ordering::Relaxed);
        }

        current = chunk_end;
    }

    if log_enabled!(Level::Debug) {
        let region_size = end - start;
        debug!(
            "Fuzzy initial scan: size={}MB, reads={} success + {} failed, found={}",
            region_size / 1024 / 1024,
            read_success,
            read_failed,
            results.len()
        );
    }

    // 更新总找到数
    if let Some(counter) = total_found_counter {
        counter.store(results.len(), Ordering::Relaxed);
    }

    Ok(results)
}

/// 使用 rayon 并行处理缓冲区，按页分割任务
/// 每个成功的页独立并行处理，无需比较操作
#[inline]
fn scan_buffer_parallel(
    buffer: &[u8],
    buffer_addr: u64,
    region_start: u64,
    region_end: u64,
    element_size: usize,
    value_type: ValueType,
    page_size: usize,
    page_status: &PageStatusBitmap,
) -> Vec<FuzzySearchResultItem> {
    let buffer_end = buffer_addr + buffer.len() as u64;
    let search_start = buffer_addr.max(region_start);
    let search_end = buffer_end.min(region_end);

    if search_start >= search_end {
        return Vec::new();
    }

    let num_pages = page_status.num_pages();

    // 收集所有成功页的索引
    let success_pages: Vec<usize> = (0..num_pages).filter(|&i| page_status.is_page_success(i)).collect();

    if success_pages.is_empty() {
        return Vec::new();
    }

    // 使用 rayon 并行处理每个成功的页
    success_pages
        .par_iter()
        .flat_map(|&page_idx| scan_single_page(buffer, buffer_addr, search_start, search_end, element_size, value_type, page_size, page_idx))
        .collect()
}

/// 扫描单个页内的所有元素
#[inline]
fn scan_single_page(
    buffer: &[u8],
    buffer_addr: u64,
    search_start: u64,
    search_end: u64,
    element_size: usize,
    value_type: ValueType,
    page_size: usize,
    page_idx: usize,
) -> Vec<FuzzySearchResultItem> {
    let page_start_addr = buffer_addr + (page_idx * page_size) as u64;
    let page_end_addr = page_start_addr + page_size as u64;

    // 与搜索范围取交集
    let effective_start = page_start_addr.max(search_start);
    let effective_end = page_end_addr.min(search_end);

    if effective_start >= effective_end {
        return Vec::new();
    }

    // 对齐到元素边界
    let rem = effective_start % element_size as u64;
    let first_addr = if rem == 0 {
        effective_start
    } else {
        effective_start + element_size as u64 - rem
    };

    if first_addr >= effective_end {
        return Vec::new();
    }

    // 预计算元素数量，一次性分配
    let elements_count = ((effective_end - first_addr) as usize) / element_size;
    let mut results = Vec::with_capacity(elements_count);

    // 批量处理：直接遍历字节切片，无需逐元素检查页状态
    let start_offset = (first_addr - buffer_addr) as usize;
    let end_offset = (effective_end - buffer_addr) as usize;

    // 确保不越界
    let safe_end = end_offset.min(buffer.len());

    let mut offset = start_offset;
    let mut addr = first_addr;

    while offset + element_size <= safe_end {
        // 直接从 buffer 切片创建结果项
        let item = FuzzySearchResultItem::from_bytes(addr, &buffer[offset..offset + element_size], value_type);
        results.push(item);

        offset += element_size;
        addr += element_size as u64;
    }

    results
}

/// 将有序的地址列表聚类为批次
///
/// 策略：
/// - 相邻地址（间隙 < BATCH_MAX_GAP）合并为同一批次
/// - 批次大小超过 BATCH_MAX_SIZE 时强制分割
/// - 利用地址已排序的特性
///
/// # 参数
/// * `items` - 有序的地址列表
///
/// # 返回
/// 返回地址批次列表
fn cluster_addresses(items: &[FuzzySearchResultItem]) -> Vec<AddressBatch> {
    if items.is_empty() {
        return Vec::new();
    }

    let mut batches = Vec::new();
    let mut current_batch: Option<AddressBatch> = None;

    for (idx, item) in items.iter().enumerate() {
        let addr = item.address;
        let size = item.value_type.size();

        match &mut current_batch {
            Some(batch) => {
                let batch_end = batch.start_addr + batch.total_size as u64;
                let gap = addr.saturating_sub(batch_end);
                let new_total_size = (addr + size as u64 - batch.start_addr) as usize;

                // 决策：是否合并到当前批次
                if gap <= BATCH_MAX_GAP && new_total_size <= BATCH_MAX_SIZE {
                    // 合并：更新批次大小并添加地址引用
                    batch.total_size = new_total_size;
                    batch.items.push(BatchItemRef {
                        offset: (addr - batch.start_addr) as usize,
                        item_index: idx,
                        value_size: size,
                    });
                } else {
                    // 完成当前批次，开始新批次
                    batches.push(current_batch.take().unwrap());
                    current_batch = Some(AddressBatch::new(addr, size, idx));
                }
            },
            None => {
                // 首个批次
                current_batch = Some(AddressBatch::new(addr, size, idx));
            },
        }
    }

    // 添加最后一个批次
    if let Some(batch) = current_batch {
        batches.push(batch);
    }

    batches
}

/// 并行批量读取内存
///
/// 使用 Rayon 并行处理各个批次，每个批次单次读取整段内存
/// 批量读取失败时自动降级为逐个读取
///
/// # 参数
/// * `batches` - 地址批次列表
/// * `items` - 原始地址列表
/// * `processed_counter` - 已处理计数器
/// * `total_found_counter` - 找到总数计数器
/// * `update_progress` - 进度更新回调
/// * `check_cancelled` - 取消检查闭包
///
/// # 返回
/// 返回成功读取的 (地址项, 当前值) 元组列表
fn parallel_batch_read<P, F>(
    batches: &[AddressBatch],
    items: &[FuzzySearchResultItem],
    processed_counter: Option<&Arc<AtomicUsize>>,
    total_found_counter: Option<&Arc<AtomicUsize>>,
    update_progress: &P,
    check_cancelled: Option<&F>,
) -> Result<Vec<(FuzzySearchResultItem, Vec<u8>)>>
where
    P: Fn(usize, usize) + Sync,
    F: Fn() -> bool + Sync,
{
    let total_items = items.len();
    let cancelled = Arc::new(AtomicBool::new(false));
    let cancelled_clone = Arc::clone(&cancelled);

    // 并行处理批次
    let results: Result<Vec<(FuzzySearchResultItem, Vec<u8>)>> = batches
        .par_iter()
        .enumerate()
        .take_any_while(|&(_idx, _batch)| {
            // 检查取消状态
            if cancelled_clone.load(Ordering::Relaxed) {
                return false;
            }
            if let Some(check_fn) = check_cancelled {
                if check_fn() {
                    cancelled_clone.store(true, Ordering::Relaxed);
                    return false;
                }
            }
            true
        })
        .try_fold(
            || Vec::new(), // 线程本地累加器
            |mut acc, (batch_idx, batch)| -> Result<Vec<(FuzzySearchResultItem, Vec<u8>)>> {
                let driver_manager = DRIVER_MANAGER.read().map_err(|_| anyhow!("Failed to acquire DriverManager lock"))?;

                // 分配批次缓冲区
                let mut buffer = vec![0u8; batch.total_size];

                // 单次批量读取整个段
                match driver_manager.read_memory_unified(
                    batch.start_addr,
                    &mut buffer,
                    None, // 不跟踪页状态
                ) {
                    Ok(_) => {
                        // 从批次缓冲区提取各个地址的值
                        for item_ref in &batch.items {
                            let value_bytes = &buffer[item_ref.offset..item_ref.offset + item_ref.value_size];
                            let original_item = &items[item_ref.item_index];
                            acc.push((original_item.clone(), value_bytes.to_vec()));
                        }
                    },
                    Err(e) => {
                        if log_enabled!(Level::Debug) {
                            debug!(
                                "Batch read failed at 0x{:X} (size {}), falling back to individual reads: {:?}",
                                batch.start_addr, batch.total_size, e
                            );
                        }

                        // 逐个读取批次内的地址
                        for item_ref in &batch.items {
                            let original_item = &items[item_ref.item_index];
                            let mut small_buffer = vec![0u8; item_ref.value_size];

                            if driver_manager.read_memory_unified(original_item.address, &mut small_buffer, None).is_ok() {
                                acc.push((original_item.clone(), small_buffer));
                            }
                        }
                    },
                }

                drop(driver_manager); // 显式释放读锁

                if batch_idx % PROGRESS_UPDATE_BATCH_SIZE == 0 {
                    if let Some(counter) = processed_counter {
                        let processed = counter.fetch_add(batch.items.len(), Ordering::Relaxed) + batch.items.len();
                        let found = total_found_counter.map(|c| c.load(Ordering::Relaxed)).unwrap_or(0);
                        update_progress(processed, found);
                    }
                } else if let Some(counter) = processed_counter {
                    // 更新计数器
                    counter.fetch_add(batch.items.len(), Ordering::Relaxed);
                }

                Ok(acc)
            },
        )
        .try_reduce(
            || Vec::new(),
            |mut a, b| {
                a.extend(b);
                Ok(a)
            },
        );

    results
}

/// 模糊搜索细化
/// 读取已有结果的当前值，并根据条件过滤
/// 返回新的 BPlusTreeSet
///
/// # 参数
/// * `items` - 之前的搜索结果
/// * `condition` - 模糊搜索条件
/// * `processed_counter` - 已处理计数器（可选）
/// * `total_found_counter` - 找到总数计数器（可选）
/// * `update_progress` - 进度更新回调
/// * `check_cancelled` - 取消检查闭包（可选）
///
/// # 返回
/// 返回满足条件的结果项（包含新值，有序）
pub(crate) fn fuzzy_refine_search<P, F>(
    items: &Vec<FuzzySearchResultItem>,
    condition: FuzzyCondition,
    processed_counter: Option<&Arc<AtomicUsize>>,
    total_found_counter: Option<&Arc<AtomicUsize>>,
    update_progress: &P,
    check_cancelled: Option<&F>,
) -> Result<BPlusTreeSet<FuzzySearchResultItem>>
where
    P: Fn(usize, usize) + Sync,
    F: Fn() -> bool + Sync,
{
    if items.is_empty() {
        return Ok(BPlusTreeSet::new(BPLUS_TREE_ORDER));
    }

    let total_items = items.len();

    let batches = cluster_addresses(items);

    if log_enabled!(Level::Debug) {
        debug!(
            "Fuzzy refine: {} items -> {} batches (avg {:.1} items/batch)",
            items.len(),
            batches.len(),
            items.len() as f64 / batches.len() as f64
        );
    }

    let items_with_current_value = parallel_batch_read(&batches, items, processed_counter, total_found_counter, update_progress, check_cancelled)?;

    if log_enabled!(Level::Debug) {
        debug!("Fuzzy refine: read {} / {} items successfully", items_with_current_value.len(), total_items);
    }

    let cancelled = Arc::new(AtomicBool::new(false));
    let cancelled_clone = Arc::clone(&cancelled);

    let matched: Vec<FuzzySearchResultItem> = items_with_current_value
        .par_iter()
        .take_any_while(|_| {
            if cancelled_clone.load(Ordering::Relaxed) {
                return false;
            }
            if let Some(check_fn) = check_cancelled {
                if check_fn() {
                    cancelled_clone.store(true, Ordering::Relaxed);
                    return false;
                }
            }
            true
        })
        .filter_map(|(old_item, current_value)| {
            if old_item.matches_condition(current_value, condition) {
                if let Some(counter) = total_found_counter {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
                Some(FuzzySearchResultItem::from_bytes(old_item.address, current_value, old_item.value_type))
            } else {
                None
            }
        })
        .collect();

    let mut results = BPlusTreeSet::new(BPLUS_TREE_ORDER);
    for item in matched {
        results.insert(item);
    }

    if log_enabled!(Level::Debug) {
        debug!("Fuzzy refine: checked {} items, found {} matches", items.len(), results.len());
    }

    // 最终更新进度到 100%
    if let Some(counter) = total_found_counter {
        counter.store(results.len(), Ordering::Relaxed);
    }
    update_progress(total_items, results.len());

    Ok(results)
}
