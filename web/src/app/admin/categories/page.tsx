"use client";
import { useEffect, useState } from "react";
import { listCategories, createCategory, updateCategory, deleteCategory } from "@/lib/categories";
import type { Category } from "@/lib/types";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function AdminCategoriesPage() {
  const [categories, setCategories] = useState<Category[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [editing, setEditing] = useState<Partial<Category> | null>(null);

  const refresh = async () => {
    setLoading(true);
    setError(null);
    try {
      const cats = await listCategories();
      setCategories(cats);
    } catch (e: any) {
      setError(e?.message || "Failed to load categories");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    refresh();
  }, []);

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!editing) return;
    setLoading(true);
    try {
      if (editing.id) {
        await updateCategory(editing.id, editing);
      } else {
        await createCategory(editing);
      }
      setEditing(null);
      await refresh();
    } catch (e: any) {
      setError(e?.message || "Save failed");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-semibold">Categories</h2>
        <Button onClick={() => setEditing({ name: "", description: "" })}>New Category</Button>
      </div>
      {error && <div className="text-destructive">{error}</div>}
      {loading && <div>Loading...</div>}

      <table className="w-full overflow-hidden rounded-md border text-sm">
        <thead>
          <tr className="bg-accent/30">
            <th className="p-2 text-left">ID</th>
            <th className="p-2 text-left">Name</th>
            <th className="p-2 text-left">Description</th>
            <th className="p-2 text-left">Actions</th>
          </tr>
        </thead>
        <tbody>
          {categories.map((c) => (
            <tr key={c.id} className="border-t hover:bg-accent/20">
              <td className="p-2">{c.id}</td>
              <td className="p-2">{c.name}</td>
              <td className="p-2">{c.description || ""}</td>
              <td className="p-2 space-x-2">
                <Button variant="outline" onClick={() => setEditing(c)}>
                  Edit
                </Button>
                <Button
                  variant="destructive"
                  onClick={async () => {
                    await deleteCategory(c.id);
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

      {editing && (
        <form onSubmit={onSubmit} className="space-y-3">
          <Card>
            <CardHeader>
              <CardTitle>{editing.id ? "Edit category" : "New category"}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <div className="grid grid-cols-1 gap-3 md:grid-cols-2">
                <div>
                  <label className="mb-1 block text-sm font-medium">Name</label>
                  <Input value={editing.name ?? ""} onChange={(e) => setEditing({ ...editing, name: e.target.value })} required />
                </div>
                <div className="md:col-span-2">
                  <label className="mb-1 block text-sm font-medium">Description</label>
                  <Textarea value={editing.description ?? ""} onChange={(e) => setEditing({ ...editing, description: e.target.value })} />
                </div>
              </div>
              <div className="flex justify-end gap-2">
                <Button type="button" variant="outline" onClick={() => setEditing(null)}>
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
