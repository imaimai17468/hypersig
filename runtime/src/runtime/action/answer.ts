/** The event `crates/hypersig/src/axum.rs` sends last on every answer whose handler finished. */
export const END_EVENT = "hs-end";

/** A failed request: the text the page shows for it, and the HTTP status, 0 where none came back. */
export interface Failure {
  readonly message: string;
  readonly status: number;
}

/** The media type of `response`, lowercase and without its parameters, or `""` without one. */
function mediaType(response: Response): string {
  return (response.headers.get("content-type") ?? "").split(";")[0]!.trim().toLowerCase();
}

/**
 * The failure a response stands for, or `null` for a 2xx event stream the
 * runtime reads. A proxy that sends a signed-out visitor to its sign-in page
 * answers the redirected `fetch` with a 200 HTML page, which carries no
 * patches, so a redirected answer and any other media type fail with status 0.
 */
export async function refused(response: Response): Promise<Failure | null> {
  if (!response.ok) return { message: await failureText(response), status: response.status };
  if (response.redirected)
    return { message: `the answer was redirected to ${response.url}`, status: 0 };
  const media = mediaType(response);
  if (media !== "text/event-stream" || response.body === null)
    return { message: `the answer is ${media || "empty"}, not an event stream`, status: 0 };
  return null;
}

/**
 * The text the error signal shows for an answer outside 2xx. A handler that
 * fails writes the sentence for the visitor as `text/plain`, which axum gives
 * a `String`, while a proxy's error page is HTML, so only a plain text body
 * is kept, and the status and its reason stand in for any other.
 */
async function failureText(response: Response): Promise<string> {
  const body = mediaType(response) === "text/plain" ? (await response.text()).trim() : "";
  return body === "" ? `${response.status} ${response.statusText}`.trim() : body;
}
