# Build Stage
FROM rust:1.83-bookworm AS builder

WORKDIR /app
COPY . .
RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build --release && \
  cp ./target/release/liege-bot /

# Run stage
FROM debian:bookworm AS final
RUN apt-get update && apt-get install -y openssl

WORKDIR /app
COPY --from=builder /liege-bot /app/liege-bot
EXPOSE 8787
ENTRYPOINT ["/app/liege-bot"]
