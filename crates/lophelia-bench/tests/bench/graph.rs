use lophelia_bench::graph::{BenchGraph, read_graph, write_graph};

#[test]
fn a_written_graph_reads_back_unchanged() {
    let graph = BenchGraph {
        ids: vec!["a.md".into(), "b/c.md".into(), "é.md".into()],
        communities: vec![0, 1, 1],
        edges: vec![(0, 1), (0, 2), (1, 2)],
    };
    let text = write_graph(&graph);
    assert_eq!(read_graph(&text), Ok(graph), "{text}");
}

#[test]
fn edges_outside_the_nodes_or_looping_are_refused() {
    let with_edge = |edge: &str| {
        format!(
            "{{\"version\":1,\"nodes\":[{{\"id\":\"a.md\",\"community\":0}},\
             {{\"id\":\"b.md\",\"community\":0}},{{\"id\":\"c.md\",\"community\":1}}],\
             \"edges\":[{edge}]}}"
        )
    };
    let code = |text: String| read_graph(&text).map(|_| ()).map_err(|e| e.code());
    assert_eq!(code(with_edge("[0,9]")), Err("edge_out_of_range"));
    assert_eq!(code(with_edge("[1,1]")), Err("self_loop"));
    assert_eq!(code(with_edge("[0,1]")), Ok(()));
}
