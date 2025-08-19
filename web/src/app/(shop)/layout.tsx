import { Header } from "@/components/layout/Header";
import { Footer } from "@/components/layout/Footer";

export default function ShopLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-dvh flex flex-col">
      <Header />
      <main className="mx-auto w-full max-w-6xl flex-1 px-4 py-6">{children}</main>
      <Footer />
    </div>
  );
}
