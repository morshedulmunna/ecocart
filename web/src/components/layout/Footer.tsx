import * as React from "react";
import { Container } from "./Container";

export function Footer() {
  return (
    <footer className="border-t py-10 text-sm">
      <Container className="flex flex-col items-center justify-between gap-6 md:flex-row">
        <div className="flex items-center gap-2 text-foreground/80">
          <span className="text-base font-semibold">EcoCart</span>
          <span className="text-muted-foreground">© {new Date().getFullYear()}</span>
        </div>
        <div className="flex items-center gap-4 text-muted-foreground">
          <a className="rounded-md px-2 py-1 hover:bg-accent" href="/products">
            Products
          </a>
          <a className="rounded-md px-2 py-1 hover:bg-accent" href="/cart">
            Cart
          </a>
          <a className="rounded-md px-2 py-1 hover:bg-accent" href="/auth/login">
            Login
          </a>
        </div>
      </Container>
    </footer>
  );
}
