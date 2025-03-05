FROM rust:latest AS builder

EXPOSE 12000

RUN apt-get update && apt-get install -y openssl libssl-dev

WORKDIR /usr/src/file_server
COPY file_server/Cargo.toml ./
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY . .

RUN cargo build -p file_server --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl libssl-dev
COPY --from=builder /usr/src/file_server/target/release/file_server /usr/local/bin/file_server

CMD ["file_server"]