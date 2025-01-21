# Liege Bot

A multipurpose Discord Bot written in Rust which can be added to your user or a guild. It is
webhook-based meaning it does not connect to the gateway and can be added to the User.

## Features

Since the bot is still under heavy development, some features may not exist yet

- AI Chat / AI Image using an API
- Sandboxed Code Execution in various languages
- Economy and Gambling
- Activity versions of the AI and Code Execution features

## Environment

You will need to set environment variables using a `.env` file. You can see the required variables
in the [`.env.example`](.env.example) file. You can copy the example environment like this:

```shell
cp .env.example .env
```

## Development

Since development is done under docker, there is no need to install any dependencies specific for
this project, except for `docker` and `just`.

### Registering commands

To make the commands show up on Discord, you will need to register them with the Discord API. This
needs to be run again every time commands are changed.

```shell
just register-commands
```

Now, all of the commands should be registered on Discord. Please note that you may need to reload
or restart your app for these changes to take effect.

### Starting

You can start the development server using the following command:

```shell
just dev
```

This will start a webserver at [`http://localhost:8700`](http://localhost:8700)

### Tunneling

You will need a tunneling service like `cloudflared` to make it availible to Discord for the
Interactions Endpoint and the Activity.

```shell
$ cloudflared tunnel --url http://localhost:8700
#   Your quick Tunnel has been created! Visit it at (it may take some time to be reachable):
#   https://example.trycloudflare.com
```

Afterwards, on the Discord Developer Portal, set...

- the **Interactions Endpoint URL** to
  `https://example.trycloudflare.com/api/interactions`
- the **Root Mapping** to `example.trycloudflare.com` <sub>_(not including a protocol or path)_</sub>

## Production

To create a production instance, there is a docker image and docker compose file availible. You can download the compose file using the following command:

```shell
curl -fsSL https://github.com/sinjs/liege-bot/raw/refs/heads/master/docker-compose.yaml -o docker-compose.yaml
```

Afterwards, you will need to register the commands to the Discord API. This will need to be re-run
every time the command structure is changed:

```shell
docker compose run backend register-commands
```

Finally, use docker compose to start the bot:

```shell
docker compose up -d
```

This will start a webserver at [`http://localhost:8700`](http://localhost:8700)

### Updating

To update the production environment to the latest version, you can use the following commands:

```shell
docker compose pull
docker compose up --force-recreate --build -d
```
