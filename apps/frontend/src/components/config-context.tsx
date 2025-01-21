import { createContext } from "react";

export type Config = {
  discordAppId: string;
};

export const ConfigContext = createContext<Config | null>(null);
