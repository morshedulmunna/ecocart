"use client";
import { useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle, CardFooter } from "@/components/ui/card";
import { loginUser } from "@/lib/auth";
import { Header } from "@/components/layout/Header";
import { Footer } from "@/components/layout/Footer";
import { Eye, EyeOff, Mail, Lock } from "lucide-react";
import { Spinner } from "@/components/ui/spinner";

export default function LoginPage() {
  const router = useRouter();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [emailError, setEmailError] = useState<string | null>(null);
  const [passwordError, setPasswordError] = useState<string | null>(null);
  const [showPassword, setShowPassword] = useState(false);

  const isValidEmail = (value: string) => /[^\s@]+@[^\s@]+\.[^\s@]+/.test(value);

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    const nextEmailError = email.trim().length === 0 ? "Email is required" : !isValidEmail(email) ? "Enter a valid email" : null;
    const nextPasswordError = password.trim().length < 6 ? "Password must be at least 6 characters" : null;
    setEmailError(nextEmailError);
    setPasswordError(nextPasswordError);
    if (nextEmailError || nextPasswordError) {
      setLoading(false);
      return;
    }
    try {
      await loginUser(email, password);
      router.push("/");
    } catch (err: any) {
      setError(err?.message || "Login failed");
    } finally {
      setLoading(false);
    }
  };

  return (
    <>
      <Header />
      <div className="min-h-dvh flex items-center justify-center px-4 py-16">
        <div className="w-full max-w-sm">
          <div className="mb-6 text-center">
            <div className="text-2xl font-semibold">
              <span className="bg-gradient-to-r from-green-600 to-emerald-600 bg-clip-text text-transparent">EcoCart</span>
            </div>
          </div>
          <div className="relative rounded-2xl bg-gradient-to-br from-green-600/30 to-emerald-600/30 p-[1px]">
            <Card className="rounded-2xl shadow-lg">
              <CardHeader className="w-full flex justify-center flex-col items-center">
                <CardTitle className="text-xl">Welcome back</CardTitle>
                <p className="text-sm text-muted-foreground">Sign in to your EcoCart account</p>
              </CardHeader>
              <CardContent>
                <form onSubmit={onSubmit} className="space-y-3">
                  <div className="space-y-1">
                    <Label htmlFor="email">Email</Label>
                    <div className="relative">
                      <Mail className="pointer-events-none absolute left-2 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
                      <Input
                        id="email"
                        type="email"
                        autoComplete="email"
                        value={email}
                        aria-invalid={!!emailError}
                        className="pl-9"
                        onChange={(e) => {
                          setEmail(e.target.value);
                          if (emailError) setEmailError(null);
                        }}
                        onBlur={() => setEmailError(email.trim() ? (isValidEmail(email) ? null : "Enter a valid email") : "Email is required")}
                        required
                      />
                    </div>
                    {emailError && <div className="text-destructive text-xs">{emailError}</div>}
                  </div>
                  <div className="space-y-1">
                    <Label htmlFor="password">Password</Label>
                    <div className="relative">
                      <Lock className="pointer-events-none absolute left-2 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
                      <Input
                        id="password"
                        type={showPassword ? "text" : "password"}
                        autoComplete="current-password"
                        value={password}
                        aria-invalid={!!passwordError}
                        className="pl-9"
                        onChange={(e) => {
                          setPassword(e.target.value);
                          if (passwordError) setPasswordError(null);
                        }}
                        onBlur={() => setPasswordError(password.trim().length >= 6 ? null : "Password must be at least 6 characters")}
                        required
                      />
                      <button type="button" className="absolute right-2 top-1/2 -translate-y-1/2 rounded-md p-1 text-muted-foreground hover:bg-accent" onClick={() => setShowPassword((v) => !v)} aria-label={showPassword ? "Hide password" : "Show password"}>
                        {showPassword ? <EyeOff className="size-4" /> : <Eye className="size-4" />}
                      </button>
                    </div>
                    {passwordError && <div className="text-destructive text-xs">{passwordError}</div>}
                  </div>
                  {error && (
                    <div className="text-destructive text-sm" role="alert" aria-live="polite">
                      {error}
                    </div>
                  )}
                  <Button className="w-full bg-gradient-to-r from-green-600 to-emerald-600 text-white hover:opacity-90" type="submit" disabled={loading || !!emailError || !!passwordError}>
                    {loading ? (
                      <span className="inline-flex items-center gap-2">
                        <Spinner size={16} /> Signing in...
                      </span>
                    ) : (
                      "Sign in"
                    )}
                  </Button>
                </form>
              </CardContent>
              <CardFooter className="flex flex-col gap-2">
                <div className="text-sm text-muted-foreground">
                  Don&apos;t have an account?{" "}
                  <Link href="/auth/register" className="text-primary hover:underline">
                    Register
                  </Link>
                </div>
                <div className="text-xs text-muted-foreground">
                  Or{" "}
                  <Link href="/products" className="hover:underline">
                    continue shopping
                  </Link>
                </div>
              </CardFooter>
            </Card>
          </div>
        </div>
      </div>
      <Footer />
    </>
  );
}
