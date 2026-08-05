# syntax=docker/dockerfile:1

FROM rust:1-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM scratch
COPY --from=builder /app/target/release/fpm-monitor /usr/local/bin/fpm-monitor
USER 65534:65534
ENTRYPOINT ["/usr/local/bin/fpm-monitor"]
