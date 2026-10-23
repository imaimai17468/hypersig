import { execFileSync } from "node:child_process";
import { postgresContainer, redisContainer } from "./playwright.config.ts";

/** Removes the PostgreSQL and Redis containers, which outlive the shells that started them. */
export default function teardown(): void {
  for (const container of [postgresContainer, redisContainer]) {
    try {
      execFileSync("docker", ["rm", "-f", container], { stdio: "ignore" });
    } catch {
      // `docker run --rm` has removed the container already when its server stopped on its own.
    }
  }
}
