//! Deterministic Louvain community detection on an unweighted undirected graph.

use std::collections::BTreeMap;

/// Below this, a modularity gain is treated as rounding noise.
const EPSILON: f64 = 1e-12;

/// A level of the aggregation: `edges` are `(u, v, w)` with `u <= v`; a
/// self-loop `(u, u, w)` holds the weight inside `u`.
struct Level {
    n: usize,
    edges: Vec<(usize, usize, f64)>,
}

impl Level {
    fn degrees(&self) -> Vec<f64> {
        let mut k = vec![0.0; self.n];
        for &(u, v, w) in &self.edges {
            k[u] += w;
            k[v] += w;
        }
        k
    }

    fn neighbours(&self) -> Vec<Vec<(usize, f64)>> {
        let mut adj = vec![Vec::new(); self.n];
        for &(u, v, w) in &self.edges {
            if u != v {
                adj[u].push((v, w));
                adj[v].push((u, w));
            }
        }
        adj
    }

    fn modularity(&self, labels: &[usize]) -> f64 {
        let k = self.degrees();
        let m2: f64 = k.iter().sum();
        if m2 == 0.0 {
            return 0.0;
        }
        let mut inside: BTreeMap<usize, f64> = BTreeMap::new();
        let mut total: BTreeMap<usize, f64> = BTreeMap::new();
        for &(u, v, w) in &self.edges {
            if labels[u] == labels[v] {
                *inside.entry(labels[u]).or_default() += w;
            }
        }
        for (u, &d) in k.iter().enumerate() {
            *total.entry(labels[u]).or_default() += d;
        }
        total
            .iter()
            .map(|(c, &d)| 2.0 * inside.get(c).copied().unwrap_or(0.0) / m2 - (d / m2) * (d / m2))
            .sum()
    }

    /// Local moves until no node gains by moving; returns dense labels in
    /// order of first appearance, or `None` when no node moved.
    fn local_moves(&self) -> Option<Vec<usize>> {
        let k = self.degrees();
        let m2: f64 = k.iter().sum();
        if m2 == 0.0 {
            return None;
        }
        let adj = self.neighbours();
        let mut community: Vec<usize> = (0..self.n).collect();
        let mut total = k.clone();
        let mut moved_once = false;
        let mut weight_to = vec![0.0; self.n];
        loop {
            let mut moved = false;
            for i in 0..self.n {
                let own = community[i];
                let mut candidates: Vec<usize> = Vec::new();
                for &(j, w) in &adj[i] {
                    let c = community[j];
                    if weight_to[c] == 0.0 {
                        candidates.push(c);
                    }
                    weight_to[c] += w;
                }
                total[own] -= k[i];
                let gain = |c: usize, w: f64| w - total[c] * k[i] / m2;
                let mut best = own;
                let mut best_gain = gain(own, weight_to[own]);
                candidates.sort_unstable();
                for &c in &candidates {
                    let g = gain(c, weight_to[c]);
                    if g > best_gain + EPSILON {
                        best = c;
                        best_gain = g;
                    }
                }
                for &c in &candidates {
                    weight_to[c] = 0.0;
                }
                total[best] += k[i];
                if best != own {
                    community[i] = best;
                    moved = true;
                    moved_once = true;
                }
            }
            if !moved {
                break;
            }
        }
        moved_once.then(|| renumber(&community))
    }

    fn aggregate(&self, labels: &[usize]) -> Level {
        let n = labels.iter().max().map_or(0, |&m| m + 1);
        let mut merged: BTreeMap<(usize, usize), f64> = BTreeMap::new();
        for &(u, v, w) in &self.edges {
            let (a, b) = (labels[u], labels[v]);
            *merged.entry((a.min(b), a.max(b))).or_default() += w;
        }
        Level {
            n,
            edges: merged.into_iter().map(|((a, b), w)| (a, b, w)).collect(),
        }
    }
}

fn renumber(labels: &[usize]) -> Vec<usize> {
    let mut dense: BTreeMap<usize, usize> = BTreeMap::new();
    labels
        .iter()
        .map(|&c| {
            let next = dense.len();
            *dense.entry(c).or_insert(next)
        })
        .collect()
}

fn to_level(n: usize, edges: &[(u32, u32)]) -> Level {
    let mut merged: BTreeMap<(usize, usize), f64> = BTreeMap::new();
    for &(a, b) in edges {
        let (a, b) = (a as usize, b as usize);
        if a != b && a < n && b < n {
            merged.insert((a.min(b), a.max(b)), 1.0);
        }
    }
    Level {
        n,
        edges: merged.into_iter().map(|((a, b), w)| (a, b, w)).collect(),
    }
}

/// Community of each node, numbered by order of first appearance. Nodes are
/// visited by increasing index and gain ties go to the smallest community, so
/// the result never depends on chance.
pub fn louvain(n: usize, edges: &[(u32, u32)]) -> Vec<u32> {
    let mut level = to_level(n, edges);
    let mut assignment: Vec<usize> = (0..n).collect();
    let mut quality = level.modularity(&(0..level.n).collect::<Vec<_>>());
    while let Some(labels) = level.local_moves() {
        let next = level.modularity(&labels);
        if next <= quality + EPSILON {
            break;
        }
        quality = next;
        for a in assignment.iter_mut() {
            *a = labels[*a];
        }
        level = level.aggregate(&labels);
    }
    renumber(&assignment)
        .into_iter()
        .map(|c| c as u32)
        .collect()
}

/// Modularity of `communities` on the unweighted graph, shown as a sanity check.
pub fn modularity(n: usize, edges: &[(u32, u32)], communities: &[u32]) -> f64 {
    let labels: Vec<usize> = communities.iter().map(|&c| c as usize).collect();
    if labels.len() != n {
        return 0.0;
    }
    to_level(n, edges).modularity(&labels)
}
