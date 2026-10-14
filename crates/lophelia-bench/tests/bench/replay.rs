use lophelia_bench::evaluate::evaluate;
use lophelia_bench::graph::write_graph;
use lophelia_bench::repo_root;
use lophelia_bench::snapshot::snapshot;
use lophelia_bench::stability::{read_scenario, run_stability};
use lophelia_bench::vault::write_vault;

use crate::{BASE_SCENARIO, TempDir, sbm_100};

#[test]
fn running_twice_gives_the_same_scores_and_files() {
    let graph = sbm_100();
    let dir = TempDir::new("replay");

    let references = dir.path().join("references");
    let scores = || {
        let e = evaluate(&graph, &references, "sbm-100").expect("evaluate");
        (
            e.nodes,
            e.edges,
            e.communities,
            e.engine_silhouette.to_bits(),
            e.naive_silhouette.to_bits(),
            e.reference_gap.map(f64::to_bits),
        )
    };
    assert_eq!(scores(), scores());

    let steps = read_scenario(BASE_SCENARIO).expect("scenario");
    assert_eq!(
        run_stability(&graph, &steps).expect("stability"),
        run_stability(&graph, &steps).expect("stability")
    );

    let vault = dir.path().join("vault");
    write_vault(&graph, &vault, &repo_root().expect("repo root")).expect("write vault");
    let once = write_graph(&snapshot(&vault).expect("snapshot").0);
    let twice = write_graph(&snapshot(&vault).expect("snapshot").0);
    assert!(once == twice, "snapshot output differs between runs");
}
