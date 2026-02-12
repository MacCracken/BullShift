# BullShift Roadmap

**Version:** 1.0.0-alpha.2  
**Last Updated:** February 12, 2026

---

## v1.1 - Core Stability (Next Release)

### Focus: Bug fixes and stability improvements

- [ ] Fix pre-existing Rust compilation errors in logging and security modules
- [ ] Implement proper error types in Rust (replace `Result<T, String>` with custom error enums)
- [ ] Add structured logging across all modules
- [ ] Performance benchmarking and optimization

---

## v1.2 - Data Persistence

### Focus: Database backend

- [ ] Add database backend (consider sled or rusqlite)
- [ ] Implement portfolio/position persistence
- [ ] Add historical trade storage

---

## v1.3 - Enhanced Charting

### Focus: Advanced charting improvements

- [ ] Real-time chart data integration
- [ ] Additional chart types (renko, kagi, P&F) interactivity
- [ ] Drawing tools (trendlines, fibonacci, annotations)
- [ ] Multi-symbol comparison charts

---

## v1.4 - Broker Expansions

### Focus: Additional broker integrations

- [ ] Interactive Brokers API integration
- [ ] Tradier broker integration
- [ ] Robinhood API integration
- [ ] Unified broker abstraction layer

---

## v1.5 - AI Enhancement (Scheduled)

### Focus: AI Bridge backend implementations

**Status:** Scheduled for future release

- [ ] Anthropic API integration
- [ ] Ollama local LLM integration
- [ ] Local LLM support (LLM running locally)
- [ ] Custom provider configuration
- [ ] Encryption for API keys using SecurityManager

---

## v2.0 - Production Ready

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
| 1.0.0-alpha | 2026-02-10 | ✅ Released |
| 1.0.0-alpha.2 | 2026-02-11 | ✅ Released |
| 1.1.0 | TBD | Planned |

---

*This roadmap is subject to change based on priorities and community feedback.*
