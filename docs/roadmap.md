# BullShift Roadmap

**Version:** 2026.3.10
**Versioning:** CalVer `YYYY.M.D` (patches as `YYYY.M.D-N`)
**Last Updated:** March 10, 2026

---

## 2026.3.5 - AI Enhancement (Complete)

### Focus: AI Bridge backend implementations

**Status:** Complete

- [x] Encryption for API keys using SecurityManager
- [x] SecureYeoman as AI provider (`POST http://localhost:18789/api/v1/chat`)
- [x] Anthropic API integration (Claude API via `x-api-key` + `anthropic-version` headers)
- [x] Ollama local LLM integration (`POST http://localhost:11434/api/generate`)
- [x] Custom provider configuration (Flutter UI + API server endpoints)

---

## 2026.3.5-1 - SecureYeoman Deep Integration (Complete)

### Focus: Remaining integration paths with SecureYeoman autonomous agent system

**Status:** Complete

> **Done (2026.2.22):** BullShift REST API server (`api_server` binary) added so
> SecureYeoman's MCP tools can reach BullShift without FFI.

- [x] **BullShift integration module** — `SecureYeomanBridge` adapter with trade event
      emission, broadcast subscriptions, agent order validation, and bidirectional
      event forwarding (`src/integration/`)
- [x] **Cryptographic audit trail** — HMAC-SHA256 signed audit chain with tamper
      detection, chain verification, and optional SecureYeoman audit emission
      (`src/audit/`)
- [x] **News & sentiment feed** — `SentimentRouter` aggregates signals from
      SecureYeoman event bus AND independent sources (RSS, webhooks, Reddit,
      Twitter, custom APIs). `SecureYeomanEventSource`, `RssFeedSource`, and
      `WebhookSource` implement the `NewsSource` trait for BullRunnr integration
      (`src/sentiment/`)
- [x] **RBAC for multi-user setups** — role-based access control with Admin, Trader,
      Analyst, ReadOnly, Agent, and Custom roles. Fine-grained permissions,
      API key auth, user management, and SecureYeoman RBAC sync (`src/rbac/`)

---

## 2026.3.5-2 - Trading Extensions (Complete)

### Focus: Webhook notifications, spreadsheet integration, algorithmic trading, options trading

**Status:** Complete

- [x] **Webhook notifications** — `WebhookManager` in `src/webhooks/` with Slack,
      Discord, JSON, and FormEncoded formats. 12 trigger types (trade executed,
      order filled, price alert, etc.), retry logic with exponential backoff,
      HMAC-SHA256 payload signatures, delivery tracking.
- [x] **Excel/Google Sheets integration** — `SheetsManager` in `src/sheets/` with
      CSV/TSV/JSON export, Google Sheets API v4 append/read, scheduled exports,
      RFC 4180 CSV escaping. Export trades, positions, and account data.
- [x] **Algorithmic trading** — `AlgoEngine` in `src/algo/` with 8 strategy types
      (MA Crossover, Mean Reversion, Breakout, VWAP, TWAP, Grid, Trailing Stop,
      Pairs Trading). Signal generation, performance tracking, price history.
- [x] **Options trading** — `OptionsManager` in `src/options/` with Black-Scholes
      pricing, full Greeks (delta, gamma, theta, vega, rho), options chains,
      position management, portfolio Greeks. 9 strategy types including spreads,
      straddles, iron condors, covered calls, and protective puts.

---

## 2026.3.5-3 - Production Ready (Complete)

### Focus: Production deployment features

**Status:** Complete

- [x] **Production deployment guide** — comprehensive guide covering Docker,
      cloud deployments, bare-metal systemd setup, security checklist, and
      monitoring integration (`docs/guides/production-deployment.md`)
- [x] **Docker containerization** — multi-stage Dockerfile with non-root user,
      health checks, docker-compose with resource limits (`Dockerfile`,
      `docker-compose.yml`, `.env.example`, `.dockerignore`)
- [x] **Cloud deployment (AWS, GCP, Azure)** — ECS Fargate task definition with
      Secrets Manager, Cloud Run Knative service, Azure Container Apps ARM
      template (`deploy/aws/`, `deploy/gcp/`, `deploy/azure/`)
- [x] **CI/CD pipeline setup** — GitHub Actions for check/lint/test/build on
      every push, Docker build on main, cross-platform release on version tags
      (`.github/workflows/ci.yml`, `.github/workflows/release.yml`)
- [x] **Monitoring and alerting** — `monitoring` module with health checks
      (component-level with latency), metrics (counters, gauges, histograms),
      Prometheus text export, threshold-based alert rules with severity levels
      and cooldown (`src/monitoring/`)

---

## 2026.3.5-4 - Platform Extensions (Complete)

### Focus: Additional brokers, plugin system, indicator framework, mobile improvements

**Status:** Complete

- [x] **Additional broker integrations** — 4 new brokers added to `src/trading/brokers/`:
      Charles Schwab (`schwab.rs`) with OAuth2 Bearer auth and sandbox support,
      Coinbase (`coinbase.rs`) for crypto trading via Advanced Trade API,
      Kraken (`kraken.rs`) for crypto with API-Key/API-Sign auth,
      Webull (`webull.rs`) with options and fractional shares support.
      Total brokers: 8 (Alpaca, Interactive Brokers, Tradier, Robinhood, Schwab,
      Coinbase, Kraken, Webull)
- [x] **Plugin system** — `PluginRegistry` in `src/plugins/` with Plugin trait,
      event-driven architecture (TradeExecuted, PriceUpdate, OrderFilled, etc.),
      plugin lifecycle management (register, pause, resume, shutdown), and
      action dispatch (SubmitOrder, SendNotification, EmitEvent). Supports
      DataSource, Indicator, Strategy, Notification, Integration, and Custom types.
- [x] **Custom indicator framework** — `IndicatorRegistry` in `src/indicators/`
      with Indicator trait and 7 built-in indicators: SMA, EMA, RSI, MACD,
      Bollinger Bands, ATR, Stochastic. Factory pattern for creating indicators
      by name with configurable parameters. Custom indicator registration support.
- [x] **Mobile app improvements** — `src/mobile/` with push notification manager
      (iOS APNs, Android FCM, Web payloads), offline data sync (change queue,
      conflict detection and resolution), and biometric authentication support
      (FaceID, TouchID, Fingerprint, PIN with challenge-response verification).

---

## 2026.3.5-5 - Real-time Streaming (Complete)

### Focus: WebSocket streaming API for real-time data

**Status:** Complete

- [x] **WebSocket streaming server** — `StreamingServer` in `src/websocket/` with
      broadcast-based pub/sub architecture. 5 channel types (Prices per-symbol,
      Trades, Orders, Positions, Alerts). Client session management with
      subscription tracking. Convenience publishers for price updates, trades,
      order updates, and alerts. Server stats reporting. 15 tests.

---

## 2026.3.9 - MCP API Endpoints & Portfolio Features (Complete)

### Focus: REST endpoints for MCP tool integration, multi-currency support, tax lot tracking

**Status:** Complete

- [x] **Market data API** — `GET /v1/market/:symbol` returns real-time quote
      (last price, bid/ask, volume, OHLC, change %) via Alpaca market data API.
      New `ApiQuote` type and `AlpacaApi::get_quote()` method
      (`src/trading/api.rs`)
- [x] **Algo strategies API** — `GET /v1/algo/strategies` lists all strategies
      with performance metrics, `GET /v1/algo/strategies/:id` gets a single
      strategy, `POST /v1/algo/strategies` creates new strategies,
      `GET /v1/algo/signals` returns recent signals. Exposes `AlgoEngine` over
      REST for SecureYeoman MCP tool `bullshift_algo_status`
- [x] **Sentiment API** — `GET /v1/sentiment` returns overview with sources and
      recent signals, `GET /v1/sentiment/:symbol` returns per-symbol aggregate
      with signal history, `GET /v1/sentiment/signals` returns raw signals.
      Exposes `SentimentRouter` for MCP tool `bullshift_sentiment`
- [x] **Alerts API** — `GET /v1/alerts` lists active alerts, `POST /v1/alerts`
      creates alert rules, `GET /v1/alerts/rules` lists rules,
      `DELETE /v1/alerts/rules/:id` removes rules. Exposes `AlertManager` for
      MCP tool `bullshift_alerts`
- [x] **Multi-currency portfolio** — `Currency` enum (USD, EUR, GBP, JPY, CAD,
      AUD, CHF, USDT, USDC), `ExchangeRates` with conversion via base currency,
      `CurrencyBalance` per-currency summaries, `deposit()`/`withdraw()` for
      multi-currency cash, `Portfolio::with_currency()` constructor, all position
      values auto-converted to base currency. 11 new tests
      (`src/trading/portfolio.rs`)
- [x] **Tax lot tracking** — `TaxLotTracker` in `src/trading/tax_lots.rs` with 5
      cost basis methods (FIFO, LIFO, Highest Cost, Lowest Cost, Specific ID),
      `TaxLot` per-purchase tracking, `RealizedGainLoss` for dispositions,
      long-term vs short-term classification, `TaxReport` annual generation with
      wash sale detection, `SymbolTaxSummary` per-symbol reporting, commission
      proration. 22 new tests

---

## SecureYeoman & AGNOS Integration

### MCP Tool Registration
**Status:** Ready — all REST endpoints implemented, SecureYeoman MCP tools complete.

The BullShift `api_server` binary exposes REST endpoints that SecureYeoman proxies as MCP tools via `registerApiProxyTool()` in `tool-utils.ts`.

**Registered in SecureYeoman** (`packages/mcp/src/tools/trading-tools.ts`):
- [x] **`bullshift_health`** — Verify API server is reachable
- [x] **`bullshift_get_account`** — Account balance, available funds, margin
- [x] **`bullshift_get_positions`** — All open positions with P&L
- [x] **`bullshift_submit_order`** — Place market/limit/stop/stop-limit orders
- [x] **`bullshift_cancel_order`** — Cancel open orders by ID

**Planned** (tracked in SecureYeoman roadmap Phase 145, BullShift endpoints ready):
- [ ] **`bullshift_market_data`** — Price quotes and candles via GET `/v1/market/:symbol`
- [ ] **`bullshift_algo_status`** — Active algo strategies and performance via GET `/v1/algo/strategies`
- [ ] **`bullshift_sentiment`** — Aggregated sentiment signals via GET `/v1/sentiment`
- [ ] **`bullshift_alerts`** — Price/trade alerts via GET/POST `/v1/alerts`

### AGNOS Docker Base Migration
**Status:** Complete — runtime image swapped to `ghcr.io/maccracken/agnosticos:latest`.

- [x] **Swap runtime stage to AGNOS** — Dockerfile now uses `ghcr.io/maccracken/agnosticos:latest` as runtime base (was `debian:bookworm-slim`). Gains: AGNOS non-root user, tini PID 1, agent-runtime directories, sandbox and marketplace paths ready.
- [x] **Audit chain unification** — `AuditConfig` gains `agnos_audit_url` field (auto-populated from `AGNOS_AUDIT_URL` env var). When set, every audit entry is forwarded to AGNOS daimon via `POST {url}/v1/audit/forward` with `x-agent-id: bullshift` header. Reuses existing `AuditEntry` format (`src/audit/mod.rs`).

### WebSocket Streaming + SecureYeoman
**Priority:** Low — enhancement opportunity.

- [ ] **Real-time price/trade events in SecureYeoman dashboard** — BullShift's WebSocket streaming server (5 channel types) could feed SecureYeoman's Agent World or a custom dashboard widget. Requires SecureYeoman WebSocket transport (tracked in SecureYeoman roadmap under "WebSocket Mode for AI Providers").

---

## AGNOS Marketplace Onboarding (In Progress)

### Focus: Package BullShift as an installable .agnos-agent marketplace app

**Status:** In progress — 5 of 6 items complete. Sandbox verification pending (requires AGNOS environment).

| Item | Effort | Status | Description |
|------|--------|--------|-------------|
| Build Flutter Linux bundle | 1 hour | Complete | `build.sh agnos` target produces Flutter Linux bundle, API server binary, and icon assets in `dist/agnos/`. Bundle path: `flutter/build/linux/x64/release/bundle/` |
| Add app icon | 30 min | Complete | `flutter/assets/icons/bullshift.png` (256x256) and `bullshift.svg` created. Declared in `pubspec.yaml` assets. `build.sh agnos` copies to `dist/agnos/icons/` for AGNOS recipe `$PKG/usr/share/icons/` install step |
| Audit chain forwarding to AGNOS | 2 hours | Complete | `AuditConfig.agnos_audit_url` (from `AGNOS_AUDIT_URL` env var) forwards entries to `POST {url}/v1/audit/forward` with `x-agent-id: bullshift` (`src/audit/mod.rs`) |
| Agent registration with daimon | 1 hour | Complete | `AgnosAgentRegistration` in `src/agnos/mod.rs` — registers via `POST /v1/agents/register` on startup, heartbeats every 30s, deregisters on graceful shutdown. Env var: `AGNOS_AGENT_REGISTRY_URL` |
| LLM calls through hoosh | 1 hour | Complete | `BearlyManaged` checks `AGNOS_LLM_GATEWAY_URL` env var. When set, all AI requests route through `POST {gateway}/v1/chat/completions` with `x-agent-id: bullshift` header (`src/ai_bridge/mod.rs`) |
| Verify sandbox in AGNOS | 2 hours | Ready to test | Verification script at `scripts/verify-agnos-sandbox.sh`. Tests: AGNOS env detection, binary accessibility, data dir persistence, broker API connectivity (8 hosts), daimon/audit/LLM gateway endpoints, API server smoke test. Run inside AGNOS to complete |

**AGNOS-side work (done):**
- Recipe updated to `2026.3.10` with correct binary name (`api_server` → `bullshift-api`)
- Broker API hosts added to sandbox allowed list (Alpaca, Tradier, Robinhood, Schwab, Coinbase, Kraken, Webull, Binance)
- `agpkg pack-flutter` command uncommented and ready
- Wayland requirements declared (core, xdg-shell)

---

## Future Considerations

### Nice to Have

*(No remaining items — multi-currency and tax lots now implemented)*

---

## Release History

| Version | Date | Status |
|---------|------|--------|
| 2026.3.10 | 2026-03-10 | Released |
| 2026.3.9 | 2026-03-09 | Released |
| 2026.3.5 | 2026-03-05 | Released |
| 2026.2.22 | 2026-02-22 | Released |
| 2026.2.16 | 2026-02-16 | Released |

---

*This roadmap is subject to change based on priorities and community feedback.*
