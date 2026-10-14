#!/usr/bin/env node
// Fails when git tracks a WASM file or an embedded WASM or worker blob (story s1, criterion 3; ADR-0004).
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

// Built at run time from the WASM header (magic "\0asm", version 1) so this file never holds the
// marker it hunts. The whole 8-byte header is used, not the magic alone, so prose naming the
// 6-character magic prefix does not trip the check.
const WASM_HEADER_BASE64 = Buffer.from([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00])
  .toString("base64")
  .slice(0, 10);

const EMBEDDED_DATA_URL =
  /data:(?:text|application)\/(?:x-)?(?:javascript|ecmascript|wasm|octet-stream)[^,\s]*;base64,/i;

export function findBlobs(files) {
  const offenders = [];
  for (const { path, content } of files) {
    const reasons = [];
    if (path.toLowerCase().endsWith(".wasm")) reasons.push("WASM file");
    if (content.includes(WASM_HEADER_BASE64)) reasons.push("base64-encoded WASM");
    if (EMBEDDED_DATA_URL.test(content)) reasons.push("base64 data: URL of a script or module");
    if (reasons.length > 0) offenders.push({ path, reasons });
  }
  return offenders;
}

function trackedFiles() {
  return execFileSync("git", ["ls-files", "-z"], { encoding: "utf8" })
    .split("\0")
    .filter((path) => path !== "")
    .map((path) => ({
      path,
      content: existsSync(path) ? readFileSync(path).toString("latin1") : "",
    }));
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const offenders = findBlobs(trackedFiles());
  for (const { path, reasons } of offenders) {
    console.error(`check-no-blobs: ${path}: ${reasons.join(", ")}`);
  }
  if (offenders.length > 0) process.exit(1);
  console.log("check-no-blobs: no WASM file or embedded blob is tracked");
}
