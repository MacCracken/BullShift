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

## 2027.1.x - Production Ready

### Focus: Production deployment features

- [ ] Production deployment guide
- [ ] Docker containerization
- [ ] Cloud deployment (AWS, GCP, Azure)
- [ ] CI/CD pipeline setup
- [ ] Monitoring and alerting

---

## Future Considerations

### Nice to Have

- Mobile app improvements
- Plugin system for extensions

### Community Requests

- Additional broker integrations (based on user feedback)
- Custom indicator development framework

---

## Release History

| Version | Date | Status |
|---------|------|--------|
| 2026.3.6 | 2026-03-05 | Released |
| 2026.3.5 | 2026-03-05 | Released |
| 2026.2.22 | 2026-02-22 | Released |
| 2026.2.16 | 2026-02-16 | Released |

---

*This roadmap is subject to change based on priorities and community feedback.*
