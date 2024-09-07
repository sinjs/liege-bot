import { verifyKeyMiddleware } from "discord-interactions";
import express, { type Request, type Response } from "express";
import { env, getCommands } from "./util";
import {
  codeBlock,
  EmbedBuilder,
  InteractionResponseType,
  InteractionType,
  type APIApplicationCommandInteraction,
  type APIInteraction,
  type APIInteractionResponse,
} from "discord.js";

const app = express();
const commands = await getCommands();

async function handleCommand(
  command: APIApplicationCommandInteraction,
  req: Request,
  res: Response
) {
  const foundCommand = commands.find(
    (foundCommand) => foundCommand.data.name === command.data.name
  );

  if (!foundCommand) return false; // Unknown command

  try {
    const commandResponse = foundCommand.execute(command);

    if (commandResponse instanceof Promise) {
      const awaitedCommandResponse = await commandResponse;
      return res.json(awaitedCommandResponse);
    }

    return res.json(commandResponse);
  } catch (error) {
    console.error(error);

    return res.json({
      type: InteractionResponseType.ChannelMessageWithSource,
      data: {
        embeds: [
          new EmbedBuilder()
            .setColor("Red")
            .setDescription(
              `**The command failed to execute due to an error:**\n${codeBlock(
                String(error)
              )}`
            )
            .toJSON(),
        ],
      },
    } satisfies APIInteractionResponse);
  }
}

app.post(
  "/interactions",
  verifyKeyMiddleware(env("DISCORD_PUBLIC_KEY")),
  async (req, res) => {
    const message: APIInteraction = req.body;
    if (message.type === InteractionType.ApplicationCommand) {
      return await handleCommand(message, req, res);
    }
  }
);

app.listen(7890, () => console.log("App listening on http://localhost:7890"));
