// QA scenarios for s2-contrat-moteur: a scratch binary calling the public
// interface of lophelia-core, built outside the worktree. Each criterion is a
// scenario name given as the first argument.
use lophelia_core::contract::{Graph, LayoutInput, Settings};
use lophelia_core::positions::{self, DEFAULT_MAX_BYTES};

fn ids(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}
fn bits(v: &[f64]) -> Vec<u64> {
    v.iter().map(|x| x.to_bits()).collect()
}
fn forces() -> [f64; 4] {
    Settings::default().forces()
}
fn lay(g: &Graph, edges: &[u32], kn: &[u32], kx: &[f64], ch: &[u32], seed: u32, f: &[f64]) {
    let r = Settings::from_parts(seed, f).and_then(|settings| {
        g.layout(&LayoutInput { edges, known_nodes: kn, known_xyz: kx, changed: ch, settings })
    });
    match r {
        Ok(v) => println!("  Ok({} values) {:?}", v.len(), v),
        Err(e) => println!("  Err code={} msg={}", e.code(), e),
    }
}
fn rd(text: &str, g: &Graph) {
    match positions::read(text, g, DEFAULT_MAX_BYTES) {
        Ok(k) => println!("  Ok nodes={:?} xyz={:?}", k.nodes, k.xyz),
        Err(e) => println!("  Err code={} msg={}", e.code(), e),
    }
}
fn wr(g: &Graph, xyz: &[f64]) {
    match positions::write(g, xyz) {
        Ok(t) => println!("  Ok {t:?}"),
        Err(e) => println!("  Err code={} msg={}", e.code(), e),
    }
}
fn entry(path_literal: &str) -> String {
    format!("{{\"version\":1,\"positions\":[{{\"path\":{path_literal},\"x\":1,\"y\":1,\"z\":1}}]}}")
}

fn main() {
    let s = std::env::args().nth(1).unwrap_or_default();
    match s.as_str() {
        "c1" => {
            let g = Graph::new(ids(&["notes/a.md", "b.md", "c d/é.md"])).unwrap();
            println!("ids: {:?}  node_count = {}", g.ids(), g.node_count());
            println!("layout: edges [0,1, 1,2], known node 1 = (1,2,3), changed [2], seed 7, default forces {:?}", forces());
            lay(&g, &[0, 1, 1, 2], &[1], &[1.0, 2.0, 3.0], &[2], 7, &forces());
            println!("layout: no known positions, changed [] (every node is treated as created)");
            lay(&g, &[], &[], &[], &[], 7, &forces());
            println!("layout: only node 0 known, changed [] (nodes 1 and 2 have no position: created)");
            lay(&g, &[], &[0], &[9.0, 9.0, 9.0], &[], 7, &forces());
            println!("graph with no node:");
            lay(&Graph::new(vec![]).unwrap(), &[], &[], &[], &[], 7, &forces());
        }
        "c1-error" => {
            let g = Graph::new(ids(&["a.md", "b.md"])).unwrap();
            println!("Graph::new with duplicate id a.md:");
            match Graph::new(ids(&["a.md", "a.md"])) {
                Ok(_) => println!("  Ok"),
                Err(e) => println!("  Err code={} msg={}", e.code(), e),
            }
            println!("odd edges [0]:");
            lay(&g, &[0], &[], &[], &[], 0, &forces());
            println!("edge index out of range [0,5]:");
            lay(&g, &[0, 5], &[], &[], &[], 0, &forces());
            println!("changed index out of range [9]:");
            lay(&g, &[], &[], &[], &[9], 0, &forces());
            println!("known index out of range [7]:");
            lay(&g, &[], &[7], &[1.0, 1.0, 1.0], &[], 0, &forces());
            println!("known_xyz with 2 values for 1 known node:");
            lay(&g, &[], &[0], &[1.0, 1.0], &[], 0, &forces());
            println!("forces with 3 values:");
            lay(&g, &[], &[], &[], &[], 0, &[1.0, 2.0, 3.0]);
            println!("forces with 5 values:");
            lay(&g, &[], &[], &[], &[], 0, &[1.0, 2.0, 3.0, 4.0, 5.0]);
            println!("forces with NaN center:");
            lay(&g, &[], &[], &[], &[], 0, &[f64::NAN, 1.0, 1.0, 1.0]);
            println!("forces with +inf link distance:");
            lay(&g, &[], &[], &[], &[], 0, &[0.05, 30.0, 1.0, f64::INFINITY]);
            println!("known_xyz with -inf:");
            lay(&g, &[], &[0], &[1.0, f64::NEG_INFINITY, 1.0], &[], 0, &forces());
            println!("known index given twice:");
            lay(&g, &[], &[0, 0], &[1.0, 1.0, 1.0, 2.0, 2.0, 2.0], &[], 0, &forces());
            println!("u32::MAX index in edges:");
            lay(&g, &[0, u32::MAX], &[], &[], &[], 0, &forces());
            println!("negative link distance [0.05,30,1,-30]:");
            lay(&g, &[], &[], &[], &[], 0, &[0.05, 30.0, 1.0, -30.0]);
            println!("huge link distance 1e308 with 2 nodes:");
            lay(&g, &[], &[], &[], &[], 0, &[0.05, 30.0, 1.0, 1e308]);
        }
        "c2" => {
            let g = Graph::new(ids(&["a.md", "b.md", "c.md"])).unwrap();
            let kx = [0.1, -0.0, 1e300, -5e-324, 2.5, 3.0, f64::MAX, f64::MIN_POSITIVE, -7.25];
            let s = Settings::default();
            let shuffled = [kx[6], kx[7], kx[8], kx[0], kx[1], kx[2], kx[3], kx[4], kx[5]];
            let out = g
                .layout(&LayoutInput { edges: &[0, 1, 1, 2], known_nodes: &[2, 0, 1], known_xyz: &shuffled, changed: &[], settings: s })
                .unwrap();
            println!("known_nodes [2,0,1], changed [], positions by node index: {:?}", kx);
            println!("output: {:?}", out);
            println!("bit-identical to input: {}", bits(&out) == bits(&kx));
            let out2 = g
                .layout(&LayoutInput { edges: &[0, 1, 1, 2], known_nodes: &[0, 1, 2], known_xyz: &kx, changed: &[], settings: Settings { seed: 999, ..s } })
                .unwrap();
            println!("same, other seed 999: bit-identical {}", bits(&out2) == bits(&kx));
        }
        "c2-error" => {
            let g = Graph::new(ids(&["a.md", "b.md"])).unwrap();
            println!("changed [], all known, one coordinate NaN:");
            lay(&g, &[], &[0, 1], &[1.0, 2.0, 3.0, f64::NAN, 0.0, 0.0], &[], 0, &forces());
            println!("changed [], all known, known_xyz short by one:");
            lay(&g, &[], &[0, 1], &[1.0, 2.0, 3.0, 4.0, 5.0], &[], 0, &forces());
        }
        "c3" => {
            let names = ["a.md", "b.md", "c.md", "d/e.md"];
            let g = Graph::new(ids(&names)).unwrap();
            let run = |seed| {
                g.layout(&LayoutInput { edges: &[0, 1, 2, 3], known_nodes: &[0], known_xyz: &[1.0, 1.0, 1.0], changed: &[1], settings: Settings { seed, ..Settings::default() } })
                    .unwrap()
            };
            let a = run(42);
            let b = run(42);
            let c = run(43);
            println!("seed 42 call 1: {:?}", a);
            println!("seed 42 call 2: {:?}", b);
            println!("identical (bits): {}", bits(&a) == bits(&b));
            println!("seed 43:        {:?}", c);
            println!("seed 43 differs from 42 on created nodes: {}", bits(&a) != bits(&c));
            let rev = ["d/e.md", "c.md", "b.md", "a.md"];
            let g2 = Graph::new(ids(&rev)).unwrap();
            let r = g2
                .layout(&LayoutInput { edges: &[], known_nodes: &[3], known_xyz: &[1.0, 1.0, 1.0], changed: &[], settings: Settings { seed: 42, ..Settings::default() } })
                .unwrap();
            let mut same = true;
            for (j, n) in rev.iter().enumerate() {
                let i = names.iter().position(|x| x == n).unwrap();
                if bits(&r[3 * j..3 * j + 3]) != bits(&a[3 * i..3 * i + 3]) {
                    same = false;
                }
            }
            println!("ids in reverse order, edges and changed dropped: same position per id: {}", same);
            println!("seed u32::MAX: {:?}", run(u32::MAX));
        }
        "c4" => {
            let g = Graph::new(ids(&["é.md", "b/c.md", "Z.md", "a.md", "say \"hi\".md", "a..b.md"])).unwrap();
            let xyz = [1.0, 2.0, 3.0, -5e-324, 0.1, -0.0, 1e300, -1e-300, f64::MAX, 0.1 + 0.2, 1.0 / 3.0, -2.5, 7.0, 8.0, 9.0, 0.0, 0.0, 0.0];
            println!("ids: {:?}", g.ids());
            println!("xyz: {:?}", xyz);
            let t = positions::write(&g, &xyz).unwrap();
            print!("written file:\n{}", t);
            let k = positions::read(&t, &g, DEFAULT_MAX_BYTES).unwrap();
            let mut back = vec![0.0f64; xyz.len()];
            for (&i, c) in k.nodes.iter().zip(k.xyz.chunks(3)) {
                let i = i as usize;
                back[3 * i..3 * i + 3].copy_from_slice(c);
            }
            println!("read back, nodes in file order: {:?}", k.nodes);
            println!("round trip bit-identical: {}", bits(&back) == bits(&xyz));
            println!("rewrite byte-identical: {}", positions::write(&g, &back).unwrap() == t);
            let e = Graph::new(vec![]).unwrap();
            println!("graph with no node, write:");
            wr(&e, &[]);
            println!("read back:");
            rd(&positions::write(&e, &[]).unwrap(), &e);
        }
        "c4-error" => {
            let g = Graph::new(ids(&["a.md", "b.md"])).unwrap();
            println!("write with 5 values for 2 nodes:");
            wr(&g, &[1.0; 5]);
            println!("write with NaN:");
            wr(&g, &[1.0, f64::NAN, 1.0, 1.0, 1.0, 1.0]);
            println!("write with +inf:");
            wr(&g, &[1.0, f64::INFINITY, 1.0, 1.0, 1.0, 1.0]);
            println!("write for a node whose id is ../x.md:");
            wr(&Graph::new(ids(&["../x.md"])).unwrap(), &[1.0, 1.0, 1.0]);
        }
        "c5" => {
            let g = Graph::new(ids(&["a.md"])).unwrap();
            println!("valid v1 file:");
            rd("{\"version\":1,\"positions\":[\n{\"path\":\"a.md\",\"x\":1,\"y\":2,\"z\":3}\n]}\n", &g);
            println!("valid v1 file, fields reordered, version last:");
            rd("{\"positions\":[{\"z\":3,\"y\":2,\"x\":1,\"path\":\"a.md\"}],\"version\":1}", &g);
            let base = "{\"version\":1,\"positions\":[]}";
            let mut big = String::from(base);
            big.push_str(&" ".repeat(DEFAULT_MAX_BYTES - base.len()));
            println!("file of exactly DEFAULT_MAX_BYTES = {} bytes (valid JSON padded with spaces):", big.len());
            rd(&big, &g);
        }
        "c5-error" => {
            let g = Graph::new(ids(&["a.md"])).unwrap();
            let cases: Vec<(&str, String)> = vec![
                ("x = 1e999", "{\"version\":1,\"positions\":[{\"path\":\"a.md\",\"x\":1e999,\"y\":0,\"z\":0}]}".into()),
                ("x = -1e400", "{\"version\":1,\"positions\":[{\"path\":\"a.md\",\"x\":-1e400,\"y\":0,\"z\":0}]}".into()),
                ("x = NaN (bare literal)", "{\"version\":1,\"positions\":[{\"path\":\"a.md\",\"x\":NaN,\"y\":0,\"z\":0}]}".into()),
                ("x = \"1\" (string)", "{\"version\":1,\"positions\":[{\"path\":\"a.md\",\"x\":\"1\",\"y\":0,\"z\":0}]}".into()),
                ("broken JSON (truncated)", "{\"version\":1,\"positions\":[{\"path\":\"a.md\",\"x\":1,".into()),
                ("empty string", "".into()),
                ("not an object: []", "[]".into()),
                ("future version 2", "{\"version\":2,\"positions\":[]}".into()),
                ("version 0", "{\"version\":0,\"positions\":[]}".into()),
                ("version 99999999999999999999 (beyond u64)", "{\"version\":99999999999999999999,\"positions\":[]}".into()),
                ("version -1", "{\"version\":-1,\"positions\":[]}".into()),
                ("version 1.0", "{\"version\":1.0,\"positions\":[]}".into()),
                ("version \"1\"", "{\"version\":\"1\",\"positions\":[]}".into()),
                ("no version", "{\"positions\":[]}".into()),
                ("version 1, no positions", "{\"version\":1}".into()),
                ("version 1, unknown entry field w", "{\"version\":1,\"positions\":[{\"path\":\"a.md\",\"x\":1,\"y\":0,\"z\":0,\"w\":0}]}".into()),
                ("version 1, unknown top-level field", "{\"version\":1,\"positions\":[],\"extra\":true}".into()),
                ("version 1, entry missing z", "{\"version\":1,\"positions\":[{\"path\":\"a.md\",\"x\":1,\"y\":0}]}".into()),
                ("trailing garbage", "{\"version\":1,\"positions\":[]} xyz".into()),
                ("array nested 100000 deep", format!("{}{}", "[".repeat(100000), "]".repeat(100000))),
                ("version holding an array nested 100000 deep", format!("{{\"version\":{}{}}}", "[".repeat(100000), "]".repeat(100000))),
                ("duplicate path", "{\"version\":1,\"positions\":[{\"path\":\"a.md\",\"x\":1,\"y\":0,\"z\":0},{\"path\":\"a.md\",\"x\":2,\"y\":0,\"z\":0}]}".into()),
                ("duplicate key version (1 then 2)", "{\"version\":1,\"version\":2,\"positions\":[]}".into()),
            ];
            for (name, text) in cases {
                println!("{name}:");
                rd(&text, &g);
            }
            let base = "{\"version\":1,\"positions\":[]}";
            let mut big = String::from(base);
            big.push_str(&" ".repeat(DEFAULT_MAX_BYTES + 1 - base.len()));
            println!("file of DEFAULT_MAX_BYTES + 1 = {} bytes (valid JSON padded):", big.len());
            rd(&big, &g);
            println!("file above the limit that is also broken JSON (size checked first):");
            rd(&"{".repeat(DEFAULT_MAX_BYTES + 10), &g);
        }
        "c6" => {
            let g = Graph::new(ids(&["a.md", "a..b.md", "d/..x/n.md"])).unwrap();
            let text = "{\"version\":1,\"positions\":[\n{\"path\":\"a..b.md\",\"x\":1,\"y\":1,\"z\":1},\n{\"path\":\"a.md\",\"x\":2,\"y\":2,\"z\":2},\n{\"path\":\"d/..x/n.md\",\"x\":3,\"y\":3,\"z\":3},\n{\"path\":\"gone.md\",\"x\":4,\"y\":4,\"z\":4}\n]}\n";
            println!("ids: {:?}", g.ids());
            print!("input file (gone.md has no node):\n{}", text);
            let k = positions::read(text, &g, DEFAULT_MAX_BYTES).unwrap();
            println!("read: nodes={:?} xyz={:?}", k.nodes, k.xyz);
            let out = g
                .layout(&LayoutInput { edges: &[], known_nodes: &k.nodes, known_xyz: &k.xyz, changed: &[], settings: Settings::default() })
                .unwrap();
            let w = positions::write(&g, &out).unwrap();
            print!("next write:\n{}", w);
            println!("gone.md present in next write: {}", w.contains("gone.md"));
        }
        "c6-error" => {
            let g = Graph::new(ids(&["a.md"])).unwrap();
            let paths = [
                r#""""#,
                r#""/etc/passwd""#,
                r#""\\server\\share.md""#,
                r#""C:/x.md""#,
                r#""c:x.md""#,
                r#""..""#,
                r#""../a.md""#,
                r#""a/../b.md""#,
                r#""a\\..\\b.md""#,
                r#""a/..""#,
                r#""./a.md""#,
                r#""a//b.md""#,
                r#""a/./b.md""#,
                r#""a.md\u0000""#,
                r#""   ""#,
                r#""1:x.md""#,
            ];
            for p in paths {
                println!("path JSON literal {p}:");
                rd(&entry(p), &g);
            }
            println!("malformed path on an entry with no matching node, after a valid one:");
            rd("{\"version\":1,\"positions\":[{\"path\":\"a.md\",\"x\":1,\"y\":1,\"z\":1},{\"path\":\"../evil.md\",\"x\":1,\"y\":1,\"z\":1}]}", &g);
        }
        _ => println!("unknown scenario"),
    }
}
