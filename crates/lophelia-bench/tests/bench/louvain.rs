use lophelia_bench::louvain::louvain;

#[test]
fn two_bridged_cliques_and_a_lone_node_make_three_communities() {
    let mut edges = Vec::new();
    for offset in [0u32, 5] {
        for i in 0..5 {
            for j in i + 1..5 {
                edges.push((offset + i, offset + j));
            }
        }
    }
    edges.push((4, 5));
    let communities = louvain(11, &edges);
    assert_eq!(communities, vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2]);
}
