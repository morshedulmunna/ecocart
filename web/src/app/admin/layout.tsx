"use client";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useState } from "react";
import { cn } from "@/lib/utils";
import { ThemeToggle } from "@/components/ui/theme-toggle";
import { Button } from "@/components/ui/button";
import { LayoutDashboard, Package, Folder, ChevronLeft, ChevronRight } from "lucide-react";

export default function AdminLayout({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();
  const [collapsed, setCollapsed] = useState(false);

  const navItems = [
    { href: "/admin", label: "Dashboard", Icon: LayoutDashboard },
    { href: "/admin/products", label: "Products", Icon: Package },
    { href: "/admin/categories", label: "Categories", Icon: Folder },
  ];

  return (
    <div className={cn("min-h-dvh grid grid-cols-1", collapsed ? "md:grid-cols-[72px_1fr]" : "md:grid-cols-[220px_1fr]")}>
      <aside className="hidden border-r bg-muted/20 md:block">
        <div className="sticky top-0 flex h-dvh flex-col p-3">
          <div className="mb-3 flex items-center justify-between gap-2 px-1">
            <Link href="/admin" className="flex items-center gap-2">
              <span className="inline-block size-6 rounded-md bg-primary/10" />
              <span className={cn("text-base font-bold", collapsed && "hidden")}>EcoCart Admin</span>
            </Link>
            <Button aria-label="Toggle sidebar" variant="ghost" size="icon" onClick={() => setCollapsed((v) => !v)}>
              {collapsed ? <ChevronRight className="size-4" /> : <ChevronLeft className="size-4" />}
            </Button>
          </div>
          <nav className="mt-2 space-y-1 text-sm">
            {navItems.map(({ href, label, Icon }) => {
              const active = pathname === href || pathname.startsWith(`${href}/`);
              return (
                <Link key={href} href={href} title={label} aria-current={active ? "page" : undefined} className={cn("flex items-center rounded-md px-2 py-2 transition-colors hover:bg-accent", active && "bg-accent text-accent-foreground")}>
                  <Icon className="size-4" />
                  <span className={cn("ml-2", collapsed && "hidden")}>{label}</span>
                </Link>
              );
            })}
          </nav>
          <div className="mt-auto flex items-center justify-center gap-2 p-1">
            <ThemeToggle />
          </div>
        </div>
      </aside>
      <div className="flex min-h-dvh flex-col">
        <header className="sticky top-0 z-10 flex items-center justify-between border-b bg-background/80 px-4 py-3 backdrop-blur supports-[backdrop-filter]:bg-background/60 md:px-6">
          <div className="md:hidden">
            <Link href="/admin" className="text-base font-semibold">
              EcoCart Admin
            </Link>
          </div>
          <div className="ml-auto flex items-center gap-2">
            <ThemeToggle />
          </div>
        </header>
        <main className="mx-auto w-full max-w-6xl flex-1 p-4 md:p-6">{children}</main>
      </div>
    </div>
  );
}
