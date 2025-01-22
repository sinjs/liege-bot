import { ReactNode, useEffect, useState } from "react";

import { Config, ConfigContext } from "./config-context";
import { GlobalConfig } from "@/lib/config";
import { api } from "@/lib/utils";

export function ConfigProvider({
  children,
  fallback,
}: {
  children: ReactNode;
  fallback?: ReactNode;
}) {
  const [config, setConfig] = useState<Config | null>(null);

  useEffect(() => {
    async function load() {
      const config: Config = await fetch(api("/config")).then((response) =>
        response.json()
      );
      GlobalConfig.config = config;
      setConfig(config);
    }

    load();
  }, []);

  useEffect(() => {
    if (config) GlobalConfig.config = config;
  }, [config]);

  return (
    <ConfigContext.Provider value={config}>
      {config ? children : fallback ?? <>Configuring...</>}
    </ConfigContext.Provider>
  );
}
