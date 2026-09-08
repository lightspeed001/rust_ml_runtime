# Build stage
FROM rust:1.75-slim as builder

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy source and build
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy built binary from builder
COPY --from=builder /app/target/release/ml_runtime /usr/local/bin/

# Set entrypoint
ENTRYPOINT ["ml_runtime"]

