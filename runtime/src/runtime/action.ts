import type { Runtime } from "../runtime.ts";
import { Calls } from "./action/call.ts";
import { Patches } from "./action/patch.ts";

/** The calls statements make to the server's actions, and the patches the actions answer with. */
export class Actions {
  readonly calls: Calls;
  readonly patches: Patches;

  constructor(rt: Runtime) {
    this.calls = new Calls(rt);
    this.patches = new Patches(rt);
  }
}
