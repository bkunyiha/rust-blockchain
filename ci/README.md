# CI/CD Infrastructure

This directory contains all infrastructure and deployment configurations for the blockchain network.

## Directory Structure

```
ci/
├── docker-compose/          # Docker Compose deployment
│   ├── configs/            # Docker Compose configuration files
│   └── README.md           # Quick start guide
└── kubernetes/             # Kubernetes deployment
    ├── manifests/          # Kubernetes manifest files
    └── README.md           # Quick start guide
```

## Docker Compose

**Location**: `ci/docker-compose/`

Docker Compose is ideal for:
- Local development
- Single-host deployments
- Quick testing and prototyping
- Simpler setup and management

**Quick Start:**
```bash
cd ci/docker-compose/configs
# Use either `docker compose` (recommended) or `docker-compose` (legacy)
docker compose up -d
```

**Documentation**:
- Quick Start: [`ci/docker-compose/README.md`](docker-compose/README.md)
- Complete Guide: see the Docker Compose Deployment chapter in the published book *Building a Blockchain in Rust*.

## Kubernetes

**Location**: `ci/kubernetes/`

Kubernetes is recommended for:
- Production deployments
- Multi-node clusters
- Automatic scaling
- High availability
- Enterprise-grade orchestration

**Quick Start:**
```bash
cd ci/kubernetes/manifests
./deploy.sh
```

**Documentation**:
- Quick Start: [`ci/kubernetes/README.md`](kubernetes/README.md)
- Complete Guide: see the Kubernetes Deployment chapter in the published book *Building a Blockchain in Rust*.

## Choosing Between Docker Compose and Kubernetes

| Use Case | Recommendation |
|----------|---------------|
| Local development | Docker Compose |
| Single server deployment | Docker Compose |
| Production multi-node | Kubernetes |
| Need autoscaling | Kubernetes |
| Simple setup | Docker Compose |
| Enterprise features | Kubernetes |

