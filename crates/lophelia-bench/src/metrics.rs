//! Layout scores.

use std::collections::BTreeMap;

/// Euclidean distance between the points `p[0..3]` and `q[0..3]`.
pub fn gap(p: &[f64], q: &[f64]) -> f64 {
    let (dx, dy, dz) = (p[0] - q[0], p[1] - q[1], p[2] - q[2]);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Distance between nodes `i` and `j` of a flat `xyz` array.
pub fn distance(xyz: &[f64], i: usize, j: usize) -> f64 {
    gap(&xyz[3 * i..], &xyz[3 * j..])
}

/// Mean silhouette of the communities, Euclidean distance in 3D; a node alone
/// in its community scores 0. `None` below 2 communities or 2 nodes.
pub fn silhouette(xyz: &[f64], communities: &[u32]) -> Option<f64> {
    let n = communities.len();
    if n < 2 || Some(xyz.len()) != n.checked_mul(3) {
        return None;
    }
    let mut dense: BTreeMap<u32, usize> = BTreeMap::new();
    for &c in communities {
        let next = dense.len();
        dense.entry(c).or_insert(next);
    }
    let k = dense.len();
    if k < 2 {
        return None;
    }
    let label: Vec<usize> = communities.iter().map(|c| dense[c]).collect();
    let mut size = vec![0usize; k];
    for &c in &label {
        size[c] += 1;
    }

    let mut total = 0.0;
    let mut sums = vec![0.0; k];
    for i in 0..n {
        sums.iter_mut().for_each(|s| *s = 0.0);
        for j in 0..n {
            if j != i {
                sums[label[j]] += distance(xyz, i, j);
            }
        }
        let own = label[i];
        if size[own] < 2 {
            continue;
        }
        let a = sums[own] / (size[own] - 1) as f64;
        let b = (0..k)
            .filter(|&c| c != own)
            .map(|c| sums[c] / size[c] as f64)
            .fold(f64::INFINITY, f64::min);
        let spread = a.max(b);
        if spread > 0.0 {
            total += (b - a) / spread;
        }
    }
    Some(total / n as f64)
}
