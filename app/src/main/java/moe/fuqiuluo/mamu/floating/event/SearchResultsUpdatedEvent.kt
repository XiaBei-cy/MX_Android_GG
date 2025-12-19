package moe.fuqiuluo.mamu.floating.event

import moe.fuqiuluo.mamu.floating.data.model.DisplayMemRegionEntry

/**
 * 搜索结果更新事件（从保存地址界面搜索后发送）
 */
data class SearchResultsUpdatedEvent(
    val totalCount: Long,
    val ranges: List<DisplayMemRegionEntry>
)
