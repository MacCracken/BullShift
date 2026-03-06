# BullShift Code Audit Report

**Date:** March 5, 2026
**Version:** 2026.3.5
**Scope:** Full codebase (Rust backend, Flutter frontend, git repo, documentation, dependencies)

---

## Executive Summary

A comprehensive code audit identified **8 critical**, **10 high**, **12 medium**, and **6 low** severity issues across the Rust backend, Flutter frontend, git repository, and project documentation. Critical issues include build artifacts tracked in git (1.6GB), missing Rust dependencies preventing compilation, mutex poisoning risks in the database layer, and broken documentation links.

---

## 1. Git Repository Issues

### 1.1 Build Artifacts Tracked in Git (CRITICAL - FIXED)
- **1,569 files** from `rust/target/` were committed (commit `47e5b46`), bloating the repo to 1.8GB
- `.gitignore` already had `target/` but files were tracked before the rule was added
- **Fix applied:** `git rm -r --cached rust/target/`

### 1.2 Commit History Hygiene (LOW)
- Vague commit messages: "more fixes", "more updates", "more work fixing widgets"
- No feature branches — all development on `main`
- Only 1 git tag (`2026.2.16`) despite multiple releases
- **Recommendation:** Adopt conventional commits (`feat:`, `fix:`, `refactor:`), use feature branches, tag all releases

---

## 2. Rust Backend Findings

### 2.1 Missing `async-trait` Dependency (CRITICAL - FIXED)
- **File:** `rust/Cargo.toml`
- `#[async_trait]` used in `bullrunnr/mod.rs` but crate missing from dependencies
- **Fix applied:** Added `async-trait = "0.1"` to `[dependencies]`

### 2.2 Mutex Poisoning — 11 Instances (CRITICAL - FIXED)
- **File:** `rust/src/database/mod.rs`
- All database methods used `self.conn.lock().unwrap()` — panics if any thread previously panicked while holding the lock
- **Fix applied:** Replaced with `self.conn.lock().unwrap_or_else(|e| e.into_inner())` to recover from poisoned locks

### 2.3 NaN Panic in Float Sorting — 4 Locations (CRITICAL - FIXED)
- **Files:** `paper_hands/mod.rs:888`, `bullrunnr/mod.rs:186,365`, `trendsetter/mod.rs:311`
- `partial_cmp().unwrap()` on `f64` values panics if any value is NaN
- **Fix applied:** Changed to `unwrap_or(std::cmp::Ordering::Equal)`

### 2.4 API Server `serde_json` Unwraps (CRITICAL - FIXED)
- **File:** `rust/src/bin/api_server.rs` (lines 92, 105, 120)
- `serde_json::to_value().unwrap()` in HTTP handlers — crashes server on serialization failure
- **Fix applied:** Replaced with match expressions returning `500 INTERNAL_SERVER_ERROR`

### 2.5 Hardcoded API Key Placeholders (CRITICAL - FIXED)
- **File:** `rust/src/ai_bridge/mod.rs` (lines 398, 482)
- Hardcoded `"YOUR_API_KEY"` strings instead of reading from provider config
- **Fix applied:** Now reads from `provider.api_key`

### 2.6 Unused `ffi` Crate (MEDIUM - FIXED)
- **File:** `rust/Cargo.toml`
- `ffi = "0.1"` listed but never imported anywhere
- **Fix applied:** Removed

### 2.7 Overly Broad Tokio Features (MEDIUM - FIXED)
- **File:** `rust/Cargo.toml`
- `features = ["full"]` enables ~60+ features; only a subset needed
- **Fix applied:** Changed to specific features: `["rt-multi-thread", "macros", "sync", "time", "net", "io-util"]`

### 2.8 `tonic-build` in Wrong Section (MEDIUM - FIXED)
- **File:** `rust/Cargo.toml`
- Was in `[dev-dependencies]` but needed in `[build-dependencies]` for proto compilation
- **Fix applied:** Moved to `[build-dependencies]`

### 2.9 WebSocket Thread Leak (HIGH - OPEN)
- **File:** `rust/src/data_stream/mod.rs:261-274`
- `thread::spawn` creates OS thread in a loop with no cleanup/shutdown mechanism
- **Recommendation:** Use `tokio::spawn` with a shutdown signal channel

### 2.10 Unbounded Collections (HIGH - OPEN)
- **Logging buffer:** `logging/mod.rs` — Vec grows to 1000 entries before flush, no memory limit
- **AI responses:** `ai_bridge/mod.rs:172` — all responses stored forever
- **Article cache:** `bullrunnr/mod.rs:97` — 1000 full articles with content
- **Active alerts:** `trendsetter/mod.rs:81` — no auto-cleanup or max size
- **Recommendation:** Add bounded buffers (circular/LRU) or periodic eviction

### 2.11 String Error Types Throughout (HIGH - OPEN)
- All public APIs use `Result<T, String>` instead of structured error types
- `thiserror` is already a dependency but unused
- **Recommendation:** Create `BullShiftError` enum using `thiserror::Error` derive

### 2.12 Excessive Cloning in Hot Paths (MEDIUM - OPEN)
- `trading/api.rs:80-86` — `get_headers()` creates new HashMap and clones credentials every call
- `bullrunnr/mod.rs:226-255` — clones full articles into cache
- **Recommendation:** Cache headers; use `Arc<NewsArticle>` for shared ownership

### 2.13 AI Bridge Code Duplication (MEDIUM - OPEN)
- **File:** `rust/src/ai_bridge/mod.rs` (863 lines)
- 5 provider methods (OpenAI, Anthropic, Ollama, LocalLLM, Custom) with ~95% duplicate boilerplate
- **Recommendation:** Extract generic `send_request()` with provider-specific JSON parsing

### 2.14 Security: Nonce Generation (MEDIUM - OPEN)
- **File:** `rust/src/security/mod.rs:266-282`
- Uses `assume_unique_for_key` with random nonces — nonce reuse breaks AES-GCM if RNG fails
- **Recommendation:** Use monotonic counter for nonce generation

---

## 3. Flutter Frontend Findings

### 3.1 Missing Import — Compilation Error (HIGH - FIXED)
- **File:** `flutter/lib/modules/watchlist/watchlist_provider.dart`
- Uses `debugPrint()` without importing `package:flutter/foundation.dart`
- **Fix applied:** Added missing import

### 3.2 Unsafe FFI Cast (HIGH - FIXED)
- **File:** `flutter/lib/services/rust_trading_engine.dart:73`
- Cast `symbolPtr.cast<Char>()` incompatible with struct expecting `Pointer<Utf8>`
- **Fix applied:** Removed unnecessary `.cast<Char>()` calls

### 3.3 Invalid `firstWhere` with Null (HIGH - FIXED)
- **File:** `flutter/lib/modules/paper_hands/paper_hands_provider.dart:181`
- `orElse: () => null` on non-nullable type — invalid Dart
- **Fix applied:** Replaced with `indexWhere()` + index check

### 3.4 Unused `riverpod` Dependency (HIGH - FIXED)
- **File:** `flutter/pubspec.yaml`
- `riverpod: ^2.4.9` imported but never used in any Dart file
- **Fix applied:** Removed from pubspec.yaml

### 3.5 Unmaintained `charts_flutter` (CRITICAL - OPEN)
- **File:** `flutter/pubspec.yaml`
- `charts_flutter: ^0.12.0` last updated 2019, no longer maintained
- **Recommendation:** Replace with `fl_chart` (actively maintained)

### 3.6 API Call in Consumer Builder (HIGH - OPEN)
- **File:** `flutter/lib/modules/core_trading/core_trading_view.dart:28-30`
- `marketDataProvider.loadSymbolData()` called inside `Consumer3.builder`
- Triggers unnecessary API calls on every widget rebuild
- **Recommendation:** Move to `didChangeDependencies` or use listener pattern

### 3.7 Advanced Charting God Class (MEDIUM - OPEN)
- **File:** `flutter/lib/widgets/advanced_charting_widget.dart` (2,491 lines)
- Single widget handles charting, drawing tools, indicators, multi-symbol comparison, real-time updates
- **Recommendation:** Decompose into: `CandlestickChart`, `VolumeChart`, `IndicatorChart`, `DrawingToolsPanel`

### 3.8 Resource Leaks (MEDIUM - OPEN)
- `DrawingToolManager` in `advanced_charting_widget.dart:43` — no `dispose()` method
- `DynamicLibrary.open()` in `rust_trading_engine.dart` — library handle never released
- **Recommendation:** Implement proper resource cleanup

### 3.9 Unsafe Type Casts (MEDIUM - OPEN)
- Multiple provider files use direct `as String`, `as double` casts on `Map<String, dynamic>` values
- **Files:** `trendsetter_provider.dart`, `bullrunnr_provider.dart`, `bearly_managed_provider.dart`
- **Recommendation:** Use safe type checking with `is` and `?` operators

### 3.10 Debug Print in Security Code (MEDIUM - OPEN)
- **File:** `flutter/lib/services/security_manager.dart` (lines 46, 100, 214)
- Uses `print()` which outputs to console in production — potential information disclosure
- **Recommendation:** Replace with conditional `debugPrint()` or remove

---

## 4. Documentation Issues

### 4.1 Broken README Links (HIGH - FIXED)
- 9 documentation links in README.md pointed to old paths after docs reorganization
- Files moved to `docs/guides/` and `docs/reference/` subdirectories
- **Fix applied:** Updated all links to correct paths

---

## 5. Build & Tooling

### 5.1 Build Script Issues (LOW - OPEN)
- `build.sh:65` — redundant `$?` check after `set -e` (error already causes exit)
- `build.sh:87` — uses deprecated `flutter packages pub run`; should be `dart run`

### 5.2 Versioning (FIXED)
- Created `VERSION` file as single source of truth
- Created `bump-version.sh` script to sync versions across VERSION, Cargo.toml, pubspec.yaml, README

---

## 6. Remediation Summary

| # | Issue | Severity | Status |
|---|-------|----------|--------|
| 1 | Build artifacts in git (1.6GB) | CRITICAL | FIXED |
| 2 | Missing `async-trait` in Cargo.toml | CRITICAL | FIXED |
| 3 | Mutex poisoning (11 instances) | CRITICAL | FIXED |
| 4 | NaN panic in float sorting (4 locations) | CRITICAL | FIXED |
| 5 | API server serde unwraps | CRITICAL | FIXED |
| 6 | Hardcoded API key placeholders | CRITICAL | FIXED |
| 7 | Broken README doc links (9 links) | HIGH | FIXED |
| 8 | Missing Flutter import (compilation) | HIGH | FIXED |
| 9 | Unsafe FFI cast | HIGH | FIXED |
| 10 | Invalid `firstWhere` with null | HIGH | FIXED |
| 11 | Unused `riverpod` dependency | HIGH | FIXED |
| 12 | Unused `ffi` crate in Cargo.toml | MEDIUM | FIXED |
| 13 | Overly broad tokio features | MEDIUM | FIXED |
| 14 | `tonic-build` in wrong dep section | MEDIUM | FIXED |
| 15 | Unmaintained `charts_flutter` | CRITICAL | OPEN |
| 16 | WebSocket thread leak | HIGH | OPEN |
| 17 | Unbounded collections | HIGH | OPEN |
| 18 | String error types (use thiserror) | HIGH | OPEN |
| 19 | API call in Consumer builder | HIGH | OPEN |
| 20 | Excessive cloning in hot paths | MEDIUM | OPEN |
| 21 | AI bridge code duplication | MEDIUM | OPEN |
| 22 | Nonce generation (counter vs RNG) | MEDIUM | OPEN |
| 23 | Advanced charting god class | MEDIUM | OPEN |
| 24 | Resource leaks (DrawingToolManager, FFI) | MEDIUM | OPEN |
| 25 | Unsafe type casts in providers | MEDIUM | OPEN |
| 26 | Debug print in security code | MEDIUM | OPEN |
| 27 | Build script issues | LOW | OPEN |
| 28 | Commit message conventions | LOW | OPEN |

**Fixed:** 14 issues | **Open:** 14 issues

---

## 7. Recommended Next Steps (Priority Order)

1. Replace `charts_flutter` with `fl_chart`
2. Fix WebSocket thread lifecycle with shutdown signal
3. Add bounded buffers to all unbounded collections
4. Create `BullShiftError` enum with `thiserror`
5. Move `loadSymbolData()` out of Consumer builder
6. Decompose `advanced_charting_widget.dart` into smaller widgets
7. Add proper resource cleanup (dispose patterns)
8. Replace unsafe type casts with null-safe alternatives
9. Remove debug print statements from security code
10. Adopt conventional commits and feature branch workflow
