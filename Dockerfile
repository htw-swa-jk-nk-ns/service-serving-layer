# syntax=docker/dockerfile:experimental
FROM rust as cacher
RUN mkdir app
WORKDIR app
ENV USER root
RUN cargo init .
RUN echo "fn main(){}" > src/lib.rs > src/lib.rs
COPY Cargo.lock .
COPY Cargo.toml .
RUN --mount=type=cache,target=/root/.cargo/registry cargo build --release


FROM rust as builder
# Copy files to container and build
WORKDIR /usr/src/serivce-serving-layer
COPY . .
COPY --from=cacher /app/target target
# RUN cargo build --release
# Install dependencies, build, install as a binary under the name service-serving-layer and link to $PATH
RUN cargo install --path . --verbose

# Run
FROM debian:buster
RUN apt-get update && apt-get --yes install openssl
RUN openssl version
COPY --from=builder /usr/local/cargo/bin/service-serving-layer /usr/local/bin/service-serving-layer
CMD ["service-serving-layer"]