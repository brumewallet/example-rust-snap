import { readFileSync, rmSync, writeFileSync } from "fs";

const wasm = readFileSync("./wasm/pkg/example_bg.wasm")

writeFileSync(`./wasm/pkg/example.wasm.js`, `export const data = "data:application/wasm;base64,${wasm.toString("base64")}";`);
writeFileSync(`./wasm/pkg/example.wasm.d.ts`, `export const data: string;`);

const disposableJs = `  [Symbol.dispose]() {
        this.free()
    }

    free() {`

const disposableTs = `  [Symbol.dispose](): void
  free(): void;`

const glueJs = readFileSync(`./wasm/pkg/example.js`, "utf8")
  .replace("async function __wbg_init", "export async function __wbg_init")
  .replace("input = new URL('example_bg.wasm', import.meta.url);", "throw new Error();")
  .replaceAll("  free() {", disposableJs)

const glueTs = readFileSync(`./wasm/pkg/example.d.ts`, "utf8")
  .replace("export default function __wbg_init", "export function __wbg_init")
  .replaceAll("  free(): void;", disposableTs)

writeFileSync(`./wasm/pkg/example.js`, glueJs)
writeFileSync(`./wasm/pkg/example.d.ts`, glueTs)

rmSync(`./wasm/pkg/.gitignore`, { force: true });