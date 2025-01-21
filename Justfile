set dotenv-required

build-dev:
  docker compose -f docker-compose.dev.yaml build

dev: build-dev
  docker compose -f docker-compose.dev.yaml up -V
