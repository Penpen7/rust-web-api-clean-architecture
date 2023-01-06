FROM rust:1.66.0-buster

WORKDIR /home/rust
COPY . .

RUN cargo install cargo-watch && cargo build
CMD cargo watch -x run
