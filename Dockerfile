# Build stage for Rust binary
FROM rust:1.91.1-slim AS rust-builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY bitcoin/Cargo.toml ./bitcoin/
# Copy other workspace members' Cargo.toml files (needed for workspace resolution)
COPY bitcoin-desktop-ui-iced/Cargo.toml ./bitcoin-desktop-ui-iced/
COPY bitcoin-wallet-ui-iced/Cargo.toml ./bitcoin-wallet-ui-iced/
COPY bitcoin-api/Cargo.toml ./bitcoin-api/

# Copy source code for all workspace members (Cargo needs them to resolve workspace)
COPY bitcoin/src ./bitcoin/src
COPY bitcoin-desktop-ui-iced/src ./bitcoin-desktop-ui-iced/src
COPY bitcoin-wallet-ui-iced/src ./bitcoin-wallet-ui-iced/src
COPY bitcoin-api/src ./bitcoin-api/src

# Build the blockchain binary
RUN cargo build --release -p blockchain

# Build stage for React web UI
FROM node:20-slim AS web-ui-builder

# Set working directory
WORKDIR /app

# Copy package files
COPY bitcoin-web-ui/package.json bitcoin-web-ui/package-lock.json ./bitcoin-web-ui/

# Install dependencies
WORKDIR /app/bitcoin-web-ui
RUN npm ci

# Copy source files
COPY bitcoin-web-ui/ ./

# Build React app
RUN npm run build

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    netcat-openbsd \
    libc-bin \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy binary from Rust builder
COPY --from=rust-builder /app/target/release/blockchain /app/blockchain

# Copy built React web UI from web-ui-builder
# The Rust server looks for bitcoin-web-ui/dist relative to the binary location
COPY --from=web-ui-builder /app/bitcoin-web-ui/dist /app/bitcoin-web-ui/dist

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

# Node configuration (can be overridden)
# NODE_IS_MINER defaults to "no" (webserver mode) as a safe default:
# - Prevents accidental mining if container is run directly without docker-compose
# - docker-compose.yml explicitly sets NODE_IS_MINER=yes for miner service (overrides this default)
# - This default is a fallback; docker-compose always sets the correct value for each service
ENV NODE_IS_MINER=no
ENV NODE_IS_WEB_SERVER=yes
ENV NODE_CONNECT_NODES=local
# NODE_MINING_ADDRESS is required and must be set at runtime

# Use entrypoint script for flexible node configuration
ENTRYPOINT ["/app/docker-entrypoint.sh"]
