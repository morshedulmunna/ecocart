import Image from "next/image";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import type { Product } from "@/lib/types";
import { addToCart } from "@/store/cart";
import { resolveImageUrl } from "@/lib/utils";

type Props = { product: Product };

export function ProductCard({ product }: Props) {
  return (
    <Card className="h-full">
      <CardHeader>
        <CardTitle className="truncate" title={product.name}>
          {product.name}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="relative aspect-square w-full overflow-hidden rounded-md bg-accent">
          {product.image_url ? (
            <Image src={resolveImageUrl(product.image_url)!} alt={product.name} fill unoptimized className="object-cover transition-transform duration-300 hover:scale-105" />
          ) : (
            <div className="flex h-full items-center justify-center text-muted-foreground">No image</div>
          )}
        </div>
        <p className="mt-2 line-clamp-2 text-sm text-muted-foreground">{product.description}</p>
        <p className="mt-2 text-lg font-semibold">${product.price.toFixed(2)}</p>
      </CardContent>
      <CardFooter>
        <Button onClick={() => addToCart(product, 1)} className="w-full">
          Add to cart
        </Button>
      </CardFooter>
    </Card>
  );
}
