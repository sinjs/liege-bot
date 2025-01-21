import { ReactNode, useEffect, useMemo, useState } from "react";

import { DiscordSDK } from "@discord/embedded-app-sdk";
import { Auth, AuthContext } from "./auth-context";
import { useConfig } from "@/hooks/use-config";

export function AuthProvider({
  children,
  fallback,
}: {
  children: ReactNode;
  fallback?: ReactNode;
}) {
  const config = useConfig();

  console.log(import.meta.env);
  const discordSdk = useMemo(
    () => new DiscordSDK(config.discordAppId),
    [config.discordAppId]
  );

  const [auth, setAuth] = useState<Auth | null>(null);

  useEffect(() => {
    async function initialize() {
      await discordSdk.ready();

      const { code } = await discordSdk.commands.authorize({
        client_id: config.discordAppId,
        response_type: "code",
        state: "",
        prompt: "none",
        scope: ["applications.commands", "identify"],
      });

      const response = await fetch("/.proxy/api/token", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          code,
        }),
      });
      const { token, discord_access_token } = await response.json();

      const discordAuth = await discordSdk.commands.authenticate({
        access_token: discord_access_token,
      });

      if (discordAuth == null) {
        throw new Error("Authenticate command failed");
      }

      setAuth({ discord: discordAuth, token });
    }

    initialize();
  }, [discordSdk, config.discordAppId]);

  return (
    <AuthContext.Provider value={{ auth, discordSdk }}>
      {auth ? children : fallback ?? <>Logging in...</>}
    </AuthContext.Provider>
  );
}
