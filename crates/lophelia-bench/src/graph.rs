//! Bench graph file, format version 1: one line per node, then one line per
//! undirected edge `[a,b]` with `a < b`.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::BenchError;

pub const FORMAT_VERSION: u64 = 1;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BenchGraph {
    pub ids: Vec<String>,
    pub communities: Vec<u32>,
    pub edges: Vec<(u32, u32)>,
}

impl BenchGraph {
    /// Edges flattened as `LayoutInput` takes them.
    pub fn flat_edges(&self) -> Vec<u32> {
        self.edges.iter().flat_map(|&(a, b)| [a, b]).collect()
    }

    pub fn community_count(&self) -> usize {
        self.communities.iter().collect::<BTreeSet<_>>().len()
    }
}

#[derive(Deserialize)]
struct Header {
    #[serde(default)]
    version: Option<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileV1 {
    #[allow(dead_code)]
    version: u64,
    nodes: Vec<NodeV1>,
    edges: Vec<[u32; 2]>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NodeV1 {
    id: String,
    community: u32,
}

#[derive(Serialize)]
struct NodeOut<'a> {
    id: &'a str,
    community: u32,
}

pub fn read_graph(text: &str) -> Result<BenchGraph, BenchError> {
    let header: Header = serde_json::from_str(text).map_err(json_error)?;
    match header.version.as_ref().and_then(|v| v.as_u64()) {
        Some(FORMAT_VERSION) => {}
        Some(other) => return Err(BenchError::UnsupportedVersion(other)),
        None => {
            return Err(BenchError::InvalidJson(
                "no integer \"version\" field".into(),
            ));
        }
    }
    let file: FileV1 = serde_json::from_str(text).map_err(json_error)?;

    let mut seen = BTreeSet::new();
    for node in &file.nodes {
        if !seen.insert(node.id.as_str()) {
            return Err(BenchError::DuplicateNode(node.id.clone()));
        }
    }
    let n = file.nodes.len();
    let mut edges = BTreeSet::new();
    for [a, b] in file.edges {
        if a as usize >= n || b as usize >= n {
            return Err(BenchError::EdgeOutOfRange {
                edge: [a, b],
                nodes: n,
            });
        }
        if a == b {
            return Err(BenchError::SelfLoop(a));
        }
        if !edges.insert((a.min(b), a.max(b))) {
            return Err(BenchError::DuplicateEdge([a, b]));
        }
    }
    let (ids, communities) = file.nodes.into_iter().map(|n| (n.id, n.community)).unzip();
    Ok(BenchGraph {
        ids,
        communities,
        edges: edges.into_iter().collect(),
    })
}

/// Canonical text: nodes sorted by the UTF-8 bytes of their id, edges
/// renumbered accordingly, normalized, sorted and deduplicated.
pub fn write_graph(graph: &BenchGraph) -> String {
    let mut order: Vec<usize> = (0..graph.ids.len()).collect();
    order.sort_by(|&a, &b| graph.ids[a].cmp(&graph.ids[b]));
    let mut rank = vec![0u32; order.len()];
    for (new, &old) in (0u32..).zip(&order) {
        rank[old] = new;
    }
    let edges: BTreeSet<(u32, u32)> = graph
        .edges
        .iter()
        .filter_map(|&(a, b)| {
            let (a, b) = (*rank.get(a as usize)?, *rank.get(b as usize)?);
            (a != b).then_some((a.min(b), a.max(b)))
        })
        .collect();

    let mut out = format!("{{\"version\":{FORMAT_VERSION},\"nodes\":[\n");
    let nodes: Vec<String> = order
        .iter()
        .map(|&i| {
            let node = NodeOut {
                id: &graph.ids[i],
                community: graph.communities.get(i).copied().unwrap_or(0),
            };
            serde_json::to_string(&node).unwrap_or_default()
        })
        .collect();
    push_lines(&mut out, &nodes);
    out.push_str("],\"edges\":[\n");
    let edges: Vec<String> = edges.iter().map(|(a, b)| format!("[{a},{b}]")).collect();
    push_lines(&mut out, &edges);
    out.push_str("]}\n");
    out
}

fn push_lines(out: &mut String, lines: &[String]) {
    out.push_str(&lines.join(",\n"));
    if !lines.is_empty() {
        out.push('\n');
    }
}

pub(crate) fn json_error(e: serde_json::Error) -> BenchError {
    BenchError::InvalidJson(e.to_string())
}
