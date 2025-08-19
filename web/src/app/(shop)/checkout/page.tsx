"use client";
import { useEffect, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import Link from "next/link";
import { getCart, cartTotal, clearCart } from "@/store/cart";

export default function CheckoutPage() {
  const [items, setItems] = useState(getCart());
  const [total, setTotal] = useState(cartTotal());

  useEffect(() => {
    const id = setInterval(() => {
      setItems(getCart());
      setTotal(cartTotal());
    }, 500);
    return () => clearInterval(id);
  }, []);

  const onPlaceOrder = () => {
    // Placeholder UX; integrate backend order endpoint later
    alert("Order placed! (demo)");
    clearCart();
  };

  return (
    <div className="space-y-6">
      <h1 className="text-xl font-semibold">Checkout</h1>
      <div className="grid grid-cols-1 gap-6 md:grid-cols-12">
        <section className="space-y-4 md:col-span-8">
          <Card>
            <CardHeader>
              <CardTitle>Shipping information</CardTitle>
            </CardHeader>
            <CardContent className="grid grid-cols-1 gap-3 md:grid-cols-2">
              <Input placeholder="Full name" />
              <Input placeholder="Email" type="email" />
              <Input placeholder="Phone" />
              <Input placeholder="Address" className="md:col-span-2" />
              <Input placeholder="City" />
              <Input placeholder="Postal code" />
              <Input placeholder="Country" />
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Payment</CardTitle>
            </CardHeader>
            <CardContent className="grid grid-cols-1 gap-3 md:grid-cols-2">
              <Input placeholder="Card holder name" />
              <Input placeholder="Card number" />
              <Input placeholder="Expiry MM/YY" />
              <Input placeholder="CVC" />
              <div className="md:col-span-2 flex justify-end">
                <Button className="bg-gradient-to-r from-green-600 to-emerald-600 text-white hover:opacity-90" onClick={onPlaceOrder} disabled={items.length === 0}>
                  Place order (${total.toFixed(2)})
                </Button>
              </div>
            </CardContent>
          </Card>
        </section>

        <aside className="md:col-span-4">
          <Card>
            <CardHeader>
              <CardTitle>Order summary</CardTitle>
            </CardHeader>
            <CardContent className="space-y-2 text-sm">
              {items.length === 0 ? (
                <div className="text-muted-foreground">
                  Your cart is empty.{" "}
                  <Link className="text-primary hover:underline" href="/products">
                    Add items
                  </Link>
                </div>
              ) : (
                <>
                  <div className="flex items-center justify-between">
                    <span>Subtotal</span>
                    <span>${total.toFixed(2)}</span>
                  </div>
                  <div className="flex items-center justify-between text-muted-foreground">
                    <span>Shipping</span>
                    <span>Calculated at checkout</span>
                  </div>
                  <div className="flex items-center justify-between font-semibold">
                    <span>Total</span>
                    <span>${total.toFixed(2)}</span>
                  </div>
                </>
              )}
            </CardContent>
          </Card>
        </aside>
      </div>
    </div>
  );
}
