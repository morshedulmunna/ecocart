export type Category = {
  id: number;
  name: string;
  description?: string | null;
};

export type Product = {
  id: number;
  name: string;
  description: string;
  price: number;
  category_id: number;
  stock: number;
  image_url?: string | null;
};

export type Paginated<T> = {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  totalPages: number;
};

export type CartItem = {
  product: Product;
  quantity: number;
};

export type User = {
  id: string;
  username: string;
  email: string;
  role: "user" | "admin" | string;
  created_at?: string;
};
