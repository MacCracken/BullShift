# Changelog

All notable changes to BullShift Trading Platform will be documented in this file.

## [2026.3.9] - 2026-03-09

### Added
- **Market data API endpoint** — `GET /v1/market/:symbol` returns real-time quote
  with last price, bid/ask, volume, OHLC, change, and change percentage via
  Alpaca market data snapshot API. New `ApiQuote` type and `AlpacaApi::get_quote()`
  method (`src/trading/api.rs`)
- **Algo strategies API** — 4 new endpoints in `api_server`:
  - `GET /v1/algo/strategies` — list all strategies with performance metrics
  - `GET /v1/algo/strategies/:id` — get single strategy details
  - `POST /v1/algo/strategies` — create new strategy (9 types supported)
  - `GET /v1/algo/signals` — recent signals with configurable limit
  Exposes `AlgoEngine` over REST for SecureYeoman MCP tool integration
- **Sentiment API** — 3 new endpoints:
  - `GET /v1/sentiment` — overview with sources and recent signals
  - `GET /v1/sentiment/:symbol` — per-symbol aggregate sentiment + signals
  - `GET /v1/sentiment/signals` — raw signal feed with limit parameter
  Exposes `SentimentRouter` for MCP tool `bullshift_sentiment`
- **Alerts API** — 4 new endpoints:
  - `GET /v1/alerts` — list active (unresolved) alerts
  - `POST /v1/alerts` — create alert rules with metric, condition, threshold,
    severity, and cooldown
  - `GET /v1/alerts/rules` — list all alert rules
  - `DELETE /v1/alerts/rules/:id` — remove an alert rule
  Exposes `AlertManager` for MCP tool `bullshift_alerts`
- **Multi-currency portfolio support** — `Currency` enum supporting 9 currencies
  (USD, EUR, GBP, JPY, CAD, AUD, CHF, USDT, USDC), `ExchangeRates` with
  conversion via base currency, `CurrencyBalance` summaries, `deposit()`/
  `withdraw()` for multi-currency cash management, `Portfolio::with_currency()`
  constructor, position values auto-converted to base currency for total value
  calculation. 11 new tests (`src/trading/portfolio.rs`)
- **Tax lot tracking and reporting** — `TaxLotTracker` in `src/trading/tax_lots.rs`
  with 5 cost basis methods (FIFO, LIFO, Highest Cost, Lowest Cost, Specific ID),
  per-purchase `TaxLot` tracking with remaining quantity, `RealizedGainLoss`
  for dispositions with commission proration, long-term vs short-term capital
  gains classification (365-day threshold), `TaxReport` annual generation with
  short/long-term gains/losses breakdown and wash sale detection (30-day window),
  `SymbolTaxSummary` per-symbol open lot reporting. 22 new tests
- **Security audit CI job** — `cargo audit` added to GitHub Actions CI pipeline,
  gating build and Docker jobs on security check
- **SecurityManager fallback** — Linux key derivation now falls back to file-based
  storage (`~/.bullshift/.encryption_key`) when `secret-tool` (libsecret) is
  unavailable (fixes CI test failures)

### Security
- Input validation on `record_buy()` / `record_buy_with_date()` — rejects
  non-positive/non-finite quantity, negative/non-finite price and commission
- Symbol sanitization (`validate_symbol()`) — max 10 chars, alphanumeric + `.` + `-`
- Query limit capping (`clamp_limit()`) — prevents DoS via unbounded queries
- Division-by-zero guards in `ExchangeRates::convert()` and `set_rate()`
- Deposit/withdraw validation — rejects negative and non-finite amounts

### Technical
- 372 tests total (358 lib + 14 bin), 0 failures, 0 clippy warnings
- 46 new tests: 13 api_server endpoints, 11 multi-currency portfolio, 22 tax lots
- API server expanded from 10 to 21 endpoints (trading, market data, algo,
  sentiment, alerts, AI)
- No new Rust dependencies added
- CalVer format corrected to YYYY.M.D (was inconsistently using wrong formats)

---

## [2026.3.5] - 2026-03-05

### Added
- **API key encryption for AI providers** — `BearlyManaged` encrypts API keys
  via `SecurityManager` on `add_provider()`. Keys stored encrypted at rest,
  decrypted only at request time. New methods: `update_provider_api_key()`,
  `has_encrypted_api_key()`, `resolve_api_key()`
- **SecurityManager API key storage** — `store_api_key()`, `get_api_key()`,
  `has_api_key()`, `remove_api_key()` for AI provider credential management
- **SecureYeoman AI provider** — `SecureYeoman` variant in `AIProviderType`.
  `send_secureyeoman_request()` sends chat completions to
  `POST http://localhost:18789/api/v1/chat`. Optional bearer token auth
- **AI provider API endpoints** — `api_server` gains AI provider management:
  `GET/POST /v1/ai/providers`, `POST /v1/ai/providers/:id/configure`,
  `POST /v1/ai/providers/:id/test`, `POST /v1/ai/chat`
- **Flutter AI bridge service** — `AiBridgeService` HTTP client replaces
  simulated provider operations with graceful fallback
- **SecureYeoman integration bridge** — `SecureYeomanBridge` in `src/integration/`
  with trade event emission, broadcast subscriptions, agent order validation,
  and bidirectional event forwarding
- **Cryptographic audit trail** — `AuditTrail` in `src/audit/` with HMAC-SHA256
  signed entries, tamper-evident chain, chain verification, and optional
  SecureYeoman audit emission
- **Multi-source sentiment routing** — `SentimentRouter` in `src/sentiment/`
  aggregates signals from SecureYeoman event bus AND independent sources
  (RSS, webhooks). `SecureYeomanEventSource`, `RssFeedSource`, `WebhookSource`
- **RBAC system** — `RbacManager` in `src/rbac/` with 6 roles and 14
  fine-grained permissions. API key auth, user management, SecureYeoman sync
- **Webhook notifications** — `WebhookManager` in `src/webhooks/` with Slack,
  Discord, JSON, FormEncoded formats. 12 trigger types, retry with backoff,
  HMAC-SHA256 signatures, delivery tracking
- **Excel/Google Sheets integration** — `SheetsManager` in `src/sheets/` with
  CSV/TSV/JSON export, Google Sheets API v4, scheduled exports, RFC 4180 escaping
- **Algorithmic trading engine** — `AlgoEngine` in `src/algo/` with 8 strategy
  types (MA Crossover, Mean Reversion, Breakout, VWAP, TWAP, Grid, Trailing
  Stop, Pairs Trading). Signal generation, performance tracking
- **Options trading** — `OptionsManager` in `src/options/` with Black-Scholes
  pricing, full Greeks, options chains, portfolio Greeks. 9 strategy types
- **Docker containerization** — multi-stage Dockerfile with non-root user,
  health checks, docker-compose with resource limits
- **CI/CD pipelines** — GitHub Actions: formatting, clippy, tests, release
  build, Docker build on main, cross-platform release on version tags
- **Cloud deployment configs** — AWS ECS Fargate, Google Cloud Run, Azure
  Container Apps templates
- **Monitoring and alerting** — `monitoring` module with health checks, metrics
  (counters, gauges, histograms), Prometheus export, threshold-based alert rules
- **Production deployment guide** — Docker, cloud, bare-metal systemd, security
  checklist (`docs/guides/production-deployment.md`)
- **8 broker integrations** — Alpaca, Interactive Brokers, Tradier, Robinhood,
  Charles Schwab (OAuth2, sandbox), Coinbase (crypto, Advanced Trade API),
  Kraken (crypto, HMAC-SHA512), Webull (options, fractional shares)
- **Unified broker abstraction** — `TradingApiManager` with registration,
  capability queries, runtime switching, `BrokerCapabilities` metadata
- **Plugin system** — `PluginRegistry` in `src/plugins/` with 6 plugin types,
  6 event types, event-driven lifecycle management
- **Custom indicator framework** — `IndicatorRegistry` in `src/indicators/`
  with 7 built-in indicators (SMA, EMA, RSI, MACD, Bollinger Bands, ATR,
  Stochastic). Factory pattern, custom registration
- **Mobile app support** — push notifications (APNs, FCM, Web), offline sync
  with conflict resolution, biometric auth (FaceID, TouchID, Fingerprint, PIN)
- **Real-time WebSocket streaming** — `StreamingServer` in `src/websocket/`
  with broadcast pub/sub. 5 channel types (Prices, Trades, Orders, Positions,
  Alerts). Client session management, subscription tracking
- **BullRunnr module** — compiled as part of library (was previously orphaned)
- **Comprehensive code audit** — 28 findings identified and 27 resolved
- **VERSION file** and `bump-version.sh` for consistent versioning
- **`BullShiftError` migration** — structured error types across all modules

### Changed
- `TradingApiManager.set_default()` returns `bool` indicating success
- `TradingApiManager.register_broker()` replaces `add_api()`
- Advanced charting decomposed from 2491-line god class into 7 focused files
- 36 unsafe type casts migrated to `safe_cast.dart`

### Fixed
- **AES-256-GCM encryption buffer bug** — pre-resize causing 16 null bytes
- Missing `async-trait` dependency preventing compilation
- Mutex poisoning risk in 11 database methods
- NaN panic in float sorting (4 locations)
- API server `serde_json::to_value().unwrap()` crash risk (3 locations)
- FFI-unsafe `Option<f64>` in `TradeOrder`
- WebSocket thread leak, unbounded collections, nonce generation
- Excessive cloning in trading API hot path
- AI bridge code duplication (5 providers → 3 helpers)
- BullRunnr: `MarketSentiment` Default, VaderSentimentAnalyzer lifetime
- 9 broken README links, resource leaks, unused dependencies

### Technical
- 326 tests total, 0 failures, 0 clippy warnings
- `api_server` expanded from 5 to 10 endpoints (trading + AI)
- Anthropic default model updated to `claude-sonnet-4-6`
- No new Rust dependencies added (uses existing tokio, ring, axum)

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
  BearlyManaged's provider list (UI only; backend HTTP bridge added in 2026.3.5).

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

*Last Updated: March 9, 2026*
