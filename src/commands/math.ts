import { codeBlock, InteractionResponseType } from "discord.js";
import type { Command } from "../commands";
import {
  embeds,
  ExtendedSlashCommandBuilder,
  getOption,
  getUser,
  type StringOption,
} from "../util";
import Mexp from "math-expression-evaluator";
import { insertUserAction } from "../lib/user-actions";
import { UserActionType } from "@prisma/client";

export default {
  data: new ExtendedSlashCommandBuilder()
    .makeUserCommand()
    .setName("math")
    .setDescription("Evaluates math expressions")
    .addStringOption((option) =>
      option /* */
        .setName("expression")
        .setDescription("The expression to evaluate")
        .setRequired(true)
    ),
  async execute(interaction) {
    const expression = getOption<StringOption>(interaction, "expression");
    const user = getUser(interaction);

    try {
      const evaluated = new Mexp().eval(expression.value);

      await insertUserAction({
        user_id: user.id,
        type: UserActionType.CodeExecution,
        code_expression: expression.value,
        code_output: evaluated.toString(),
        code_language: "math",
      });

      return {
        type: InteractionResponseType.ChannelMessageWithSource,
        data: {
          embeds: embeds({ type: "success" }, (embed) =>
            embed.setDescription(
              `**Expression:**\n${codeBlock(
                expression.value
              )}\n**Result:**\n${codeBlock(evaluated.toString())}`
            )
          ),
        },
      };
    } catch (error) {
      return {
        type: InteractionResponseType.ChannelMessageWithSource,
        data: {
          embeds: embeds({ type: "error" }, (embed) =>
            embed.setDescription(
              `**Expression:**\n${codeBlock(
                expression.value
              )}\n**Error:**\n${codeBlock(String(error))}`
            )
          ),
        },
      };
    }
  },
} satisfies Command;
