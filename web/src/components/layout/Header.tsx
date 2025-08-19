"use client";
import Link from "next/link";
import { useEffect, useState } from "react";
import { ShoppingCart, LogIn, Package, LayoutDashboard, LogOut } from "lucide-react";
import { Container } from "./Container";
import { ThemeToggle } from "@/components/ui/theme-toggle";
import { cn } from "@/lib/utils";

export function Header() {
  const [cartCount, setCartCount] = useState(0);
  const [isAuthed, setIsAuthed] = useState(false);
  const [userName, setUserName] = useState<string | null>(null);
  const [userRole, setUserRole] = useState<string | null>(null);
  const [menuOpen, setMenuOpen] = useState(false);
  const [anchorEl, setAnchorEl] = useState<HTMLButtonElement | null>(null);
  useEffect(() => {
    const id = setInterval(() => {
      try {
        const raw = localStorage.getItem("ecocart.cart");
        const items = raw ? (JSON.parse(raw) as any[]) : [];
        setCartCount(items.reduce((n, it) => n + (it?.quantity ?? 0), 0));
        const hasToken = !!localStorage.getItem("ecocart.access_token");
        setIsAuthed(hasToken);
        const userRaw = localStorage.getItem("ecocart.user");
        if (userRaw) {
          const u = JSON.parse(userRaw);
          setUserName(u?.username ?? null);
          setUserRole(u?.role ?? null);
        } else {
          setUserName(null);
          setUserRole(null);
        }
      } catch {
        setCartCount(0);
      }
    }, 500);
    return () => clearInterval(id);
  }, []);

  useEffect(() => {
    function onDocClick(e: MouseEvent) {
      if (!menuOpen) return;
      const target = e.target as Node;
      if (anchorEl && !anchorEl.contains(target)) {
        const menu = document.getElementById("header-user-menu");
        if (menu && !menu.contains(target)) setMenuOpen(false);
      }
    }
    function onEsc(e: KeyboardEvent) {
      if (e.key === "Escape") setMenuOpen(false);
    }
    document.addEventListener("click", onDocClick);
    document.addEventListener("keydown", onEsc);
    return () => {
      document.removeEventListener("click", onDocClick);
      document.removeEventListener("keydown", onEsc);
    };
  }, [menuOpen, anchorEl]);

  return (
    <header className={cn("sticky top-0 z-20 border-b bg-background/10 backdrop-blur supports-[backdrop-filter]:backdrop-blur")}>
      <Container className="flex items-center justify-between py-3">
        <Link href="/" className="text-lg font-semibold">
          <span className="bg-gradient-to-r from-green-600 to-emerald-600 bg-clip-text text-transparent">EcoCart</span>
        </Link>
        <nav className="flex items-center gap-2 text-sm">
          <Link href="/products" className="flex items-center gap-2 rounded-md px-2 py-1 hover:bg-accent">
            <Package className="size-4" /> Products
          </Link>
          {/* <Link href="/admin/products" className="flex items-center gap-2 rounded-md px-2 py-1 hover:bg-accent">
            <LayoutDashboard className="size-4" /> Admin
          </Link> */}
          {!isAuthed ? (
            <Link href="/auth/login" className="flex items-center gap-2 rounded-md px-2 py-1 hover:bg-accent">
              <LogIn className="size-4" /> Login
            </Link>
          ) : (
            <div className="relative">
              <button ref={setAnchorEl} onClick={() => setMenuOpen((v) => !v)} aria-label="User menu" className="inline-flex size-9 items-center justify-center rounded-full border bg-background text-sm font-medium" title={userName ?? "Account"}>
                <span className="sr-only">Open user menu</span>
                {(userName ?? "U").slice(0, 1).toUpperCase()}
              </button>
              {menuOpen && (
                <div id="header-user-menu" className="absolute right-0 mt-2 w-44 rounded-md border bg-popover p-1 text-sm shadow-lg">
                  <div className="px-2 py-2 text-xs text-muted-foreground">Signed in as {userName ?? "User"}</div>
                  {userRole === "admin" && (
                    <Link href="/admin" className="flex items-center gap-2 rounded-sm px-2 py-2 hover:bg-accent">
                      <LayoutDashboard className="size-4" /> Admin Dashboard
                    </Link>
                  )}
                  <Link href="/profile" className="block rounded-sm px-2 py-2 hover:bg-accent">
                    Profile
                  </Link>
                  <Link href="/settings" className="block rounded-sm px-2 py-2 hover:bg-accent">
                    Settings
                  </Link>

                  <button
                    className="flex w-full items-center gap-2 rounded-sm px-2 py-2 text-left hover:bg-accent"
                    onClick={async () => {
                      setMenuOpen(false);
                      await fetch("/api/session", { method: "DELETE" });
                      localStorage.removeItem("ecocart.access_token");
                      localStorage.removeItem("ecocart.refresh_token");
                      localStorage.removeItem("ecocart.token_exp");
                      localStorage.removeItem("ecocart.user");
                      window.location.href = "/auth/login";
                    }}
                  >
                    <LogOut className="size-4" /> Logout
                  </button>
                </div>
              )}
            </div>
          )}
          <Link href="/cart" className="relative inline-flex items-center gap-2 rounded-md px-2 py-1 hover:bg-accent">
            <ShoppingCart className="size-5" />
            <span className="sr-only">Cart</span>
            {cartCount > 0 && <span className="absolute -right-2 -top-2 inline-flex h-5 min-w-5 items-center justify-center rounded-full bg-primary px-1 text-xs font-semibold text-primary-foreground">{cartCount}</span>}
          </Link>
          <div className="ml-1 h-5 w-px bg-border" />
          <ThemeToggle />
        </nav>
      </Container>
    </header>
  );
}
