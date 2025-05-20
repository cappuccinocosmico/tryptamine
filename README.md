Next steps to get up and running:

```bash
# 1) Add cert-manager repo and update dependencies
helm repo add jetstack https://charts.jetstack.io
helm repo update
cd charts/blog
helm dependency update          # pulls cert-manager into charts/blog/charts/

# 2) Deploy cert-manager (if not already installed)
helm install cert-manager jetstack/cert-manager \
  --namespace cert-manager --create-namespace \
  --version v1.8.0 \
  --set installCRDs=true

# 3) Install your blog chart
# On a real cluster (production):
helm upgrade --install blog --namespace blog --create-namespace -f values.yaml

# On Minikube:
helm upgrade --install blog --namespace blog --create-namespace -f values-minikube.yaml
