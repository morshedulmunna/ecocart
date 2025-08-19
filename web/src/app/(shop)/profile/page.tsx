"use client";
import { useEffect, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import type { User } from "@/lib/types";

export default function ProfilePage() {
  const [user, setUser] = useState<User | null>(null);
  const [editing, setEditing] = useState(false);
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");

  useEffect(() => {
    const raw = typeof window !== "undefined" ? window.localStorage.getItem("ecocart.user") : null;
    if (raw) {
      const u = JSON.parse(raw) as User;
      setUser(u);
      setUsername(u.username);
      setEmail(u.email);
    }
  }, []);

  return (
    <div className="space-y-6">
      <h1 className="text-xl font-semibold">Profile</h1>
      <Card>
        <CardHeader>
          <CardTitle>Account</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          {!user ? (
            <div className="text-sm text-muted-foreground">You are not signed in.</div>
          ) : (
            <>
              <div className="grid grid-cols-1 gap-3 md:grid-cols-2">
                <div>
                  <label className="mb-1 block text-sm font-medium">Username</label>
                  <Input disabled={!editing} value={username} onChange={(e) => setUsername(e.target.value)} />
                </div>
                <div>
                  <label className="mb-1 block text-sm font-medium">Email</label>
                  <Input disabled={!editing} type="email" value={email} onChange={(e) => setEmail(e.target.value)} />
                </div>
              </div>
              <div className="text-sm text-muted-foreground">Role: {user.role}</div>
              <div className="flex gap-2">
                {!editing ? (
                  <Button onClick={() => setEditing(true)}>Edit</Button>
                ) : (
                  <>
                    <Button
                      onClick={() => {
                        setEditing(false);
                      }}
                    >
                      Save
                    </Button>
                    <Button
                      variant="outline"
                      onClick={() => {
                        setEditing(false);
                        setUsername(user.username);
                        setEmail(user.email);
                      }}
                    >
                      Cancel
                    </Button>
                  </>
                )}
              </div>
            </>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
