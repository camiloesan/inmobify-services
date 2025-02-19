FROM rust:latest AS builder

EXPOSE 12000

RUN apt-get update && apt-get install -y openssl libssl-dev libpq-dev

WORKDIR /usr/src/appointments
COPY appointments/Cargo.toml ./
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY appointments/.env ./
COPY . .

RUN cargo build -p appointments --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl libssl-dev libpq-dev
COPY --from=builder /usr/src/appointments/target/release/appointments /usr/local/bin/appointments
COPY --from=builder /usr/src/appointments/.env /.env

CMD ["appointments"]