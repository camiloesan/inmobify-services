FROM rust:latest AS builder

EXPOSE 12000

RUN apt-get update && apt-get install -y openssl libssl-dev libpq-dev

WORKDIR /usr/src/auth
COPY auth/Cargo.toml ./
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY auth/.env ./
COPY . .

RUN cargo build -p auth --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl libssl-dev libpq-dev
COPY --from=builder /usr/src/auth/target/release/auth /usr/local/bin/auth
COPY --from=builder /usr/src/auth/.env /.env

CMD ["auth"]