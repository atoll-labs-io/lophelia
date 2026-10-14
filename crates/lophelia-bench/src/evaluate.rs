//! Scores the engine layout of a graph against the naive layout and a recorded reference.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use lophelia_core::contract::{Graph, LayoutInput, Settings};
use serde::{Deserialize, Serialize};

use crate::graph::{BenchGraph, json_error};
use crate::metrics::silhouette;
use crate::naive::naive_layout;
use crate::{BenchError, read_file, write_file};

#[derive(Debug, Clone, PartialEq)]
pub struct Evaluation {
    pub nodes: usize,
    pub edges: usize,
    pub communities: usize,
    pub engine_silhouette: f64,
    pub naive_silhouette: f64,
    pub reference_gap: Option<f64>,
    pub elapsed: Duration,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Reference {
    silhouette: f64,
}

pub fn reference_path(references_dir: &Path, stem: &str) -> PathBuf {
    references_dir.join(format!("{stem}.json"))
}

/// The engine layout from scratch, timed around `layout` alone.
pub fn engine_layout(graph: &BenchGraph) -> Result<(Vec<f64>, Duration), BenchError> {
    let engine = Graph::new(graph.ids.clone())?;
    let edges = graph.flat_edges();
    let input = LayoutInput {
        edges: &edges,
        known_nodes: &[],
        known_xyz: &[],
        changed: &[],
        settings: Settings::default(),
    };
    let start = Instant::now();
    let xyz = engine.layout(&input)?;
    let elapsed = start.elapsed();
    check_finite(&xyz)?;
    Ok((xyz, elapsed))
}

pub fn check_finite(xyz: &[f64]) -> Result<(), BenchError> {
    if xyz.iter().all(|v| v.is_finite()) {
        Ok(())
    } else {
        Err(BenchError::NonFiniteLayout)
    }
}

pub fn evaluate(
    graph: &BenchGraph,
    references_dir: &Path,
    stem: &str,
) -> Result<Evaluation, BenchError> {
    let (xyz, elapsed) = engine_layout(graph)?;
    let engine_silhouette =
        silhouette(&xyz, &graph.communities).ok_or(BenchError::UndefinedSilhouette)?;
    let naive = naive_layout(graph.ids.len(), &graph.edges);
    let naive_silhouette =
        silhouette(&naive, &graph.communities).ok_or(BenchError::UndefinedSilhouette)?;
    let reference_gap =
        read_reference(references_dir, stem)?.map(|reference| engine_silhouette - reference);
    Ok(Evaluation {
        nodes: graph.ids.len(),
        edges: graph.edges.len(),
        communities: graph.community_count(),
        engine_silhouette,
        naive_silhouette,
        reference_gap,
        elapsed,
    })
}

pub fn read_reference(references_dir: &Path, stem: &str) -> Result<Option<f64>, BenchError> {
    let path = reference_path(references_dir, stem);
    if !path.exists() {
        return Ok(None);
    }
    let reference: Reference = serde_json::from_str(&read_file(&path)?).map_err(json_error)?;
    Ok(Some(reference.silhouette))
}

pub fn record_reference(
    references_dir: &Path,
    stem: &str,
    silhouette: f64,
) -> Result<PathBuf, BenchError> {
    let path = reference_path(references_dir, stem);
    let text = serde_json::to_string(&Reference { silhouette }).map_err(json_error)?;
    write_file(&path, &format!("{text}\n"))?;
    Ok(path)
}

pub fn format_evaluation(stem: &str, e: &Evaluation) -> String {
    let reference = match e.reference_gap {
        Some(gap) => format!("écart à la référence : {gap:+.6}"),
        None => "référence : aucune".to_string(),
    };
    format!(
        "graphe : {stem} ({} nœuds, {} arêtes, {} communautés)\n\
         silhouette moteur : {:.6}\n\
         silhouette naïve : {:.6}\n\
         temps de calcul du moteur : {:.3} ms\n\
         {reference}\n",
        e.nodes,
        e.edges,
        e.communities,
        e.engine_silhouette,
        e.naive_silhouette,
        e.elapsed.as_secs_f64() * 1000.0,
    )
}
