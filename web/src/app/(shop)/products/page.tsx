"use client";
import { Suspense, useEffect, useMemo, useState } from "react";
import { useSearchParams, usePathname, useRouter } from "next/navigation";
import { listProducts } from "@/lib/products";
import { listCategories } from "@/lib/categories";
import type { Category, Paginated, Product } from "@/lib/types";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { Badge } from "@/components/ui/badge";
import { ProductCard } from "@/components/product/ProductCard";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Search as SearchIcon, Filter as FilterIcon, X, ChevronLeft, ChevronRight } from "lucide-react";

function ProductsPageContent() {
  const searchParams = useSearchParams();
  const pathname = usePathname();
  const router = useRouter();
  const [products, setProducts] = useState<Product[]>([]);
  const [total, setTotal] = useState(0);
  const [totalPages, setTotalPages] = useState(1);
  const [categories, setCategories] = useState<Category[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [q, setQ] = useState("");
  const [categoryId, setCategoryId] = useState<number | undefined>(undefined);
  const [minPrice, setMinPrice] = useState<string>("");
  const [maxPrice, setMaxPrice] = useState<string>("");
  const [page, setPage] = useState(1);
  const [sort, setSort] = useState("relevance");
  const [mobileOpen, setMobileOpen] = useState(false);
  const pageSize = 12;

  useEffect(() => {
    listCategories()
      .then(setCategories)
      .catch(() => setCategories([]));
  }, []);

  // Initialize from URL query params and react to back/forward
  useEffect(() => {
    const sp = new URLSearchParams(searchParams.toString());
    const nextQ = sp.get("q") ?? "";
    const nextCategory = sp.get("category_id");
    const nextMin = sp.get("min_price") ?? "";
    const nextMax = sp.get("max_price") ?? "";
    const nextPageStr = sp.get("page") ?? "1";
    const nextPage = Number(nextPageStr);

    setQ(nextQ);
    setCategoryId(nextCategory ? Number(nextCategory) : undefined);
    setMinPrice(nextMin);
    setMaxPrice(nextMax);
    setPage(Number.isNaN(nextPage) || nextPage < 1 ? 1 : nextPage);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchParams]);

  function updateQueryParams(next: Partial<{ q: string; category_id?: number; min_price?: string; max_price?: string; page?: number }>) {
    const sp = new URLSearchParams(searchParams.toString());
    const setOrDelete = (key: string, value?: string | number | null) => {
      if (value === undefined || value === null || value === "") sp.delete(key);
      else sp.set(key, String(value));
    };
    setOrDelete("q", next.q ?? q);
    setOrDelete("category_id", next.category_id ?? categoryId);
    setOrDelete("min_price", next.min_price ?? minPrice);
    setOrDelete("max_price", next.max_price ?? maxPrice);
    setOrDelete("page", next.page ?? page);
    const url = `${pathname}?${sp.toString()}`;
    router.replace(url, { scroll: false });
  }

  const fetchProducts = async () => {
    setLoading(true);
    setError(null);
    try {
      const data: Paginated<Product> = await listProducts({
        q: q || undefined,
        category_id: categoryId,
        min_price: minPrice ? Number(minPrice) : undefined,
        max_price: maxPrice ? Number(maxPrice) : undefined,
        page,
        pageSize,
      });
      setProducts(data.items);
      setTotal(data.total);
      setTotalPages(data.totalPages || 1);
    } catch (e: any) {
      setError(e?.message || "Failed to load products");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchProducts();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [q, categoryId, minPrice, maxPrice, page]);

  // totalPages comes from API now
  const clearFilters = () => {
    setQ("");
    setCategoryId(undefined);
    setMinPrice("");
    setMaxPrice("");
    setSort("relevance");
    setPage(1);
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-lg font-semibold">Products</h1>
        <div className="flex items-center gap-2">
          <Badge className="hidden md:inline">{total} results</Badge>
          <Button variant="outline" size="sm" className="md:hidden inline-flex items-center gap-2" onClick={() => setMobileOpen(true)}>
            <FilterIcon className="size-4" /> Filters
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 gap-6 md:grid-cols-12">
        <aside className="hidden md:block md:col-span-3">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2 text-base">
                <FilterIcon className="size-4 text-green-600 dark:text-green-400" /> Filters
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <div className="relative">
                <SearchIcon className="pointer-events-none absolute left-2 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
                <Input
                  placeholder="Search products"
                  value={q}
                  onChange={(e) => {
                    setQ(e.target.value);
                    updateQueryParams({ q: e.target.value, page: 1 });
                  }}
                  className="pl-9"
                />
              </div>
              <Select
                value={String(categoryId ?? "")}
                onChange={(e) => {
                  const v = e.target.value ? Number(e.target.value) : undefined;
                  setCategoryId(v);
                  updateQueryParams({ category_id: v, page: 1 });
                }}
              >
                <option value="">All Categories</option>
                {categories.map((c) => (
                  <option key={c.id} value={c.id}>
                    {c.name}
                  </option>
                ))}
              </Select>
              <Input placeholder="Min price" type="number" inputMode="decimal" value={minPrice} onChange={(e) => setMinPrice(e.target.value)} onBlur={(e) => updateQueryParams({ min_price: e.target.value || undefined, page: 1 })} />
              <Input placeholder="Max price" type="number" inputMode="decimal" value={maxPrice} onChange={(e) => setMaxPrice(e.target.value)} onBlur={(e) => updateQueryParams({ max_price: e.target.value || undefined, page: 1 })} />
              <Select value={sort} onChange={(e) => setSort(e.target.value)}>
                <option value="relevance">Sort: Relevance</option>
                <option value="price_asc">Price: Low to High</option>
                <option value="price_desc">Price: High to Low</option>
                <option value="newest">Newest</option>
              </Select>
              <div className="flex gap-2 pt-2">
                <Button
                  variant="outline"
                  size="sm"
                  className="inline-flex items-center gap-2"
                  onClick={() => {
                    clearFilters();
                    updateQueryParams({ q: "", category_id: undefined, min_price: undefined, max_price: undefined, page: 1 });
                  }}
                >
                  <X className="size-4" /> Clear
                </Button>
                <Button size="sm" onClick={() => fetchProducts()}>
                  Apply
                </Button>
              </div>
            </CardContent>
          </Card>
        </aside>

        <section className="md:col-span-9 space-y-6">
          {error && <div className="text-destructive">{error}</div>}
          {loading && (
            <div className="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-4">
              {Array.from({ length: 8 }).map((_, i) => (
                <div key={i} className="space-y-2">
                  <Skeleton className="h-40 w-full" />
                  <Skeleton className="h-4 w-3/4" />
                  <Skeleton className="h-4 w-1/2" />
                </div>
              ))}
            </div>
          )}

          {products.length === 0 && !loading ? (
            <Card>
              <CardContent className="py-10 text-center text-sm text-muted-foreground">No products found. Try adjusting filters.</CardContent>
            </Card>
          ) : (
            <div className="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-4">
              {products.map((p) => (
                <ProductCard key={p.id} product={p} />
              ))}
            </div>
          )}

          <div className="flex items-center justify-center gap-3">
            <Button
              variant="outline"
              disabled={page <= 1}
              onClick={() => {
                const next = Math.max(1, page - 1);
                setPage(next);
                updateQueryParams({ page: next });
              }}
              className="inline-flex items-center gap-2"
            >
              <ChevronLeft className="size-4" /> Prev
            </Button>
            <span className="text-xs md:text-sm">
              Page {page} / {totalPages}
            </span>
            <Button
              variant="outline"
              onClick={() => {
                const next = page + 1;
                setPage(next);
                updateQueryParams({ page: next });
              }}
              className="inline-flex items-center gap-2"
            >
              Next <ChevronRight className="size-4" />
            </Button>
          </div>
        </section>
      </div>

      {mobileOpen && (
        <div className="fixed inset-0 z-40 md:hidden">
          <div className="absolute inset-0 bg-black/40" onClick={() => setMobileOpen(false)} />
          <div className="absolute inset-x-0 bottom-0 rounded-t-2xl border bg-background p-4 shadow-lg">
            <div className="mb-3 flex items-center justify-between">
              <div className="text-sm font-medium">Filters</div>
              <Button variant="outline" size="sm" className="inline-flex items-center gap-2" onClick={() => setMobileOpen(false)}>
                <X className="size-4" /> Close
              </Button>
            </div>
            <div className="space-y-3">
              <div className="relative">
                <SearchIcon className="pointer-events-none absolute left-2 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
                <Input placeholder="Search products" value={q} onChange={(e) => setQ(e.target.value)} onBlur={(e) => updateQueryParams({ q: e.target.value, page: 1 })} className="pl-9" />
              </div>
              <Select
                value={String(categoryId ?? "")}
                onChange={(e) => {
                  const v = e.target.value ? Number(e.target.value) : undefined;
                  setCategoryId(v);
                  updateQueryParams({ category_id: v, page: 1 });
                }}
              >
                <option value="">All Categories</option>
                {categories.map((c) => (
                  <option key={c.id} value={c.id}>
                    {c.name}
                  </option>
                ))}
              </Select>
              <Input placeholder="Min price" type="number" inputMode="decimal" value={minPrice} onChange={(e) => setMinPrice(e.target.value)} onBlur={(e) => updateQueryParams({ min_price: e.target.value || undefined, page: 1 })} />
              <Input placeholder="Max price" type="number" inputMode="decimal" value={maxPrice} onChange={(e) => setMaxPrice(e.target.value)} onBlur={(e) => updateQueryParams({ max_price: e.target.value || undefined, page: 1 })} />
              <Select value={sort} onChange={(e) => setSort(e.target.value)}>
                <option value="relevance">Sort: Relevance</option>
                <option value="price_asc">Price: Low to High</option>
                <option value="price_desc">Price: High to Low</option>
                <option value="newest">Newest</option>
              </Select>
              <div className="flex gap-2 pt-2">
                <Button variant="outline" size="sm" className="inline-flex items-center gap-2" onClick={clearFilters}>
                  <X className="size-4" /> Clear
                </Button>
                <Button
                  size="sm"
                  onClick={() => {
                    setMobileOpen(false);
                    fetchProducts();
                  }}
                >
                  Apply
                </Button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default function ProductsPage() {
  return (
    <Suspense fallback={<div className="py-10 text-center text-sm text-muted-foreground">Loading products...</div>}>
      <ProductsPageContent />
    </Suspense>
  );
}
