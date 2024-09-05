import {
  APIVersion,
  ApplicationIntegrationType,
  EmbedBuilder,
  InteractionContextType,
  SlashCommandBuilder,
  type APIApplicationCommandInteractionDataOption,
  type APIApplicationCommandInteractionDataStringOption,
  type APIInteractionResponse,
  type ColorResolvable,
  type RESTPatchAPIWebhookJSONBody,
  type RESTPatchAPIWebhookWithTokenMessageJSONBody,
  type RESTPostAPIApplicationCommandsJSONBody,
} from "discord.js";

import { join } from "node:path";
import { readdir } from "node:fs/promises";
import type { APICommand, Command } from "./commands";
import ky from "ky";

export function env(key: string): string {
  const value = process.env[key];
  if (!value) throw new TypeError(`Environment variable ${key} must be set`);
  return value;
}

export class ExtendedSlashCommandBuilder extends SlashCommandBuilder {
  integrationTypes = [ApplicationIntegrationType.GuildInstall];
  contexts = [InteractionContextType.BotDM, InteractionContextType.Guild];

  makeUserCommand() {
    this.integrationTypes = [
      ApplicationIntegrationType.GuildInstall,
      ApplicationIntegrationType.UserInstall,
    ];
    this.contexts = [
      InteractionContextType.BotDM,
      InteractionContextType.Guild,
      InteractionContextType.PrivateChannel,
    ];

    return this;
  }

  setIntegrationTypes(integrationTypes: ApplicationIntegrationType[]) {
    this.integrationTypes = integrationTypes;
    return this;
  }

  setContexts(contexts: InteractionContextType[]) {
    this.contexts = contexts;
    return this;
  }

  constructor() {
    super();
  }

  toJSON() {
    return {
      ...super.toJSON(),
      integration_types: this.integrationTypes,
      contexts: this.contexts,
    };
  }
}

type CommandOrJson<UseJson extends boolean> = UseJson extends true
  ? RESTPostAPIApplicationCommandsJSONBody
  : Command;

export async function getCommands<UseJson extends boolean = false>(
  json?: UseJson
): Promise<CommandOrJson<UseJson>[]> {
  const commandsPath = join(__dirname, "commands");
  const commandFiles = await readdir(commandsPath);

  let commands: CommandOrJson<UseJson>[] = [];

  for (const file of commandFiles) {
    const filePath = join(commandsPath, file);
    const { default: command }: { default: Command } = await import(filePath);
    commands.push(
      (json ? command.data.toJSON() : command) as CommandOrJson<UseJson>
    );
  }

  return commands;
}

class OptionNotSpecifiedError extends Error {
  constructor(public optionName: string) {
    super(`The option '${optionName}' has not been specified.`);
  }
}

export type StringOption = APIApplicationCommandInteractionDataStringOption;

export function getOption<
  OptionT extends APIApplicationCommandInteractionDataOption
>(command: APICommand, name: string): OptionT {
  const option = getOptionalOption<OptionT>(command, name);
  if (!option) throw new OptionNotSpecifiedError(name);
  return option;
}

export function getOptionalOption<
  OptionT extends APIApplicationCommandInteractionDataOption
>(command: APICommand, name: string): OptionT | undefined {
  return command.data.options?.find((option) => option.name === name) as
    | OptionT
    | undefined;
}

type EmbedType = "success" | "error" | "warning" | "none";

const embedTypeColorMap: Record<EmbedType, ColorResolvable | undefined> = {
  error: "Red",
  warning: "Orange",
  success: "Green",
  none: undefined,
};

type EmbedOptions = {
  type?: EmbedType;
  addAuthorizeUrl?: boolean;
};

export function embeds(
  { type, addAuthorizeUrl }: EmbedOptions,
  ...embeds: ((embed: EmbedBuilder) => EmbedBuilder)[]
) {
  return embeds.map((embedFactory) => {
    const embed = embedFactory(new EmbedBuilder());

    const authorizeURL = `https://discord.com/oauth2/authorize?client_id=${env(
      "DISCORD_APPLICATION_ID"
    )}`;

    if (embed.data.description && addAuthorizeUrl)
      embed.setDescription(
        `${embed.data.description}\n-# [Click here to add the bot](${authorizeURL})`
      );

    if (!embed.data.color && type && embedTypeColorMap[type])
      embed.setColor(embed.data.color ?? embedTypeColorMap[type]);

    return embed.toJSON();
  });
}

export async function editInitialResponse(
  interactionToken: string,
  response: RESTPatchAPIWebhookWithTokenMessageJSONBody
) {
  await ky.patch(
    `https://discord.com/api/v${APIVersion}/webhooks/${process.env.DISCORD_APPLICATION_ID}/${interactionToken}/messages/@original`,
    { json: response }
  );
}
