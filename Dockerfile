FROM rust:1.66.0-buster as chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

FROM rust:1.66.0-buster as develop
WORKDIR /app
EXPOSE 8080
RUN rustup component add clippy rustfmt
RUN cargo install cargo-watch
COPY --from=cacher /app/target target
CMD cargo watch -x fmt -x "run --bin main"
