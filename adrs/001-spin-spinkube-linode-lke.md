# Architecture Decision Record (ADR): Building Keel on Spin + Linode LKE via SpinKube

## Context

Keel is a composable SaaS architecture built on the **WASI Component Model**, enabling reusable business capabilities across applications. A critical requirement is **developer experience (DX)**: developers should be able to quickly build, test, and deploy new capabilities without friction. Deployment must also support **global scale**, **low latency**, and **high availability**, while avoiding AWS for philosophical and DX reasons.

Several deployment options were evaluated:
- **Fermyon Cloud / Fermyon Wasm Functions**: Strong alignment with Spin and WASI, but less direct control over infrastructure and scaling.
- **GKE Autopilot (Google Cloud)**: Excellent autoscaling, but CLI and ecosystem too noisy and overwhelming.
- **Fly.io FKS (K8s)**: Great DX vision, but currently in beta with missing features (CronJobs, HPA, sidecars).
- **Linode Kubernetes Engine (LKE)**: Full Kubernetes compliance, simple and clear CLI (linode-cli), transparent pricing, good multi-region support.

Given Keel’s architectural commitment to **Spin** and **WASI**, and the need for a production-ready Kubernetes target with exceptional DX, the decision is to standardize on:
- **Spin for app development and component composition.**
- **SpinKube for Kubernetes integration.**
- **Linode LKE clusters for regional deployment.**

This combination balances **DX simplicity**, **infra transparency**, and **global scalability**.

## Decision

Keel will be built as a set of **Spin applications**, deployed to **Linode LKE clusters** using **SpinKube**. Global deployments will consist of multiple LKE clusters in different regions, fronted by geo-aware DNS to automatically route users to the nearest cluster.

## Developer Workflow Story

### 1. Initialize a new Spin App Repository

```bash
# Install Spin CLI
dev setup: curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash

# Create new app
spin new http-rust my-service
cd my-service

# Define components and WIT interfaces in spin.toml
```

### 2. Local Development & Testing

```bash
# Run locally
spin build
spin up

# Test endpoints
curl http://127.0.0.1:3000/hello
```

### 3. Add Kubernetes Support via SpinKube

```bash
# Install SpinKube plugin
spin plugin install kube

# Scaffold K8s manifests for app
spin kube scaffold > my-service.yaml
```

### 4. Provision Linode LKE Cluster

```bash
# Authenticate
linode-cli configure

# Create cluster
linode-cli lke cluster-create \
  --label keel-cluster-us \
  --region us-central \
  --k8s_version 1.30 \
  --node_pools.type=g6-standard-4 \
  --node_pools.count=2

# Fetch kubeconfig
linode-cli lke kubeconfig-view <cluster-id> > kubeconfig-us.yaml
export KUBECONFIG=$(pwd)/kubeconfig-us.yaml

kubectl get nodes   # verify cluster access
```

### 5. Install SpinKube Operator

```bash
helm repo add spinkube https://spinkube.github.io/helm-charts
helm repo update

kubectl create namespace spinkube
helm install spinkube-operator spinkube/spinkube --namespace spinkube
```

### 6. Deploy Spin App to LKE

```bash
# Deploy app manifests
kubectl apply -f my-service.yaml

# Verify
kubectl get spinapps
kubectl get pods
```

### 7. Expose Service via Linode Load Balancer

```yaml
# my-service-lb.yaml
apiVersion: v1
kind: Service
metadata:
  name: my-service-lb
spec:
  type: LoadBalancer
  ports:
    - port: 80
      targetPort: 80
  selector:
    core.spinkube.dev/app-name: my-service
```
```bash
kubectl apply -f my-service-lb.yaml
kubectl get svc my-service-lb   # wait for EXTERNAL-IP
```

### 8. Enable Autoscaling

```bash
kubectl autoscale deployment my-service \
  --cpu-percent=70 \
  --min=1 --max=10
```

### 9. Deploy Additional Regions

```bash
# Repeat steps 4–8 for each region
e.g. us-east, eu-central, ap-southeast

# Use separate kubeconfigs
linode-cli lke kubeconfig-view <cluster-id> > kubeconfig-eu.yaml
export KUBECONFIG=$(pwd)/kubeconfig-eu.yaml
```

### 10. Configure Global Routing

- Use a geo-aware DNS service (e.g. Linode DNS, Cloudflare) to direct users to the closest regional cluster based on latency or geographic location.
- Example: `keel.example.com` resolves to different `my-service-lb` IPs depending on region.

### 11. Observability & Logs

```bash
# View pod logs
kubectl logs -l core.spinkube.dev/app-name=my-service

# Metrics (if using Prometheus)
kubectl port-forward svc/prometheus 9090:9090
```

## Consequences

- **DX Gains**: Developers interact with Spin + SpinKube + linode-cli only; no GCP/AWS console clutter. App lifecycle is consistent from local dev to global deployment.
- **Global HA**: Multiple LKE clusters provide regional redundancy. DNS-level routing ensures users hit the nearest cluster.
- **Scalability**: Kubernetes HPA ensures components scale dynamically. Each cluster scales independently.
- **Portability**: Spin apps remain provider-agnostic. If LKE becomes unsuitable, workloads can migrate to any other K8s provider.
- **Operational Overhead**: Slightly higher than fully managed serverless (Fermyon Cloud/Wasm Functions), since we manage K8s clusters. But offset by simplicity of LKE and clarity of linode-cli.

## Future Alternatives

- **Fermyon Cloud / Wasm Functions**: If Fermyon evolves into a more mature global deployment platform with strong observability and pricing clarity, we could simplify infra further by deploying directly to their managed runtime. This would reduce ops overhead but at the cost of infra control.
- **Fly.io FKS**: Once FKS matures (adds HPA, CronJobs, sidecars), it could become the best DX for global Kubernetes clusters. Keel could migrate to Fly for built-in global routing and edge-friendly infra.
- **GKE Autopilot**: If DX pain is reduced (e.g., simplified CLI workflows, reduced service sprawl), GKE Autopilot offers the most battle-tested global autoscaling. A fallback option if Linode’s global footprint proves insufficient.

---

## Status
**Accepted** — Keel will standardize on Spin apps deployed to Linode LKE via SpinKube for initial global deployments.
