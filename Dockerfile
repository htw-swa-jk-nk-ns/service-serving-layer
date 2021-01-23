# syntax=docker/dockerfile:experimental
FROM rust as cacher

WORKDIR service-serving-layer
RUN mkdir .cargo
ENV CARGO_HOME /service-serving-layer/.cargo
# cache
ENV USER root
# Init an empty project
RUN cargo init .
COPY Cargo.lock .
COPY Cargo.toml .
# Build dependency
RUN --mount=type=cache,target=/service-serving-layer/.cargo/registry \
    --mount=type=cache,target=/service-serving-layer/release \
    cargo build --release
RUN rm -r src

FROM rust as builder
WORKDIR service-serving-layer
ENV CARGO_HOME /service-serving-layer/.cargo
# Copy files to container and build
# cache until here
COPY --from=cacher /service-serving-layer/* .
COPY src .
RUN cargo build --release
# Install dependencies, build, install as a binary under the name service-serving-layer and link to $PATH
RUN cargo install --path .

# Run
FROM debian:buster
RUN apt-get update && apt-get --yes install openssl
RUN openssl version
COPY --from=builder /usr/local/cargo/bin/service-serving-layer /usr/local/bin/service-serving-layer
CMD ["service-serving-layer"]