FROM lukemathwalker/cargo-chef:0.1.71-rust-1.88 AS chef
WORKDIR /app

# Planning Stage
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build Stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin liege-bot \
  && cp ./target/release/liege-bot /

# Run stage
FROM debian:bookworm AS final
RUN apt-get update && \
  apt-get install -y openssl ca-certificates && \
  update-ca-certificates

WORKDIR /app
COPY --from=builder /liege-bot /app/liege-bot
EXPOSE 8787
ENTRYPOINT ["/app/liege-bot"]
