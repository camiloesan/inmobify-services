FROM rust:latest AS builder

EXPOSE 12000

RUN apt-get update && apt-get install -y openssl libssl-dev

WORKDIR /usr/src/database_logger
COPY database_logger/Cargo.toml ./
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY . .

RUN cargo build -p database_logger --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl libssl-dev
COPY --from=builder /usr/src/database_logger/target/release/database_logger /usr/local/bin/database_logger

CMD ["database_logger"]