import { CommandResponse, DiscordSDK } from "@discord/embedded-app-sdk";
import { createContext } from "react";

export type Auth = {
  token: string;
  discord: CommandResponse<"authenticate">;
};

export interface AuthContextValue {
  discordSdk: DiscordSDK;
  auth: Auth | null;
}

export const AuthContext = createContext<AuthContextValue | null>(null);
