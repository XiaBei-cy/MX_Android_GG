# Mamu

**Advanced Android Memory Debugging and Manipulation Tool**

[![Version](https://img.shields.io/badge/version-1.0-blue.svg)](https://github.com/fuqiuluo/mamu)
[![License](https://img.shields.io/badge/license-GPLv3-green.svg)](./LICENSE)
[![Platform](https://img.shields.io/badge/platform-Android%207.0%2B-orange.svg)](https://developer.android.com)
[![Architecture](https://img.shields.io/badge/arch-ARM64--v8a-red.svg)](https://developer.android.com/ndk/guides/abis)
[![Language](https://img.shields.io/badge/language-Kotlin%20%2B%20Rust-purple.svg)](https://kotlinlang.org)

[中文文档](./README.zh-CN.md) | **English**

---

> **⚠️ CRITICAL SECURITY NOTICES**
>
> - **ROOT ACCESS REQUIRED**: This tool requires privileged system access
> - **EDUCATIONAL USE ONLY**: For research, learning, and authorized security testing purposes only
> - **LEGAL RESPONSIBILITY**: Users assume all liability for the use of this software
> - **DETECTION RISK**: May be detected by anti-cheat systems and security mechanisms
> - **NO WARRANTY**: Provided AS-IS without any warranty or guarantee
> - **USE RESPONSIBLY**: Respect software licenses, terms of service, and applicable laws

---

## Introduction

**Mamu** is a powerful Android memory debugging and manipulation tool designed for educational research, security testing, and reverse engineering purposes. Similar to tools like GameGuardian, Mamu provides real-time process memory search, modification, and debugging capabilities with a modern, user-friendly interface.

Built with a hybrid architecture combining **Kotlin/Jetpack Compose** for the UI layer and **Rust** for performance-critical native operations, Mamu delivers optimal performance while maintaining code safety and efficiency. The tool leverages Linux `ptrace` system calls for memory access and supports multiple search algorithms and memory access modes.

Whether you're a security researcher, reverse engineer, or curious developer exploring Android internals, Mamu provides a comprehensive toolkit for understanding and manipulating process memory in real-time.

## Key Features

### Core Capabilities

- **Real-Time Memory Search & Modification**
  - Exact value search (integers, floats, doubles, text, hex)
  - Fuzzy search for unknown values
  - Refined search to narrow down results
  - Batch modification and value freezing

- **Floating Window Interface**
  - **Search Tab**: Multiple search algorithms with real-time progress
  - **Settings Tab**: Process binding, memory range configuration, detection evasion
  - **Saved Addresses Tab**: Bookmark and manage memory locations
  - **Memory Preview Tab**: Hex dump visualization with ASCII representation
  - **Breakpoints Tab**: Debugging breakpoints and watchpoints

- **Advanced Memory Access**
  - Multiple access modes: NORMAL, WRITETHROUGH, NOCACHE, PGFAULT
  - Permission-based memory region filtering
  - Automatic retry on access failures
  - Memory region mapping and analysis

- **Process Management**
  - ptrace-based process attachment
  - Real-time process death monitoring
  - Detailed process information (PID, name, memory maps)
  - Safe detachment and cleanup

### Technical Highlights

- **Custom JNI Macro Framework**: Seamless Kotlin-Rust integration with priority-based initialization
- **B+ Tree Result Indexing**: Efficient storage and retrieval of large search result sets
- **Parallel Search Algorithms**: Multi-threaded search using Rayon for optimal performance
- **String Obfuscation**: Anti-detection through compile-time string obfuscation (obfstr)
- **MMKV Configuration**: High-performance key-value storage for app settings
- **Material Design 3**: Modern, adaptive UI with 11 theme variants
- **Aggressive Size Optimization**: LTO, symbol stripping, and size optimization for minimal APK size

## Screenshots

> **Note**: Screenshots will be added in future releases

The app features:
- Modern Material Design 3 UI with dynamic theming
- Responsive floating window overlay with drag-and-drop positioning
- Five specialized tabs for different memory operations
- Process selection dialog with detailed system information
- Memory search interface with real-time progress indicators
- Hex memory viewer with selection highlighting and operations

## System Requirements

### Device Requirements

| Requirement | Specification |
|------------|---------------|
| **Operating System** | Android 7.0+ (API 24+) |
| **Architecture** | ARM64-v8a (aarch64) only |
| **Root Access** | Required (Magisk, KernelSU, or SuperSU) |
| **SELinux** | Permissive mode recommended for full functionality |
| **Storage** | ~15 MB for app installation |

### Build Requirements

| Requirement | Version |
|------------|---------|
| **Android Studio** | Latest stable (2024.1+) |
| **Android SDK** | compileSdk 36, targetSdk 35 |
| **Android NDK** | Latest stable with LLVM toolchain |
| **Rust** | Latest stable (1.70+) |
| **Gradle** | 8.13+ |
| **JDK** | Java 11 |

### Required Permissions

- `QUERY_ALL_PACKAGES` - Package visibility (auto-granted via root)
- `SYSTEM_ALERT_WINDOW` - Floating window overlay
- `FOREGROUND_SERVICE` - Background service operation
- `FOREGROUND_SERVICE_SPECIAL_USE` - Android 14+ requirement
- `POST_NOTIFICATIONS` - Notification display (Android 13+)
- `INTERNET` - Optional driver download functionality
- **Root Access** - Core requirement for ptrace and memory operations

## Installation

### Option A: Download Pre-built APK

> **Coming Soon**: Pre-built APK releases will be available on the [Releases](../../releases) page

1. Download the latest APK from the releases page
2. Enable "Install from Unknown Sources" in Android settings
3. Install the APK on your rooted device
4. Launch the app and grant root access when prompted
5. Allow all required permissions (most are auto-granted via root)

### Option B: Build from Source

See the [Building from Source](#building-from-source) section below for detailed instructions.

## Quick Start Guide

### First Launch

1. **Permission Setup**
   - Launch the app (opens `PermissionSetupActivity`)
   - Grant root access in the Magisk/KernelSU authorization dialog
   - App automatically grants `QUERY_ALL_PACKAGES` and `SYSTEM_ALERT_WINDOW` permissions
   - System information and driver status are displayed
   - Automatic redirect to `MainActivity` on successful setup

2. **Enable Floating Window**
   - Review system information and driver status on the home screen
   - Tap the floating window toggle button
   - Grant overlay permission if prompted
   - Floating window appears with notification indicator

3. **Bind to Target Process**
   - Open the **Settings** tab in the floating window
   - Tap "Select Debug Process" button
   - Choose your target application from the process list
   - App performs `ptrace(ATTACH)` and begins monitoring

4. **Search Memory**
   - Switch to the **Search** tab
   - Select data type (Auto, DWORD, Float, Double, QWORD, etc.)
   - Enter the search value
   - Tap "Search" to perform exact value search
   - Review results with pagination
   - Use "Refine" for subsequent searches to narrow down results

5. **Modify Values**
   - Long-press a search result to open the modification dialog
   - Enter the new value
   - Tap confirm to write to memory
   - Option to "freeze" the value for continuous updates

## Building from Source

### Prerequisites

#### Install Rust Toolchain

```bash
# Linux/macOS/WSL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows (use rustup-init.exe from https://rustup.rs)
```

#### Add Android Target

```bash
rustup target add aarch64-linux-android
```

#### Set Environment Variables

**Linux/macOS:**
```bash
export ANDROID_SDK_ROOT=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_SDK_ROOT/ndk/<version>

# Example:
# export ANDROID_NDK_HOME=$ANDROID_SDK_ROOT/ndk/26.1.10909125
```

**Windows (PowerShell):**
```powershell
$env:ANDROID_SDK_ROOT = "C:\Users\<YourUsername>\AppData\Local\Android\Sdk"
$env:ANDROID_NDK_HOME = "$env:ANDROID_SDK_ROOT\ndk\<version>"
```

**Windows (Command Prompt):**
```cmd
set ANDROID_SDK_ROOT=C:\Users\<YourUsername>\AppData\Local\Android\Sdk
set ANDROID_NDK_HOME=%ANDROID_SDK_ROOT%\ndk\<version>
```

### Build Steps

#### Clone the Repository

```bash
git clone https://github.com/fuqiuluo/mamu.git
cd mamu
```

#### Build Debug APK

```bash
# Automatically compiles Rust library and builds APK
./gradlew assembleDebug

# Output: app/build/outputs/apk/debug/app-debug.apk
```

#### Build Release APK

```bash
./gradlew assembleRelease

# Output: app/build/outputs/apk/release/app-release.apk
# Note: Requires signing configuration in build.gradle.kts
```

#### Install to Device

```bash
# Install debug build via ADB
./gradlew installDebug

# Or manually
adb install app/build/outputs/apk/debug/app-debug.apk
```

### Manual Rust Build (Optional)

The Gradle build system automatically compiles the Rust library via the `buildRustAndroid` task. Manual compilation is rarely needed but can be done:

```bash
cd app/src/main/rust

# Debug build
cargo build --target aarch64-linux-android

# Release build (with optimizations)
cargo build --target aarch64-linux-android --release
```

**Build Flow**: The Gradle build automatically triggers `buildRustAndroid → copyRustLibs → preBuild`, ensuring the Rust library is compiled and copied to `app/src/main/jniLibs/arm64-v8a/libmamu_core.so` before the Android build begins.

### Troubleshooting Build Issues

| Issue | Solution |
|-------|----------|
| **NDK not found** | Set `ANDROID_NDK_HOME` environment variable or install NDK via SDK Manager |
| **Rust target missing** | Run `rustup target add aarch64-linux-android` |
| **Linker errors** | Ensure NDK version has LLVM toolchain (r21+) |
| **Permission denied on gradlew** | Run `chmod +x gradlew` (Linux/macOS) |

## Architecture Overview

Mamu uses a layered hybrid architecture combining Kotlin for UI/presentation and Rust for performance-critical operations.

### Layer Diagram

```
┌─────────────────────────────────────────────────────────┐
│          UI Layer (Jetpack Compose + ViewBinding)       │
│   - Compose screens (HomeScreen, dialogs)              │
│   - XML layouts (FloatingWindow, custom keyboard)      │
├─────────────────────────────────────────────────────────┤
│      Presentation Layer (Controllers + ViewModels)      │
│   - MainViewModel (StateFlow for home screen)          │
│   - FloatingControllers (Search, Settings, etc.)       │
│   - FloatingWindowStateManager (service state sync)    │
├─────────────────────────────────────────────────────────┤
│        Data/Repository Layer (DataSources)              │
│   - SystemDataSource (root, SELinux, system info)      │
│   - DriverDataSource (native driver status)            │
│   - MMKV (configuration persistence)                   │
├─────────────────────────────────────────────────────────┤
│         JNI Bridge Layer (Kotlin Facades)               │
│   - WuwaDriver (process mgmt, ptrace, memory regions)  │
│   - SearchEngine (memory search algorithms)            │
│   - MemoryOps (memory read/write operations)           │
├─────────────────────────────────────────────────────────┤
│         Native Layer (Rust: libmamu_core.so)            │
│   - driver_manager.rs (process attachment, ptrace)     │
│   - search/engine/* (parallel search algorithms)       │
│   - memory_mode.rs (access mode management)            │
│   - jni_interface/* (JNI bindings with macros)         │
│   - bplustree (result indexing)                        │
├─────────────────────────────────────────────────────────┤
│           Kernel APIs (Linux System Calls)              │
│   - ptrace (PTRACE_ATTACH, PTRACE_PEEKDATA, etc.)      │
│   - /proc filesystem (maps, stat, cmdline)             │
│   - mmap (memory-mapped I/O)                           │
│   - socket (optional kernel driver communication)      │
└─────────────────────────────────────────────────────────┘
```

### Key Components

#### Activities
- **PermissionSetupActivity**: Entry point; validates root access, grants permissions, checks SELinux status
- **MainActivity**: Primary UI showing system status, driver information, and floating window toggle
- **DriverInstallActivity**: Driver installation flow (minimal implementation)

#### Services
- **FloatingWindowService**: Foreground service managing the overlay UI with 5 tab-based controllers
- **RootFileSystemService**: Root file system operations

#### Controllers (Floating Window)
- **SearchController**: Search UI, result pagination, filtering, and modification
- **SettingsController**: Process binding, memory access configuration, hiding modes
- **SavedAddressController**: Bookmark management for memory addresses
- **MemoryPreviewController**: Hex dump visualization with byte selection
- **BreakpointController**: Debugging breakpoint management

All controllers extend `FloatingController<T>` for lifecycle management and shared service access.

#### State Management
- **FloatingWindowStateManager**: Singleton using `MutableStateFlow` for reactive state synchronization
- **MainViewModel**: MVVM pattern for home screen state

#### JNI Bridges (Kotlin → Rust)
- **WuwaDriver**: Process management, memory region queries, ptrace operations
- **SearchEngine**: Memory search algorithms (exact/fuzzy/refine), result retrieval
- **MemoryOps**: Direct memory read/write operations

#### Rust Native Library (`app/src/main/rust`)
- **Custom JNI Macro Framework**: Declarative JNI bindings using `#[jni_method]` attribute macro
- **B+ Tree Implementation**: Efficient result indexing for large search result sets
- **Parallel Search**: Multi-threaded search using Rayon for CPU parallelism
- **String Obfuscation**: Compile-time obfuscation using `obfstr` crate

### Data Flow Example: Memory Search

```
1. User enters value in SearchDialog (Compose UI)
   ↓
2. SearchController.onSearchCompleted() invoked
   ↓
3. Call SearchEngine.searchExact(value, type, ranges) [JNI → Rust]
   ↓
4. Rust queries WuwaDriver.queryMemRegions() for target regions
   ↓
5. Native search runs with SearchProgressCallback for UI updates
   ↓
6. Results stored in B+ tree, returns result count
   ↓
7. SearchController.loadSearchResults() fetches paginated results
   ↓
8. SearchEngine.getResults(start, count) retrieves chunk [Rust → Kotlin]
   ↓
9. SearchResultAdapter displays results in RecyclerView
```

For detailed architecture documentation, see [CLAUDE.md](./CLAUDE.md).

## Technology Stack

### Android/Kotlin Stack

| Technology | Version | Purpose |
|-----------|---------|---------|
| **Kotlin** | 2.2.21 | Primary Android app language |
| **Jetpack Compose** | 2025.12.00 BOM | Modern declarative UI framework |
| **Material Design 3** | Latest | UI components and theming |
| **Coroutines** | 1.10.2 | Async/concurrent programming |
| **StateFlow** | (Kotlin std) | Reactive state management |
| **ViewBinding** | (AGP) | Type-safe view access for XML layouts |
| **MMKV** | 2.3.0 | High-performance key-value storage (Tencent) |
| **libsu** | 6.0.0 | Root shell library (by Magisk author) |
| **RecyclerView** | 1.4.0 | Efficient list displays |

### Rust Stack

| Crate | Version | Purpose |
|-------|---------|---------|
| **jni** | 0.21.1 | Java Native Interface bindings |
| **nix** | 0.30.1 | Unix system calls (ptrace, socket, mman) |
| **tokio** | 1.x | Async runtime (full features) |
| **rayon** | 1.11.0 | Data parallelism for search |
| **memmap2** | 0.9 | Memory-mapped I/O |
| **obfstr** | 0.4.4 | Compile-time string obfuscation |
| **capstone** | 0.13.0 | Disassembly engine |
| **reqwest** | 0.12.24 | HTTP client (rustls-tls) |
| **jni-macro** | Custom | Custom JNI attribute macro framework |
| **bplustree** | Custom | Custom B+ tree implementation |

### Build Tools

- **Gradle**: 8.13.1
- **Android Gradle Plugin**: 8.13.1
- **Cargo**: Rust package manager
- **rustc**: Rust compiler

## Usage Examples

### Example 1: Exact Value Search (Game Coins)

**Scenario**: Find and modify a game's coin count displaying "1234"

```
1. Bind to game process
   - Open Settings tab → "Select Debug Process"
   - Choose game from process list
   - Wait for "Process bound successfully" message

2. Initial search
   - Switch to Search tab
   - Select data type: "DWORD" (4 bytes)
   - Enter value: "1234"
   - Tap "Search" button
   - Results found: 156 addresses

3. Refine search
   - Spend coins in-game (new value: 1189)
   - Tap "Refine" button
   - Enter new value: "1189"
   - Results narrowed to: 3 addresses

4. Modify value
   - Long-press each result
   - Select "Modify" from context menu
   - Enter new value: "9999"
   - Tap "Confirm"
   - Return to game → coin count updated to 9999
```

### Example 2: Fuzzy Search (Unknown Changing Values)

**Scenario**: Find a value that's changing but you don't know the exact number

```
1. Start fuzzy search
   - Bind to target process
   - Switch to Search tab
   - Tap "Fuzzy Search" button
   - Select data type: "Float"
   - Tap "Start Scan" (records baseline of all float values)
   - Initial scan: 2,340,567 values recorded

2. Refine by changes
   - Perform action in-game that increases the value
   - Tap "Refine" → select condition "Increased"
   - Results: 45,678 addresses

3. Continue refining
   - Perform action again
   - Tap "Refine" → "Increased"
   - Results: 3,421 addresses
   - Repeat until < 100 results

4. Switch to exact search
   - Note the current value in-game (e.g., 75.5)
   - Tap "Exact Search" tab
   - Enter "75.5"
   - Results: 2 addresses
   - Test modification on each to find the correct one
```

### Example 3: Memory Preview (Hex Dump Analysis)

**Scenario**: Examine memory around a found address

```
1. Navigate to address
   - Find address via search (e.g., 0x7AB3C4F000)
   - Long-press result
   - Select "Go to Memory Preview"

2. View hex dump
   - Memory Preview tab opens at that address
   - Hex bytes displayed on left: "4D 61 6D 75 00 01 02 03..."
   - ASCII representation on right: "Mamu...."
   - Current address highlighted

3. Select and operate
   - Tap to select byte range
   - Long-press for operations menu:
     - Copy (hex or ASCII)
     - Export to file
     - Modify bytes
     - Save as bookmark
```

## Development Guide

### Adding New Features

#### Kotlin Side

1. **Add Model Classes** (`app/src/main/java/moe/fuqiuluo/mamu/data/model/`)
   ```kotlin
   data class MemoryRegion(
       val startAddress: Long,
       val endAddress: Long,
       val permissions: String,
       val pathname: String
   )
   ```

2. **Add UI Components**
   - Compose: Create `@Composable` functions in `ui/screen/` or `ui/component/`
   - ViewBinding: Add XML layouts in `res/layout/` and bind in controllers

3. **Add ViewModel State** (if needed)
   ```kotlin
   class MyViewModel : ViewModel() {
       private val _state = MutableStateFlow(MyState())
       val state: StateFlow<MyState> = _state.asStateFlow()

       fun performAction() {
           viewModelScope.launch {
               // Call JNI bridge
               val result = WuwaDriver.myNativeMethod()
               _state.update { it.copy(result = result) }
           }
       }
   }
   ```

4. **Call JNI Bridge**
   ```kotlin
   val result = WuwaDriver.bindProcess(pid)
   val searchResults = SearchEngine.searchExact(value, type, ranges)
   ```

#### Rust Side

1. **Implement Core Logic** (`app/src/main/rust/src/core/` or `src/search/`)
   ```rust
   pub fn my_feature(param: i32) -> bool {
       // Implementation
       log::debug!("my_feature called with param: {}", param);
       true
   }
   ```

2. **Add JNI Binding** (`app/src/main/rust/src/jni_interface/`)
   ```rust
   use jni_macro::jni_method;

   #[jni_method(
       90,
       "moe/fuqiuluo/mamu/driver/MyClass",
       "nativeMyMethod",
       "(I)Z"
   )]
   pub fn jni_my_method(
       mut env: JNIEnv,
       obj: JObject,
       param: jint
   ) -> jboolean {
       match my_feature(param as i32) {
           true => JNI_TRUE,
           false => JNI_FALSE,
       }
   }
   ```

3. **Update Global State** (if needed in `src/core/globals.rs`)

4. **Rebuild**
   ```bash
   ./gradlew assembleDebug  # Automatically rebuilds Rust
   ```

### Code Conventions

- **Kotlin**: Follow existing patterns, use `StateFlow` for reactive UI, coroutines for async
- **Rust**: Use `#[jni_method]` macro for JNI, `log::debug!()` for logging, `obfstr!()` for sensitive strings
- **Always check existing imports** before adding new dependencies
- **Follow CLAUDE.md** for detailed development guidelines

## Debugging

### Kotlin Debugging

**Android Studio Debugger:**
- Set breakpoints in Kotlin code
- Run app in debug mode (Shift+F9)
- Use "Attach to Process" for running app

**LogCat Filtering:**
```bash
# Filter by package name
adb logcat | grep -E "moe.fuqiuluo.mamu"

# Filter by tag
adb logcat | grep -E "MamuApp|SearchController|SettingsController"

# Clear logs
adb logcat -c
```

### Rust Debugging

**Enable Native Logging:**
```bash
# Create marker file to enable Rust logging
adb shell "touch /data/user/0/moe.fuqiuluo.mamu/files/log_enable"

# Restart app to activate logging
adb shell am force-stop moe.fuqiuluo.mamu
```

**View Rust Logs:**
```bash
# Filter Rust library logs
adb logcat | grep mamu_core

# View all logs with native stack traces
adb logcat -v threadtime
```

**LLDB Debugging (Advanced):**
- Use Android NDK's lldb-server for native debugging
- Set breakpoints in Rust code
- Requires debug build (`cargo build` without `--release`)

### Common Issues

| Issue | Cause | Solution |
|-------|-------|----------|
| **NDK not found** | ANDROID_NDK_HOME not set | Set environment variable or install via SDK Manager |
| **Rust target missing** | aarch64-linux-android target not installed | Run `rustup target add aarch64-linux-android` |
| **ptrace fails** | SELinux enforcing mode | Run `adb shell setenforce 0` or check root access |
| **Service detected** | Target app queries installed services | Use hiding modes in Settings tab |
| **Build fails** | Gradle/Cargo cache corruption | Run `./gradlew clean && cargo clean` |

## Testing

### Kotlin Unit Tests

```bash
# Run all unit tests
./gradlew test

# Run specific test class
./gradlew test --tests "moe.fuqiuluo.mamu.MyTestClass"

# View test report
open app/build/reports/tests/testDebugUnitTest/index.html
```

### Android Instrumented Tests

```bash
# Requires connected device or emulator
./gradlew connectedAndroidTest

# Run specific test
./gradlew connectedAndroidTest -Pandroid.testInstrumentationRunnerArguments.class=moe.fuqiuluo.mamu.MyInstrumentedTest
```

### Rust Unit Tests

```bash
cd app/src/main/rust

# Run all tests (host machine, not Android)
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

**Note**: Rust tests run on the host machine, not on Android devices. For Android-specific testing, use Kotlin instrumented tests that call JNI methods.

## Security Considerations

### Critical Security Warnings

> **⚠️ PRIVILEGED ACCESS RISKS**
>
> - This tool requires **root access** and **ptrace capabilities**, granting it access to all process memory
> - Improper use can cause **system instability**, **data loss**, or **security vulnerabilities**
> - **Never use** on production devices or devices containing sensitive data
> - **Always test** on dedicated development/testing devices

### Detection Risks

Target applications may detect Mamu through:

| Detection Method | Risk Level | Mitigation |
|-----------------|------------|------------|
| **Service Query** | High | FloatingWindowService visible via `queryIntentServices()` |
| **ptrace Detection** | Medium | Apps can check `/proc/self/status` for TracerPid |
| **Filesystem Scanning** | Medium | Package name and files are visible in `/data/app/` |
| **Root Detection** | Low | Standard root hiding methods apply |

### Use Cases (Authorized Only)

**Legitimate Uses:**
- Educational research and learning about Android internals
- Authorized security testing and penetration testing
- Reverse engineering for security research
- Personal experimentation on owned devices
- CTF competitions and security challenges

**Prohibited Uses:**
- Circumventing security mechanisms in production apps
- Violating terms of service or software licenses
- Cheating in online multiplayer games
- Unauthorized access to protected data
- Any illegal or unethical activities

### Legal Responsibility

> **By using this software, you acknowledge:**
>
> - You have **authorized access** to the target application and device
> - You understand and accept **all legal risks** and responsibilities
> - You will **not use** this tool for illegal purposes
> - The developers assume **no liability** for misuse or damages
> - You will comply with all **applicable laws and regulations**

### Anti-Detection Features

Mamu includes several anti-detection mechanisms:

- **String Obfuscation**: Sensitive strings obfuscated at compile-time using `obfstr`
- **Conditional Logging**: Native logging only enabled when marker file exists
- **Hiding Modes**: 4 hiding modes available in Settings tab
- **Package Rename**: Can be recompiled with custom package name
- **Optional Kernel Driver**: Advanced memory access beyond ptrace (requires separate kernel module)

## Roadmap

### Planned Features

- [ ] **Group Search**: Search for related values in memory (e.g., X, Y coordinates)
- [ ] **Offset Calculator**: Calculate offsets between related addresses
- [ ] **Search History**: Save and replay previous search sessions
- [ ] **Base Conversion Tools**: Hex/Decimal/Binary/Octal converter
- [ ] **Script Automation**: Lua scripting support for automated operations
- [ ] **Multi-Architecture**: Support for arm, x86, x86_64 (beyond ARM64)
- [ ] **Enhanced Anti-Detection**: Additional stealth mechanisms
- [ ] **CSV Import/Export**: Batch import/export saved addresses
- [ ] **Multi-Language UI**: Full English translation and internationalization
- [ ] **Kernel Driver Integration**: Enhanced kernel module support
- [ ] **Memory Diff**: Compare memory snapshots over time
- [ ] **Speed Hack**: Modify game speed (time manipulation)

### Under Consideration

- Assembly code injection
- Pointer chain scanning
- Automated pattern recognition
- Cloud sync for saved addresses
- Plugin system for extensions

## FAQ

### General Questions

**Q: Why does Mamu require root access?**

A: Memory debugging requires `ptrace` system calls and direct access to `/proc` filesystem, both of which are privileged operations restricted to root. Additionally, granting `QUERY_ALL_PACKAGES` permission programmatically requires root.

**Q: What Android versions are supported?**

A: Mamu supports Android 7.0 (API 24) and above, tested up to Android 14 (API 34). Some features may require specific Android versions (e.g., `FOREGROUND_SERVICE_SPECIAL_USE` on Android 14+).

**Q: Why is only ARM64-v8a architecture supported?**

A: Modern Android devices (2017+) predominantly use ARM64. Supporting additional ABIs (arm, x86, x86_64) requires compiling Rust for multiple targets and extensive testing. This may be added in future releases.

**Q: Can target applications detect Mamu?**

A: Yes. Applications can detect Mamu through service queries, ptrace detection, filesystem scanning, or root detection methods. Use the hiding modes in the Settings tab and consider recompiling with a custom package name for better stealth.

### Technical Questions

**Q: How does Mamu differ from GameGuardian?**

A: While both are memory manipulation tools, Mamu features:
- Modern Kotlin/Compose UI vs. older Android UI
- Rust native library vs. C/C++
- Open source (GPLv3) vs. closed source
- Custom JNI macro framework
- B+ tree result indexing for better performance

**Q: What is the custom JNI macro framework?**

A: Mamu includes a custom Rust attribute macro (`#[jni_method]`) that simplifies JNI bindings by automatically generating boilerplate code and providing priority-based initialization. See `app/src/main/rust/jni-macro/README.md`.

**Q: Why is the APK size so small?**

A: Aggressive Rust release optimizations including:
- LTO (Link Time Optimization)
- Size optimization (`opt-level = "z"`)
- Symbol stripping
- Dead code elimination
- Single architecture (ARM64 only)

**Q: How do I enable Rust logging?**

A: Create a marker file: `adb shell "touch /data/user/0/moe.fuqiuluo.mamu/files/log_enable"`, then restart the app. Logs appear in LogCat filtered by "mamu_core".

### Legal & Safety Questions

**Q: Is using Mamu legal?**

A: Legality depends on jurisdiction and use case. Using Mamu on your own device for educational purposes is generally legal. Using it to circumvent security, violate ToS, or gain unfair advantages may be illegal. **Consult local laws and seek legal advice if uncertain.**

**Q: Can I use Mamu for game modding?**

A: Only on single-player games or your own private servers with permission. Using Mamu for online multiplayer games violates terms of service and may result in bans. It's also unethical and potentially illegal.

**Q: What if I brick my device?**

A: While Mamu includes safety measures, improper use of memory manipulation can cause app crashes or system instability. The developers assume **no liability** for any damages. **Always backup** and test on non-critical devices.

## Contributing

We welcome contributions from the community! Whether it's bug fixes, new features, documentation improvements, or translations, your help is appreciated.

### How to Contribute

1. **Fork the Repository**
   ```bash
   # Click "Fork" on GitHub, then clone your fork
   git clone https://github.com/your-username/mamu.git
   cd mamu
   ```

2. **Create a Feature Branch**
   ```bash
   git checkout -b feature/my-new-feature
   # or
   git checkout -b fix/issue-123
   ```

3. **Make Your Changes**
   - Follow existing code conventions (see [CLAUDE.md](./CLAUDE.md))
   - Write clear, descriptive commit messages
   - Test thoroughly on a physical rooted device
   - Add tests for new features

4. **Commit Your Changes**
   ```bash
   git add .
   git commit -m "feat: add group search functionality"
   # or
   git commit -m "fix: resolve crash on process detachment"
   ```

5. **Push to Your Fork**
   ```bash
   git push origin feature/my-new-feature
   ```

6. **Submit a Pull Request**
   - Go to the original repository on GitHub
   - Click "New Pull Request"
   - Select your branch
   - Provide a detailed description of your changes

### Code Style Guidelines

**Kotlin:**
- Follow existing patterns and naming conventions
- Use `StateFlow` for reactive UI state management
- Use `viewModelScope` or component-scoped `CoroutineScope(SupervisorJob())`
- Add KDoc comments for public APIs

**Rust:**
- Use `#[jni_method]` macro for all JNI exports
- Use `log::debug!()`, `log::info!()`, `log::error!()` for logging
- Use `obfstr!()` for sensitive string literals
- Follow Rust naming conventions (snake_case)
- Add rustdoc comments for public functions

**General:**
- Check existing imports and dependencies before adding new ones
- Never commit secrets, keys, or sensitive data
- Follow security best practices
- Write self-documenting code; add comments only when logic isn't self-evident

### Areas for Contribution

- **Features**: Implement roadmap items (group search, offset calculator, etc.)
- **Performance**: Optimize search algorithms, reduce memory usage
- **UI/UX**: Improve user interface, add animations, enhance accessibility
- **Documentation**: Improve README, add tutorials, translate to other languages
- **Testing**: Add unit tests, instrumented tests, improve test coverage
- **Bug Fixes**: Fix reported issues on GitHub Issues
- **Anti-Detection**: Improve stealth mechanisms
- **Architecture Support**: Add arm, x86, x86_64 support

### Getting Help

- **Questions**: Use [GitHub Discussions](../../discussions)
- **Bug Reports**: Use [GitHub Issues](../../issues)
- **Development Chat**: (To be set up)

## Project Structure

```
mamu/
├── app/
│   ├── src/
│   │   ├── main/
│   │   │   ├── java/moe/fuqiuluo/mamu/
│   │   │   │   ├── data/
│   │   │   │   │   ├── local/           # DataSources (SystemDataSource, DriverDataSource)
│   │   │   │   │   └── model/           # Domain models (SystemInfo, DriverStatus, etc.)
│   │   │   │   ├── driver/              # JNI bridges (WuwaDriver, SearchEngine, MemoryOps)
│   │   │   │   ├── floating/
│   │   │   │   │   ├── adapter/         # RecyclerView adapters
│   │   │   │   │   ├── controller/      # FloatingWindow feature controllers
│   │   │   │   │   ├── service/         # FloatingWindowService
│   │   │   │   │   └── state/           # FloatingWindowStateManager
│   │   │   │   ├── ui/
│   │   │   │   │   ├── screen/          # Compose screens (HomeScreen, etc.)
│   │   │   │   │   ├── component/       # Reusable Compose components
│   │   │   │   │   ├── dialog/          # Dialog states and composables
│   │   │   │   │   └── viewmodel/       # ViewModels (MainViewModel, etc.)
│   │   │   │   ├── MamuApplication.kt   # Application class
│   │   │   │   ├── MainActivity.kt      # Main activity
│   │   │   │   └── PermissionSetupActivity.kt  # Permission setup activity
│   │   │   ├── rust/                    # Rust native library
│   │   │   │   ├── src/
│   │   │   │   │   ├── core/            # Driver manager, globals, memory modes
│   │   │   │   │   ├── jni_interface/   # JNI method bindings
│   │   │   │   │   ├── search/          # Search engine algorithms
│   │   │   │   │   ├── ext/             # Extension utilities
│   │   │   │   │   └── lib.rs           # Rust library entry point
│   │   │   │   ├── bplustree/           # Custom B+ tree crate
│   │   │   │   ├── jni-macro/           # Custom JNI macro framework
│   │   │   │   │   ├── jni-derive/      # Procedural macro implementation
│   │   │   │   │   ├── jni-facade/      # Facade API
│   │   │   │   │   └── README.md        # JNI macro documentation
│   │   │   │   ├── Cargo.toml           # Rust dependencies and build config
│   │   │   │   └── build.rs             # Rust build script
│   │   │   ├── res/                     # Android resources
│   │   │   │   ├── drawable/            # Vector drawables and icons
│   │   │   │   ├── layout/              # XML layouts (FloatingWindow, dialogs)
│   │   │   │   ├── mipmap-*/            # Launcher icons (multiple densities)
│   │   │   │   ├── values/              # Strings, colors, dimensions, styles, themes
│   │   │   │   └── values-zh-rCN/       # Chinese localization
│   │   │   ├── AndroidManifest.xml      # App manifest
│   │   │   └── jniLibs/                 # Compiled .so files (auto-generated)
│   │   │       └── arm64-v8a/
│   │   │           └── libmamu_core.so  # Rust native library
│   │   └── test/                        # Unit tests
│   ├── build.gradle.kts                 # App module build configuration
│   └── proguard-rules.pro               # ProGuard rules
├── gradle/
│   ├── libs.versions.toml               # Dependency versions (Gradle version catalog)
│   └── wrapper/                         # Gradle wrapper
├── .claude/
│   └── skills/                          # Claude Code skills
│       └── android-material-design-3/   # Material Design 3 skill resources
├── build.gradle.kts                     # Root project build configuration
├── settings.gradle.kts                  # Project settings
├── gradlew                              # Gradle wrapper script (Linux/macOS)
├── gradlew.bat                          # Gradle wrapper script (Windows)
├── CLAUDE.md                            # Developer documentation (architecture, guidelines)
├── README.md                            # This file (English)
├── README.zh-CN.md                      # Chinese documentation
└── LICENSE                              # GPLv3 license

```

For detailed structure and component descriptions, see [CLAUDE.md](./CLAUDE.md).

## Acknowledgments

### Built With

- [Kotlin](https://kotlinlang.org/) - Modern, concise programming language for Android
- [Jetpack Compose](https://developer.android.com/jetpack/compose) - Modern declarative UI framework
- [Rust](https://www.rust-lang.org/) - Systems programming language focused on safety and performance
- [MMKV](https://github.com/Tencent/MMKV) - High-performance key-value storage by Tencent
- [libsu](https://github.com/topjohnwu/libsu) - Root shell library by the author of Magisk
- [Rayon](https://github.com/rayon-rs/rayon) - Data parallelism library for Rust
- [nix](https://github.com/nix-rust/nix) - Rust-friendly Unix system call bindings
- [Capstone](https://www.capstone-engine.org/) - Disassembly framework
- [Material Design 3](https://m3.material.io/) - Google's design system

### Inspired By

- **GameGuardian** - Popular Android memory manipulation tool
- **Cheat Engine** - PC memory scanner and debugger

### Special Thanks

- **fuqiuluo** - Original developer and primary contributor
- The Magisk community for root access infrastructure
- The Rust community for excellent tooling and libraries
- Android open-source community

## License

This project is licensed under the **GNU General Public License v3.0 (GPLv3)**.

**Key Points:**
- ✅ You can freely use, modify, and distribute this software
- ✅ Source code must be made available when distributing
- ✅ Derivative works must also be licensed under GPLv3
- ✅ Changes must be documented
- ❌ No warranty or liability
- ❌ Cannot be used in proprietary software

See the [LICENSE](./LICENSE) file for the full license text.

**Why GPLv3?** This strong copyleft license ensures that all derivatives of this educational/research tool remain open source and accessible to the community, preventing misuse in closed-source commercial applications.

## Contact & Support

### Reporting Issues

Found a bug? Please report it via [GitHub Issues](../../issues).

**When reporting, include:**
- Device information (model, Android version)
- Root method (Magisk version, KernelSU, etc.)
- Steps to reproduce the issue
- Expected vs. actual behavior
- Logcat output (if applicable)
- Screenshots (if UI-related)

**Enable Rust logging before reporting crashes:**
```bash
adb shell "touch /data/user/0/moe.fuqiuluo.mamu/files/log_enable"
adb logcat -c  # Clear old logs
# Reproduce the issue
adb logcat > logcat.txt  # Save logs to file
```

### Questions & Discussions

Have questions or want to discuss features? Use [GitHub Discussions](../../discussions).

**Discussion Categories:**
- General questions
- Feature requests
- Development help
- Show and tell
- Security research

### Stay Updated

- **GitHub Repository**: [fuqiuluo/mamu](https://github.com/fuqiuluo/mamu)
- **Release Notes**: See [Releases](../../releases) for changelog
- **Development Blog**: (Coming soon)

---

## Disclaimer

**THIS SOFTWARE IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND**, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES, OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT, OR OTHERWISE, ARISING FROM, OUT OF, OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

**By using Mamu, you acknowledge that:**

- You have read and understood this README and the LICENSE
- You accept all risks associated with memory manipulation
- You will use this tool responsibly and legally
- You understand that the developers assume no liability for any damages or legal consequences

**For educational and research purposes only. Use at your own risk.**

---

Made with ❤️ by the Mamu development team

**[⬆ Back to Top](#mamu)**
