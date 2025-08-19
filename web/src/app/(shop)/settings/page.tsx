"use client";
import { useEffect, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ThemeToggle } from "@/components/ui/theme-toggle";

export default function SettingsPage() {
  const [emailPrefs, setEmailPrefs] = useState(true);
  useEffect(() => {
    const raw = typeof window !== "undefined" ? window.localStorage.getItem("ecocart.email_prefs") : null;
    if (raw != null) setEmailPrefs(raw === "true");
  }, []);

  const save = () => {
    if (typeof window !== "undefined") window.localStorage.setItem("ecocart.email_prefs", String(emailPrefs));
  };

  return (
    <div className="space-y-6">
      <h1 className="text-xl font-semibold">Settings</h1>
      <Card>
        <CardHeader>
          <CardTitle>Appearance</CardTitle>
        </CardHeader>
        <CardContent className="flex items-center gap-3">
          <ThemeToggle />
          <div className="text-sm text-muted-foreground">Toggle theme</div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Notifications</CardTitle>
        </CardHeader>
        <CardContent className="flex items-center justify-between">
          <div className="text-sm">Email updates</div>
          <Button variant="outline" onClick={() => setEmailPrefs((v) => !v)}>
            {emailPrefs ? "On" : "Off"}
          </Button>
        </CardContent>
      </Card>

      <div className="flex justify-end">
        <Button onClick={save}>Save changes</Button>
      </div>
    </div>
  );
}
