FROM rust:1-slim-buster AS build

RUN cargo new --bin app

WORKDIR /app

COPY Cargo.toml /app/
COPY Cargo.lock /app/
RUN cargo build --release

COPY src /app/src
RUN touch src/main.rs
RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /app/target/release/rinha_de_backend_rust /app/rinha_de_backend_rust

CMD "/app/rinha_de_backend_rust"
