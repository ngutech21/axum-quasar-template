FROM rust:1.65.0 as builder

WORKDIR /usr/src/axum-quasar
COPY . .

RUN cargo install --path .



FROM ubuntu
RUN apt-get update && apt-get install -y 
COPY --from=builder /usr/local/cargo/bin/axum-quasar /usr/local/bin/axum-quasar
COPY --from=builder /usr/src/axum-quasar/quasar-project /usr/local/bin/quasar-project

WORKDIR /usr/local/bin
CMD ["axum-quasar"]