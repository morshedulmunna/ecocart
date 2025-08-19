"use client";
import { useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle, CardFooter } from "@/components/ui/card";
import { registerUser, loginUser } from "@/lib/auth";
import { Eye, EyeOff } from "lucide-react";
import { Spinner } from "@/components/ui/spinner";
import { Header } from "@/components/layout/Header";
import { Footer } from "@/components/layout/Footer";

export default function RegisterPage() {
  const router = useRouter();
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [usernameError, setUsernameError] = useState<string | null>(null);
  const [emailError, setEmailError] = useState<string | null>(null);
  const [passwordError, setPasswordError] = useState<string | null>(null);
  const [confirmError, setConfirmError] = useState<string | null>(null);
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirm, setShowConfirm] = useState(false);

  const isValidEmail = (value: string) => /[^\s@]+@[^\s@]+\.[^\s@]+/.test(value);

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    const nextUsernameError = username.trim().length < 2 ? "Username must be at least 2 characters" : null;
    const nextEmailError = email.trim().length === 0 ? "Email is required" : !isValidEmail(email) ? "Enter a valid email" : null;
    const nextPasswordError = password.trim().length < 6 ? "Password must be at least 6 characters" : null;
    const nextConfirmError = confirmPassword !== password ? "Passwords do not match" : null;
    setUsernameError(nextUsernameError);
    setEmailError(nextEmailError);
    setPasswordError(nextPasswordError);
    setConfirmError(nextConfirmError);
    if (nextUsernameError || nextEmailError || nextPasswordError || nextConfirmError) {
      setLoading(false);
      return;
    }
    try {
      await registerUser({ username, email, password });
      await loginUser(email, password);
      router.push("/");
    } catch (err: any) {
      setError(err?.message || "Registration failed");
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
            <Card>
              <CardHeader className="w-full flex justify-center flex-col items-center">
                <CardTitle className="text-xl">Create your account</CardTitle>
                <p className="text-sm text-muted-foreground">Join EcoCart and start shopping sustainably</p>
              </CardHeader>
              <CardContent>
                <form onSubmit={onSubmit} className="space-y-3">
                  <div className="space-y-1">
                    <Label htmlFor="username">Username</Label>
                    <Input
                      id="username"
                      value={username}
                      aria-invalid={!!usernameError}
                      onChange={(e) => {
                        setUsername(e.target.value);
                        if (usernameError) setUsernameError(null);
                      }}
                      onBlur={() => setUsernameError(username.trim().length >= 2 ? null : "Username must be at least 2 characters")}
                      required
                    />
                    {usernameError && <div className="text-destructive text-xs">{usernameError}</div>}
                  </div>
                  <div className="space-y-1">
                    <Label htmlFor="email">Email</Label>
                    <Input
                      id="email"
                      type="email"
                      value={email}
                      aria-invalid={!!emailError}
                      onChange={(e) => {
                        setEmail(e.target.value);
                        if (emailError) setEmailError(null);
                      }}
                      onBlur={() => setEmailError(email.trim() ? (isValidEmail(email) ? null : "Enter a valid email") : "Email is required")}
                      required
                    />
                    {emailError && <div className="text-destructive text-xs">{emailError}</div>}
                  </div>
                  <div className="space-y-1">
                    <Label htmlFor="password">Password</Label>
                    <div className="relative">
                      <Input
                        id="password"
                        type={showPassword ? "text" : "password"}
                        value={password}
                        aria-invalid={!!passwordError}
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
                  <div className="space-y-1">
                    <Label htmlFor="confirm">Confirm password</Label>
                    <div className="relative">
                      <Input
                        id="confirm"
                        type={showConfirm ? "text" : "password"}
                        value={confirmPassword}
                        aria-invalid={!!confirmError}
                        onChange={(e) => {
                          setConfirmPassword(e.target.value);
                          if (confirmError) setConfirmError(null);
                        }}
                        onBlur={() => setConfirmError(confirmPassword === password ? null : "Passwords do not match")}
                        required
                      />
                      <button type="button" className="absolute right-2 top-1/2 -translate-y-1/2 rounded-md p-1 text-muted-foreground hover:bg-accent" onClick={() => setShowConfirm((v) => !v)} aria-label={showConfirm ? "Hide password" : "Show password"}>
                        {showConfirm ? <EyeOff className="size-4" /> : <Eye className="size-4" />}
                      </button>
                    </div>
                    {confirmError && <div className="text-destructive text-xs">{confirmError}</div>}
                  </div>
                  {error && (
                    <div className="text-destructive text-sm" role="alert" aria-live="polite">
                      {error}
                    </div>
                  )}
                  <Button className="w-full" type="submit" disabled={loading || !!usernameError || !!emailError || !!passwordError || !!confirmError}>
                    {loading ? (
                      <span className="inline-flex items-center gap-2">
                        <Spinner size={16} /> Creating...
                      </span>
                    ) : (
                      "Create account"
                    )}
                  </Button>
                </form>
              </CardContent>
              <CardFooter className="flex flex-col gap-2">
                <div className="text-sm text-muted-foreground">
                  Already have an account?{" "}
                  <Link href="/auth/login" className="text-primary hover:underline">
                    Login
                  </Link>
                </div>
                <div className="text-xs text-muted-foreground">
                  Or{" "}
                  <Link href="/products" className="hover:underline">
                    browse products
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
