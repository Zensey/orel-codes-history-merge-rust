FROM rust:1.41.0-alpine

ADD . /app/
WORKDIR /app

RUN cargo fetch;
RUN cargo test --release -- --nocapture
