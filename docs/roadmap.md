# BullShift Roadmap

**Version:** 2026.2.22  
**Last Updated:** February 22, 2026

---

## 2026.2.16 - Initial Release

### Focus: Core functionality and data persistence

- [x] Fix pre-existing Rust compilation errors in logging and security modules
- [x] Implement proper error types in Rust (replace `Result<T, String>` with custom error enums)
- [x] Add structured logging across all modules
- [x] Performance benchmarking and optimization - added criterion benchmarks for logging and security
- [x] Add database backend (rusqlite)
- [x] Implement portfolio/position persistence
- [x] Add historical trade storage

---

## 2026.3.x - Enhanced Charting

### Focus: Advanced charting improvements

- [x] Real-time chart data integration
- [x] Additional chart types (renko, kagi, P&F) interactivity
- [x] Drawing tools (trendlines, fibonacci, annotations)
- [x] Multi-symbol comparison charts

---

## 2026.4.x - Broker Expansions

### Focus: Additional broker integrations

- [ ] Interactive Brokers API integration
- [ ] Tradier broker integration
- [ ] Robinhood API integration
- [ ] Unified broker abstraction layer

---

## 2026.5.x - AI Enhancement (Scheduled)

### Focus: AI Bridge backend implementations

**Status:** Scheduled for future release

- [ ] Anthropic API integration
- [ ] Ollama local LLM integration
- [ ] Local LLM support (LLM running locally)
- [ ] Custom provider configuration
- [ ] Encryption for API keys using SecurityManager
- [ ] **SecureYeoman as AI provider** — wire BearlyManaged's `SecureYeoman` provider type
      to `POST http://localhost:18789/api/v1/chat` (provider entry already in UI; backend
      HTTP bridge and credential flow to be implemented here)

---

---

## 2026.6.x - SecureYeoman Deep Integration (Planned)

### Focus: Remaining integration paths with SecureYeoman autonomous agent system

**Status:** Planned — MCP tool layer (see below) is the prerequisite

> **Done (2026.2.22):** BullShift REST API server (`api_server` binary) added so
> SecureYeoman's MCP tools can reach BullShift without FFI.

- [ ] **BullShift integration module in SecureYeoman** — full `Integration` adapter
      (`packages/core/src/integrations/bullshift/`) so SecureYeoman agents can
      subscribe to trade events and trigger orders autonomously, not just via MCP tools
- [ ] **Cryptographic audit trail** — emit trade events to SecureYeoman's audit chain
      so every order submission and fill is cryptographically signed for compliance
- [ ] **News & sentiment feed** — subscribe BullRunnr to SecureYeoman's event bus
      (Twitter/X, Reddit, webhook integrations) instead of independent scraping,
      eliminating duplicate fetch work and consolidating sentiment signals
- [ ] **RBAC for multi-user setups** — use SecureYeoman's role-based access control
      to gate which users/agents may submit live orders vs. read-only queries

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

- Options trading support
- Algorithmic trading execution
- Mobile app improvements
- Plugin system for extensions

### Community Requests

- Additional broker integrations (based on user feedback)
- Custom indicator development framework
- Webhook notifications
- Excel/Google Sheets integration

---

## Release History

| Version | Date | Status |
|---------|------|--------|
| 2026.2.22 | 2026-02-22 | Released |
| 2026.2.16 | 2026-02-16 | Released |

---

*This roadmap is subject to change based on priorities and community feedback.*
