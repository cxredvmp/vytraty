# Vytraty

Vytraty is a REST API for an expense tracker web application.

## Run

### Clone the repository:

```sh
git clone https://github.com/cxredvmp/vytraty
cd vytraty
```

### Set environment variables:

```sh
cp .env.example .env
```

### Run the application:

```sh
podman build --security-opt label=disable -t vytraty .
podman compose up
```
