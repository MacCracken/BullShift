# BullShift Roadmap

**Version:** 2026.3.6
**Last Updated:** March 5, 2026

---

## 2026.5.x - AI Enhancement (Complete)

### Focus: AI Bridge backend implementations

**Status:** Complete

- [x] Encryption for API keys using SecurityManager
- [x] SecureYeoman as AI provider (`POST http://localhost:18789/api/v1/chat`)
- [x] Anthropic API integration (Claude API via `x-api-key` + `anthropic-version` headers)
- [x] Ollama local LLM integration (`POST http://localhost:11434/api/generate`)
- [x] Custom provider configuration (Flutter UI + API server endpoints)

---

## 2026.6.x - SecureYeoman Deep Integration (Complete)

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

## 2026.3.6 - Trading Extensions (Complete)

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

## 2027.1.x - Production Ready (Complete)

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

## 2027.2.x - Platform Extensions (Complete)

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

## 2027.3.x - Real-time Streaming (Complete)

### Focus: WebSocket streaming API for real-time data

**Status:** Complete

- [x] **WebSocket streaming server** — `StreamingServer` in `src/websocket/` with
      broadcast-based pub/sub architecture. 5 channel types (Prices per-symbol,
      Trades, Orders, Positions, Alerts). Client session management with
      subscription tracking. Convenience publishers for price updates, trades,
      order updates, and alerts. Server stats reporting. 15 tests.

---

## SecureYeoman & AGNOS Integration

### MCP Tool Registration
**Status:** Ready — BullShift REST API server exists, SecureYeoman integration module complete.

The BullShift `api_server` binary (added 2026-02-22) exposes REST endpoints that SecureYeoman can proxy as MCP tools. Candidates for `packages/mcp/src/tools/manifest.ts` registration:

- [ ] **`bullshift_portfolio`** — Portfolio summary (positions, P&L, Greeks) via GET `/api/portfolio`
- [ ] **`bullshift_trade`** — Execute trades via POST `/api/orders` with agent order validation from `SecureYeomanBridge`
- [ ] **`bullshift_market_data`** — Price quotes and candles via GET `/api/market/:symbol`
- [ ] **`bullshift_algo_status`** — Active algo strategies and performance via GET `/api/algo/strategies`
- [ ] **`bullshift_sentiment`** — Aggregated sentiment signals via GET `/api/sentiment` (feeds from `SentimentRouter`)
- [ ] **`bullshift_alerts`** — Price/trade alerts via GET/POST `/api/alerts`

These should use SecureYeoman's `registerApiProxyTool()` factory in `tool-utils.ts` for zero-code registration.

### AGNOS Docker Base Migration
**Priority:** Medium — depends on AGNOS Alpha (Q2 2026).

- [ ] **Swap runtime stage to `agnos:latest`** — Current Dockerfile uses `debian:bookworm-slim` for the runtime stage. BullShift is already Rust, so the migration is straightforward. Gains: sandboxed trade execution via `agent-runtime`, cryptographic audit chain integration (complements BullShift's existing HMAC-SHA256 audit trail), resource quotas per trading strategy.
- [ ] **Audit chain unification** — BullShift's `src/audit/` HMAC chain and AGNOS's cryptographic audit chain can be unified. Forward BullShift audit events to AGNOS audit subsystem for tamper-evident logging at the OS level.

### WebSocket Streaming + SecureYeoman
**Priority:** Low — enhancement opportunity.

- [ ] **Real-time price/trade events in SecureYeoman dashboard** — BullShift's WebSocket streaming server (5 channel types) could feed SecureYeoman's Agent World or a custom dashboard widget. Requires SecureYeoman WebSocket transport (tracked in SecureYeoman roadmap under "WebSocket Mode for AI Providers").

---

## Future Considerations

### Nice to Have

- Multi-currency portfolio support
- Tax lot tracking and reporting

---

## Release History

| Version | Date | Status |
|---------|------|--------|
| 2027.3.0 | 2026-03-05 | Released |
| 2026.3.6 | 2026-03-05 | Released |
| 2026.3.5 | 2026-03-05 | Released |
| 2026.2.22 | 2026-02-22 | Released |
| 2026.2.16 | 2026-02-16 | Released |

---

*This roadmap is subject to change based on priorities and community feedback.*
