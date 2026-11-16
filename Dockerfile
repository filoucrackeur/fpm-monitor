# syntax=docker/dockerfile:1

FROM rust:1-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

FROM scratch

LABEL org.opencontainers.image.title="fpm-monitor"
LABEL org.opencontainers.image.description="PHP-FPM pool monitor with an interactive terminal dashboard"
LABEL org.opencontainers.image.source="https://github.com/filoucrackeur/fpm-monitor"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.url="https://github.com/filoucrackeur/fpm-monitor"
LABEL org.opencontainers.image.documentation="https://github.com/filoucrackeur/fpm-monitor#readme"

COPY --from=builder /app/target/release/fpm-monitor /usr/local/bin/fpm-monitor
USER 65534:65534
ENTRYPOINT ["/usr/local/bin/fpm-monitor"]
