//! Evaluation bench: scores a layout of frozen graphs so that every engine
//! change is judged on numbers.

pub mod evaluate;
pub mod generate;
pub mod graph;
pub mod louvain;
pub mod metrics;
pub mod naive;
pub mod rng;
pub mod snapshot;
pub mod stability;
pub mod vault;

use std::fmt;
use std::path::{Path, PathBuf};

use lophelia_core::contract::ContractError;
use lophelia_core::positions::PositionsError;

#[derive(Debug, Clone, PartialEq)]
pub enum BenchError {
    Io { path: String, detail: String },
    InvalidJson(String),
    UnsupportedVersion(u64),
    DuplicateNode(String),
    EdgeOutOfRange { edge: [u32; 2], nodes: usize },
    SelfLoop(u32),
    DuplicateEdge([u32; 2]),
    Contract(ContractError),
    Positions(PositionsError),
    NonFiniteLayout,
    UndefinedSilhouette,
    InvalidStep(usize),
    UnknownNode(String),
    LinkExists([String; 2]),
    LinkMissing([String; 2]),
    NodeExists(String),
    ZeroRadius,
    InvalidTarget(String),
    InsideRepository(String),
    TargetNotEmpty(String),
    UnsafeNodeId(String),
}

impl BenchError {
    pub fn code(&self) -> &'static str {
        match self {
            BenchError::Io { .. } => "io",
            BenchError::InvalidJson(_) => "invalid_json",
            BenchError::UnsupportedVersion(_) => "unsupported_version",
            BenchError::DuplicateNode(_) => "duplicate_node",
            BenchError::EdgeOutOfRange { .. } => "edge_out_of_range",
            BenchError::SelfLoop(_) => "self_loop",
            BenchError::DuplicateEdge(_) => "duplicate_edge",
            BenchError::Contract(e) => e.code(),
            BenchError::Positions(e) => e.code(),
            BenchError::NonFiniteLayout => "non_finite_layout",
            BenchError::UndefinedSilhouette => "undefined_silhouette",
            BenchError::InvalidStep(_) => "invalid_step",
            BenchError::UnknownNode(_) => "unknown_node",
            BenchError::LinkExists(_) => "link_exists",
            BenchError::LinkMissing(_) => "link_missing",
            BenchError::NodeExists(_) => "node_exists",
            BenchError::ZeroRadius => "zero_radius",
            BenchError::InvalidTarget(_) => "invalid_target",
            BenchError::InsideRepository(_) => "inside_repository",
            BenchError::TargetNotEmpty(_) => "target_not_empty",
            BenchError::UnsafeNodeId(_) => "unsafe_node_id",
        }
    }

    pub fn io(path: &Path, e: &std::io::Error) -> BenchError {
        BenchError::Io {
            path: path.display().to_string(),
            detail: e.to_string(),
        }
    }
}

impl fmt::Display for BenchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BenchError::Io { path, detail } => write!(f, "{path}: {detail}"),
            BenchError::InvalidJson(detail) => write!(f, "invalid JSON: {detail}"),
            BenchError::UnsupportedVersion(v) => write!(f, "unsupported format version {v}"),
            BenchError::DuplicateNode(id) => write!(f, "node {id:?} is given twice"),
            BenchError::EdgeOutOfRange { edge, nodes } => {
                write!(f, "edge {edge:?} points outside the {nodes} nodes")
            }
            BenchError::SelfLoop(i) => write!(f, "edge links node {i} to itself"),
            BenchError::DuplicateEdge(edge) => write!(f, "edge {edge:?} is given twice"),
            BenchError::Contract(e) => write!(f, "engine: {e}"),
            BenchError::Positions(e) => write!(f, "positions.json: {e}"),
            BenchError::NonFiniteLayout => write!(f, "the engine returned a non-finite position"),
            BenchError::UndefinedSilhouette => {
                write!(f, "silhouette needs at least 2 communities and 2 nodes")
            }
            BenchError::InvalidStep(i) => write!(
                f,
                "step {i} must hold exactly one of \"link\", \"unlink\", \"create\""
            ),
            BenchError::UnknownNode(id) => write!(f, "unknown node {id:?}"),
            BenchError::LinkExists([a, b]) => write!(f, "{a:?} and {b:?} are already linked"),
            BenchError::LinkMissing([a, b]) => write!(f, "{a:?} and {b:?} are not linked"),
            BenchError::NodeExists(id) => write!(f, "node {id:?} already exists"),
            BenchError::ZeroRadius => write!(f, "the layout has a zero radius"),
            BenchError::InvalidTarget(path) => write!(f, "{path}: not a usable target folder"),
            BenchError::InsideRepository(path) => {
                write!(f, "{path} is inside the repository")
            }
            BenchError::TargetNotEmpty(path) => write!(f, "{path} is not empty"),
            BenchError::UnsafeNodeId(id) => {
                write!(f, "node {id:?} is not a relative path to a .md note")
            }
        }
    }
}

impl std::error::Error for BenchError {}

impl From<ContractError> for BenchError {
    fn from(e: ContractError) -> Self {
        BenchError::Contract(e)
    }
}

impl From<PositionsError> for BenchError {
    fn from(e: PositionsError) -> Self {
        BenchError::Positions(e)
    }
}

/// The repository this crate was built from.
pub fn repo_root() -> Result<PathBuf, BenchError> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    root.canonicalize().map_err(|e| BenchError::io(&root, &e))
}

pub fn read_file(path: &Path) -> Result<String, BenchError> {
    std::fs::read_to_string(path).map_err(|e| BenchError::io(path, &e))
}

pub fn write_file(path: &Path, text: &str) -> Result<(), BenchError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| BenchError::io(parent, &e))?;
    }
    std::fs::write(path, text).map_err(|e| BenchError::io(path, &e))
}
