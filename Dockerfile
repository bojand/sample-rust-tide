FROM rust:1.49 as base
WORKDIR /app
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef 

FROM base as planner
COPY . .
RUN cargo --frozen --locked chef prepare --recipe-path /app/recipe.json

FROM base as cacher
WORKDIR /app
COPY --from=planner /app/recipe.json /app/recipe.json
RUN cargo --frozen --locked chef cook --release --recipe-path /app/recipe.json

FROM rust:1.49 as builder
WORKDIR /app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target /app/target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --frozen --locked --release --bin sample-rust-tide

FROM debian:buster-slim as runtime
WORKDIR /app
COPY --from=builder /app/target/release/sample-rust-tide /usr/local/bin/sample-rust-tide
ENTRYPOINT ["/usr/local/bin/sample-rust-tide"]