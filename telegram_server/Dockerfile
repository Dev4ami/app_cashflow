FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

FROM ubuntu:latest

RUN apt-get update && apt-get install -y libssl-dev pkg-config ca-certificates && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/telegram_server /app/telegram_server

COPY .env .env

CMD ["./telegram_server"]
