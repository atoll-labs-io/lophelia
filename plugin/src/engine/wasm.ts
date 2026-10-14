// Only initSync is imported: the glue's default export is an async init that
// calls fetch, and leaving it unreferenced lets esbuild drop it from the bundle.
import { engine_version, initSync } from "../../wasm-pkg/lophelia_wasm.js";
import wasmBytes from "../../wasm-pkg/lophelia_wasm_bg.wasm";

let initialized = false;

function ensureEngine(): void {
  if (!initialized) {
    initSync({ module: wasmBytes });
    initialized = true;
  }
}

export function engineVersion(): string {
  ensureEngine();
  return engine_version();
}
