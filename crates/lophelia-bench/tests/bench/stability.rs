use lophelia_bench::graph::read_graph;
use lophelia_bench::stability::{Step, half_farthest_displacement, read_scenario, run_stability};

use crate::{BASE_SCENARIO, SBM_100, SBM_1000, SBM_2500, sbm_100};

#[test]
fn far_half_displacement_counts_only_the_farthest_half() {
    // Node 0 is the edited one, at the origin; others on the x axis.
    #[rustfmt::skip]
    let before = [
        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        2.0, 0.0, 0.0,
        3.0, 0.0, 0.0,
        4.0, 0.0, 0.0,
    ];
    let mut after = before;
    after[3 + 1] = 0.5; // nearest node (1) moves by 0.5: outside the far half
    after[12 + 1] = 0.2; // farthest node (4) moves by 0.2
    // Barycentre at x = 2, so R = 2; the far half is nodes 4 and 3.
    let value = half_farthest_displacement(&before, &after, 0).expect("radius");
    assert!((value - 0.2 / 2.0 * 100.0).abs() < 1e-12, "{value}");
}

#[test]
fn edits_against_the_graph_are_refused_by_name() {
    let graph = sbm_100();
    let unlink_absent = [Step::Unlink(
        "c00/n00-0000.md".into(),
        "c01/n01-0000.md".into(),
    )];
    let unknown = [Step::Link("c00/n00-0000.md".into(), "nowhere.md".into())];
    let code = |steps: &[Step]| {
        run_stability(&graph, steps)
            .map(|_| ())
            .map_err(|e| e.code())
    };
    assert_eq!(code(&unlink_absent), Err("link_missing"));
    assert_eq!(code(&unknown), Err("unknown_node"));
}

#[test]
fn base_scenario_runs_on_every_graph_and_the_engine_rereads_exactly() {
    let steps = read_scenario(BASE_SCENARIO).expect("scenario");
    for text in [SBM_100, SBM_1000, SBM_2500] {
        let graph = read_graph(text).expect("graph");
        let report = run_stability(&graph, &steps).expect("stability");
        assert_eq!(report.steps.len(), steps.len());
        for step in &report.steps {
            assert_eq!(step.drift, 0.0, "{}", step.label);
        }
    }
}
