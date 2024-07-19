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

## Manually provision and deprovision App

Push image: 
```
bazel run //oci:push_rust_app_server_image
```

Deploy the container app:
```
az deployment sub create -n rust-app-deployment --location francecentral --template-file subscription.bicep \
    --parameters hashSecret=$HASH_SECRET jwtSecret=$JWT_SECRET databaseUrl=$DATABASE_URL registryPassword=$READ_PACKAGE_PAT
```

Undeploy the container app:
```
az deployment sub delete -n rust-app-deployment
```

## Bazel build 

cd third-party && rm Cargo.lock && cargo generate-lockfile && cd ..
bazel run //third-party:vendor
bazel build //crates/webapp:webapp

## Add pre-commit hook

Edit the file ./git/hooks/pre-commit :
``` bash
#!/bin/sh

set -e

if ! bazel build //crates/webapp:webapp_clippy
then
    echo "There are some clippy issues. Please check with"
    echo "  bazel build //crates/webapp:webapp_clippy"
    exit 1
fi

exit 0
```
