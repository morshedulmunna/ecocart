// Runtime-aware API base URL selection
// - On the server (SSR / Route handlers), talk to the Docker network service name
// - In the browser, use same-origin and rely on Next.js rewrites to proxy to the API
export const INTERNAL_API_BASE_URL = process.env.INTERNAL_API_BASE_URL || process.env.API_BASE_URL || "http://api:8080";

export function getApiUrl(path: string): string {
  const normalizedPath = path.startsWith("/") ? path : `/${path}`;
  if (typeof window === "undefined") {
    return `${INTERNAL_API_BASE_URL}${normalizedPath}`;
  }
  return normalizedPath;
}

export function isProduction(): boolean {
  return process.env.NODE_ENV === "production";
}
