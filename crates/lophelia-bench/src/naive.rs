//! Naive reference layout: linear springs on links, uniform `1/d²` repulsion
//! between every pair, bounded and linearly cooled steps. Every parameter is
//! fixed here so the reference never moves.

use crate::rng::Rng;

pub const SEED: u64 = 0x6c6f_7068_656c_6961;
pub const ITERATIONS: usize = 300;
/// Side of the starting cube per cube root of the node count.
pub const START_SPACING: f64 = 1.0;
pub const REST_LENGTH: f64 = 1.0;
pub const SPRING: f64 = 1.0;
pub const REPULSION: f64 = 1.0;
/// First step bound, as a fraction of the starting cube side.
pub const START_STEP: f64 = 0.1;
/// Keeps the repulsion finite between nearly coincident nodes.
pub const MIN_DISTANCE: f64 = 1e-3;

pub fn naive_layout(n: usize, edges: &[(u32, u32)]) -> Vec<f64> {
    let side = START_SPACING * (n as f64).cbrt().max(1.0);
    let mut rng = Rng::new(SEED);
    let mut xyz: Vec<f64> = (0..3 * n).map(|_| (rng.next_f64() - 0.5) * side).collect();
    let edges: Vec<(usize, usize)> = edges
        .iter()
        .map(|&(a, b)| (a as usize, b as usize))
        .filter(|&(a, b)| a < n && b < n && a != b)
        .collect();

    let mut force = vec![0.0; 3 * n];
    for step in 0..ITERATIONS {
        force.iter_mut().for_each(|f| *f = 0.0);
        for i in 0..n {
            for j in i + 1..n {
                let (d, unit) = direction(&xyz, i, j);
                let push = REPULSION / (d * d);
                for a in 0..3 {
                    force[3 * i + a] -= push * unit[a];
                    force[3 * j + a] += push * unit[a];
                }
            }
        }
        for &(i, j) in &edges {
            let (d, unit) = direction(&xyz, i, j);
            let pull = SPRING * (d - REST_LENGTH);
            for a in 0..3 {
                force[3 * i + a] += pull * unit[a];
                force[3 * j + a] -= pull * unit[a];
            }
        }
        let bound = START_STEP * side * (1.0 - step as f64 / ITERATIONS as f64);
        for i in 0..n {
            let f = &force[3 * i..3 * i + 3];
            let norm = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt();
            if norm > 0.0 {
                let scale = norm.min(bound) / norm;
                for a in 0..3 {
                    xyz[3 * i + a] += f[a] * scale;
                }
            }
        }
    }
    xyz
}

/// Distance from `i` to `j`, floored at `MIN_DISTANCE`, and the unit vector
/// from `i` towards `j` (zero when they coincide).
fn direction(xyz: &[f64], i: usize, j: usize) -> (f64, [f64; 3]) {
    let delta = [
        xyz[3 * j] - xyz[3 * i],
        xyz[3 * j + 1] - xyz[3 * i + 1],
        xyz[3 * j + 2] - xyz[3 * i + 2],
    ];
    let raw = (delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt();
    if raw == 0.0 {
        return (MIN_DISTANCE, [0.0; 3]);
    }
    (
        raw.max(MIN_DISTANCE),
        [delta[0] / raw, delta[1] / raw, delta[2] / raw],
    )
}
