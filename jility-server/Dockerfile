# Build stage
FROM rust:latest as builder

WORKDIR /app

# Copy workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY jility-server/Cargo.toml ./jility-server/

# Copy all local crate dependencies (these are part of the source)
COPY crates ./crates

# Build dependencies with dummy binary (cached layer)
# This caches both external dependencies AND local crate compilation
RUN mkdir -p jility-server/src && \
    echo "fn main() {}" > jility-server/src/main.rs && \
    cargo build --manifest-path jility-server/Cargo.toml --release && \
    rm -rf jility-server/src

# Copy actual binary source code
COPY jility-server/src ./jility-server/src

# Build the application
RUN touch jility-server/src/main.rs && \
    cargo build --manifest-path jility-server/Cargo.toml --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/jility-server/target/release/jility-server /usr/local/bin/jility-server

# Create data directory
RUN mkdir -p /app/.jility

# Environment variables (can be overridden)
ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:///app/.jility/data.db?mode=rwc
ENV BIND_ADDRESS=0.0.0.0:3900

EXPOSE 3900

CMD ["jility-server"]
