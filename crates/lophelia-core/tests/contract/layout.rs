use lophelia_core::contract::{ContractError, Graph, LayoutInput, Settings};

pub fn graph(ids: &[&str]) -> Graph {
    match Graph::new(ids.iter().map(|s| s.to_string()).collect()) {
        Ok(graph) => graph,
        Err(e) => panic!("graph refused: {e}"),
    }
}

pub fn input<'a>(
    edges: &'a [u32],
    known_nodes: &'a [u32],
    known_xyz: &'a [f64],
    changed: &'a [u32],
) -> LayoutInput<'a> {
    LayoutInput {
        edges,
        known_nodes,
        known_xyz,
        changed,
        settings: Settings::default(),
    }
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|v| v.to_bits()).collect()
}

#[test]
fn created_and_known_nodes_get_one_position_each() {
    let g = graph(&["a.md", "b.md", "c.md"]);
    let out = g
        .layout(&input(
            &[0, 1, 1, 2],
            &[0, 2],
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            &[],
        ))
        .expect("layout");
    assert_eq!(out.len(), 9);
    assert_eq!(&out[0..3], &[1.0, 2.0, 3.0]);
    assert_eq!(&out[6..9], &[4.0, 5.0, 6.0]);
    assert!(out[3..6].iter().all(|v| v.is_finite()));
}

#[test]
fn nothing_changed_and_all_known_returns_input_exactly() {
    let g = graph(&["a.md", "b.md", "c.md"]);
    // Known nodes given out of index order: 2, 0, 1.
    let known_xyz = [
        0.1, -0.0, 1e300, //
        -5e-324, 0.1, -0.0, //
        1e300, -5e-324, 0.1,
    ];
    let out = g
        .layout(&input(&[0, 1], &[2, 0, 1], &known_xyz, &[]))
        .expect("layout");
    let mut expected = Vec::new();
    expected.extend_from_slice(&known_xyz[3..6]);
    expected.extend_from_slice(&known_xyz[6..9]);
    expected.extend_from_slice(&known_xyz[0..3]);
    assert_eq!(bits(&out), bits(&expected));
}

#[test]
fn same_seed_same_input_same_positions() {
    let g = graph(&["a.md", "b.md", "c.md", "d.md"]);
    let i = input(&[0, 1, 2, 3], &[1], &[7.0, 8.0, 9.0], &[0]);
    let first = g.layout(&i).expect("layout");
    let second = g.layout(&i).expect("layout");
    assert_eq!(bits(&first), bits(&second));
}

#[test]
fn another_seed_moves_created_nodes() {
    let g = graph(&["a.md", "b.md", "c.md"]);
    let mut i = input(&[], &[], &[], &[]);
    i.settings.seed = 1;
    let one = g.layout(&i).expect("layout");
    i.settings.seed = 2;
    let two = g.layout(&i).expect("layout");
    assert_ne!(bits(&one), bits(&two));
}

#[test]
fn duplicate_identifier_is_refused() {
    let err = Graph::new(vec!["a.md".into(), "b.md".into(), "a.md".into()])
        .expect_err("duplicate accepted");
    assert_eq!(err, ContractError::DuplicateNode("a.md".into()));
    assert_eq!(err.code(), "duplicate_node");
}

#[test]
fn out_of_range_index_is_refused_in_edges_and_changed() {
    let g = graph(&["a.md", "b.md"]);
    assert_eq!(
        g.layout(&input(&[0, 2], &[], &[], &[])),
        Err(ContractError::IndexOutOfRange(2))
    );
    assert_eq!(
        g.layout(&input(&[0, 1], &[], &[], &[5])),
        Err(ContractError::IndexOutOfRange(5))
    );
}

#[test]
fn non_finite_known_coordinate_is_refused() {
    let g = graph(&["a.md"]);
    let err = g
        .layout(&input(&[], &[0], &[0.0, f64::NAN, 0.0], &[]))
        .expect_err("NaN accepted");
    assert!(matches!(err, ContractError::NonFinite(_)), "{err:?}");
}

#[test]
fn malformed_flat_arrays_are_refused() {
    let g = graph(&["a.md", "b.md"]);
    assert!(matches!(
        g.layout(&input(&[0, 1, 0], &[], &[], &[])),
        Err(ContractError::MalformedInput(_))
    ));
    assert!(matches!(
        g.layout(&input(&[], &[0], &[1.0, 2.0], &[])),
        Err(ContractError::MalformedInput(_))
    ));
}
