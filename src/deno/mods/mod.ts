export * from "../../../wasm/pkg/example.js";

// @deno-types="../../../wasm/pkg/example.d.ts"
import { __wbg_init, InitOutput } from "../../../wasm/pkg/example.js";
import { data } from "../../../wasm/pkg/example.wasm.js";

let output: InitOutput | undefined = undefined

export async function initBundledOnce() {
  return output ??= await __wbg_init(data)
}