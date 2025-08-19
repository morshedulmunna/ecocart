import { apiFetch, apiUpload } from "./http";
import type { Paginated, Product } from "./types";

export type ProductListQuery = {
  q?: string;
  category_id?: number;
  min_price?: number;
  max_price?: number;
  page?: number;
  pageSize?: number;
};

export async function listProducts(query: ProductListQuery = {}): Promise<Paginated<Product>> {
  const params = new URLSearchParams();
  Object.entries(query).forEach(([k, v]) => {
    if (v === undefined || v === null || v === "") return;
    params.set(k, String(v));
  });
  const q = params.toString();
  const res = await apiFetch<{ items: Product[]; page: number; page_size: number; total: number; total_pages: number }>(`/api/products${q ? `?${q}` : ""}`);
  // Normalize snake_case keys from API to camelCase for the app
  return {
    items: res.items,
    page: res.page,
    pageSize: res.page_size,
    total: res.total,
    totalPages: res.total_pages,
  };
}

export async function getProduct(id: number): Promise<Product> {
  return apiFetch<Product>(`/api/products/${id}`);
}

export async function createProduct(payload: Partial<Product>): Promise<Product> {
  return apiFetch<Product>("/api/products", { method: "POST", body: JSON.stringify(payload) });
}

export async function updateProduct(id: number, payload: Partial<Product>): Promise<Product> {
  return apiFetch<Product>(`/api/products/${id}`, { method: "PUT", body: JSON.stringify(payload) });
}

export async function deleteProduct(id: number): Promise<{ deleted: boolean }> {
  return apiFetch<{ deleted: boolean }>(`/api/products/${id}`, { method: "DELETE" });
}

export async function uploadProductImage(file: File): Promise<{ url: string }> {
  const form = new FormData();
  form.append("file", file);
  return apiUpload<{ url: string }>("/api/products/upload", form);
}
