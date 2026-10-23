The crate compiles to `wasm32-unknown-unknown` and runs on Cloudflare Workers, and `npx wrangler dev` serves it at http://localhost:8787.

- A worker has no tokio networking or timers, so `axum` is built without its default features, and a stream waits with `worker::Delay` wrapped in `worker::send::SendFuture`.
- A crate that reads the clock or random numbers needs its wasm feature. `getrandom` 0.2 needs `js`, without which the build fails, and `time` needs `wasm-bindgen`, without which reading the clock panics with `time not implemented on this platform`.
- Requests reach many isolates that share no memory, so a `static` or a value in the router's state reaches only one of them. The template keeps its data in D1, and its `Hub` Durable Object hands each save to the streams of every isolate.
