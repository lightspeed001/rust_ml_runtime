# Build stage with multi-arch support
FROM --platform=$BUILDPLATFORM rust:1.75-slim AS builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*
        
# Set build environment
ENV RUSTFLAGS="-C target-cpu=native"
WORKDIR /app


# Copy source and build
COPY . .
RUN cargo build --release --target=$TARGETPLATFORM


# Runtime stage
FROM gcr.io/distroless/base-debian12:nonroot

# Copy built binary and model directory
WORKDIR /
COPY --from=builder --chown=nonroot:nonroot /app/target/${TARGETPLATFORM}/release/ml_runtime /usr/local/bin/
COPY --chown=nonroot:nonroot model/ /app/model/

# Set non-root user
USER nonroot:nonroot

# Set entrypoint
ENTRYPOINT ["ml_runtime"]
