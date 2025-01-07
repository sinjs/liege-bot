# Build Stage
FROM rust:1.83 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Build dependencies first for caching
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
  cargo build --release && \
  rm -rf src

COPY . .

RUN cargo build --release

# Run stage
FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /app/target/release/liege-bot /app/liege-bot

EXPOSE 8787
CMD ["/app/liege-bot"]
