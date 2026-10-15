// Puts the shared stylesheet and an empty favicon into the head of every
// server-rendered page. See `nuxt.config.ts` for why this is not `app.head`.
import css from "#todomvc-css";

export default defineNitroPlugin((nitroApp) => {
  nitroApp.hooks.hook("render:html", (html) => {
    html.head.push(`<link rel="icon" href="data:,"><style>${css}</style>`);
  });
});
