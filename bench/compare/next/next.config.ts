import type { NextConfig } from "next";

// The React Compiler is opt-in in Next.js 16.4.1 and stays off.
const nextConfig: NextConfig = {
  // `bench/compare/` has its own lockfile for the harness. Without this,
  // Turbopack picks that directory as the workspace root.
  turbopack: { root: import.meta.dirname },
};

export default nextConfig;
