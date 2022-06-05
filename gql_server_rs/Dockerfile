
FROM rust:slim as chef

# We use cargo chef for dependency caching
# https://www.lpalmieri.com/posts/fast-rust-docker-builds/
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json

# Pre-compile dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release


FROM debian:bullseye-slim as runtime
WORKDIR /app
COPY --from=builder /app/target/release/gql_server_rs /usr/local/bin
ENTRYPOINT ["/usr/local/bin/gql_server_rs"]
