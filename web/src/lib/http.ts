/**
 * Minimal HTTP client for the EcoCart API.
 * - Attaches Authorization header when access token exists
 * - Parses JSON and normalizes API errors
 */
import { getApiUrl } from "./config";

export type ApiError = {
  code: string;
  message: string;
  details?: unknown;
};

function getAccessToken(): string | null {
  if (typeof window === "undefined") return null;
  return window.localStorage.getItem("ecocart.access_token");
}

export async function apiFetch<T>(path: string, init?: RequestInit): Promise<T> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    ...(init?.headers as Record<string, string>),
  };

  const token = getAccessToken();
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  const res = await fetch(getApiUrl(path), { ...init, headers });
  const contentType = res.headers.get("content-type") || "";
  const isJson = contentType.includes("application/json");
  const payload = isJson ? await res.json() : await res.text();

  if (!res.ok) {
    const error: ApiError = isJson && payload && typeof payload === "object" ? { code: payload.code ?? "INTERNAL_ERROR", message: payload.message ?? "Request failed", details: payload.details } : { code: "INTERNAL_ERROR", message: String(payload || res.statusText) };
    throw error;
  }

  return payload as T;
}

export async function apiUpload<T>(path: string, form: FormData): Promise<T> {
  const headers: Record<string, string> = {};
  const token = getAccessToken();
  if (token) headers["Authorization"] = `Bearer ${token}`;

  const res = await fetch(getApiUrl(path), { method: "POST", body: form, headers });
  const payload = await res.json();
  if (!res.ok) {
    const error: ApiError = { code: payload?.code ?? "INTERNAL_ERROR", message: payload?.message ?? "Upload failed", details: payload?.details };
    throw error;
  }
  return payload as T;
}
