#!/usr/bin/env node
// Fails when a bundle holds a network API (story s1, criterion 4; ADR-0004).
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const identifier = (name) => new RegExp(`(?<![\\w$])${name}(?![\\w$])`);
const netModule = `["'](?:node:)?(?:http|https|net)["']`;

const RULES = [
  { name: "fetch", pattern: identifier("fetch") },
  { name: "XMLHttpRequest", pattern: identifier("XMLHttpRequest") },
  { name: "WebSocket", pattern: identifier("WebSocket") },
  { name: "requestUrl", pattern: identifier("requestUrl") },
  {
    name: "http/https/net module",
    pattern: new RegExp(
      `(?<![\\w$])require\\s*\\(\\s*${netModule}\\s*\\)` +
        `|(?<![\\w$])import\\s*\\(\\s*${netModule}\\s*\\)` +
        `|(?<![\\w$])from\\s*${netModule}` +
        `|(?<![\\w$])import\\s*${netModule}`,
    ),
  },
];

export function findNetworkApis(source) {
  return RULES.filter((rule) => rule.pattern.test(source)).map((rule) => rule.name);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const file = process.argv[2];
  if (!file) {
    console.error("usage: node scripts/check-network.mjs <bundle.js>");
    process.exit(2);
  }
  const found = findNetworkApis(readFileSync(file, "utf8"));
  if (found.length > 0) {
    console.error(`check-network: ${file} contains network APIs: ${found.join(", ")}`);
    process.exit(1);
  }
  console.log(`check-network: ${file} holds no network API`);
}
