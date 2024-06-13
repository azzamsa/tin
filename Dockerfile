# Use `scratch` to get more smaller image.
# Read [Tiny and Fast Docker image for Rust Application](https://azzamsa.com/n/rust-docker/)

ARG VCS_REVISION

FROM docker.io/lukemathwalker/cargo-chef:0.1.62-rust-1.78 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ARG VCS_REVISION
RUN VCS_REVISION=$VCS_REVISION cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/tin /
CMD ["./tin"]
