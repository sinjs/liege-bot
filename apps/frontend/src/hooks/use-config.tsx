import { ConfigContext } from "@/components/config-context";
import { useContext } from "react";

export function useConfig() {
  const context = useContext(ConfigContext);
  if (!context) throw new TypeError("Config context is null");
  return context;
}
