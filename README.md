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
