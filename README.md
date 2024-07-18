[![Rust](https://github.com/jlcanela/rust-devops-azure-sample/actions/workflows/rust.yml/badge.svg)](https://github.com/jlcanela/rust-devops-azure-sample/actions/workflows/rust.yml)

# Install 'just'

Install 'just': 
```
cargo install just
```

List commands:
```
just -l
```

# Start the App

Run locally:
```
just run
```

# Test the App 

Run Clippy: 
```
just lint
```

Create a user with curl:
```
# just test
curl -v -X POST -H "Content-Type: application/json" -d '{"username":"jlc","password":"pass"}' http://localhost:8080/user
```

# Build & Run Docker Image

Build 'latest':
```
# just build-docker
docker build -t rust-devops-azure-sample:latest .
```

Run docker:
```
# just run-docker
docker run --env-file .env -p 8080:8080 rust-devops-azure-sample:latest
```

# Check docker vulnerabilities

List vulnerabilities:
```
# just security
docker scout cves local://rust-devops-azure-sample:latest
```

# Release

Run manually the release_draft.yml workflow specifying the new version. 
- Version must be semver, version bump is not compatible with previous version an error is issued
- If build and tag are ok, an image for 'main' branch is updated in ghcr.io

# Configure Azure

For more information, look at [Microsoft container apps](https://learn.microsoft.com/en-us/azure/container-apps/tutorial-code-to-cloud?tabs=bash%2Ccsharp&pivots=acr-remote) documentation.

## Setup Azure account

Login and register App & OperationalInsights providers:
```
az login
az account set --subscription <subscription-id>
az provider register -n Microsoft.App --wait
az provider register -n Microsoft.OperationalInsights --wait
```

Setup Credentials for entra-id
```
az ad sp create-for-rbac --name "SampleRustApplication" --role contributor --scopes /subscriptions/<subscription-id>/resourceGroups/default-rg --sdk-auth
```

## Provison and deprovison App

Provision ContainerApps:
```
az containerapp env create -n ProdRustEnv -g default-rg \
    --location eastus2 --enable-workload-profiles false

set -a
source .env
set +a

az containerapp create -n my-rust-app2 -g default-rg \
    --image ghcr.io/jlcanela/rust-azure-webapp-sample:main \
    --environment ProdRustEnv \
    --ingress external \
    --env-vars "HASH_SECRET=$HASH_SECRET" "JWT_SECRET=$JWT_SECRET" "DATABASE_URL=$DATABASE_URL" \
    --target-port 8080
```

Delete ContainerApp:
```
az containerapp delete -n my-rust-app -g default-rg
az containerapp env delete -n ProdRustEnv -g default-rg 
```


## Bazel build 

cd third-party && rm Cargo.lock && cargo generate-lockfile && cd ..
bazel run //third-party:vendor
bazel build //crates/webapp:hello_world



bazel build //...