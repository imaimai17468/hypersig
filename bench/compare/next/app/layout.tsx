import { readFileSync } from "node:fs";
import path from "node:path";
import type { ReactNode } from "react";

// Every app in the benchmark inlines the same stylesheet. `next start` runs
// with `bench/compare/next/` as the working directory.
const css = readFileSync(path.join(process.cwd(), "..", "shared", "todomvc.css"), "utf8");

export const metadata = { title: "Next.js TodoMVC" };

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <head>
        <style dangerouslySetInnerHTML={{ __html: css }} />
      </head>
      <body>{children}</body>
    </html>
  );
}
