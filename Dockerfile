FROM rust:1.49 as base
WORKDIR /app
RUN cargo install cargo-build-deps

FROM base as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo --frozen --locked build-deps --release
COPY src /app/src
RUN cargo build --frozen --locked --release --bin sample-rust-tide

FROM debian:buster-slim as runtime
WORKDIR /app
COPY --from=builder /app/target/release/sample-rust-tide /app/sample-rust-tide
COPY --from=builder /app/templates /app/templates
ENTRYPOINT ["/app/sample-rust-tide"]
