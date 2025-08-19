import Link from "next/link";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Container } from "@/components/layout/Container";
import { Header } from "@/components/layout/Header";
import { Footer } from "@/components/layout/Footer";
import { ArrowRight, Leaf, ShieldCheck, ShoppingCart as CartIcon, Truck, Filter, Search } from "lucide-react";

export default function Home() {
  return (
    <div className="min-h-dvh flex flex-col">
      <Header />
      <main className="flex-1">
        {/* Hero */}
        <section className="relative overflow-hidden">
          <Container className="py-20 text-center">
            <Badge className="mx-auto border-green-600/40 text-green-700 dark:text-green-400">Sustainable by design</Badge>
            <h1 className="mt-4 text-4xl font-bold tracking-tight md:text-6xl">Eco-friendly products for everyday life</h1>
            <p className="mx-auto mt-4 max-w-2xl text-muted-foreground">Shop thoughtfully curated, sustainable goods. Filter by category, compare prices, and build a cart that’s good for you and the planet.</p>
            <div className="mt-8 flex flex-wrap items-center justify-center gap-3">
              <Link href="/products">
                <Button size="lg" className="gap-2">
                  Shop now <ArrowRight className="size-4" />
                </Button>
              </Link>
              <Link href="/auth/register">
                <Button size="lg" variant="outline">
                  Create account
                </Button>
              </Link>
            </div>

            <div className="mx-auto mt-12 grid max-w-3xl grid-cols-2 gap-6 text-left md:grid-cols-4">
              <div className="flex items-center gap-3">
                <Leaf className="size-5 text-green-600 dark:text-green-400" />
                <span className="text-sm text-muted-foreground">Eco-certified items</span>
              </div>
              <div className="flex items-center gap-3">
                <Filter className="size-5 text-green-600 dark:text-green-400" />
                <span className="text-sm text-muted-foreground">Smart filters</span>
              </div>
              <div className="flex items-center gap-3">
                <Truck className="size-5 text-green-600 dark:text-green-400" />
                <span className="text-sm text-muted-foreground">Fast delivery</span>
              </div>
              <div className="flex items-center gap-3">
                <ShieldCheck className="size-5 text-green-600 dark:text-green-400" />
                <span className="text-sm text-muted-foreground">Secure checkout</span>
              </div>
            </div>
          </Container>
        </section>

        {/* Features */}
        <section>
          <Container className="py-12">
            <div className="grid grid-cols-1 gap-6 md:grid-cols-3">
              <Card>
                <CardHeader>
                  <CardTitle className="flex items-center gap-2 text-base">
                    <Search className="size-4 text-green-600 dark:text-green-400" /> Find what matters
                  </CardTitle>
                </CardHeader>
                <CardContent className="text-sm text-muted-foreground">Powerful search with category and price filters to discover the right product faster.</CardContent>
              </Card>
              <Card>
                <CardHeader>
                  <CardTitle className="flex items-center gap-2 text-base">
                    <CartIcon className="size-4 text-green-600 dark:text-green-400" /> Seamless cart
                  </CardTitle>
                </CardHeader>
                <CardContent className="text-sm text-muted-foreground">Add, edit, and manage items effortlessly. Your cart stays saved across sessions.</CardContent>
              </Card>
              <Card>
                <CardHeader>
                  <CardTitle className="flex items-center gap-2 text-base">
                    <ShieldCheck className="size-4 text-green-600 dark:text-green-400" /> Protected admin
                  </CardTitle>
                </CardHeader>
                <CardContent className="text-sm text-muted-foreground">Role-gated admin panel for product CRUD and secure image uploads.</CardContent>
              </Card>
            </div>
          </Container>
        </section>

        {/* Call to action */}
        <section className="pb-20 pt-8">
          <Container>
            <div className="relative overflow-hidden rounded-2xl border bg-gradient-to-r from-green-600 to-emerald-600 p-8 text-white">
              <div className="absolute -right-10 -top-10 size-40 rounded-full bg-white/10" />
              <div className="absolute -bottom-10 -left-10 size-40 rounded-full bg-white/10" />
              <div className="relative z-10 flex flex-col items-start gap-3 md:flex-row md:items-center md:justify-between">
                <div>
                  <h2 className="text-2xl font-semibold tracking-tight">Ready to shop sustainably?</h2>
                  <p className="text-white/80">Join EcoCart and get access to curated eco products.</p>
                </div>
                <div className="flex gap-3">
                  <Link href="/products">
                    <Button size="lg" className="bg-white text-green-700 hover:bg-white/90">
                      Browse products
                    </Button>
                  </Link>
                  <Link href="/auth/register">
                    <Button size="lg" variant="outline" className="border-white text-white hover:bg-white/10">
                      Create account
                    </Button>
                  </Link>
                </div>
              </div>
            </div>
          </Container>
        </section>
      </main>
      <Footer />
    </div>
  );
}
