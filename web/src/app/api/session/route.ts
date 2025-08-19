import { NextResponse } from "next/server";
import { getApiUrl } from "@/lib/config";

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const res = await fetch(getApiUrl("/api/auth/login"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email: body.email, password: body.password }),
    });
    const data = await res.json();
    if (!res.ok) {
      return NextResponse.json(data, { status: res.status });
    }

    const nowSec = Math.floor(Date.now() / 1000);
    const exp = new Date((nowSec + (data?.expires_in ?? 3600)) * 1000);

    const response = NextResponse.json(data);
    response.cookies.set("ecocart.access_token", String(data.access_token), {
      httpOnly: true,
      sameSite: "lax",
      secure: process.env.NODE_ENV === "production",
      expires: exp,
      path: "/",
    });
    response.cookies.set("ecocart.refresh_token", String(data.refresh_token), {
      httpOnly: true,
      sameSite: "lax",
      secure: process.env.NODE_ENV === "production",
      path: "/",
    });
    response.cookies.set("ecocart.token_exp", String(nowSec + (data?.expires_in ?? 3600)), {
      httpOnly: true,
      sameSite: "lax",
      secure: process.env.NODE_ENV === "production",
      path: "/",
    });
    // Mirror user role for middleware-based authorization
    if (data?.user?.role) {
      response.cookies.set("ecocart.user_role", String(data.user.role), {
        httpOnly: true,
        sameSite: "lax",
        secure: process.env.NODE_ENV === "production",
        expires: exp,
        path: "/",
      });
    }
    return response;
  } catch (e: any) {
    return NextResponse.json({ code: "INTERNAL_ERROR", message: e?.message || "Login failed" }, { status: 500 });
  }
}

export async function DELETE() {
  const response = NextResponse.json({ ok: true });
  response.cookies.set("ecocart.access_token", "", { httpOnly: true, expires: new Date(0), path: "/" });
  response.cookies.set("ecocart.refresh_token", "", { httpOnly: true, expires: new Date(0), path: "/" });
  response.cookies.set("ecocart.token_exp", "", { httpOnly: true, expires: new Date(0), path: "/" });
  response.cookies.set("ecocart.user_role", "", { httpOnly: true, expires: new Date(0), path: "/" });
  return response;
}
