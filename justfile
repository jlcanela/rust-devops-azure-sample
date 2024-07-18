
# Run locally
run:
    cargo run

# Lint with clippy
lint:
    cargo clippy

# Test with curl (don’t forget to start the server first !) 
test:
    curl -v -X POST -H "Content-Type: application/json" -d '{"username":"jlc","password":"pass"}' http://localhost:8080/user

# Build webapp binary
build:
    bazel build //crates/webapp:webapp

# Build docker image and push to local registry
docker-build:
    bazel run //oci:rust_app_server_image_tarball

# Start docker image with shell
debug-image:
    #docker run -it --env-file .env -p 8080:8080 rust_app_server:latest /bin/sh
    docker run -it --env-file .env --entrypoint=/busybox/sh -p 8080:8080 rust_app_server:latest

# Update third party dependencies
update-deps:
    cd third-party && rm Cargo.lock && cargo generate-lockfile && cd ..
    bazel run //third-party:vendor

# Push Image to ghcr.io (requires login)
docker-push:
    docker tag rust-devops-azure-sample:latest ghcr.io/jlcanela/rust-devops-azure-sample:0.0.1
    docker push ghcr.io/jlcanela/rust-devops-azure-sample:0.0.1

# Run webapp with docker
docker-run:
    docker run -it --env-file .env --entrypoint=/hello_world -p 8080:8080 rust_app_server:latest
    #docker run --env-file .env -p 8080:8080 rust-devops-azure-sample:latest

# Scan vulnerabilities with docker scout
security:
    docker scout cves local://rust-devops-azure-sample:latest
