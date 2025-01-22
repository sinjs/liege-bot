import { AuthContext, AuthContextValue } from "@/components/auth-context";
import { isDesignMode } from "@/lib/dev";
import { useContext } from "react";

export function useAuth(): AuthContextValue {
  const context = useContext(AuthContext);

  if (isDesignMode()) {
    return {
      auth: {
        token: "",
        discord: {
          access_token: "",
          expires: "",
          scopes: [],
          application: { id: "", description: "", name: "" },
          user: { discriminator: "", id: "", public_flags: 0, username: "" },
        },
      },
      discordSdk: null!,
    };
  }

  if (!context) throw new TypeError("Auth context is null");
  return context;
}
