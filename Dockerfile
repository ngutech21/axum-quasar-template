# build frontend
FROM node:18-bullseye-slim as frontend-builder
WORKDIR /app
COPY quasar-project ./
RUN yarn global add @quasar/cli
RUN yarn
RUN quasar build



# build backend
FROM rust:1.65.0-slim-bullseye as rust-builder
WORKDIR /rust-app
COPY . /rust-app  
# ignore quasar-project
RUN cargo build --release



#FROM gcr.io/distroless/cc-debian11:debug
FROM debian:bullseye-slim
COPY --from=frontend-builder /app/dist/spa /app/quasar-project/dist/spa
COPY --from=rust-builder /rust-app/target/release/axum-quasar /app
COPY --from=rust-builder /rust-app/migrations /app/migrations
COPY --from=rust-builder /rust-app/dummy_data.json /app

WORKDIR /app
CMD ["./axum-quasar"]