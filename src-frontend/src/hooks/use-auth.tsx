import { AuthContext } from "@/components/auth-provider";
import { useContext } from "react";

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) throw new TypeError("Auth context is null");
  return context;
}
