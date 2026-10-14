#!/usr/bin/env bash
# Builds the plugin, installs it in a throwaway test vault and starts Obsidian on an isolated
# profile with the Chrome DevTools Protocol open (ADR-0006). ~/.config/obsidian is never touched:
# a second launch on the user's profile would hand over to their running instance.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
base="${XDG_CACHE_HOME:-$HOME/.cache}/lophelia"
vault="$base/vault-test"
profile="$base/obsidian-profile"
port="${LOPHELIA_CDP_PORT:-9222}"

if [ ! -d "$root/plugin/node_modules" ]; then
  npm --prefix "$root/plugin" ci
fi
npm --prefix "$root/plugin" run build

plugin_dir="$vault/.obsidian/plugins/lophelia"
mkdir -p "$plugin_dir" "$profile"
cp "$root/plugin/main.js" "$root/plugin/manifest.json" "$root/plugin/styles.css" "$plugin_dir/"
printf '["lophelia"]\n' > "$vault/.obsidian/community-plugins.json"
if [ ! -f "$vault/Bienvenue.md" ]; then
  printf '# Bienvenue\n\nVault de test de Lophelia.\n' > "$vault/Bienvenue.md"
fi

# The vault is registered in the isolated profile only, marked open so Obsidian starts on it.
printf '{"vaults":{"lopheliatest0001":{"path":"%s","ts":%s,"open":true}}}\n' \
  "$vault" "$(date +%s000)" > "$profile/obsidian.json"

# A fresh profile opens the vault in restricted mode, which keeps community plugins off; the
# switch lives in the profile's local storage, so it is turned off through CDP, in this profile only.
enable_plugin() {
  CDP_PORT="$1" node --input-type=module -e '
    const base = `http://127.0.0.1:${process.env.CDP_PORT}`;
    const expression = `(async () => {
      while (!window.app?.plugins?.manifests?.lophelia) await new Promise((r) => setTimeout(r, 200));
      if (!app.plugins.isEnabled()) await app.plugins.setEnable(true);
      if (!app.plugins.plugins.lophelia) await app.plugins.enablePluginAndSave("lophelia");
      return Boolean(app.plugins.plugins.lophelia);
    })()`;
    let page;
    for (let i = 0; i < 60 && !page; i++) {
      const targets = await (await fetch(`${base}/json/list`)).json();
      page = targets.find((t) => t.type === "page" && t.url.startsWith("app://obsidian.md"));
      if (!page) await new Promise((r) => setTimeout(r, 500));
    }
    if (!page) throw new Error("no Obsidian window on CDP");
    const ws = new WebSocket(page.webSocketDebuggerUrl);
    await new Promise((r) => (ws.onopen = r));
    const loaded = await new Promise((resolve) => {
      ws.onmessage = (m) => resolve(JSON.parse(m.data).result?.result?.value === true);
      ws.send(JSON.stringify({ id: 1, method: "Runtime.evaluate", params: { expression, awaitPromise: true, returnByValue: true } }));
    });
    ws.close();
    if (!loaded) throw new Error("the Lophelia plugin did not load");
  '
}

obsidian --user-data-dir="$profile" --remote-debugging-port="$port" >"$base/obsidian.log" 2>&1 &
pid=$!
trap 'kill "$pid" 2>/dev/null || true' EXIT INT TERM

for _ in $(seq 1 60); do
  if curl -fs "http://127.0.0.1:$port/json/version" >/dev/null 2>&1; then
    enable_plugin "$port"
    echo "Obsidian test vault: $vault"
    echo "CDP: http://127.0.0.1:$port"
    wait "$pid"
    exit 0
  fi
  if ! kill -0 "$pid" 2>/dev/null; then
    echo "dev: Obsidian exited before opening port $port; see $base/obsidian.log" >&2
    exit 1
  fi
  sleep 0.5
done
echo "dev: no CDP endpoint on port $port after 30 s; see $base/obsidian.log" >&2
exit 1
