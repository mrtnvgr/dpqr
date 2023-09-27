FROM rust:slim as build

RUN apt update && apt install -y pkg-config libssl-dev

WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:unstable-slim

RUN apt update && apt install -y pkg-config libssl-dev

WORKDIR /app

COPY --from=build /app/target/release/dpqr .
COPY --from=build /app/www .
CMD ["./dpqr"]
