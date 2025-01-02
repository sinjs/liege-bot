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

The following environment variables will need to be set, for example by using a `.env` file:

```shell
AI_TOKEN=x           # API token for the AI API
CODE_TOKEN=x         # API token for the code execution API
DISCORD_PUBLIC_KEY=x # Discord application public key
DISCORD_APP_ID=x     # Discord application / client ID
DISCORD_TOKEN=x      # Discord bot token
```

## Local Development

To run the bot locally, you will need some sort of reverse proxy, for example `ngrok`.

1. Register the commands to the Discord API. This will need to be re-run every time the command
   structure is changed

   ```shell
   cargo run -- register-commands
   ```

   If you only want to register commands for a specific guild, you may use the `-g <GUILD_ID>` flag.
   For more information about command line arguments, use `--help`.

2. Start the bot and interaction handler

   ```shell
   cargo run -- run
   ```

   The server will be running on `http://localhost:8787`.

3. Start the reverse proxy, for example using ngrok

   ```shell
   ngrok http 8787
   ```

4. Set the Interactions Endpoint URL on the Discord Developer Portal to the reverse proxy URL, for
   example `https://quickest-whole-hotel.ngrok-free.app/interactions`.
