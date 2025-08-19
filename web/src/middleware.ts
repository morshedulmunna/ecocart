import type { NextRequest } from "next/server";
import { NextResponse } from "next/server";

export function middleware(req: NextRequest) {
  if (req.nextUrl.pathname.startsWith("/admin")) {
    const token = req.cookies.get("ecocart.access_token")?.value;
    const role = req.cookies.get("ecocart.user_role")?.value;
    if (!token || role !== "admin") {
      const url = req.nextUrl.clone();
      url.pathname = "/auth/login";
      url.searchParams.set("next", req.nextUrl.pathname);
      return NextResponse.redirect(url);
    }
  }
  return NextResponse.next();
}

export const config = {
  matcher: ["/admin/:path*"],
};
