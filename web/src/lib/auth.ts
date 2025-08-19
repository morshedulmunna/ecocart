/**
 * Auth helpers for registration/login/refresh/logout backed by the API.
 */
import { apiFetch } from "./http";

export type LoginResponse = {
  user: { id: string; username: string; email: string; role: string };
  access_token: string;
  refresh_token: string;
  token_type: "Bearer";
  expires_in: number;
};

export type RegisterPayload = {
  username: string;
  email: string;
  password: string;
};

export function setAuthTokens(tokens: LoginResponse) {
  if (typeof window === "undefined") return;
  window.localStorage.setItem("ecocart.access_token", tokens.access_token);
  window.localStorage.setItem("ecocart.refresh_token", tokens.refresh_token);
  const expiry = String(Math.floor(Date.now() / 1000) + tokens.expires_in);
  window.localStorage.setItem("ecocart.token_exp", expiry);
  if (tokens.user) {
    window.localStorage.setItem("ecocart.user", JSON.stringify(tokens.user));
  }
}

export function clearAuthTokens() {
  if (typeof window === "undefined") return;
  window.localStorage.removeItem("ecocart.access_token");
  window.localStorage.removeItem("ecocart.refresh_token");
  window.localStorage.removeItem("ecocart.token_exp");
  window.localStorage.removeItem("ecocart.user");
}

export async function registerUser(payload: RegisterPayload): Promise<{ id: string }> {
  return apiFetch<{ id: string }>("/api/auth/register", {
    method: "POST",
    body: JSON.stringify(payload),
  });
}

export async function loginUser(email: string, password: string): Promise<LoginResponse> {
  // Call our session route to set HttpOnly cookies, but also mirror to localStorage for client fetch convenience
  const res = await fetch("/api/session", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ email, password }) });
  const data = (await res.json()) as LoginResponse;
  if (!res.ok) throw new Error((data as any)?.message || "Login failed");
  setAuthTokens(data);
  return data;
}

export async function logoutUser(): Promise<void> {
  try {
    await fetch("/api/session", { method: "DELETE" });
  } finally {
    clearAuthTokens();
  }
}
