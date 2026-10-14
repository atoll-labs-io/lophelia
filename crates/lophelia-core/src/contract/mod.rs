//! The layout contract: nodes are addressed by index once their identifiers
//! have been registered in a [`Graph`]; every array crossing the boundary is flat.

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ContractError {
    DuplicateNode(String),
    IndexOutOfRange(u32),
    MalformedInput(&'static str),
    NonFinite(&'static str),
}

impl ContractError {
    pub fn code(&self) -> &'static str {
        match self {
            ContractError::DuplicateNode(_) => "duplicate_node",
            ContractError::IndexOutOfRange(_) => "index_out_of_range",
            ContractError::MalformedInput(_) => "malformed_input",
            ContractError::NonFinite(_) => "non_finite",
        }
    }
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractError::DuplicateNode(id) => write!(f, "node {id:?} is given twice"),
            ContractError::IndexOutOfRange(index) => {
                write!(f, "node index {index} is out of range")
            }
            ContractError::MalformedInput(what) => write!(f, "malformed input: {what}"),
            ContractError::NonFinite(what) => write!(f, "non-finite number in {what}"),
        }
    }
}

impl std::error::Error for ContractError {}

/// Layout settings. The four forces are provisional until s4 tunes them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    pub seed: u32,
    pub center: f64,
    pub repulsion: f64,
    pub link_strength: f64,
    pub link_distance: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            seed: 0,
            center: 0.05,
            repulsion: 30.0,
            link_strength: 1.0,
            link_distance: 30.0,
        }
    }
}

impl Settings {
    /// `forces` holds, in this order: center, repulsion, link strength, link distance.
    pub fn from_parts(seed: u32, forces: &[f64]) -> Result<Settings, ContractError> {
        match *forces {
            [center, repulsion, link_strength, link_distance] => Ok(Settings {
                seed,
                center,
                repulsion,
                link_strength,
                link_distance,
            }),
            _ => Err(ContractError::MalformedInput(
                "forces must hold exactly 4 values",
            )),
        }
    }

    pub fn forces(&self) -> [f64; 4] {
        [
            self.center,
            self.repulsion,
            self.link_strength,
            self.link_distance,
        ]
    }
}

/// `edges` holds (source, target) pairs flattened; `known_xyz` holds three
/// values per entry of `known_nodes`.
#[derive(Debug, Clone, Copy)]
pub struct LayoutInput<'a> {
    pub edges: &'a [u32],
    pub known_nodes: &'a [u32],
    pub known_xyz: &'a [f64],
    pub changed: &'a [u32],
    pub settings: Settings,
}

#[derive(Debug, Clone)]
pub struct Graph {
    ids: Vec<String>,
    index: HashMap<String, u32>,
}

impl Graph {
    pub fn new(ids: Vec<String>) -> Result<Graph, ContractError> {
        if u32::try_from(ids.len()).is_err() {
            return Err(ContractError::MalformedInput("too many nodes"));
        }
        let mut index = HashMap::with_capacity(ids.len());
        for (i, id) in (0u32..).zip(ids.iter()) {
            if index.insert(id.clone(), i).is_some() {
                return Err(ContractError::DuplicateNode(id.clone()));
            }
        }
        Ok(Graph { ids, index })
    }

    pub fn node_count(&self) -> usize {
        self.ids.len()
    }

    pub fn ids(&self) -> &[String] {
        &self.ids
    }

    pub fn index_of(&self, id: &str) -> Option<u32> {
        self.index.get(id).copied()
    }

    fn check_index(&self, index: u32) -> Result<usize, ContractError> {
        let i = index as usize;
        if i < self.ids.len() {
            Ok(i)
        } else {
            Err(ContractError::IndexOutOfRange(index))
        }
    }

    /// Returns `3 × node_count` values: x, y, z of node `i` at `3i..3i+3`.
    pub fn layout(&self, input: &LayoutInput) -> Result<Vec<f64>, ContractError> {
        if !input.edges.len().is_multiple_of(2) {
            return Err(ContractError::MalformedInput(
                "edges must hold (source, target) pairs",
            ));
        }
        if Some(input.known_xyz.len()) != input.known_nodes.len().checked_mul(3) {
            return Err(ContractError::MalformedInput(
                "known_xyz must hold 3 values per known node",
            ));
        }
        if !input.settings.forces().iter().all(|v| v.is_finite()) {
            return Err(ContractError::NonFinite("settings"));
        }
        for &index in input.edges.iter().chain(input.changed) {
            self.check_index(index)?;
        }

        let n = self.ids.len();
        let mut known: Vec<Option<[f64; 3]>> = vec![None; n];
        let (triples, _) = input.known_xyz.as_chunks::<3>();
        for (&index, &[x, y, z]) in input.known_nodes.iter().zip(triples) {
            let i = self.check_index(index)?;
            if !(x.is_finite() && y.is_finite() && z.is_finite()) {
                return Err(ContractError::NonFinite("known_xyz"));
            }
            match known.get_mut(i) {
                Some(slot @ None) => *slot = Some([x, y, z]),
                Some(Some(_)) => {
                    return Err(match self.ids.get(i) {
                        Some(id) => ContractError::DuplicateNode(id.clone()),
                        None => ContractError::IndexOutOfRange(index),
                    });
                }
                None => return Err(ContractError::IndexOutOfRange(index)),
            }
        }

        // Provisional (s2): a known node keeps its position even when it is in
        // `changed`; s8 adjusts around changed nodes without changing this API.
        let half = input.settings.link_distance * (n as f64).cbrt().max(1.0);
        let mut out = Vec::with_capacity(n.saturating_mul(3));
        for (id, slot) in self.ids.iter().zip(known) {
            let xyz = match slot {
                Some(xyz) => xyz,
                None => created_position(id, input.settings.seed, half),
            };
            out.extend_from_slice(&xyz);
        }
        Ok(out)
    }
}

/// Depends only on the seed, the identifier and the half extent, never on
/// index order, so a host that reorders its nodes gets the same layout.
fn created_position(id: &str, seed: u32, half: f64) -> [f64; 3] {
    let mut state = u64::from(seed) ^ fnv1a64(id.as_bytes());
    let mut draw = || {
        let unit = (splitmix64(&mut state) >> 11) as f64 / (1u64 << 53) as f64;
        (2.0 * unit - 1.0) * half
    };
    [draw(), draw(), draw()]
}

// Written out because the standard library hasher is not stable across Rust releases.
fn fnv1a64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &b| {
        (hash ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
