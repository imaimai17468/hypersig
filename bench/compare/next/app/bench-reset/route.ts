import { reset } from "../todos";

export async function POST(): Promise<Response> {
  reset();
  return new Response(null, { status: 204 });
}
