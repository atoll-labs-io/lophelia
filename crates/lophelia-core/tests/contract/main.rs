mod layout;
mod positions;

use lophelia_core::positions::{self as file, DEFAULT_MAX_BYTES};

#[test]
fn stale_path_is_dropped_and_other_positions_survive_read_layout_write() {
    let g = layout::graph(&["b.md", "a.md", "new.md"]);
    let text = "{\"version\":1,\"positions\":[\n\
        {\"path\":\"a.md\",\"x\":0.1,\"y\":-0.0,\"z\":1e300},\n\
        {\"path\":\"b.md\",\"x\":-5e-324,\"y\":2.5,\"z\":3.0},\n\
        {\"path\":\"gone.md\",\"x\":1.0,\"y\":1.0,\"z\":1.0}\n\
        ]}\n";
    let known = file::read(text, &g, DEFAULT_MAX_BYTES).expect("read");
    let xyz = g
        .layout(&layout::input(&[0, 2], &known.nodes, &known.xyz, &[2]))
        .expect("layout");
    let written = file::write(&g, &xyz).expect("write");

    assert!(!written.contains("gone.md"), "{written}");
    let again = file::read(&written, &g, DEFAULT_MAX_BYTES).expect("reread");
    let at = |k: &file::Known, id: &str| {
        let index = g.index_of(id).expect("id");
        let slot = k.nodes.iter().position(|&n| n == index).expect("present");
        k.xyz[3 * slot..3 * slot + 3]
            .iter()
            .map(|v| v.to_bits())
            .collect::<Vec<_>>()
    };
    for id in ["a.md", "b.md"] {
        assert_eq!(at(&again, id), at(&known, id), "{id}");
    }
}
