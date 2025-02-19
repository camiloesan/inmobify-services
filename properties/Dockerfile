FROM rust:latest AS builder

EXPOSE 12000

RUN apt-get update && apt-get install -y openssl libssl-dev libpq-dev

WORKDIR /usr/src/properties
COPY properties/Cargo.toml ./
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY properties/.env ./
COPY . .

RUN cargo build -p properties --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl libssl-dev libpq-dev
COPY --from=builder /usr/src/properties/target/release/properties /usr/local/bin/properties
COPY --from=builder /usr/src/properties/.env /.env

CMD ["properties"]