import { REST } from "@discordjs/rest";
import { env, ExtendedSlashCommandBuilder, getCommands } from "./util";
import { Routes } from "discord-api-types/rest";
import {
  SlashCommandBuilder,
  type APIApplicationCommand,
  type APIApplicationCommandInteraction,
  type APIApplicationCommandInteractionDataOption,
  type APIInteractionResponse,
  type RESTPostAPIApplicationCommandsJSONBody,
  type SlashCommandOptionsOnlyBuilder,
  type SlashCommandSubcommandsOnlyBuilder,
} from "discord.js";

export type Command = {
  data:
    | SlashCommandBuilder
    | SlashCommandOptionsOnlyBuilder
    | SlashCommandSubcommandsOnlyBuilder;
  execute(
    command: APICommand
  ): APIInteractionResponse | Promise<APIInteractionResponse>;
};

export type APICommand = APIApplicationCommandInteraction & {
  data: { options?: APIApplicationCommandInteractionDataOption[] };
};

const { clientId, token } = {
  clientId: env("DISCORD_APPLICATION_ID"),
  token: env("DISCORD_TOKEN"),
};

const rest = new REST().setToken(token);
const commands = await getCommands(true);

console.log(`Started published ${commands.length} application (/) commands.`);

const data = (await rest.put(Routes.applicationCommands(clientId), {
  body: commands,
})) as APIApplicationCommand[];

console.log(`Successfully published ${data.length} application (/) commands.`);
