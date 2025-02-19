FROM rust:latest AS builder

EXPOSE 12000

RUN apt-get update && apt-get install -y openssl libssl-dev libpq-dev

WORKDIR /usr/src/payments
COPY payments/Cargo.toml ./
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY payments/.env ./
COPY . .

RUN cargo build -p payments --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl libssl-dev libpq-dev
COPY --from=builder /usr/src/payments/target/release/payments /usr/local/bin/payments
COPY --from=builder /usr/src/payments/.env /.env

CMD ["payments"]