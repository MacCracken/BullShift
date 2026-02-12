# ADR-001: Rust + Flutter Architecture

**Date:** 2026-02-12  
**Status:** Accepted  
**Context:** System architecture decision for BullShift trading platform

## Decision

BullShift uses a hybrid Rust + Flutter architecture where:
- **Rust**: Performance-critical backend (trading execution, data streaming, security)
- **Flutter**: Cross-platform UI layer

## Consequences

### Positive
- Sub-100ms order execution through Rust
- Single codebase for Linux, macOS, Windows, iOS, Android
- Native performance for chart rendering and calculations
- FFI boundary enables easy integration of existing Rust trading libraries

### Negative
- Requires maintaining two codebases
- FFI boundary adds complexity
- Flutter LSP cannot analyze Rust dependencies

## Alternatives Considered

- **Pure Flutter/Dart**: Rejected due to performance requirements for trading
- **Pure Rust withegui/iced**: Rejected due to cross-platform UI requirements
- **Web-based (React/Electron)**: Rejected due to latency requirements
