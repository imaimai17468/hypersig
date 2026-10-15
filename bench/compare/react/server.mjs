// Serves the built single-page app from `dist/` and the JSON API, on
// 127.0.0.1:4105. The list lives in memory, seeded as SPEC.md describes.
//
// API:
//   GET    /api/todos        -> the list as JSON
//   POST   /api/todos        {id, title}  -> 204, appends a todo
//   PATCH  /api/todos/:id    {done}       -> 204, sets done
//   DELETE /api/todos/:id                 -> 204, removes the todo
//   POST   /bench-reset                   -> 204, reseeds

import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import { extname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";

const dist = fileURLToPath(new URL("dist/", import.meta.url));
const TYPES = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json",
  ".svg": "image/svg+xml",
  ".ico": "image/x-icon",
};

function seed() {
  return Array.from({ length: 2000 }, (_, index) => {
    const id = index + 1;
    return { id, title: `todo ${id}`, done: id % 3 === 0 };
  });
}

let todos = seed();

async function body(req) {
  const chunks = [];
  for await (const chunk of req) chunks.push(chunk);
  return JSON.parse(Buffer.concat(chunks).toString("utf8") || "{}");
}

function send(res, status, data, type) {
  res.writeHead(status, data === undefined ? {} : { "content-type": type, "content-length": Buffer.byteLength(data) });
  res.end(data);
}

async function serveFile(res, path) {
  const file = normalize(join(dist, path));
  if (!file.startsWith(dist)) return send(res, 404, "not found", "text/plain");
  try {
    const data = await readFile(file);
    const headers = { "content-type": TYPES[extname(file)] ?? "application/octet-stream", "content-length": data.length };
    if (path.startsWith("/assets/")) headers["cache-control"] = "public, max-age=31536000, immutable";
    res.writeHead(200, headers);
    res.end(data);
  } catch {
    send(res, 404, "not found", "text/plain");
  }
}

const server = createServer(async (req, res) => {
  const { pathname } = new URL(req.url, "http://localhost");
  try {
    if (pathname === "/bench-reset" && req.method === "POST") {
      todos = seed();
      return send(res, 204);
    }
    if (pathname === "/api/todos") {
      if (req.method === "GET") return send(res, 200, JSON.stringify(todos), "application/json");
      if (req.method === "POST") {
        const { id, title } = await body(req);
        const trimmed = String(title ?? "").trim();
        if (Number.isInteger(id) && trimmed !== "") todos.push({ id, title: trimmed, done: false });
        return send(res, 204);
      }
    }
    const match = /^\/api\/todos\/(\d+)$/.exec(pathname);
    if (match) {
      const id = Number(match[1]);
      if (req.method === "PATCH") {
        const { done } = await body(req);
        todos = todos.map((todo) => (todo.id === id ? { ...todo, done: Boolean(done) } : todo));
        return send(res, 204);
      }
      if (req.method === "DELETE") {
        todos = todos.filter((todo) => todo.id !== id);
        return send(res, 204);
      }
    }
    if (req.method === "GET") return serveFile(res, pathname === "/" ? "/index.html" : pathname);
    send(res, 404, "not found", "text/plain");
  } catch (error) {
    send(res, 400, String(error), "text/plain");
  }
});

server.listen(4105, "127.0.0.1");
