FROM rust:1.66.0-buster as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.66.0-buster as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

FROM rust:1.66.0-buster as develop
WORKDIR /app
EXPOSE 8080
COPY --from=cacher /app/target target
RUN cargo install cargo-watch
CMD cargo watch -x "run --bin main"
