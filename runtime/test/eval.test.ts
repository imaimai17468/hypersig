import { expect, it } from "vitest";
import { AstError, evaluate } from "../src/eval.ts";
import { ctx } from "./eval/context.ts";

it("rejects an operation it does not know", () => {
  expect(() => evaluate(["cube", ["lit", 2]], ctx)).toThrow(AstError);
});
