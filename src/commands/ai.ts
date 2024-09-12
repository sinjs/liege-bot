import {
  type APIInteractionResponse,
  ApplicationCommandOptionType,
  InteractionResponseType,
  MessageFlagsBitField,
  type APIApplicationCommandInteractionDataSubcommandOption,
} from "discord.js";
import type { APICommand, Command } from "../commands";
import {
  editInitialResponse,
  embeds,
  ExtendedSlashCommandBuilder,
  getSubcommandOption,
  getSubcommandOptionalOption,
  getUser,
  OptionNotSpecifiedError,
  type StringOption,
} from "../util";
import { db } from "../lib/db";
import ky, { HTTPError } from "ky";
import { insertUserAction } from "../lib/user-actions";
import { UserActionType } from "@prisma/client";

type APISubcommand =
  Required<APIApplicationCommandInteractionDataSubcommandOption>;

const tokenNotLinkedResponse: APIInteractionResponse = {
  type: InteractionResponseType.ChannelMessageWithSource,
  data: {
    embeds: embeds({ type: "error" }, (embed) =>
      embed.setDescription(
        `❌ You do not have an AI token linked. Use \`/ai link\` to link your token.`
      )
    ),
  },
};

async function getToken(userID: string) {
  const linkedToken = await db.linkedAIToken.findFirst({
    where: { user_id: userID },
  });

  return linkedToken;
}

async function generateImage(
  command: APICommand,
  subcommand: APISubcommand
): Promise<APIInteractionResponse> {
  const user = getUser(command);

  const imagePromptOption = getSubcommandOption<StringOption>(
    subcommand.options,
    "prompt"
  );

  const token = await getToken(user.id);

  if (!token) return tokenNotLinkedResponse;

  async function run() {
    try {
      const res = await ky.post("https://ai.nigga.church/v2/generate/image", {
        headers: { Authorization: token!.ai_token },
        json: {
          prompt: imagePromptOption.value,
        },
        timeout: 30 * 1000,
      });

      const { imageURL } = await res.json<{ imageURL: string }>();

      await insertUserAction({
        user_id: user.id,
        type: UserActionType.AIGenerateImage,
        ai_prompt: imagePromptOption.value,
        ai_result: imageURL,
      });

      await editInitialResponse(command.token, {
        content: imageURL,
      });
    } catch (error) {
      if (error instanceof HTTPError && error.response.status === 500) {
        const response: { modelResponse?: string } =
          await error.response.json();
        if (!response.modelResponse) throw error;
        await editInitialResponse(command.token, {
          content:
            "**The image could not be generated, however the AI model returned a text as a response.**\n" +
            response.modelResponse,
        });
      } else {
        throw error;
      }
    }
  }

  return new Promise<APIInteractionResponse>((resolve) => {
    resolve({
      type: InteractionResponseType.DeferredChannelMessageWithSource,
    });
    run();
  });
}

async function generateText(
  command: APICommand,
  subcommand: APISubcommand
): Promise<APIInteractionResponse> {
  const user = getUser(command);

  const textPromptOption = getSubcommandOption<StringOption>(
    subcommand.options,
    "prompt"
  );

  const token = await getToken(user.id);

  if (!token) return tokenNotLinkedResponse;

  async function run() {
    const res = await ky.post("https://ai.nigga.church/v2/generate/text", {
      headers: { Authorization: token!.ai_token },
      json: {
        model: "llama-3-8b-instruct",
        messages: [
          {
            role: "user",
            content: textPromptOption.value,
          },
        ],
      },
      timeout: 30 * 1000,
    });

    const { response } = await res.json<{ response: string }>();

    await insertUserAction({
      user_id: user.id,
      type: UserActionType.AIGenerateText,
      ai_prompt: textPromptOption.value,
      ai_result: response,
    });

    await editInitialResponse(command.token, {
      content: response,
    });
  }

  return new Promise<APIInteractionResponse>((resolve) => {
    resolve({
      type: InteractionResponseType.DeferredChannelMessageWithSource,
    });
    run();
  });
}

async function linkToken(
  command: APICommand,
  subcommand: APISubcommand
): Promise<APIInteractionResponse> {
  const user = getUser(command);

  const tokenOption = getSubcommandOptionalOption<StringOption>(
    subcommand.options,
    "token"
  );

  if (tokenOption) {
    const response = await ky.get("https://ai.nigga.church/currentKey", {
      headers: { Authorization: tokenOption.value },
      throwHttpErrors: false,
    });

    if (!response.ok)
      return {
        type: InteractionResponseType.ChannelMessageWithSource,
        data: {
          flags: MessageFlagsBitField.Flags.Ephemeral,
          embeds: embeds({ type: "error" }, (embed) =>
            embed.setDescription(`❌ The provided AI token is not valid.`)
          ),
        },
      };

    await db.linkedAIToken.upsert({
      where: { user_id: user.id },
      create: { user_id: user.id, ai_token: tokenOption.value },
      update: { ai_token: tokenOption.value },
    });
  } else {
    const token = await getToken(user.id);

    if (!token)
      return {
        type: InteractionResponseType.ChannelMessageWithSource,
        data: {
          flags: MessageFlagsBitField.Flags.Ephemeral,
          embeds: embeds({ type: "success" }, (embed) =>
            embed.setDescription(
              `❌ You need to have a token linked to remove it. Use \`/ai link <token>\` to link a token.`
            )
          ),
        },
      };

    await db.linkedAIToken.delete({
      where: { user_id: user.id },
    });
  }

  return {
    type: InteractionResponseType.ChannelMessageWithSource,
    data: {
      flags: MessageFlagsBitField.Flags.Ephemeral,
      embeds: embeds({ type: "success" }, (embed) =>
        embed.setDescription(
          `✅ Successfully ${
            tokenOption ? "updated" : "removed"
          } the AI token ${tokenOption ? "in" : "from"} your account.`
        )
      ),
    },
  };
}

export default {
  data: new ExtendedSlashCommandBuilder()
    .makeUserCommand()
    .setName("ai")
    .setDescription("Execute AI commands")
    .addSubcommand((subcommand) =>
      subcommand
        .setName("image")
        .setDescription("Generate an AI image")
        .addStringOption((option) =>
          option
            .setName("prompt")
            .setDescription("The prompt to generate the image from")
            .setRequired(true)
        )
    )
    .addSubcommand((subcommand) =>
      subcommand
        .setName("text")
        .setDescription("Chat with AI and ask it any question")
        .addStringOption((option) =>
          option
            .setName("prompt")
            .setDescription("The prompt to ask AI")
            .setRequired(true)
        )
    )
    .addSubcommand((subcommand) =>
      subcommand
        .setName("link")
        .setDescription("Link your AI token")
        .addStringOption((option) =>
          option
            .setName("token")
            .setDescription("The new AI token, leave empty to remove.")
            .setRequired(false)
        )
    ),
  async execute(command) {
    const subcommand = command.data.options?.find(
      (o) => o.type === ApplicationCommandOptionType.Subcommand
    ) as APISubcommand;

    if (!subcommand || !subcommand.options)
      throw new OptionNotSpecifiedError("subcommand");

    switch (subcommand.name) {
      case "text":
        return await generateText(command, subcommand);
      case "image":
        return await generateImage(command, subcommand);
      case "link":
        return await linkToken(command, subcommand);
    }

    throw new Error(`Subcommand ${subcommand.name} does not exist`);
  },
} satisfies Command;
