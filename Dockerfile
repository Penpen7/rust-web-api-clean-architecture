FROM rust:1.66.0-buster

WORKDIR /home/rust
COPY . .

RUN cargo build
CMD cargo run
