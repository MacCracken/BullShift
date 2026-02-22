# Changelog

All notable changes to BullShift Trading Platform will be documented in this file.

## [Unreleased] - 2026.2.22

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

*Last Updated: February 16, 2026*
