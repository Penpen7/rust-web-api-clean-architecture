FROM rust:1.66.0-buster

WORKDIR /app
COPY Cargo.* /app
COPY ./src /app/src
RUN cargo install cargo-watch && cargo build

WORKDIR /app
EXPOSE 8080
CMD cargo watch -x "run -v --bin main"
