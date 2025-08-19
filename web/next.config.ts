import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  output: "standalone",
  images: {
    // Allow images served from the API container via rewrites (same-origin)
    // and directly when accessed by server-side code using http://api:8080
    remotePatterns: [
      { protocol: "http", hostname: "api", port: "8080" },
      { protocol: "http", hostname: "127.0.0.1", port: "8080" },
      { protocol: "http", hostname: "localhost", port: "8080" },
      { protocol: "http", hostname: "127.0.0.1" },
      { protocol: "http", hostname: "localhost" },
    ],
  },
  async rewrites() {
    // Proxy API and uploads to the backend when running in Docker or locally
    return [
      { source: "/api/:path*", destination: "http://api:8080/api/:path*" },
      { source: "/uploads/:path*", destination: "http://api:8080/uploads/:path*" },
    ];
  },
  experimental: {
    optimizePackageImports: ["lucide-react"],
  },
};

export default nextConfig;
