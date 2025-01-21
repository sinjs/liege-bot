# All development recipes and scripts are defined in this file
# For production, you should not use any of these recipes

set dotenv-required

alias d := dev
alias r := register-commands

build-dev:
  docker compose -f docker-compose.dev.yaml build

dev: build-dev
  docker compose -f docker-compose.dev.yaml up -V --remove-orphans

register-commands: build-dev
  docker compose -f docker-compose.dev.yaml run backend-dev cargo run register-commands
