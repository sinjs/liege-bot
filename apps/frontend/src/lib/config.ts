import { Config } from "@/components/config-context";

export class GlobalConfig {
  private static _config: Config | null;

  public static get config() {
    if (!this._config) throw new Error("Config is null");
    return this._config;
  }

  public static set config(value) {
    this._config = value;
  }
}
