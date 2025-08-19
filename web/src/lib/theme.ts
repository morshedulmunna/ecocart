export type ThemeMode = "light" | "dark";

const STORAGE_KEY = "ecocart.theme";

export function getStoredTheme(): ThemeMode | null {
  if (typeof window === "undefined") return null;
  const v = window.localStorage.getItem(STORAGE_KEY);
  return v === "light" || v === "dark" ? v : null;
}

export function getPreferredTheme(): ThemeMode {
  if (typeof window === "undefined") return "light";
  const stored = getStoredTheme();
  if (stored) return stored;
  const systemDark = window.matchMedia?.("(prefers-color-scheme: dark)").matches;
  return systemDark ? "dark" : "light";
}

export function setTheme(mode: ThemeMode) {
  if (typeof window === "undefined") return;
  const root = window.document.documentElement;
  if (mode === "dark") root.classList.add("dark");
  else root.classList.remove("dark");
  window.localStorage.setItem(STORAGE_KEY, mode);
}
