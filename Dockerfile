FROM rust:1.49 as base
WORKDIR /app
RUN cargo install cargo-build-deps

FROM base as builder
RUN echo $SECRET_API
WORKDIR /app
COPY Cargo.toml Cargo.lock /app/
RUN cargo build-deps --release
COPY src /app/src
COPY templates /app/templates
RUN cargo build --release --bin sample-rust-tide

FROM debian:buster-slim as runtime
WORKDIR /app
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/target/release/sample-rust-tide /app/sample-rust-tide
ENTRYPOINT ["/app/sample-rust-tide"]
