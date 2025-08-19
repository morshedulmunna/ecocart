"use client";
import { useEffect, useMemo, useState } from "react";
import { listProducts } from "@/lib/products";
import { listCategories } from "@/lib/categories";
import type { Product, Category } from "@/lib/types";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import Link from "next/link";

export default function AdminDashboardPage() {
  const [products, setProducts] = useState<Product[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const run = async () => {
      setLoading(true);
      setError(null);
      try {
        const [res, cats] = await Promise.all([listProducts({ pageSize: 1000 }), listCategories()]);
        setProducts(res.items);
        setCategories(cats);
      } catch (e: any) {
        setError(e?.message || "Failed to load dashboard");
      } finally {
        setLoading(false);
      }
    };
    run();
  }, []);

  const totalInventoryValue = useMemo(() => products.reduce((sum, p) => sum + p.price * p.stock, 0), [products]);
  const avgProductPrice = useMemo(() => (products.length ? products.reduce((s, p) => s + p.price, 0) / products.length : 0), [products]);
  const outOfStockCount = useMemo(() => products.filter((p) => p.stock === 0).length, [products]);

  const lowStockProducts = useMemo(() => {
    return products.filter((p) => p.stock <= 5).slice(0, 5);
  }, [products]);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold tracking-tight">Dashboard</h1>
        <Button asChild>
          <Link href="/admin/products">Manage Products</Link>
        </Button>
      </div>

      {error && <div className="text-destructive">{error}</div>}

      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader>
            <CardTitle>Total Products</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-semibold">{products.length}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Categories</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-semibold">{categories.length}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Low Stock (≤ 5)</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-semibold">{products.filter((p) => p.stock <= 5).length}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Inventory Value</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-semibold">${totalInventoryValue.toFixed(2)}</div>
          </CardContent>
        </Card>
      </div>

      <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
        <Card>
          <CardHeader>
            <CardTitle>Average Price</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-semibold">${avgProductPrice.toFixed(2)}</div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>Out of Stock</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-semibold">{outOfStockCount}</div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>Top Categories</CardTitle>
          </CardHeader>
          <CardContent>
            <ul className="space-y-1 text-sm">
              {categories
                .map((c) => ({ c, count: products.filter((p) => p.category_id === c.id).length }))
                .sort((a, b) => b.count - a.count)
                .slice(0, 3)
                .map(({ c, count }) => (
                  <li key={c.id} className="flex items-center justify-between">
                    <span>{c.name}</span>
                    <span className="text-muted-foreground">{count}</span>
                  </li>
                ))}
            </ul>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Recently Added Products</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="overflow-x-auto">
            <table className="w-full overflow-hidden rounded-md border text-sm">
              <thead>
                <tr className="bg-accent/30">
                  <th className="p-2 text-left">Name</th>
                  <th className="p-2 text-left">Price</th>
                  <th className="p-2 text-left">Stock</th>
                </tr>
              </thead>
              <tbody>
                {products
                  .slice()
                  .reverse()
                  .slice(0, 5)
                  .map((p) => (
                    <tr key={p.id} className="border-t">
                      <td className="p-2">{p.name}</td>
                      <td className="p-2">${p.price.toFixed(2)}</td>
                      <td className="p-2">{p.stock}</td>
                    </tr>
                  ))}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>

      {lowStockProducts.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>Low Stock Alerts</CardTitle>
          </CardHeader>
          <CardContent>
            <ul className="list-disc space-y-1 pl-5 text-sm">
              {lowStockProducts.map((p) => (
                <li key={p.id}>
                  {p.name} — {p.stock} left
                </li>
              ))}
            </ul>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
