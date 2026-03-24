# Kubernetes Deployment

This directory contains all Kubernetes-related files for deploying the blockchain network.

## Directory Structure

```
kubernetes/
├── README.md                      # This file - quick start guide
└── manifests/                     # Kubernetes manifest files
    ├── 01-namespace.yaml         # Namespace
    ├── 02-configmap.yaml         # Configuration
    ├── 14-configmap-rate-limit.yaml # Rate limiting settings (Settings.toml)
    ├── 03-secrets.yaml           # Secrets (API keys)
    ├── 04-pvc-miner.yaml         # (Legacy/optional) old PVC approach
    ├── 05-pvc-webserver.yaml     # (Legacy/optional) old PVC approach
    ├── 15-redis.yaml             # Redis (rate limiting backend)
    ├── 06-statefulset-miner.yaml # Miner StatefulSet (chain topology)
    ├── 07-deployment-webserver.yaml # Webserver StatefulSet (per-pod storage)
    ├── 08-service-miner-headless.yaml  # Miner headless service (for StatefulSet)
    ├── 08-service-miner.yaml     # Miner service (load balanced)
    ├── 09-service-webserver-headless.yaml # Webserver headless service (for StatefulSet)
    ├── 09-service-webserver.yaml  # Webserver service
    ├── 10-hpa-webserver.yaml     # Webserver autoscaler
    ├── 11-hpa-miner.yaml         # Miner autoscaler
    ├── 12-pod-disruption-budget.yaml # High availability
    ├── 13-network-policy.yaml    # Network security (optional)
    ├── deploy.sh                 # Deployment script
    ├── undeploy.sh               # Cleanup script
    └── kustomization.yaml        # Kustomize configuration
```

## Quick Start

### Prerequisites

- Kubernetes cluster (Minikube, cloud provider, or self-hosted)
- kubectl configured and connected to cluster
- Docker image built (see below)

### Step 1: Build Docker Image

**For Minikube:**
```bash
eval $(minikube docker-env)
# IMPORTANT: build from the repository root (build context), because the Dockerfile
# uses COPY paths like `ci/docker-compose/configs/...`.
cd /path/to/repo/root
docker build -t blockchain-node:latest -f Dockerfile .
```

**For Cloud/Registry:**
```bash
cd /path/to/repo/root
docker build -t your-registry/blockchain-node:v1.0.0 -f Dockerfile .
docker push your-registry/blockchain-node:v1.0.0
# Then update image in manifests/06-statefulset-miner.yaml and 07-deployment-webserver.yaml
```

### Step 2: Update Configuration

Edit `manifests/03-secrets.yaml` to set your API keys. The mining address is optional.
```yaml
stringData:
  BITCOIN_API_ADMIN_KEY: "your-admin-key"
  BITCOIN_API_WALLET_KEY: "your-wallet-key"
  # Optional: Mining address (wallet address).
  #
  # If omitted, the container entrypoint will auto-create a wallet address
  # and persist it in the pod's wallets volume.
  # MINER_ADDRESS: ""
```

### Step 2b: Rate Limiting (Redis + Settings.toml)

The webserver uses Redis-backed rate limiting via `axum_rate_limiter`.

- Redis is deployed in-cluster (`manifests/15-redis.yaml`)
- The webserver reads a `Settings.toml` from a ConfigMap (`manifests/14-configmap-rate-limit.yaml`)
  and uses `RL_SETTINGS_PATH=/app/Settings.toml`.

To change limits/strategies, edit `manifests/14-configmap-rate-limit.yaml` and restart the webserver:

```bash
kubectl rollout restart statefulset/webserver -n blockchain
```

### Step 3: Deploy

```bash
cd manifests
./deploy.sh
```

### Step 4: Verify

```bash
kubectl get pods -n blockchain
kubectl get svc -n blockchain
kubectl get hpa -n blockchain
```

## Documentation

All comprehensive documentation is organized in the [`book-draft/ci/kubernetes/`](../../book-draft/ci/kubernetes/) directory.

**Quick Links:**
- **[Complete Guide](../../book-draft/ci/kubernetes/README.md)** - ⭐ **Comprehensive guide with introduction, architecture, migration, and operations**
- **[Architecture & Core Concepts](../../book-draft/ci/kubernetes/02-Architecture.md)** - System architecture and design
- **[Migration Guide](../../book-draft/ci/kubernetes/03-Migration.md)** - Migrating from Docker Compose to Kubernetes
- **[Kubernetes Manifests](../../book-draft/ci/kubernetes/04-Manifests.md)** - Detailed manifest documentation
- **[Deployment & Operations](../../book-draft/ci/kubernetes/05-Deployment.md)** - Deployment procedures and operations
- **[Autoscaling](../../book-draft/ci/kubernetes/06-Autoscaling.md)** - Autoscaling implementation guide
- **[Production & Advanced Topics](../../book-draft/ci/kubernetes/07-Production.md)** - Production best practices

## Key Features

- **Native autoscaling**: HPA (Horizontal Pod Autoscaler) scales based on CPU/Memory
- **Service discovery**: DNS-based service discovery (`miner-service.blockchain.svc.cluster.local`)
- **Rolling updates**: Zero-downtime deployments with automatic rollback
- **Self-healing**: Automatic pod restart on failure
- **Resource management**: CPU/Memory limits and requests for efficient resource usage
- **Multi-node support**: Distribute pods across cluster nodes
- **Persistent storage**: PVCs for data and wallet persistence
- **High availability**: Pod Disruption Budgets ensure minimum availability
- **Isolated webserver storage**: Webservers run as a StatefulSet with per-pod storage (each webserver has its own blockchain DB)

## Architecture

```
┌───────────────────────────────────────┐
│         Kubernetes Cluster            │
│                                       │
│  ┌──────────────┐  ┌────────────-──┐  │
│  │   Miner Pods │  │ Webserver Pods│  │
│  │   (2+)       │  │   (2+)        │  │
│  └──────┬───────┘  └──────┬──────-─┘  │
│         │                 │           │
│  ┌──────▼─────────────────▼─────--─┐  │
│  │      Services                   │  │
│  │  miner-service (ClusterIP)      │  │
│  │  webserver-service (LB)         │  │
│  └──────────────────────────────--─┘  │
│                                       │
│  ┌─────────────────────────────--──┐  │
│  │  HPA (Autoscalers)              │  │
│  │  Monitors CPU/Memory            │  │
│  │  Scales pods automatically      │  │
│  └──────────────────────────────--─┘  │
└───────────────────────────────────────┘
```

## Common Commands

```bash
# Deploy all resources (recommended)
cd manifests && ./deploy.sh

# Check status
kubectl get all -n blockchain

# View logs
kubectl logs -n blockchain -l app=webserver -f

# Scale manually
kubectl scale statefulset miner -n blockchain --replicas=3
kubectl scale statefulset webserver -n blockchain --replicas=5

# Update image
kubectl set image statefulset/webserver blockchain-node=blockchain-node:v1.1.0 -n blockchain

# Port forward (for local access)
kubectl port-forward -n blockchain svc/webserver-service 8080:8080
```

## Next Steps

1. Read the [Complete Guide](../../book-draft/ci/kubernetes/README.md) for comprehensive documentation
2. Review [Kubernetes Manifests](../../book-draft/ci/kubernetes/04-Manifests.md) for detailed manifest documentation
3. See [Migration Guide](../../book-draft/ci/kubernetes/03-Migration.md) for migrating from Docker Compose
4. Check [Production Guide](../../book-draft/ci/kubernetes/07-Production.md) for production best practices
5. Configure monitoring and alerting for production

