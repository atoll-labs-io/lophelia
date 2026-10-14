#!/usr/bin/env bash
# Builds the engine to WASM and writes the wasm-bindgen output to plugin/wasm-pkg/.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

locked="$(awk '/^name = "wasm-bindgen"$/ { getline; gsub(/version = |"/, ""); print; exit }' Cargo.lock)"
if [ -z "$locked" ]; then
  echo "build-wasm: wasm-bindgen not found in Cargo.lock" >&2
  exit 1
fi

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "build-wasm: wasm-bindgen CLI missing; install it with: cargo install --locked wasm-bindgen-cli --version $locked" >&2
  exit 1
fi
installed="$(wasm-bindgen --version | awk '{ print $2 }')"
if [ "$installed" != "$locked" ]; then
  echo "build-wasm: wasm-bindgen CLI $installed ($(command -v wasm-bindgen)) differs from the crate version $locked in Cargo.lock" >&2
  exit 1
fi

cargo build --release --locked --target wasm32-unknown-unknown -p lophelia-wasm

out="plugin/wasm-pkg"
rm -rf "$out"
wasm-bindgen --target web --out-dir "$out" target/wasm32-unknown-unknown/release/lophelia_wasm.wasm

npm --prefix plugin exec --no -- wasm-opt -Oz --enable-bulk-memory --enable-nontrapping-float-to-int \
  "$out/lophelia_wasm_bg.wasm" -o "$out/lophelia_wasm_bg.wasm"
