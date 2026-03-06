# BullShift API Server - Multi-stage Docker build
# Produces a minimal runtime image for the api_server binary.

# ---- Build stage ----
FROM rust:1.82-bookworm AS builder

WORKDIR /build

# Cache dependencies by copying manifests first
COPY rust/Cargo.toml rust/Cargo.lock* rust/build.rs ./
COPY rust/proto ./proto

# Create a dummy main/lib so cargo fetches deps
RUN mkdir -p src/bin && \
    echo "fn main() {}" > src/bin/api_server.rs && \
    echo "" > src/lib.rs && \
    cargo build --release --bin api_server 2>/dev/null || true && \
    rm -rf src

# Copy real source and build
COPY rust/src ./src
COPY rust/build.rs ./build.rs
RUN cargo build --release --bin api_server

# ---- Runtime stage ----
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd -r bullshift && useradd -r -g bullshift -s /sbin/nologin bullshift

COPY --from=builder /build/target/release/api_server /usr/local/bin/api_server

USER bullshift

ENV RUST_LOG=info
ENV BULLSHIFT_PORT=8787

EXPOSE 8787

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD ["/usr/local/bin/api_server", "--health-check"] || exit 1

ENTRYPOINT ["/usr/local/bin/api_server"]
