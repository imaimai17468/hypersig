import { execFileSync } from "node:child_process";
import { postgresContainer } from "./playwright.config.ts";

/** Removes the PostgreSQL container, which outlives the shell that started it. */
export default function teardown(): void {
  try {
    execFileSync("docker", ["rm", "-f", postgresContainer], { stdio: "ignore" });
  } catch {
    // `docker run --rm` has removed the container already when PostgreSQL stopped on its own.
  }
}
