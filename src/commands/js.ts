import { codeBlock, EmbedBuilder, InteractionResponseType } from "discord.js";
import type { Command } from "../commands";
import {
  editInitialResponse,
  embeds,
  ExtendedSlashCommandBuilder,
  generateSecret,
  getOption,
  getUser,
  type StringOption,
} from "../util";
import ky from "ky";
import { execute } from "../lib/codeapi";
import { db } from "../lib/db";
import { insertUserAction } from "../lib/user-actions";
import { UserActionType } from "@prisma/client";

export default {
  data: new ExtendedSlashCommandBuilder()
    .makeUserCommand()
    .setName("js")
    .setDescription("Executes JavaScript code")
    .addStringOption((option) =>
      option /* */
        .setName("code")
        .setDescription("The code to execute")
    ),
  execute(interaction) {
    async function runCommand() {
      const code = getOption<StringOption>(interaction, "code");

      const user = getUser(interaction);

      try {
        const result = await execute({
          language: "js",
          version: "*",
          files: [
            {
              name: "index.js",
              content: code?.value ?? "console.log('Hello, world!');",
            },
          ],
        });

        await insertUserAction({
          user_id: user.id,
          type: UserActionType.CodeExecution,
          code_expression: code.value,
          code_output: result.run.output,
          code_language: "js",
        });

        await editInitialResponse(interaction.token, {
          embeds: embeds({ type: "success" }, (embed) =>
            embed
              .setDescription(
                `**Code:**\n${codeBlock(code?.value)}\n**Result:**\n${codeBlock(
                  result.run.output ||
                    "[No output. Make sure to console.log the expression]"
                )}`
              )
              .setFooter({
                text: `${
                  result.run.signal ? `Signal: ${result.run.signal} | ` : ""
                }Exit Code: ${result.run.code} | Runtime: ${result.language} ${
                  result.version
                }`,
              })
          ),
        });
      } catch (error) {
        await editInitialResponse(interaction.token, {
          embeds: embeds({ type: "error" }, (embed) =>
            embed.setDescription(codeBlock(String(error)))
          ),
        });
      }
    }

    // Workaround to defer interaction response
    return new Promise((resolve) => {
      resolve({
        type: InteractionResponseType.DeferredChannelMessageWithSource,
      });
      runCommand();
    });
  },
} satisfies Command;
