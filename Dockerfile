# ===============================
# 👷 STAGE 1 - Build Rust binary
# ===============================
FROM rust:latest AS builder

# Install system dependencies required for static builds
RUN apt-get update && apt-get install -y pkg-config libssl-dev protobuf-compiler

# Set the working directory
WORKDIR /app

COPY . .

RUN cargo build --release

# ===============================
# 🐳 STAGE 2 - Minimal runtime
# ===============================
FROM alpine:latest

# Install necessary runtime dependencies
WORKDIR /app

# Copy the statically linked binary from the builder stage
COPY --from=builder /app/target/release/rustymq /usr/local/bin/rustymq

# Expose the gRPC port
EXPOSE 50053

# Set the default command
ENTRYPOINT ["/usr/local/bin/rustymq"]
