FROM rust:1.65.0 as builder
WORKDIR /usr/src/axum-quasar
COPY . .
RUN cargo install --path .



FROM debian:stable-slim
COPY --from=builder /usr/local/cargo/bin/axum-quasar /usr/local/bin/axum-quasar
COPY --from=builder /usr/src/axum-quasar/quasar-project/dist/spa /usr/local/bin/quasar-project/dist/spa

WORKDIR /usr/local/bin
CMD ["axum-quasar"]