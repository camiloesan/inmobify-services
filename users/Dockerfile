FROM rust:latest AS builder

EXPOSE 12000

RUN apt-get update && apt-get install -y openssl libssl-dev libpq-dev

WORKDIR /usr/src/users
COPY users/Cargo.toml ./
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY users/.env ./
COPY . .

RUN cargo build -p users --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl libssl-dev libpq-dev
COPY --from=builder /usr/src/users/target/release/users /usr/local/bin/users
COPY --from=builder /usr/src/users/.env /.env

CMD ["users"]