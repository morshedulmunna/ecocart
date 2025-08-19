"use client";
import { Moon, Sun } from "lucide-react";
import { useEffect, useState } from "react";
import { setTheme, getPreferredTheme, type ThemeMode } from "@/lib/theme";

export function ThemeToggle() {
  const [mode, setMode] = useState<ThemeMode>("light");
  useEffect(() => {
    setMode(getPreferredTheme());
  }, []);

  const toggle = () => {
    const next = mode === "light" ? "dark" : "light";
    setMode(next);
    setTheme(next);
  };

  return (
    <button aria-label="Toggle theme" onClick={toggle} className="inline-flex h-9 w-9 items-center justify-center rounded-md border bg-background text-foreground shadow-xs hover:bg-accent">
      {mode === "light" ? <Moon className="size-4" /> : <Sun className="size-4" />}
    </button>
  );
}
