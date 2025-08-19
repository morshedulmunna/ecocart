/**
 * LocalStorage-backed cart utilities.
 */
import type { CartItem, Product } from "@/lib/types";

const CART_KEY = "ecocart.cart";

function readCart(): CartItem[] {
  if (typeof window === "undefined") return [];
  const raw = window.localStorage.getItem(CART_KEY);
  if (!raw) return [];
  try {
    const parsed = JSON.parse(raw) as CartItem[];
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

function writeCart(items: CartItem[]) {
  if (typeof window === "undefined") return;
  window.localStorage.setItem(CART_KEY, JSON.stringify(items));
}

export function getCart(): CartItem[] {
  return readCart();
}

export function addToCart(product: Product, quantity = 1) {
  const items = readCart();
  const idx = items.findIndex((ci) => ci.product.id === product.id);
  if (idx >= 0) {
    items[idx].quantity += quantity;
  } else {
    items.push({ product, quantity });
  }
  writeCart(items);
}

export function updateQuantity(productId: number, quantity: number) {
  const items = readCart();
  const idx = items.findIndex((ci) => ci.product.id === productId);
  if (idx >= 0) {
    items[idx].quantity = Math.max(1, quantity);
    writeCart(items);
  }
}

export function removeFromCart(productId: number) {
  const items = readCart().filter((ci) => ci.product.id !== productId);
  writeCart(items);
}

export function clearCart() {
  writeCart([]);
}

export function cartTotal(): number {
  return readCart().reduce((sum, ci) => sum + ci.product.price * ci.quantity, 0);
}
