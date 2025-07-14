# ===============================
# 👷 STAGE 1 - Build Rust binary
# ===============================
FROM rust:1.77 AS builder

# Install system dependencies required for static builds
RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev

# Set environment variables for static linking
ENV RUSTFLAGS="-C target-feature=+crt-static"
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# Set the working directory
WORKDIR /app

# Copy the entire project
COPY . .

# Add the musl target and build in release mode
RUN rustup target add x86_64-unknown-linux-musl \
  && cargo build --release --target x86_64-unknown-linux-musl

# ===============================
# 🐳 STAGE 2 - Minimal runtime
# ===============================
FROM alpine:latest

# Copy the statically linked binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rustymq /usr/local/bin/rustymq

# Expose the gRPC port
EXPOSE 50051

# Set the default command
ENTRYPOINT ["/usr/local/bin/rustymq"]
