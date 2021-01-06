FROM rust as builder

# Copy files to container and build
WORKDIR /usr/src/serivce-serving-layer
COPY . .
# Install dependencies, build, install as a binary under the name service-serving-layer and link to $PATH
RUN cargo install --path .

# Run
FROM debian:buster
RUN apt-get update && apt-get --yes install openssl
RUN openssl version
COPY --from=builder /usr/local/cargo/bin/service-serving-layer /usr/local/bin/service-serving-layer
CMD ["service-serving-layer"]