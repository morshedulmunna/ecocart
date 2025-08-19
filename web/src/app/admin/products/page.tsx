"use client";
import { useEffect, useState } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Select } from "@/components/ui/select";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { listProducts, createProduct, deleteProduct, updateProduct, uploadProductImage } from "@/lib/products";
import { listCategories } from "@/lib/categories";
import type { Category, Paginated, Product } from "@/lib/types";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { resolveImageUrl } from "@/lib/utils";

export default function AdminProductsPage() {
  const [products, setProducts] = useState<Product[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const pageSize = 12;
  const [categories, setCategories] = useState<Category[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [editing, setEditing] = useState<Partial<Product> | null>(null);
  const [query, setQuery] = useState("");
  const [filterCategory, setFilterCategory] = useState<number | "">("");
  const [imagePreview, setImagePreview] = useState<string | null>(null);

  const refresh = async () => {
    setLoading(true);
    setError(null);
    try {
      const [resp, cats] = await Promise.all([listProducts({ q: query || undefined, category_id: typeof filterCategory === "number" ? filterCategory : undefined, page, pageSize }), listCategories()]);
      setProducts(resp.items);
      setTotal(resp.total);
      setCategories(cats);
    } catch (e: any) {
      setError(e?.message || "Failed to load");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    refresh();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [query, filterCategory, page]);

  const onFileChange = async (file: File) => {
    setImagePreview(URL.createObjectURL(file));
    try {
      const { url } = await uploadProductImage(file);
      setEditing((prev) => ({ ...prev, image_url: url }));
    } catch (e: any) {
      setError(e?.message || "Image upload failed");
    }
  };

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!editing) return;
    setLoading(true);
    try {
      if (editing.id) {
        await updateProduct(editing.id, editing);
      } else {
        await createProduct(editing);
      }
      setEditing(null);
      setImagePreview(null);
      await refresh();
    } catch (e: any) {
      setError(e?.message || "Save failed");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
        <h2 className="text-xl font-semibold">Products</h2>
        <div className="flex w-full flex-col gap-2 sm:flex-row sm:items-center sm:gap-3 md:w-auto">
          <Input placeholder="Search products..." value={query} onChange={(e) => setQuery(e.target.value)} className="sm:w-64" />
          <Select value={String(filterCategory)} onChange={(e) => setFilterCategory(e.target.value ? Number(e.target.value) : "")}>
            <option value="">All categories</option>
            {categories.map((c) => (
              <option key={c.id} value={c.id}>
                {c.name}
              </option>
            ))}
          </Select>
          <Button onClick={() => setEditing({ name: "", description: "", price: 0, category_id: categories[0]?.id ?? 0, stock: 0 })}>New</Button>
        </div>
      </div>
      {error && <div className="text-destructive">{error}</div>}
      {loading && <div>Loading...</div>}

      <table className="w-full overflow-hidden rounded-md border text-sm">
        <thead>
          <tr className="bg-accent/30">
            <th className="p-2 text-left">ID</th>
            <th className="p-2 text-left">Name</th>
            <th className="p-2 text-left">Price</th>
            <th className="p-2 text-left">Stock</th>
            <th className="p-2 text-left">Actions</th>
          </tr>
        </thead>
        <tbody>
          {products.map((p) => (
            <tr key={p.id} className="border-t hover:bg-accent/20">
              <td className="p-2">{p.id}</td>
              <td className="p-2">{p.name}</td>
              <td className="p-2">${p.price.toFixed(2)}</td>
              <td className="p-2">{p.stock}</td>
              <td className="p-2 space-x-2">
                <Button variant="outline" onClick={() => setEditing(p)}>
                  Edit
                </Button>
                <Button
                  variant="destructive"
                  onClick={async () => {
                    await deleteProduct(p.id);
                    await refresh();
                  }}
                >
                  Delete
                </Button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <div className="flex items-center justify-between text-sm text-muted-foreground">
        <div>Total: {total}</div>
        <div className="space-x-2">
          <Button variant="outline" disabled={page <= 1} onClick={() => setPage((p) => Math.max(1, p - 1))}>
            Prev
          </Button>
          <Button variant="outline" onClick={() => setPage((p) => p + 1)}>
            Next
          </Button>
        </div>
      </div>

      {editing && (
        <form onSubmit={onSubmit} className="space-y-3">
          <Card>
            <CardHeader>
              <CardTitle>{editing.id ? "Edit product" : "New product"}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <div className="grid grid-cols-1 gap-3 md:grid-cols-2">
                <div>
                  <Label>Name</Label>
                  <Input value={editing.name ?? ""} onChange={(e) => setEditing({ ...editing, name: e.target.value })} required />
                </div>
                <div>
                  <Label>Price</Label>
                  <Input type="number" step="0.01" value={editing.price ?? 0} onChange={(e) => setEditing({ ...editing, price: Number(e.target.value) })} required />
                </div>
                <div>
                  <Label>Stock</Label>
                  <Input type="number" value={editing.stock ?? 0} onChange={(e) => setEditing({ ...editing, stock: Number(e.target.value) })} required />
                </div>
                <div>
                  <Label>Category</Label>
                  <Select value={String(editing.category_id ?? "")} onChange={(e) => setEditing({ ...editing, category_id: Number(e.target.value) })}>
                    {categories.map((c) => (
                      <option key={c.id} value={c.id}>
                        {c.name}
                      </option>
                    ))}
                  </Select>
                </div>
                <div className="md:col-span-2">
                  <Label>Description</Label>
                  <Textarea value={editing.description ?? ""} onChange={(e) => setEditing({ ...editing, description: e.target.value })} />
                </div>
                <div className="md:col-span-2">
                  <Label>Image</Label>
                  <input
                    type="file"
                    accept="image/*"
                    onChange={(e) => {
                      const f = e.target.files?.[0];
                      if (f) onFileChange(f);
                    }}
                  />
                  {imagePreview || editing.image_url ? (
                    // eslint-disable-next-line @next/next/no-img-element
                    <img src={imagePreview ?? resolveImageUrl(editing.image_url) ?? ""} alt="preview" className="mt-2 h-40 w-40 rounded-md object-cover" />
                  ) : null}
                </div>
              </div>
              <div className="flex justify-end gap-2">
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => {
                    setEditing(null);
                    setImagePreview(null);
                  }}
                >
                  Cancel
                </Button>
                <Button type="submit">Save</Button>
              </div>
            </CardContent>
          </Card>
        </form>
      )}
    </div>
  );
}
