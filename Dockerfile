FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
WORKDIR app

FROM chef AS planner
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./sqlx-data.json ./sqlx-data.json
COPY ./templates ./templates
COPY ./assets ./assets

RUN cargo build --release --bin blackboards

FROM debian:buster-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/blackboards /usr/local/bin
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/assets /app/assets
ENTRYPOINT ["/usr/local/bin/blackboards"]
