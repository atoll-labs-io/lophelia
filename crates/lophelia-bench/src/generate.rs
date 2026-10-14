//! Synthetic graphs from a stochastic block model with known communities.

use crate::graph::BenchGraph;
use crate::rng::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Spec {
    pub name: &'static str,
    pub nodes: usize,
    pub communities: usize,
    pub seed: u64,
}

pub const SPECS: [Spec; 3] = [
    Spec {
        name: "sbm-100",
        nodes: 100,
        communities: 4,
        seed: 100,
    },
    Spec {
        name: "sbm-1000",
        nodes: 1000,
        communities: 10,
        seed: 1000,
    },
    Spec {
        name: "sbm-2500",
        nodes: 2500,
        communities: 20,
        seed: 2500,
    },
];

/// Expected number of neighbours of a node inside its own community.
const DEGREE_IN: f64 = 6.0;
/// Expected number of neighbours of a node outside its own community.
const DEGREE_OUT: f64 = 1.0;

/// Unequal sizes: community `c` weighs `1 + c / (k − 1)`, the rounding
/// remainder goes to the last one.
pub fn community_sizes(nodes: usize, communities: usize) -> Vec<usize> {
    if communities == 0 {
        return Vec::new();
    }
    let weight = |c: usize| {
        if communities == 1 {
            1.0
        } else {
            1.0 + c as f64 / (communities - 1) as f64
        }
    };
    let total: f64 = (0..communities).map(weight).sum();
    let mut sizes: Vec<usize> = (0..communities)
        .map(|c| (nodes as f64 * weight(c) / total).floor() as usize)
        .collect();
    let used: usize = sizes.iter().sum();
    if let Some(last) = sizes.last_mut() {
        *last += nodes.saturating_sub(used);
    }
    sizes
}

pub fn generate(spec: &Spec) -> BenchGraph {
    let sizes = community_sizes(spec.nodes, spec.communities);
    let mut ids = Vec::with_capacity(spec.nodes);
    let mut communities = Vec::with_capacity(spec.nodes);
    for (c, &size) in sizes.iter().enumerate() {
        for j in 0..size {
            ids.push(format!("c{c:02}/n{c:02}-{j:04}.md"));
            communities.push(c as u32);
        }
    }

    let n = ids.len();
    let p_in: Vec<f64> = sizes
        .iter()
        .map(|&s| {
            if s > 1 {
                DEGREE_IN / (s - 1) as f64
            } else {
                0.0
            }
        })
        .collect();
    let p_out: Vec<f64> = sizes
        .iter()
        .map(|&s| {
            if n > s {
                DEGREE_OUT / (n - s) as f64
            } else {
                0.0
            }
        })
        .collect();

    let mut rng = Rng::new(spec.seed);
    let mut edges = Vec::new();
    for (i, &ci) in communities.iter().enumerate() {
        let ci = ci as usize;
        for (j, &cj) in communities.iter().enumerate().skip(i + 1) {
            let cj = cj as usize;
            let p = if ci == cj {
                p_in[ci]
            } else {
                (p_out[ci] + p_out[cj]) / 2.0
            };
            if rng.next_f64() < p {
                edges.push((i as u32, j as u32));
            }
        }
    }
    BenchGraph {
        ids,
        communities,
        edges,
    }
}
