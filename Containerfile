ARG APP_NAME=vytraty

# Builder stage
ARG RUST_VERSION=1.95.0
FROM rust:${RUST_VERSION}-slim AS builder

ARG APP_NAME
WORKDIR /app

# Leverage mounts to speed up the build process
RUN --mount=type=bind,source=.,target=/app,rw \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    set -e; \
    cargo build --locked --release && \
    cp ./target/release/${APP_NAME} /bin/app

# Runner stage
FROM gcr.io/distroless/cc:nonroot AS runner

COPY --from=builder /bin/app /bin/

CMD ["/bin/app"]
