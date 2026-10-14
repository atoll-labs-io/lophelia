//! Reads a vault without Obsidian: Markdown notes, `[[…]]` links resolved by
//! note name, exclusions of `.obsidian/app.json`. An approximation of
//! Obsidian's `resolvedLinks`, enough for a bench.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Deserialize;

use crate::graph::{BenchGraph, json_error};
use crate::louvain::louvain;
use crate::{BenchError, read_file};

#[derive(Deserialize)]
struct AppJson {
    #[serde(rename = "userIgnoreFilters", default)]
    user_ignore_filters: Vec<String>,
}

/// Prefix filters to apply, and the warnings for the regex ones left out.
fn read_filters(vault: &Path) -> Result<(Vec<String>, Vec<String>), BenchError> {
    let path = vault.join(".obsidian").join("app.json");
    if !path.exists() {
        return Ok((Vec::new(), Vec::new()));
    }
    let app: AppJson = serde_json::from_str(&read_file(&path)?).map_err(json_error)?;
    let mut prefixes = Vec::new();
    let mut warnings = Vec::new();
    for filter in app.user_ignore_filters {
        if filter.len() >= 2 && filter.starts_with('/') && filter.ends_with('/') {
            warnings.push(format!("filtre regex non appliqué : {filter}"));
        } else if !filter.is_empty() {
            prefixes.push(filter);
        }
    }
    Ok((prefixes, warnings))
}

fn walk(
    dir: &Path,
    relative: &str,
    filters: &[String],
    notes: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), BenchError> {
    let mut entries = std::fs::read_dir(dir)
        .map_err(|e| BenchError::io(dir, &e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| BenchError::io(dir, &e))?;
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let Some(name) = entry.file_name().to_str().map(str::to_string) else {
            warnings.push(format!("nom non UTF-8 ignoré : {}", entry.path().display()));
            continue;
        };
        if name.starts_with('.') {
            continue;
        }
        let path = format!("{relative}{name}");
        let kind = entry
            .file_type()
            .map_err(|e| BenchError::io(&entry.path(), &e))?;
        if kind.is_dir() {
            let prefix = format!("{path}/");
            if !filters.iter().any(|f| prefix.starts_with(f.as_str())) {
                walk(&entry.path(), &prefix, filters, notes, warnings)?;
            }
        } else if kind.is_file()
            && name.ends_with(".md")
            && !filters.iter().any(|f| path.starts_with(f.as_str()))
        {
            notes.push(path);
        }
    }
    Ok(())
}

/// Targets of every single-line `[[…]]` (and `![[…]]`): before `|`, then
/// before `#`, trimmed, a final `.md` removed, lowercased.
pub fn link_targets(text: &str) -> Vec<String> {
    let mut targets = Vec::new();
    for line in text.lines() {
        let mut rest = line;
        while let Some(open) = rest.find("[[") {
            let after = &rest[open + 2..];
            let Some(close) = after.find("]]") else {
                break;
            };
            let inner = &after[..close];
            let inner = inner.split('|').next().unwrap_or("");
            let inner = inner.split('#').next().unwrap_or("").trim();
            let mut target = inner.to_lowercase();
            if target.ends_with(".md") {
                target.truncate(target.len() - 3);
            }
            if !target.is_empty() {
                targets.push(target);
            }
            rest = &after[close + 2..];
        }
    }
    targets
}

struct Resolver<'a> {
    notes: &'a [String],
    /// Lowercased path without `.md`, per note.
    stems: Vec<String>,
    by_name: BTreeMap<String, Vec<usize>>,
}

impl<'a> Resolver<'a> {
    fn new(notes: &'a [String]) -> Resolver<'a> {
        let stems: Vec<String> = notes
            .iter()
            .map(|p| p.strip_suffix(".md").unwrap_or(p).to_lowercase())
            .collect();
        let mut by_name: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (i, stem) in stems.iter().enumerate() {
            let name = stem.rsplit('/').next().unwrap_or(stem);
            by_name.entry(name.to_string()).or_default().push(i);
        }
        Resolver {
            notes,
            stems,
            by_name,
        }
    }

    /// Several candidates → the shortest path, then byte order.
    fn resolve(&self, target: &str) -> Option<usize> {
        let candidates: Vec<usize> = if target.contains('/') {
            let suffix = format!("/{target}");
            (0..self.stems.len())
                .filter(|&i| self.stems[i] == target || self.stems[i].ends_with(&suffix))
                .collect()
        } else {
            self.by_name.get(target).cloned().unwrap_or_default()
        };
        candidates.into_iter().min_by(|&a, &b| {
            let (pa, pb) = (&self.notes[a], &self.notes[b]);
            pa.len().cmp(&pb.len()).then_with(|| pa.cmp(pb))
        })
    }
}

/// The graph of a vault, with its communities found by Louvain, and the warnings met.
pub fn snapshot(vault: &Path) -> Result<(BenchGraph, Vec<String>), BenchError> {
    let (filters, mut warnings) = read_filters(vault)?;
    let mut notes = Vec::new();
    walk(vault, "", &filters, &mut notes, &mut warnings)?;
    notes.sort();

    let resolver = Resolver::new(&notes);
    let mut edges = BTreeSet::new();
    for (i, note) in notes.iter().enumerate() {
        let bytes =
            std::fs::read(vault.join(note)).map_err(|e| BenchError::io(&vault.join(note), &e))?;
        for target in link_targets(&String::from_utf8_lossy(&bytes)) {
            if let Some(j) = resolver.resolve(&target)
                && j != i
            {
                edges.insert((i.min(j) as u32, i.max(j) as u32));
            }
        }
    }
    let edges: Vec<(u32, u32)> = edges.into_iter().collect();
    let communities = louvain(notes.len(), &edges);
    Ok((
        BenchGraph {
            ids: notes,
            communities,
            edges,
        },
        warnings,
    ))
}
