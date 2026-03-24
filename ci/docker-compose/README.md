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

**Complete Book Documentation**: See [`../../book-draft/ci/docker-compose/`](../../book-draft/ci/docker-compose/) for comprehensive chapter-by-chapter guide.

**All Chapters:**
- **[Chapter 1: Introduction & Quick Start](../../book-draft/ci/docker-compose/01-Introduction.md)** - Complete Docker Compose guide with quick start, examples, and troubleshooting
- **[Chapter 2: Architecture & Container System](../../book-draft/ci/docker-compose/02-Architecture.md)** - Container naming, instance detection, volumes, and data directories
- **[Chapter 3: Execution Flow & Startup Process](../../book-draft/ci/docker-compose/03-Execution-Flow.md)** - Complete code execution order from Docker Compose to blockchain binary
- **[Chapter 4: Network Configuration](../../book-draft/ci/docker-compose/04-Network-Configuration.md)** - Node connections, miner connection chain, and network topology
- **[Chapter 5: Port Mapping & External Access](../../book-draft/ci/docker-compose/05-Port-Mapping.md)** - Port mapping details, scaling helper script, and external access strategies
- **[Chapter 6: Scaling & Deployment](../../book-draft/ci/docker-compose/06-Scaling.md)** - Scaling methods comparison, incremental scaling, and data persistence
- **[Chapter 7: Sequential Startup](../../book-draft/ci/docker-compose/07-Sequential-Startup.md)** - Sequential startup mechanism, health checks, and wait script behavior
- **[Chapter 8: Deployment Scenarios](../../book-draft/ci/docker-compose/08-Deployment-Scenarios.md)** - Common deployment scenarios, examples, and best practices
- **[Chapter 9: Accessing Webserver](../../book-draft/ci/docker-compose/09-Accessing-Webserver.md)** - How to reach the REST API and Swagger UI
- **[Chapter 10: Deployment Guide](../../book-draft/ci/docker-compose/10-Deployment-Guide.md)** - Repeatable workflows and operational tips
- **[Chapter 11: Deployment Execution Walkthrough](../../book-draft/ci/docker-compose/11-Deployment-Execution-Walkthrough.md)** - What happens step-by-step when you run compose
- **[Chapter 12: DNS Resolution Mechanism](../../book-draft/ci/docker-compose/12-DNS-Resolution-Mechanism.md)** - How container names/services become resolvable hostnames

## Key Features

- **Multi-instance scaling**: Run multiple miners and webservers
- **Automatic port mapping**: All instances accessible externally
- **Sequential startup**: Nodes wait for previous nodes
- **Isolated data**: Each instance has its own data directory
- **Health checks**: Built-in health monitoring

