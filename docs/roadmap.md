# BullShift Roadmap

**Version:** 2026.3.10
**Versioning:** CalVer `YYYY.M.D` (patches as `YYYY.M.D-N`)
**Last Updated:** March 10, 2026

---

## AGNOS Marketplace Onboarding (In Progress)

### Focus: Package BullShift as an installable .agnos-agent marketplace app

**Status:** 5 of 6 items complete. Sandbox verification pending.

| Item | Status | Description |
|------|--------|-------------|
| Verify sandbox in AGNOS | Ready to test | Verification script at `scripts/verify-agnos-sandbox.sh`. Tests: AGNOS env detection, binary accessibility, data dir persistence, broker API connectivity (8 hosts), daimon/audit/LLM gateway endpoints, API server smoke test. Run inside AGNOS to complete |

**AGNOS-side work (done):**
- Recipe updated to `2026.3.10` with correct binary name (`api_server` → `bullshift-api`)
- Broker API hosts added to sandbox allowed list (Alpaca, Tradier, Robinhood, Schwab, Coinbase, Kraken, Webull, Binance)
- `agpkg pack-flutter` command uncommented and ready
- Wayland requirements declared (core, xdg-shell)

---

## SecureYeoman MCP Tool Registration (Planned)

### Focus: Register remaining BullShift endpoints as MCP tools in SecureYeoman

**Status:** 5 of 9 tools registered. 4 remaining (BullShift endpoints ready, SY-side registration needed).

Tracked in SecureYeoman roadmap Phase 145. Registration happens in `packages/mcp/src/tools/trading-tools.ts` via `registerApiProxyTool()`.

| Tool | BullShift Endpoint | Status |
|------|--------------------|--------|
| `bullshift_market_data` | `GET /v1/market/:symbol` | Not registered |
| `bullshift_algo_status` | `GET /v1/algo/strategies` | Not registered |
| `bullshift_sentiment` | `GET /v1/sentiment` | Not registered |
| `bullshift_alerts` | `GET/POST /v1/alerts` | Not registered |

---

## WebSocket Streaming + SecureYeoman (Low Priority)

- [ ] **Real-time price/trade events in SecureYeoman dashboard** — BullShift's WebSocket streaming server (5 channel types) could feed SecureYeoman's Agent World or a custom dashboard widget. Requires SecureYeoman WebSocket transport (tracked in SecureYeoman roadmap under "Trading Dashboard Enhancements").

---

---

## Release History

| Version | Date | Highlights |
|---------|------|------------|
| 2026.3.10 | 2026-03-10 | AGNOS marketplace onboarding (icon, build target, sandbox script), cargo fmt fixes, safe cast migration complete (150+ casts across 24 files), code audit items closed (error types, charting decomposition, safe casts all confirmed complete) |
| 2026.3.9 | 2026-03-09 | MCP API endpoints (market data, algo, sentiment, alerts), multi-currency portfolio, tax lot tracking, AGNOS Docker base + audit forwarding + agent registration + LLM gateway |
| 2026.3.5 | 2026-03-05 | AI bridge (SecureYeoman, Anthropic, Ollama), SY deep integration, trading extensions, production deployment, platform extensions, WebSocket streaming |
| 2026.2.22 | 2026-02-22 | REST API server for MCP integration |
| 2026.2.16 | 2026-02-16 | Initial release |

---

*This roadmap is subject to change based on priorities and community feedback.*
