// QA scenario for s2-contrat-moteur: finite settings and finite inputs, can
// the output of layout hold a non-finite coordinate? Built as a second binary
// of the same scratch crate.
use lophelia_core::contract::{Graph, LayoutInput, Settings};
use lophelia_core::positions;

fn main() {
    for (n, link_distance) in [(2usize, 1e308), (8, 1e308), (8, 1e307), (1000, 1e306)] {
        let ids: Vec<String> = (0..n).map(|i| format!("n{i}.md")).collect();
        let g = Graph::new(ids).unwrap();
        let settings = Settings::from_parts(0, &[0.05, 30.0, 1.0, link_distance]).unwrap();
        let out = g
            .layout(&LayoutInput { edges: &[], known_nodes: &[], known_xyz: &[], changed: &[], settings })
            .unwrap();
        let non_finite = out.iter().filter(|v| !v.is_finite()).count();
        println!("n={n} link_distance={link_distance:e}: layout Ok, {non_finite} non-finite of {} values, first: {:?}", out.len(), &out[..3]);
        match positions::write(&g, &out) {
            Ok(_) => println!("  write_positions of that output: Ok"),
            Err(e) => println!("  write_positions of that output: Err code={} msg={}", e.code(), e),
        }
    }
}
