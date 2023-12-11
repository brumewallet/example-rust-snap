export * from "../../../wasm/pkg/example.js";

import { InitOutput, __wbg_init } from "../../../wasm/pkg/example.js";
import { data } from "../../../wasm/pkg/example.wasm.js";

let output: InitOutput | undefined = undefined

export async function initBundledOnce() {
  return output ??= await __wbg_init(data)
}