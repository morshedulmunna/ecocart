import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import { INTERNAL_API_BASE_URL } from "./config";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function resolveImageUrl(url?: string | null): string | undefined {
  if (!url) return undefined;
  if (url.startsWith("http://") || url.startsWith("https://")) return url;
  // On the client, images should be fetched via same-origin using rewrites
  if (typeof window !== "undefined") {
    return url.startsWith("/") ? url : `/${url}`;
  }
  // On the server (e.g., generating metadata), use internal Docker DNS base URL
  return url.startsWith("/") ? `${INTERNAL_API_BASE_URL}${url}` : `${INTERNAL_API_BASE_URL}/${url}`;
}
