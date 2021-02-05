FROM rust:1.49 as base
WORKDIR /app
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-build-deps

FROM base as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo --frozen --locked build-deps --release
COPY src /app/src
RUN cargo build --frozen --locked --release

FROM debian:buster-slim as runtime
WORKDIR /app
COPY --from=builder /app/target/release/sample-rust-tide /usr/local/bin/sample-rust-tide
ENTRYPOINT ["/usr/local/bin/sample-rust-tide"]