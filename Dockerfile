# BullShift API Server - Multi-stage Docker build
# Uses AGNOS as runtime base for marketplace integration.

# ---- Build stage ----
FROM ghcr.io/maccracken/agnosticos:latest AS base

FROM rust:bookworm AS builder

WORKDIR /build

# Cache dependencies by copying manifests first
COPY rust/Cargo.toml rust/Cargo.lock ./

# Create dummy sources so cargo fetches deps (cache layer)
RUN mkdir -p src/bin benches && \
    echo "fn main() {}" > src/bin/api_server.rs && \
    echo "" > src/lib.rs && \
    echo "fn main() {}" > benches/benchmarks.rs && \
    cargo build --release --bin api_server 2>/dev/null || true && \
    rm -rf src benches

# Copy real source and build
COPY rust/src ./src
COPY rust/benches ./benches
RUN cargo build --release --bin api_server

# ---- Runtime stage (AGNOS base) ----
FROM base

LABEL org.opencontainers.image.title="BullShift"
LABEL org.opencontainers.image.description="High-performance trading platform on AGNOS"
LABEL org.opencontainers.image.source="https://github.com/MacCracken/BullShift"
LABEL org.opencontainers.image.licenses="MIT"

USER root
RUN groupadd -g 1004 bullshift && useradd -u 1004 -g bullshift -m -s /bin/bash bullshift

COPY --from=builder /build/target/release/api_server /usr/local/bin/api_server

ENV RUST_LOG=info
ENV BULLSHIFT_PORT=8787

EXPOSE 8787

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -sf http://localhost:8787/health || exit 1

USER bullshift

ENTRYPOINT ["/usr/local/bin/api_server"]
