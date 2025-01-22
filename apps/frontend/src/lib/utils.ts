import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import { isDesignMode } from "./dev";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function api(path: string) {
  if (isDesignMode()) return `/api${path.startsWith("/") ? path : `/${path}`}`;
  return `/.proxy/api${path.startsWith("/") ? path : `/${path}`}`;
}
