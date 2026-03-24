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
- Complete Guide: [`../book-draft/ci/docker-compose/01-Introduction.md`](../book-draft/ci/docker-compose/01-Introduction.md)

**All Chapters:**
- [Chapter 1: Introduction & Quick Start](../book-draft/ci/docker-compose/01-Introduction.md)
- [Chapter 2: Architecture & Container System](../book-draft/ci/docker-compose/02-Architecture.md)
- [Chapter 3: Execution Flow & Startup Process](../book-draft/ci/docker-compose/03-Execution-Flow.md)
- [Chapter 4: Network Configuration](../book-draft/ci/docker-compose/04-Network-Configuration.md)
- [Chapter 5: Port Mapping & External Access](../book-draft/ci/docker-compose/05-Port-Mapping.md)
- [Chapter 6: Scaling & Deployment](../book-draft/ci/docker-compose/06-Scaling.md)
- [Chapter 7: Sequential Startup](../book-draft/ci/docker-compose/07-Sequential-Startup.md)
- [Chapter 8: Deployment Scenarios](../book-draft/ci/docker-compose/08-Deployment-Scenarios.md)
- [Chapter 9: Accessing Webserver](../book-draft/ci/docker-compose/09-Accessing-Webserver.md)
- [Chapter 10: Deployment Guide](../book-draft/ci/docker-compose/10-Deployment-Guide.md)
- [Chapter 11: Deployment Execution Walkthrough](../book-draft/ci/docker-compose/11-Deployment-Execution-Walkthrough.md)
- [Chapter 12: DNS Resolution Mechanism](../book-draft/ci/docker-compose/12-DNS-Resolution-Mechanism.md)

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
- Complete Guide: [`../book-draft/ci/kubernetes/README.md`](../book-draft/ci/kubernetes/README.md)

**All Chapters:**
- [Chapter 1: Introduction & Overview](../book-draft/ci/kubernetes/README.md)
- [Chapter 2: Architecture](../book-draft/ci/kubernetes/02-Architecture.md)
- [Chapter 3: Migration Guide](../book-draft/ci/kubernetes/03-Migration.md)
- [Chapter 4: Manifests](../book-draft/ci/kubernetes/04-Manifests.md)
- [Chapter 5: Deployment](../book-draft/ci/kubernetes/05-Deployment.md)
- [Chapter 6: Autoscaling](../book-draft/ci/kubernetes/06-Autoscaling.md)
- [Chapter 7: Production](../book-draft/ci/kubernetes/07-Production.md)

## Choosing Between Docker Compose and Kubernetes

| Use Case | Recommendation |
|----------|---------------|
| Local development | Docker Compose |
| Single server deployment | Docker Compose |
| Production multi-node | Kubernetes |
| Need autoscaling | Kubernetes |
| Simple setup | Docker Compose |
| Enterprise features | Kubernetes |

