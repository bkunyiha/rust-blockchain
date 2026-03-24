#!/bin/bash
# Deployment script for Kubernetes blockchain network
# Usage: ./deploy.sh [environment]
# Example: ./deploy.sh production

set -e

ENVIRONMENT=${1:-default}

echo "Deploying blockchain network to Kubernetes..."
echo "Environment: ${ENVIRONMENT}"
echo ""

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "Error: kubectl is not installed or not in PATH"
    exit 1
fi

# Check if connected to cluster
if ! kubectl cluster-info &> /dev/null; then
    echo "Error: Not connected to Kubernetes cluster"
    echo "Please configure kubectl to connect to your cluster"
    exit 1
fi

echo "Cluster information:"
kubectl cluster-info | head -1
echo ""

# Deploy in order
echo "Step 1: Creating namespace..."
kubectl apply -f 01-namespace.yaml

echo "Step 2: Creating configuration..."
kubectl apply -f 02-configmap.yaml
kubectl apply -f 14-configmap-rate-limit.yaml
kubectl apply -f 03-secrets.yaml

echo "Step 3: Creating storage..."
kubectl apply -f 04-pvc-miner.yaml

echo "Step 4: Creating rate limiting backend (Redis)..."
kubectl apply -f 15-redis.yaml

echo "Step 5: Creating StatefulSet and Deployment..."
kubectl apply -f 06-statefulset-miner.yaml
kubectl apply -f 09-service-webserver-headless.yaml

# If you're upgrading from the old setup (webserver Deployment -> StatefulSet),
# remove the old Deployment so it doesn't keep spawning pods that share PVCs.
kubectl delete deployment webserver -n blockchain --ignore-not-found=true || true

kubectl apply -f 07-deployment-webserver.yaml

echo "Step 6: Creating services..."
kubectl apply -f 08-service-miner-headless.yaml
kubectl apply -f 08-service-miner.yaml
kubectl apply -f 09-service-webserver.yaml

echo "Step 7: Creating autoscalers..."
kubectl apply -f 10-hpa-webserver.yaml
kubectl apply -f 11-hpa-miner.yaml

echo "Step 8: Creating disruption budgets..."
kubectl apply -f 12-pod-disruption-budget.yaml

echo ""
echo "Deployment complete!"
echo ""
echo "Waiting for pods to be ready..."
kubectl wait --for=condition=ready pod -l app=miner -n blockchain --timeout=300s || true
kubectl wait --for=condition=ready pod -l app=webserver -n blockchain --timeout=300s || true

echo ""
echo "Current status:"
kubectl get pods -n blockchain
echo ""
kubectl get svc -n blockchain
echo ""
kubectl get hpa -n blockchain

echo ""
echo "To view logs:"
echo "  kubectl logs -n blockchain -l app=miner -f"
echo "  kubectl logs -n blockchain -l app=webserver -f"
echo ""
echo "To access webserver:"
echo "  kubectl port-forward -n blockchain svc/webserver-service 8080:8080"
echo "  Then open http://localhost:8080"

