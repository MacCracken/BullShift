# Changelog

All notable changes to BullShift Trading Platform will be documented in this file.

## [2026.3.5] - 2026-03-05

### Added
- **API key encryption for AI providers** — `BearlyManaged` now encrypts API keys
  via `SecurityManager` on `add_provider()`. Keys are stored encrypted at rest and
  decrypted only at request time. New methods: `update_provider_api_key()`,
  `has_encrypted_api_key()`, `resolve_api_key()`.
- **SecurityManager API key storage** — `store_api_key()`, `get_api_key()`,
  `has_api_key()`, `remove_api_key()` methods for AI provider credential management.
- **SecureYeoman AI provider** — `SecureYeoman` variant added to `AIProviderType`.
  `send_secureyeoman_request()` sends chat completions to
  `POST http://localhost:18789/api/v1/chat`. Optional bearer token auth supported.
- **Interactive Brokers integration** — Client Portal Gateway API support for
  equities, options, crypto, and extended-hours trading. Requires IB Gateway
  running locally. (`rust/src/trading/brokers/interactive_brokers.rs`)
- **Tradier integration** — Full REST API integration with OAuth bearer token
  auth. Supports sandbox and production environments.
  (`rust/src/trading/brokers/tradier.rs`)
- **Robinhood integration** — OAuth2 bearer token integration with instrument
  resolution, fractional shares, and crypto support. No sandbox available.
  (`rust/src/trading/brokers/robinhood.rs`)
- **Unified broker abstraction layer** — `TradingApiManager` enhanced with
  broker registration, capability queries, runtime broker switching,
  named routing (`submit_order_to`), and broker info/status reporting.
- **`BrokerCapabilities` metadata** — each broker declares what it supports
  (fractional shares, options, crypto, short selling, extended hours, etc.)
- **Broker integration guide** — `docs/guides/broker-integration.md`

### Changed
- `TradingApiManager.set_default()` now returns `bool` indicating success
- `TradingApiManager` gains `cancel_order()` forwarding (was missing)
- `TradingApiManager.register_broker()` replaces `add_api()` (legacy kept)

### Fixed
- **AES-256-GCM encryption buffer bug** — `encrypt_sensitive_data()` was pre-resizing
  the buffer with zeroes before `seal_in_place_append_tag()`, causing 16 null bytes
  to be appended to decrypted plaintext. Fixed by letting the seal function handle
  buffer growth.

### Technical
- New `trading/brokers/` submodule with `mod.rs`, `interactive_brokers.rs`,
  `tradier.rs`, `robinhood.rs`
- Added `#[async_trait]` to `TradingApi` trait for dyn-compatibility
- Added `tower` dev-dependency for api_server test compilation
- ADR-006: Broker abstraction architecture decision record
- Added `pub mod ai_bridge` to `lib.rs` (was missing, preventing test discovery)
- 10 new tests for API key encryption and SecureYeoman provider

---

## [2026.3.5] - 2026-03-05

### Added
- **Comprehensive code audit** — 28 findings identified and 27 resolved
  (`docs/development/code-audit-2026-03.md`)
- **VERSION file** and `bump-version.sh` for consistent versioning
- **`safe_cast.dart`** extension for null-safe map value access in Flutter
- **`BullShiftError` migration** — all Rust modules now use structured error
  types instead of `Result<T, String>`

### Fixed
- Missing `async-trait` dependency preventing compilation
- Mutex poisoning risk in 11 database methods
- NaN panic in float sorting (4 locations)
- API server `serde_json::to_value().unwrap()` crash risk (3 locations)
- Hardcoded API key placeholders in AI bridge
- WebSocket thread leak (no shutdown mechanism)
- Unbounded collections in logging, AI responses, and alerts
- Nonce generation using counter+random hybrid for AES-GCM safety
- Excessive cloning in trading API hot path
- AI bridge code duplication (5 providers deduplicated to 3 helpers)
- Unmaintained `charts_flutter` replaced with `fl_chart`
- API call in Flutter Consumer builder causing redundant network requests
- Debug `print()` in security code replaced with debug-only assertions
- Resource leaks in charting widget and FFI engine
- 9 broken README documentation links
- Missing Flutter import, unsafe FFI cast, invalid `firstWhere` with null
- Unused `riverpod` and `ffi` dependencies removed
- `tonic-build` moved to correct `[build-dependencies]` section
- Tokio features narrowed from `"full"` to specific subset

### Changed
- Advanced charting widget decomposed from 2491-line god class into 7 focused
  files: `chart_toolbar.dart`, `candlestick_painter.dart`, `volume_painter.dart`,
  `indicator_painter.dart`, `comparison_chart.dart`, `chart_enums.dart`
- 36 unsafe type casts across 9 Flutter files migrated to `safe_cast.dart`
- Build script updated: removed redundant `$?` checks, `flutter packages pub run` → `dart run`

---

## [2026.2.22] - 2026-02-22

### Added
- **REST API server** (`api_server` binary) — Axum HTTP server that wraps
  the trading engine so external tools can reach BullShift without FFI.
  Endpoints: `GET /health`, `POST /v1/orders`, `GET /v1/positions`,
  `GET /v1/account`, `DELETE /v1/orders/:id`.
  Configure via `ALPACA_API_KEY`, `ALPACA_API_SECRET`, `ALPACA_SANDBOX`,
  `BULLSHIFT_PORT` env vars.
- **SecureYeoman AI provider entry** — `SecureYeoman` type pre-registered in
  BearlyManaged's provider list (UI only; HTTP bridge implementation tracked for 2026.5.x).

### Technical
- Added `axum = "0.7"` dependency for the HTTP API server binary
- Added `[[bin]] api_server` target in `Cargo.toml`

---

## [2026.2.16] - 2026-02-16

### Added
- **Database Backend** - SQLite (rusqlite) integration for persistent storage
- **Portfolio Persistence** - Save/load portfolio positions to database
- **Trade History Storage** - Historical trade records with date range queries
- **Structured Logging** - Comprehensive logging across all Rust modules
- **Custom Error Types** - Proper error handling with custom error enums in Rust

### Technical
- Added `rusqlite` crate with bundled SQLite support
- Implemented database schema for portfolios, positions, and trades
- Added `TradeHistory` module for managing historical trade data

### Previous Releases

Features from earlier alpha/beta releases included:
- Core Trading Engine with sub-100ms latency
- Real-time Position Management and P&L tracking
- TrendSetter, BullRunnr, BearlyManaged, PaperHands modules
- Cross-platform support (Linux, macOS, Windows, iOS, Android)
- AES-256 encryption and secure credential storage
- Advanced charting with multiple chart types
- Comprehensive test suite

---

For older releases, please refer to the git history.

*Last Updated: March 5, 2026*
