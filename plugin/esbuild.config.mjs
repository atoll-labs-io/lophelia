import { builtinModules } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import esbuild from "esbuild";

const here = dirname(fileURLToPath(import.meta.url));
const production = process.argv[2] === "production";

await esbuild.build({
  absWorkingDir: here,
  entryPoints: [join(here, "src/main.ts")],
  bundle: true,
  format: "cjs",
  platform: "browser",
  target: "es2022",
  external: [
    "obsidian",
    "electron",
    "@codemirror/*",
    "@lezer/*",
    ...builtinModules,
    ...builtinModules.map((name) => `node:${name}`),
  ],
  loader: { ".wasm": "binary" },
  minify: production,
  sourcemap: production ? false : "inline",
  treeShaking: true,
  logLevel: "warning",
  outfile: join(here, "main.js"),
});
