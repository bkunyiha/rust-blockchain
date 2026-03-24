#!/bin/bash
# Undeployment script for Kubernetes blockchain network
# Usage: ./undeploy.sh

set -e

echo "Undeploying blockchain network from Kubernetes..."
echo ""

# Delete resources in reverse order
echo "Deleting disruption budgets..."
kubectl delete -f 12-pod-disruption-budget.yaml --ignore-not-found=true

echo "Deleting autoscalers..."
kubectl delete -f 11-hpa-miner.yaml --ignore-not-found=true
kubectl delete -f 10-hpa-webserver.yaml --ignore-not-found=true

echo "Deleting services..."
kubectl delete -f 09-service-webserver.yaml --ignore-not-found=true
kubectl delete -f 09-service-webserver-headless.yaml --ignore-not-found=true
kubectl delete -f 08-service-miner.yaml --ignore-not-found=true
kubectl delete -f 15-redis.yaml --ignore-not-found=true

echo "Deleting deployments..."
kubectl delete -f 07-deployment-webserver.yaml --ignore-not-found=true
kubectl delete -f 06-statefulset-miner.yaml --ignore-not-found=true

echo "Deleting storage..."
kubectl delete -f 04-pvc-miner.yaml --ignore-not-found=true

echo "Deleting configuration..."
kubectl delete -f 03-secrets.yaml --ignore-not-found=true
kubectl delete -f 14-configmap-rate-limit.yaml --ignore-not-found=true
kubectl delete -f 02-configmap.yaml --ignore-not-found=true

echo "Deleting namespace..."
kubectl delete -f 01-namespace.yaml --ignore-not-found=true

echo ""
echo "Undeployment complete!"
echo ""
echo "Note: PersistentVolumeClaims were deleted. Data may be lost unless backups were made."

