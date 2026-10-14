//! Writes a test vault from a bench graph, outside the repository.

use std::path::Path;

use crate::graph::BenchGraph;
use crate::{BenchError, write_file};

pub const EXCLUDED_NOTE: &str = "node_modules/exclue.md";
pub const APP_JSON: &str = "{\"userIgnoreFilters\":[\"node_modules/\"]}\n";

fn note_name(id: &str) -> &str {
    let name = id.rsplit('/').next().unwrap_or(id);
    name.strip_suffix(".md").unwrap_or(name)
}

/// The id must stay a relative path to a `.md` note inside the vault.
fn check_id(id: &str) -> Result<(), BenchError> {
    let safe = id.ends_with(".md")
        && !id.starts_with(['/', '\\'])
        && !matches!(id.as_bytes(), [letter, b':', ..] if letter.is_ascii_alphabetic())
        && !id.contains('\\')
        && id
            .split('/')
            .all(|s| !s.is_empty() && s != "." && s != "..");
    if safe {
        Ok(())
    } else {
        Err(BenchError::UnsafeNodeId(id.to_string()))
    }
}

/// One note per node at the path of its id, `# <name>` then one `[[<name>]]`
/// line per edge, written in the note of the endpoint with the smaller index;
/// plus `node_modules/exclue.md` linked to the first node, excluded by
/// `.obsidian/app.json`.
pub fn write_vault(graph: &BenchGraph, dir: &Path, repo_root: &Path) -> Result<(), BenchError> {
    let shown = dir.display().to_string();
    let (Some(parent), Some(name)) = (dir.parent(), dir.file_name()) else {
        return Err(BenchError::InvalidTarget(shown));
    };
    let parent = if parent.as_os_str().is_empty() {
        Path::new(".")
    } else {
        parent
    };
    let parent = parent
        .canonicalize()
        .map_err(|_| BenchError::InvalidTarget(shown.clone()))?;
    let root = repo_root
        .canonicalize()
        .map_err(|e| BenchError::io(repo_root, &e))?;
    let target = parent.join(name);
    if target.starts_with(&root) {
        return Err(BenchError::InsideRepository(shown));
    }
    if target.exists() {
        let mut entries = std::fs::read_dir(&target).map_err(|e| BenchError::io(&target, &e))?;
        if entries.next().is_some() {
            return Err(BenchError::TargetNotEmpty(shown));
        }
    }
    for id in &graph.ids {
        check_id(id)?;
    }

    let mut notes: Vec<String> = graph
        .ids
        .iter()
        .map(|id| format!("# {}\n", note_name(id)))
        .collect();
    for &(a, b) in &graph.edges {
        let (low, high) = (a.min(b) as usize, a.max(b) as usize);
        if let (Some(target_id), Some(_)) = (graph.ids.get(high), notes.get(low)) {
            let line = format!("[[{}]]\n", note_name(target_id));
            notes[low].push_str(&line);
        }
    }
    for (id, text) in graph.ids.iter().zip(&notes) {
        write_file(&target.join(id), text)?;
    }
    let excluded = match graph.ids.first() {
        Some(first) => format!("# exclue\n[[{}]]\n", note_name(first)),
        None => "# exclue\n".to_string(),
    };
    write_file(&target.join(EXCLUDED_NOTE), &excluded)?;
    write_file(&target.join(".obsidian").join("app.json"), APP_JSON)?;
    Ok(())
}
