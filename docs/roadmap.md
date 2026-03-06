# BullShift Roadmap

**Version:** 2026.3.5
**Last Updated:** March 6, 2026

---

## 2026.5.x - AI Enhancement (In Progress)

### Focus: AI Bridge backend implementations

**Status:** In progress — API key encryption and SecureYeoman provider complete

- [x] Encryption for API keys using SecurityManager
- [x] SecureYeoman as AI provider (`POST http://localhost:18789/api/v1/chat`)
- [ ] Anthropic API integration
- [ ] Ollama local LLM integration
- [ ] Custom provider configuration

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
| 2026.3.5 | 2026-03-05 | Released |
| 2026.2.22 | 2026-02-22 | Released |
| 2026.2.16 | 2026-02-16 | Released |

---

*This roadmap is subject to change based on priorities and community feedback.*
