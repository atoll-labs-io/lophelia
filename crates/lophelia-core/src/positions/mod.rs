//! `positions.json`, format version 1, read and written against a [`Graph`].

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::contract::Graph;

pub const FORMAT_VERSION: u32 = 1;
/// 10 MiB.
pub const DEFAULT_MAX_BYTES: usize = 10 * 1024 * 1024;

/// Exactly the shape `LayoutInput` takes: `nodes[k]` sits at `xyz[3k..3k+3]`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Known {
    pub nodes: Vec<u32>,
    pub xyz: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PositionsError {
    TooLarge { size: usize, max: usize },
    InvalidJson(String),
    MissingVersion,
    UnsupportedVersion(u64),
    NonFiniteNumber(String),
    MalformedPath { path: String, reason: &'static str },
    DuplicatePath(String),
    LengthMismatch { expected: usize, actual: usize },
}

impl PositionsError {
    pub fn code(&self) -> &'static str {
        match self {
            PositionsError::TooLarge { .. } => "too_large",
            PositionsError::InvalidJson(_) => "invalid_json",
            PositionsError::MissingVersion => "missing_version",
            PositionsError::UnsupportedVersion(_) => "unsupported_version",
            PositionsError::NonFiniteNumber(_) => "non_finite_number",
            PositionsError::MalformedPath { .. } => "malformed_path",
            PositionsError::DuplicatePath(_) => "duplicate_path",
            PositionsError::LengthMismatch { .. } => "length_mismatch",
        }
    }
}

impl fmt::Display for PositionsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionsError::TooLarge { size, max } => {
                write!(f, "file is {size} bytes, above the {max} byte limit")
            }
            PositionsError::InvalidJson(detail) => write!(f, "invalid JSON: {detail}"),
            PositionsError::MissingVersion => write!(f, "no integer \"version\" field"),
            PositionsError::UnsupportedVersion(v) => write!(f, "unsupported format version {v}"),
            PositionsError::NonFiniteNumber(detail) => write!(f, "non-finite number: {detail}"),
            PositionsError::MalformedPath { path, reason } => {
                write!(f, "malformed path {path:?}: {reason}")
            }
            PositionsError::DuplicatePath(path) => write!(f, "path {path:?} is given twice"),
            PositionsError::LengthMismatch { expected, actual } => {
                write!(f, "expected {expected} coordinates, got {actual}")
            }
        }
    }
}

impl std::error::Error for PositionsError {}

#[derive(Deserialize)]
struct Header {
    #[serde(default)]
    version: Option<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileV1 {
    #[allow(dead_code)]
    version: u32,
    positions: Vec<EntryV1>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EntryV1 {
    path: String,
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize)]
struct EntryOut<'a> {
    path: &'a str,
    x: f64,
    y: f64,
    z: f64,
}

pub fn read(text: &str, graph: &Graph, max_bytes: usize) -> Result<Known, PositionsError> {
    if text.len() > max_bytes {
        return Err(PositionsError::TooLarge {
            size: text.len(),
            max: max_bytes,
        });
    }
    let header: Header = serde_json::from_str(text).map_err(json_error)?;
    let version = header
        .version
        .and_then(|v| v.as_u64())
        .ok_or(PositionsError::MissingVersion)?;
    match version {
        1 => read_v1(text, graph),
        other => Err(PositionsError::UnsupportedVersion(other)),
    }
}

fn read_v1(text: &str, graph: &Graph) -> Result<Known, PositionsError> {
    let file: FileV1 = serde_json::from_str(text).map_err(json_error)?;
    let mut seen = HashSet::with_capacity(file.positions.len());
    for entry in &file.positions {
        check_path(&entry.path)?;
        check_finite(&entry.path, [entry.x, entry.y, entry.z])?;
        if !seen.insert(entry.path.as_str()) {
            return Err(PositionsError::DuplicatePath(entry.path.clone()));
        }
    }
    let mut known = Known::default();
    for entry in &file.positions {
        if let Some(index) = graph.index_of(&entry.path) {
            known.nodes.push(index);
            known.xyz.extend_from_slice(&[entry.x, entry.y, entry.z]);
        }
    }
    Ok(known)
}

/// One line per node, sorted by the UTF-8 bytes of the path; only the nodes of
/// `graph` are written, so a stale path disappears.
pub fn write(graph: &Graph, xyz: &[f64]) -> Result<String, PositionsError> {
    let expected = graph.node_count().saturating_mul(3);
    if xyz.len() != expected {
        return Err(PositionsError::LengthMismatch {
            expected,
            actual: xyz.len(),
        });
    }
    let mut entries: Vec<EntryOut> = Vec::with_capacity(graph.node_count());
    let (triples, _) = xyz.as_chunks::<3>();
    for (path, &[x, y, z]) in graph.ids().iter().zip(triples) {
        check_path(path)?;
        check_finite(path, [x, y, z])?;
        entries.push(EntryOut { path, x, y, z });
    }
    entries.sort_by(|a, b| a.path.cmp(b.path));

    let mut lines = Vec::with_capacity(entries.len());
    for entry in &entries {
        lines.push(serde_json::to_string(entry).map_err(json_error)?);
    }
    let mut out = format!("{{\"version\":{FORMAT_VERSION},\"positions\":[\n");
    out.push_str(&lines.join(",\n"));
    if !lines.is_empty() {
        out.push('\n');
    }
    out.push_str("]}\n");
    Ok(out)
}

fn json_error(e: serde_json::Error) -> PositionsError {
    let detail = e.to_string();
    // serde_json has no public code for this case; the pinned version and a
    // test keep this message prefix honest.
    if detail.starts_with("number out of range") {
        PositionsError::NonFiniteNumber(detail)
    } else {
        PositionsError::InvalidJson(detail)
    }
}

fn check_finite(path: &str, xyz: [f64; 3]) -> Result<(), PositionsError> {
    if xyz.iter().all(|v| v.is_finite()) {
        Ok(())
    } else {
        Err(PositionsError::NonFiniteNumber(format!(
            "position of {path:?}"
        )))
    }
}

fn check_path(path: &str) -> Result<(), PositionsError> {
    let reason = if path.is_empty() {
        Some("empty")
    } else if path.starts_with(['/', '\\']) {
        Some("absolute")
    } else if matches!(path.as_bytes(), [letter, b':', ..] if letter.is_ascii_alphabetic()) {
        Some("drive letter")
    } else if path.split(['/', '\\']).any(|segment| segment == "..") {
        Some("parent segment")
    } else {
        None
    };
    match reason {
        Some(reason) => Err(PositionsError::MalformedPath {
            path: path.to_string(),
            reason,
        }),
        None => Ok(()),
    }
}
