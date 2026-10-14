import assert from "node:assert/strict";
import { test } from "node:test";
import { findBlobs } from "./check-no-blobs.mjs";

// Fixtures are built at run time so this file never holds a blob marker itself.
const wasmModule = Buffer.from([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x04, 0x01, 0x60]);
const wasmBase64 = wasmModule.toString("base64");

test("a tracked .wasm file is refused", () => {
  const offenders = findBlobs([{ path: "plugin/engine_bg.wasm", content: "" }]);
  assert.deepEqual(offenders, [{ path: "plugin/engine_bg.wasm", reasons: ["WASM file"] }]);
});

test("a WASM module encoded in base64 is refused", () => {
  const offenders = findBlobs([{ path: "plugin/src/wasm-inline.ts", content: `export default "${wasmBase64}";` }]);
  assert.deepEqual(offenders, [{ path: "plugin/src/wasm-inline.ts", reasons: ["base64-encoded WASM"] }]);
});

test("a worker embedded as a base64 data: URL is refused", () => {
  const worker = Buffer.from("self.onmessage=()=>{}").toString("base64");
  const url = ["data", ":application/javascript;base64,", worker].join("");
  const offenders = findBlobs([{ path: "plugin/src/worker.ts", content: `new Worker("${url}")` }]);
  assert.deepEqual(offenders, [{ path: "plugin/src/worker.ts", reasons: ["base64 data: URL of a script or module"] }]);
});

test("a lockfile with its sha512 integrity hashes passes", () => {
  const integrity = `sha512-${Buffer.alloc(64, 0xa7).toString("base64")}`;
  const lockfile = JSON.stringify({ packages: { "node_modules/esbuild": { version: "0.28.2", integrity } } });
  assert.deepEqual(findBlobs([{ path: "plugin/package-lock.json", content: lockfile }]), []);
});
