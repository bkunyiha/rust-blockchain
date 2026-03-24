# Build stage
FROM rust:1.70-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY bitcoin/Cargo.toml ./bitcoin/

# Copy source code
COPY bitcoin/src ./bitcoin/src

# Build the blockchain binary
RUN cargo build --release -p blockchain

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/blockchain /app/blockchain

# Copy entrypoint script
COPY ci/docker-compose/configs/docker-entrypoint.sh /app/docker-entrypoint.sh
RUN chmod +x /app/docker-entrypoint.sh

# Copy wait script for sequential startup
COPY ci/docker-compose/configs/wait-for-node.sh /app/wait-for-node.sh
RUN chmod +x /app/wait-for-node.sh

# Create data directory
RUN mkdir -p /app/data

# Expose ports
# 8080: Web server
# 2001: P2P network
EXPOSE 8080 2001

# Set default environment variables
ENV TREE_DIR=data1
ENV BLOCKS_TREE=blocks1
ENV NODE_ADDR=0.0.0.0:2001
ENV BITCOIN_API_ADMIN_KEY=admin-secret
ENV BITCOIN_API_WALLET_KEY=wallet-secret

# Node configuration (can be overridden)
# NODE_IS_MINER defaults to "no" (webserver mode) as a safe default:
# - Prevents accidental mining if container is run directly without docker-compose
# - docker-compose.yml explicitly sets NODE_IS_MINER=yes for miner service (overrides this default)
# - This default is a fallback; docker-compose always sets the correct value for each service
ENV NODE_IS_MINER=no
ENV NODE_IS_WEB_SERVER=yes
ENV NODE_CONNECT_NODES=local
ENV NODE_MINING_ADDRESS=

# Use entrypoint script for flexible node configuration
ENTRYPOINT ["/app/docker-entrypoint.sh"]

