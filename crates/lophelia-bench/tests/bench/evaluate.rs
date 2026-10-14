use lophelia_bench::evaluate::{evaluate, record_reference, reference_path};

use crate::{TempDir, sbm_100};

#[test]
fn reference_gap_follows_the_recorded_reference() {
    let graph = sbm_100();
    let dir = TempDir::new("references");
    let references = dir.path().join("references");

    let first = evaluate(&graph, &references, "sbm-100").expect("evaluate");
    assert_eq!(first.reference_gap, None);

    record_reference(&references, "sbm-100", first.engine_silhouette).expect("record");
    let recorded = evaluate(&graph, &references, "sbm-100").expect("evaluate");
    assert_eq!(recorded.reference_gap, Some(0.0));

    std::fs::write(
        reference_path(&references, "sbm-100"),
        "{\"silhouette\":0.25}\n",
    )
    .expect("rewrite reference");
    let rewritten = evaluate(&graph, &references, "sbm-100").expect("evaluate");
    assert_eq!(
        rewritten.reference_gap,
        Some(rewritten.engine_silhouette - 0.25)
    );
}
