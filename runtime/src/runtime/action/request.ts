import type { CallOptions } from "../../eval.ts";
import type { Runtime } from "../../runtime.ts";
import { FILES } from "../binding/ref.ts";

/** The header an upload carries, which `crates/hypersig/src/axum.rs` requires. */
const UPLOAD_HEADER = "hypersig-upload";
/**
 * The header every call carries, which `crates/hypersig/src/axum.rs` requires
 * of a GET. A page on another site cannot send it without a preflight, which
 * the server does not answer.
 */
const CALL_HEADER = "hypersig-call";
/** The header that tells the server which page field the part that made a call sits at. */
const PART_HEADER = "hypersig-part";

/** The URL of a call with the arguments `value`, setting the body and headers of `init` to match. */
export function buildRequest(
  rt: Runtime,
  method: string,
  path: string,
  value: unknown,
  init: RequestInit,
  options: CallOptions,
): URL {
  const url = new URL(path, document.baseURI);
  const body = JSON.stringify(value ?? null);
  const headers: Record<string, string> = { accept: "text/event-stream", [CALL_HEADER]: "1" };
  if (options.files !== undefined) {
    // The server refuses an upload without this header, which a form on another site cannot add.
    headers[UPLOAD_HEADER] = "1";
    init.body = uploadBody(rt, body, options.files, options.origin);
  } else if (method === "GET") {
    if (value !== null && value !== undefined) url.searchParams.set("args", body);
  } else {
    headers["content-type"] = "application/json";
    init.body = body;
  }
  if (options.part !== undefined) headers[PART_HEADER] = options.part;
  init.headers = headers;
  return url;
}

/**
 * The body of an upload: the arguments as JSON in the `args` part, then
 * each file picked in the file input named `files` in a `files` part.
 */
function uploadBody(
  rt: Runtime,
  args: string,
  files: string,
  origin: Element | undefined,
): FormData {
  const data = new FormData();
  data.append("args", new Blob([args], { type: "application/json" }));
  const input = rt.bindings.refs.find(FILES, files, origin);
  if (input instanceof HTMLInputElement && input.files !== null) {
    for (const file of input.files) data.append("files", file, file.name);
  } else if (!(input instanceof HTMLInputElement)) {
    console.warn(`hypersig: no file input named "${files}"`);
  }
  return data;
}
