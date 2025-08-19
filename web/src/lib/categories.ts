import { apiFetch } from "./http";
import type { Category } from "./types";

export async function listCategories(): Promise<Category[]> {
  return apiFetch<Category[]>("/api/categories");
}

export async function getCategory(id: number): Promise<Category> {
  return apiFetch<Category>(`/api/categories/${id}`);
}

export async function createCategory(payload: Partial<Category>): Promise<Category> {
  return apiFetch<Category>("/api/categories", { method: "POST", body: JSON.stringify(payload) });
}

export async function updateCategory(id: number, payload: Partial<Category>): Promise<Category> {
  return apiFetch<Category>(`/api/categories/${id}`, { method: "PUT", body: JSON.stringify(payload) });
}

export async function deleteCategory(id: number): Promise<{ deleted: boolean }> {
  return apiFetch<{ deleted: boolean }>(`/api/categories/${id}`, { method: "DELETE" });
}
