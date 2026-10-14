use lophelia_bench::metrics::silhouette;
use lophelia_bench::naive::naive_layout;

#[test]
fn silhouette_of_two_far_pairs_matches_the_hand_computation() {
    // Pair A at x = 0 and 1, pair B at x = 10 and 11, on one axis.
    let xyz = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 10.0, 0.0, 0.0, 11.0, 0.0, 0.0];
    // a = 1 for everyone; b = 10.5, 9.5, 9.5, 10.5.
    let expected = ((9.5 / 10.5) * 2.0 + (8.5 / 9.5) * 2.0) / 4.0;
    let value = silhouette(&xyz, &[0, 0, 1, 1]).expect("defined");
    assert!((value - expected).abs() < 1e-12, "{value} vs {expected}");
}

#[test]
fn silhouette_is_undefined_for_a_single_community() {
    let xyz = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 2.0, 0.0, 0.0];
    assert_eq!(silhouette(&xyz, &[3, 3, 3]), None);
}

#[test]
fn naive_layout_separates_two_linked_cliques() {
    let mut edges = Vec::new();
    for offset in [0u32, 6] {
        for i in 0..6 {
            for j in i + 1..6 {
                edges.push((offset + i, offset + j));
            }
        }
    }
    edges.push((0, 6));
    let communities: Vec<u32> = (0..12).map(|i| u32::from(i >= 6)).collect();
    let xyz = naive_layout(12, &edges);
    let value = silhouette(&xyz, &communities).expect("defined");
    assert!(value > 0.5, "silhouette {value}");
}
