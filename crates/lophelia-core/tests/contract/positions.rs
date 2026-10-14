use lophelia_core::positions::{DEFAULT_MAX_BYTES, PositionsError, read, write};

use crate::layout::graph;

fn line(path: &str) -> String {
    format!("{{\"path\":\"{path}\",\"x\":1.0,\"y\":2.0,\"z\":3.0}}")
}

fn file_with(paths: &[&str]) -> String {
    let lines: Vec<String> = paths.iter().map(|p| line(p)).collect();
    format!(
        "{{\"version\":1,\"positions\":[\n{}\n]}}\n",
        lines.join(",\n")
    )
}

fn read_err(text: &str) -> PositionsError {
    read(text, &graph(&["a.md"]), DEFAULT_MAX_BYTES).expect_err("file accepted")
}

/// Finite `f64` drawn from every exponent and sign, through their bit patterns.
fn finite_f64s(count: usize) -> Vec<f64> {
    let mut state: u64 = 0x5eed;
    let mut out = Vec::with_capacity(count);
    while out.len() < count {
        state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        let v = f64::from_bits(z ^ (z >> 31));
        if v.is_finite() {
            out.push(v);
        }
    }
    out
}

#[test]
fn write_then_read_gives_back_every_position_bit_for_bit() {
    let ids: Vec<String> = (0..200)
        .map(|i| format!("dossier é/\"note\" {i}\\x.md"))
        .collect();
    let g = lophelia_core::contract::Graph::new(ids).expect("graph");
    let xyz = finite_f64s(600);
    let text = write(&g, &xyz).expect("write");
    let known = read(&text, &g, DEFAULT_MAX_BYTES).expect("read");

    let mut back = vec![0u64; 600];
    for (k, &node) in known.nodes.iter().enumerate() {
        for c in 0..3 {
            back[3 * node as usize + c] = known.xyz[3 * k + c].to_bits();
        }
    }
    assert_eq!(known.nodes.len(), 200);
    assert_eq!(back, xyz.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
}

#[test]
fn written_file_has_version_first_and_one_sorted_line_per_node() {
    let g = graph(&["b.md", "Z.md", "é.md", "a.md"]);
    let text = write(&g, &[0.0; 12]).expect("write");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 6, "{text}");
    assert!(lines[0].contains("\"version\":1"));
    let paths: Vec<&str> = lines[1..5]
        .iter()
        .map(|l| l.split('"').nth(3).expect("path"))
        .collect();
    assert_eq!(paths, ["Z.md", "a.md", "b.md", "é.md"]);
    assert_eq!(lines[5], "]}");

    let two = graph(&["b/c.md", "a.md"]);
    assert_eq!(
        write(&two, &[-5e-324, 2.5, 3.0, 0.1, -0.0, 1e300]).expect("write"),
        "{\"version\":1,\"positions\":[\n\
         {\"path\":\"a.md\",\"x\":0.1,\"y\":-0.0,\"z\":1e+300},\n\
         {\"path\":\"b/c.md\",\"x\":-5e-324,\"y\":2.5,\"z\":3.0}\n\
         ]}\n"
    );
}

#[test]
fn out_of_range_number_is_named_non_finite() {
    let text = "{\"version\":1,\"positions\":[{\"path\":\"a.md\",\"x\":1e999,\"y\":0,\"z\":0}]}";
    assert!(
        matches!(read_err(text), PositionsError::NonFiniteNumber(_)),
        "{:?}",
        read_err(text)
    );
}

#[test]
fn file_above_the_limit_is_refused_before_parsing() {
    let text = file_with(&["a.md"]);
    let err = read(&text, &graph(&["a.md"]), text.len() - 1).expect_err("accepted");
    assert_eq!(
        err,
        PositionsError::TooLarge {
            size: text.len(),
            max: text.len() - 1
        }
    );
}

#[test]
fn truncated_json_is_invalid() {
    let text = file_with(&["a.md"]);
    let cut = &text[..text.len() - 5];
    assert!(matches!(read_err(cut), PositionsError::InvalidJson(_)));
}

#[test]
fn unknown_version_is_refused() {
    let text = file_with(&["a.md"]).replace("\"version\":1", "\"version\":2");
    assert_eq!(read_err(&text), PositionsError::UnsupportedVersion(2));
}

#[test]
fn missing_version_is_refused() {
    let text = file_with(&["a.md"]).replace("\"version\":1,", "");
    assert_eq!(read_err(&text), PositionsError::MissingVersion);
}

#[test]
fn malformed_paths_refuse_the_file() {
    for bad in ["/abs.md", "a/../b.md", ""] {
        let err = read_err(&file_with(&["a.md", bad]));
        assert!(
            matches!(&err, PositionsError::MalformedPath { path, .. } if path == bad),
            "{bad:?}: {err:?}"
        );
    }
}

#[test]
fn dots_inside_a_segment_are_a_legal_path() {
    let g = graph(&["a..b.md", "d/..x/n.md"]);
    let known = read(
        &file_with(&["a..b.md", "d/..x/n.md"]),
        &g,
        DEFAULT_MAX_BYTES,
    )
    .expect("legal paths refused");
    assert_eq!(known.nodes.len(), 2);
}

#[test]
fn path_unknown_to_the_host_is_ignored() {
    let g = graph(&["a.md"]);
    let known = read(&file_with(&["gone.md", "a.md"]), &g, DEFAULT_MAX_BYTES).expect("read");
    assert_eq!(known.nodes, vec![0]);
    assert_eq!(known.xyz, vec![1.0, 2.0, 3.0]);
}

#[test]
fn duplicate_path_is_refused() {
    assert_eq!(
        read_err(&file_with(&["a.md", "b.md", "a.md"])),
        PositionsError::DuplicatePath("a.md".into())
    );
}

#[test]
fn write_refuses_a_malformed_identifier_and_a_short_array() {
    let err = write(&graph(&["/abs.md"]), &[0.0; 3]).expect_err("written");
    assert!(
        matches!(err, PositionsError::MalformedPath { .. }),
        "{err:?}"
    );
    assert_eq!(
        write(&graph(&["a.md", "b.md"]), &[0.0; 5]),
        Err(PositionsError::LengthMismatch {
            expected: 6,
            actual: 5
        })
    );
}
