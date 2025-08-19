"use client";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { getCart, updateQuantity, removeFromCart, cartTotal } from "@/store/cart";
import Link from "next/link";
import Image from "next/image";
import { ChevronLeft, Plus, Minus, ShoppingCart } from "lucide-react";

export default function CartPage() {
  const [items, setItems] = useState(getCart());
  const [total, setTotal] = useState(cartTotal());

  useEffect(() => {
    const id = setInterval(() => {
      setItems(getCart());
      setTotal(cartTotal());
    }, 400);
    return () => clearInterval(id);
  }, []);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-semibold">Your Cart</h1>
        <Link href="/products" className="inline-flex items-center gap-2 text-sm text-muted-foreground hover:underline">
          <ChevronLeft className="size-4" /> Continue shopping
        </Link>
      </div>
      {items.length === 0 ? (
        <Card>
          <CardContent className="py-16 text-center text-muted-foreground">
            <div className="mx-auto mb-4 flex size-12 items-center justify-center rounded-full border">
              <ShoppingCart className="size-5" />
            </div>
            Your cart is empty.{" "}
            <Link href="/products" className="text-primary hover:underline">
              Browse products
            </Link>
          </CardContent>
        </Card>
      ) : (
        <div className="grid grid-cols-1 gap-6 md:grid-cols-12">
          <section className="md:col-span-8">
            <Card>
              <CardHeader>
                <CardTitle>Items</CardTitle>
              </CardHeader>
              <CardContent>
                <ul className="divide-y rounded-md border">
                  {items.map((ci) => (
                    <li key={ci.product.id} className="flex items-center gap-3 p-3">
                      <div className="relative hidden size-16 overflow-hidden rounded-md bg-accent md:block">{ci.product.image_url ? <Image src={ci.product.image_url} alt={ci.product.name} fill className="object-cover" /> : null}</div>
                      <div className="flex-1">
                        <div className="font-medium">{ci.product.name}</div>
                        <div className="text-xs text-muted-foreground">${ci.product.price.toFixed(2)} each</div>
                      </div>
                      <div className="flex items-center gap-2">
                        <Button variant="outline" size="sm" onClick={() => updateQuantity(ci.product.id, Math.max(1, ci.quantity - 1))} className="h-9 w-9 p-0" aria-label="Decrease quantity">
                          <Minus className="size-4" />
                        </Button>
                        <Input type="number" min={1} value={ci.quantity} onChange={(e) => updateQuantity(ci.product.id, Number(e.target.value))} className="w-16 text-center" />
                        <Button variant="outline" size="sm" onClick={() => updateQuantity(ci.product.id, ci.quantity + 1)} className="h-9 w-9 p-0" aria-label="Increase quantity">
                          <Plus className="size-4" />
                        </Button>
                        <div className="w-20 text-right text-sm font-medium">${(ci.product.price * ci.quantity).toFixed(2)}</div>
                        <Button variant="destructive" size="sm" onClick={() => removeFromCart(ci.product.id)}>
                          Remove
                        </Button>
                      </div>
                    </li>
                  ))}
                </ul>
              </CardContent>
            </Card>
          </section>
          <aside className="md:col-span-4">
            <Card>
              <CardHeader>
                <CardTitle>Order Summary</CardTitle>
              </CardHeader>
              <CardContent className="space-y-2 text-sm">
                <div className="flex items-center justify-between">
                  <span>Subtotal</span>
                  <span>${total.toFixed(2)}</span>
                </div>
                <div className="flex items-center justify-between text-muted-foreground">
                  <span>Shipping</span>
                  <span>Calculated at checkout</span>
                </div>
              </CardContent>
              <CardFooter className="flex flex-col gap-3">
                <div className="flex items-center justify-between text-base font-semibold">
                  <span>Total</span>
                  <span>${total.toFixed(2)}</span>
                </div>
                <Link href="/checkout" className="w-full">
                  <Button className="w-full bg-gradient-to-r from-green-600 to-emerald-600 text-white hover:opacity-90">Proceed to checkout</Button>
                </Link>
              </CardFooter>
            </Card>
          </aside>
        </div>
      )}
    </div>
  );
}
