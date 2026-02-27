# syntax=docker/dockerfile:1

# Stage 1: Build tools
FROM rust:1.88-slim-bookworm AS chef

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/loongclaw

# Install cargo-chef to improve dependency layer caching
RUN cargo install cargo-chef --locked

# Stage 2: Prepare dependency recipe
FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build
FROM chef AS builder

COPY --from=planner /usr/src/loongclaw/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

# Build the binary in release mode
RUN cargo build --release --locked --bin loongclaw

# Stage 4: Run
FROM debian:bookworm-slim

# Install runtime certificates and libraries
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

# Run as non-root by default
RUN useradd --create-home --home-dir /home/loongclaw --uid 10001 --shell /usr/sbin/nologin loongclaw

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /usr/src/loongclaw/target/release/loongclaw /usr/local/bin/

# Copy necessary runtime directories
COPY --from=builder /usr/src/loongclaw/web ./web
COPY --from=builder /usr/src/loongclaw/skills ./skills
COPY --from=builder /usr/src/loongclaw/scripts ./scripts

RUN mkdir -p /home/loongclaw/.loongclaw /app/tmp \
    && chown -R loongclaw:loongclaw /home/loongclaw /app

ENV HOME=/home/loongclaw
USER loongclaw

CMD ["loongclaw"]
