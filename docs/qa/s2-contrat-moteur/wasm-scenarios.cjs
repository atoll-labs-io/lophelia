// QA scenarios for s2-contrat-moteur, criterion 7: the contract of
// docs/contracts/moteur.md called from JavaScript through the WASM module.
// Run with: node wasm-scenarios.cjs <dir holding wasm-bindgen --target nodejs output>
const path = require("node:path");
const m = require(path.join(process.argv[2], "lophelia_wasm.js"));

function attempt(label, fn) {
  try {
    const v = fn();
    console.log(`${label}: Ok ${typeof v === "string" ? JSON.stringify(v) : String(v && v.constructor ? v.constructor.name + " " + Array.from(v).join(",") : v)}`);
    return v;
  } catch (e) {
    console.log(`${label}: threw ${e && e.constructor ? e.constructor.name : typeof e} message=${JSON.stringify(e && e.message)}`);
    return undefined;
  }
}

const U = (a) => new Uint32Array(a);
const F = (a) => new Float64Array(a);

console.log("exports:", Object.keys(m).sort().join(", "));
console.log("engine_version():", m.engine_version());
const forces = attempt("default_forces()", () => m.default_forces());

const e = new m.Engine(["b.md", "a.md", "c/d.md"]);
console.log("new Engine([b.md, a.md, c/d.md]).node_count():", e.node_count());

const out = attempt("layout(edges [0,1], known [1]=(1,2,3), changed [2], seed 7, default forces)", () =>
  e.layout(U([0, 1]), U([1]), F([1, 2, 3]), U([2]), 7, forces));
const out2 = e.layout(U([0, 1]), U([1]), F([1, 2, 3]), U([2]), 7, forces);
console.log("second identical call equal:", out.every((v, i) => Object.is(v, out2[i])));

const all = F([0.1, -0, 1e300, -5e-324, 2.5, 3, 7, 8, 9]);
const same = e.layout(U([]), U([0, 1, 2]), all, U([]), 1, forces);
console.log("changed empty, all known: output equals input (Object.is per value):", all.every((v, i) => Object.is(v, same[i])));

const text = attempt("write_positions(out)", () => e.write_positions(out));
const known = attempt("read_positions(text)", () => e.read_positions(text));
console.log("known.nodes:", Array.from(known.nodes).join(","), " known.xyz:", Array.from(known.xyz).join(","));
const back = e.layout(U([]), known.nodes, known.xyz, U([]), 0, forces);
console.log("round trip through JS equals out:", out.every((v, i) => Object.is(v, back[i])));
known.free();

console.log("--- invalid inputs from JS");
attempt("new Engine([a.md, a.md])", () => new m.Engine(["a.md", "a.md"]));
attempt("layout odd edges [0]", () => e.layout(U([0]), U([]), F([]), U([]), 0, forces));
attempt("layout edge index 9", () => e.layout(U([0, 9]), U([]), F([]), U([]), 0, forces));
attempt("layout forces of length 3", () => e.layout(U([]), U([]), F([]), U([]), 0, F([1, 2, 3])));
attempt("layout forces with NaN", () => e.layout(U([]), U([]), F([]), U([]), 0, F([NaN, 1, 1, 1])));
attempt("layout seed -1", () => e.layout(U([]), U([]), F([]), U([]), -1, forces));
attempt("layout seed 2**32", () => e.layout(U([]), U([]), F([]), U([]), 2 ** 32, forces));
attempt("layout seed 1.5", () => e.layout(U([]), U([]), F([]), U([]), 1.5, forces));
attempt("layout edges as a plain array [0,1]", () => e.layout([0, 1], U([]), F([]), U([]), 0, forces));
attempt("read_positions version 2", () => e.read_positions('{"version":2,"positions":[]}'));
attempt("read_positions 1e999", () => e.read_positions('{"version":1,"positions":[{"path":"a.md","x":1e999,"y":0,"z":0}]}'));
attempt("read_positions broken JSON", () => e.read_positions('{"version":1,'));
attempt("read_positions ../a.md", () => e.read_positions('{"version":1,"positions":[{"path":"../a.md","x":1,"y":0,"z":0}]}'));
attempt("read_positions 10 MiB + 1", () => e.read_positions('{"version":1,"positions":[]}' + " ".repeat(10 * 1024 * 1024 + 1 - 28)));
attempt("write_positions with 2 values", () => e.write_positions(F([1, 2])));
attempt("write_positions with NaN", () => e.write_positions(F([NaN, 0, 0, 0, 0, 0, 0, 0, 0])));
console.log("engine still usable after errors, node_count:", e.node_count());
e.free();
