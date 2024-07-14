
# Run locally
run:
    cargo run

# Lint with clippy
lint:
    cargo clippy

# Test with curl (don’t forget to start the server first !) 
test:
    curl -v -X POST -H "Content-Type: application/json" -d '{"username":"jlc","password":"pass"}' http://localhost:8080/user

# Build with docker using 'latest' tag
build-docker:
    docker build -t rust-devops-azure-sample:latest .

# Push Image to ghcr.io (requires login)
push-docker:
    docker tag rust-devops-azure-sample:latest ghcr.io/jlcanela/rust-devops-azure-sample:0.0.1
    docker push ghcr.io/jlcanela/rust-devops-azure-sample:0.0.1

# Run webapp with docker
run-docker:
    docker run --env-file .env -p 8080:8080 rust-devops-azure-sample:latest

# Scan vulnerabilities with docker scout
security:
    docker scout cves local://rust-devops-azure-sample:latest
