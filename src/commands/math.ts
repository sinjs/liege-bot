import { codeBlock, InteractionResponseType } from "discord.js";
import type { Command } from "../commands";
import { embeds, ExtendedSlashCommandBuilder, getOption, type StringOption } from "../util";
import Mexp from "math-expression-evaluator";

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
  execute(interaction) {
    const expression = getOption<StringOption>(interaction, "expression");

    try {
      const evaluated = new Mexp().eval(expression.value);

      return {
        type: InteractionResponseType.ChannelMessageWithSource,
        data: {
          embeds: embeds({ type: "success" }, (embed) =>
            embed.setDescription(
              `**Expression:**\n${codeBlock(expression.value)}\n**Result:**\n${codeBlock(
                evaluated.toString()
              )}`
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
              `**Expression:**\n${codeBlock(expression.value)}\n**Error:**\n${codeBlock(
                String(error)
              )}`
            )
          ),
        },
      };
    }
  },
} satisfies Command;
