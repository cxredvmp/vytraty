# Vytraty

Vytraty is a REST API for an expense tracker web application.

## Guide

### Clone the repository

```sh
git clone https://github.com/cxredvmp/vytraty
cd vytraty
```

### Set environment variables

```sh
cp .env.example .env
```

### Build and run

```sh
podman build --security-opt label=disable -t vytraty .
podman compose up
```

### Test

```sh
cargo test
```

### OpenAPI documentation

Swagger is accessible at `http://localhost:8080/swagger-ui/`.
