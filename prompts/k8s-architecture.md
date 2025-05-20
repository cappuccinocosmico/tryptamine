# Helm Chart Architecture & Layout Proposal

Below is an outline of how we’ll structure the repository, chart, scripts, and documentation for serving your blog via Kubernetes (with Hel
m), pushing images, managing Let’s Encrypt certificates, and supporting both remote clusters and local Minikube usage.

---

## 1. Repository Layout

```
├── build_docker_images.sh
├── charts/
│   └── blog/
│       ├── Chart.yaml
│       ├── values.yaml
│       ├── values-minikube.yaml
│       ├── templates/
│       │   ├── _helpers.tpl
│       │   ├── deployment.yaml
│       │   ├── service.yaml
│       │   ├── ingress.yaml
│       │   └── k8s-crds.yaml        ← (Cert-manager ClusterIssuer)
│       └── charts/                 ← (Helm dependency: cert-manager)
├── ci/                           ← (optional CI/CD definitions)
│   ├── pipeline.yaml
│   └── registry-creds-secret.yaml
├── prompts/
│   └── helm-ideas.md
└── README.md
```

---

## 2. High-Level Component Responsibilities

- **build_docker_images.sh**  
  - Builds the blog’s Docker image (tagged with commit SHA or semver).  
  - Pushes it to a remote registry (e.g. Docker Hub, ECR, GCR).  

- **charts/blog/**  
  - **Chart.yaml**  
    - Chart metadata, version, dependencies (e.g. cert-manager).  
  - **values.yaml**  
    - Defaults for remote clusters (replicaCount: 2, image repository & tag, domain `an.mycor.io`, TLS via cert-manager).  
  - **values-minikube.yaml**  
    - Overrides for local dev: `replicaCount: 1`, imagePullPolicy: `IfNotPresent`, domain via nip.io or `minikube.local`, skip TLS or use self-signed.  
  - **templates/**  
    - **deployment.yaml**: 2 replicas of the blog container, resource requests/limits.  
    - **service.yaml**: ClusterIP or LoadBalancer for Minikube & cloud (use `type: NodePort` in Minikube override).  
    - **ingress.yaml**: Ingress resource with annotations for cert-manager and host `an.mycor.io`.  
    - **k8s-crds.yaml**: ClusterIssuer or Issuer CRD to auto-provision Let’s Encrypt certificates.  
    - **_helpers.tpl**: Template helpers for naming, labels, secret names.  

- **cert-manager (as Helm dependency)**  
  - Manages Let’s Encrypt ACME challenge and certificate renewal.  


---

## 3. Key Design Decisions

1. **Replica Count**  
   - Default to 2 pods for production; 1 pod for local dev.  

2. **Domain & TLS**  
   - Use `an.mycor.io` as the hosted domain.  
   - Leverage cert-manager with a ClusterIssuer pointing to Let’s Encrypt (production and staging).  
   - Ingress is annotated to request certificates automatically.  

3. **Local Minikube Support**  
   - A separate `values-minikube.yaml` to override domain (e.g. `$(minikube ip).nip.io`), disable or use internal certificate, and adjust s
ervice type.  
   - Instructions in `prompts/helm-ideas.md` for installing cert-manager locally (or skipping it) and running `minikube tunnel`.  

4. **Docker Build & Push Script**  
   - `build_docker_images.sh` takes arguments for `--tag`, `--registry`, and optionally `--push`.  
   - Uses `docker build`, `docker tag`, and `docker push`.  
   - Emits an image digest for use in Helm values.  
