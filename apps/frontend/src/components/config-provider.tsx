import { ReactNode, useEffect, useState } from "react";

import { Config, ConfigContext } from "./config-context";
import { GlobalConfig } from "@/lib/config";

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
      const response = await fetch("/.proxy/api/config");
      const config: Config = await response.json();
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
