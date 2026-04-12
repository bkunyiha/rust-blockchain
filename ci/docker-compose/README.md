# Docker Compose Deployment

This directory contains all Docker Compose-related files for deploying the blockchain network.

## Directory Structure

```
docker-compose/
├── configs/                # Configuration files
│   ├── docker-compose.yml                  # Main compose file
│   ├── docker-compose.miner.yml            # Miner-only compose file
│   ├── docker-compose.webserver.yml        # Webserver-only compose file
│   ├── docker-entrypoint.sh                # Container entrypoint script
│   ├── wait-for-node.sh                    # Node wait script
│   ├── docker-compose.scale.sh             # Scaling helper script
│   ├── scale-up.sh                         # Incremental scale up
│   ├── scale-down.sh                       # Incremental scale down
│   └── generate-compose-ports.sh           # Port mapping generator
└── README.md               # This quick start guide
```

## Quick Start

### Default Setup (1 miner + 1 webserver)

```bash
cd ci/docker-compose/configs
# Optional: provide a deterministic mining address per instance (comma-separated).
# If you omit this, the container entrypoint will auto-create a wallet address
# and persist it in the wallets volume.
#
# export WALLET_ADDRESS_POOL="addr1,addr2"
docker compose up -d
```

This starts:
- `redis` (required for API rate limiting)
- `miner` (P2P + mining)
- `webserver` (REST API + Swagger UI + rate limiting)

If addresses are auto-generated, they are persisted under each container’s wallets volume
(e.g. `/app/wallets/mining_address.txt` inside the container).

### Scale to Multiple Instances

```bash
cd ci/docker-compose/configs
./docker-compose.scale.sh 3 2  # 3 miners, 2 webservers
```

## Documentation

For a comprehensive chapter-by-chapter walkthrough — covering architecture, execution flow, network configuration, port mapping, scaling, sequential startup, deployment scenarios, and DNS resolution — see the Docker Compose Deployment chapter in the published book *Building a Blockchain in Rust*.

## Key Features

- **Multi-instance scaling**: Run multiple miners and webservers
- **Automatic port mapping**: All instances accessible externally
- **Sequential startup**: Nodes wait for previous nodes
- **Isolated data**: Each instance has its own data directory
- **Health checks**: Built-in health monitoring

