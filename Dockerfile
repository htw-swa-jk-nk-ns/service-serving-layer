# syntax=docker/dockerfile:experimental
FROM rust as planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY . .
# Copy over cache
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release --bin service-serving-layer

FROM rust as runtime
RUN apt-get update && apt-get --yes install openssl
RUN openssl version
WORKDIR app
COPY --from=builder /app/target/release/service-serving-layer /usr/local/bin
CMD ["service-serving-layer"]