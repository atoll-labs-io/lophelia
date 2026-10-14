use lophelia_bench::generate::{SPECS, generate};
use lophelia_bench::graph::write_graph;

use crate::{SBM_100, SBM_1000, SBM_2500};

#[test]
fn committed_graphs_are_exactly_what_the_fixed_seeds_generate() {
    let committed = [SBM_100, SBM_1000, SBM_2500];
    for (spec, text) in SPECS.iter().zip(committed) {
        let generated = write_graph(&generate(spec));
        assert!(
            generated == text,
            "{} differs from its generation",
            spec.name
        );
    }
}
