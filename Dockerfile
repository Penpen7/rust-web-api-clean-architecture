FROM rust:1.66.0-buster

WORKDIR /app
COPY Cargo.* .
WORKDIR /app/src/bin
RUN echo "fn main(){}" >> main.rs

WORKDIR /app
RUN cargo install cargo-watch && cargo build
CMD cargo watch -x run
