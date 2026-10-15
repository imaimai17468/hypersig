import { createFileRoute } from "@tanstack/react-router";
import { reset } from "../todos.server";

export const Route = createFileRoute("/bench-reset")({
  server: {
    handlers: {
      POST: async () => {
        reset();
        return new Response(null, { status: 204 });
      },
    },
  },
});
