ARG APP_NAME=vytraty

# Builder stage
ARG RUST_VERSION=1.95.0
FROM rust:${RUST_VERSION}-slim AS builder

ARG APP_NAME
WORKDIR /app

RUN --mount=type=bind,target=/app \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --locked --release --verbose && \
    cp ./target/release/${APP_NAME} /bin/app

# Runner stage
FROM gcr.io/distroless/cc:nonroot AS runner

COPY --from=builder /bin/app /bin/

CMD ["/bin/app"]
