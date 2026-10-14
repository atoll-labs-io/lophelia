//! Replays a scripted series of edits through the path a host takes between
//! two openings (`positions.json` written, then read back) and measures 3a
//! and 3b at each step.

use std::collections::BTreeSet;

use lophelia_core::contract::{Graph, LayoutInput, Settings};
use lophelia_core::positions::{self, DEFAULT_MAX_BYTES};
use serde::Deserialize;

use crate::BenchError;
use crate::evaluate::check_finite;
use crate::graph::{BenchGraph, json_error};
use crate::metrics::gap;

#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    Link(String, String),
    Unlink(String, String),
    Create { id: String, links: Vec<String> },
}

impl Step {
    fn label(&self) -> String {
        match self {
            Step::Link(a, b) => format!("link {a} {b}"),
            Step::Unlink(a, b) => format!("unlink {a} {b}"),
            Step::Create { id, links } if links.is_empty() => format!("create {id}"),
            Step::Create { id, links } => format!("create {id} → {}", links.join(", ")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepReport {
    pub label: String,
    /// 3a: largest move of any node when the layout reruns with nothing changed.
    pub drift: f64,
    /// 3b: largest move of the half of the nodes farthest from the edited note, in % of the radius.
    pub far_half_displacement: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StabilityReport {
    pub steps: Vec<StepReport>,
}

impl StabilityReport {
    pub fn max_drift(&self) -> f64 {
        self.steps.iter().map(|s| s.drift).fold(0.0, f64::max)
    }

    pub fn max_far_half_displacement(&self) -> f64 {
        self.steps
            .iter()
            .map(|s| s.far_half_displacement)
            .fold(0.0, f64::max)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ScenarioV1 {
    version: u64,
    steps: Vec<StepV1>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StepV1 {
    link: Option<[String; 2]>,
    unlink: Option<[String; 2]>,
    create: Option<String>,
    links: Option<Vec<String>>,
}

pub fn read_scenario(text: &str) -> Result<Vec<Step>, BenchError> {
    let scenario: ScenarioV1 = serde_json::from_str(text).map_err(json_error)?;
    if scenario.version != 1 {
        return Err(BenchError::UnsupportedVersion(scenario.version));
    }
    scenario
        .steps
        .into_iter()
        .enumerate()
        .map(|(i, s)| match (s.link, s.unlink, s.create, s.links) {
            (Some([a, b]), None, None, None) => Ok(Step::Link(a, b)),
            (None, Some([a, b]), None, None) => Ok(Step::Unlink(a, b)),
            (None, None, Some(id), links) => Ok(Step::Create {
                id,
                links: links.unwrap_or_default(),
            }),
            _ => Err(BenchError::InvalidStep(i + 1)),
        })
        .collect()
}

/// 3b. `R` is the largest distance to the barycentre of `before`; the centre is
/// the edited node, taken in `before` or, for a created node, in `after`.
/// Among the nodes present on both sides other than the edited one, sorted by
/// decreasing distance to the centre (ties by increasing index), the first
/// half (at least one) is kept; the result is its largest move over `R`, × 100.
pub fn half_farthest_displacement(
    before: &[f64],
    after: &[f64],
    modified: usize,
) -> Result<f64, BenchError> {
    let n = before.len() / 3;
    if n == 0 {
        return Err(BenchError::ZeroRadius);
    }
    let mut bary = [0.0; 3];
    for i in 0..n {
        for (a, b) in bary.iter_mut().enumerate() {
            *b += before[3 * i + a] / n as f64;
        }
    }
    let radius = (0..n)
        .map(|i| gap(&before[3 * i..], &bary))
        .fold(0.0, f64::max);
    if radius == 0.0 {
        return Err(BenchError::ZeroRadius);
    }
    let source = if modified < n { before } else { after };
    let center = source
        .get(3 * modified..3 * modified + 3)
        .ok_or_else(|| BenchError::UnknownNode(format!("#{modified}")))?;

    let shared = n.min(after.len() / 3);
    let mut others: Vec<(f64, usize)> = (0..shared)
        .filter(|&i| i != modified)
        .map(|i| (gap(&before[3 * i..], center), i))
        .collect();
    others.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    let keep = (others.len() / 2).max(1).min(others.len());
    let moved = others[..keep]
        .iter()
        .map(|&(_, i)| gap(&before[3 * i..], &after[3 * i..]))
        .fold(0.0, f64::max);
    Ok(moved / radius * 100.0)
}

struct State {
    ids: Vec<String>,
    edges: BTreeSet<(u32, u32)>,
}

impl State {
    fn index(&self, id: &str) -> Result<u32, BenchError> {
        self.ids
            .iter()
            .position(|x| x == id)
            .map(|i| i as u32)
            .ok_or_else(|| BenchError::UnknownNode(id.to_string()))
    }

    fn pair(&self, a: &str, b: &str) -> Result<(u32, u32), BenchError> {
        let (i, j) = (self.index(a)?, self.index(b)?);
        if i == j {
            return Err(BenchError::SelfLoop(i));
        }
        Ok((i.min(j), i.max(j)))
    }

    fn flat_edges(&self) -> Vec<u32> {
        self.edges.iter().flat_map(|&(a, b)| [a, b]).collect()
    }

    /// Applies `step` and returns the edited node and the nodes it touches.
    fn apply(&mut self, step: &Step) -> Result<(u32, Vec<u32>), BenchError> {
        match step {
            Step::Link(a, b) => {
                let edge = self.pair(a, b)?;
                if !self.edges.insert(edge) {
                    return Err(BenchError::LinkExists([a.clone(), b.clone()]));
                }
                let (i, j) = (self.index(a)?, self.index(b)?);
                Ok((i, vec![i, j]))
            }
            Step::Unlink(a, b) => {
                let edge = self.pair(a, b)?;
                if !self.edges.remove(&edge) {
                    return Err(BenchError::LinkMissing([a.clone(), b.clone()]));
                }
                let (i, j) = (self.index(a)?, self.index(b)?);
                Ok((i, vec![i, j]))
            }
            Step::Create { id, links } => {
                if self.ids.iter().any(|x| x == id) {
                    return Err(BenchError::NodeExists(id.clone()));
                }
                let targets = links
                    .iter()
                    .map(|l| self.index(l))
                    .collect::<Result<BTreeSet<u32>, _>>()?;
                let created = self.ids.len() as u32;
                self.ids.push(id.clone());
                let mut changed = vec![created];
                for t in targets {
                    self.edges.insert((t, created));
                    changed.push(t);
                }
                Ok((created, changed))
            }
        }
    }
}

fn layout(
    engine: &Graph,
    edges: &[u32],
    known: &positions::Known,
    changed: &[u32],
) -> Result<Vec<f64>, BenchError> {
    let xyz = engine.layout(&LayoutInput {
        edges,
        known_nodes: &known.nodes,
        known_xyz: &known.xyz,
        changed,
        settings: Settings::default(),
    })?;
    check_finite(&xyz)?;
    Ok(xyz)
}

pub fn run_stability(graph: &BenchGraph, steps: &[Step]) -> Result<StabilityReport, BenchError> {
    let mut state = State {
        ids: graph.ids.clone(),
        edges: graph
            .edges
            .iter()
            .map(|&(a, b)| (a.min(b), a.max(b)))
            .collect(),
    };
    let mut engine = Graph::new(state.ids.clone())?;
    let initial = layout(
        &engine,
        &state.flat_edges(),
        &positions::Known::default(),
        &[],
    )?;
    let mut text = positions::write(&engine, &initial)?;
    let mut previous = initial;

    let mut reports = Vec::with_capacity(steps.len());
    for step in steps {
        let known = positions::read(&text, &engine, DEFAULT_MAX_BYTES)?;
        let before = layout(&engine, &state.flat_edges(), &known, &[])?;
        let drift = (0..before.len() / 3)
            .map(|i| gap(&previous[3 * i..], &before[3 * i..]))
            .fold(0.0, f64::max);

        let (modified, changed) = state.apply(step)?;
        engine = Graph::new(state.ids.clone())?;
        let known = positions::read(&text, &engine, DEFAULT_MAX_BYTES)?;
        let after = layout(&engine, &state.flat_edges(), &known, &changed)?;
        let far_half_displacement = half_farthest_displacement(&before, &after, modified as usize)?;

        text = positions::write(&engine, &after)?;
        previous = after;
        reports.push(StepReport {
            label: step.label(),
            drift,
            far_half_displacement,
        });
    }
    Ok(StabilityReport { steps: reports })
}

pub fn format_report(report: &StabilityReport) -> String {
    let mut out = String::new();
    for (i, s) in report.steps.iter().enumerate() {
        out.push_str(&format!(
            "étape {} ({}) : 3a = {} ; 3b = {:.6} % du rayon\n",
            i + 1,
            s.label,
            s.drift,
            s.far_half_displacement
        ));
    }
    out.push_str(&format!("3a max : {}\n", report.max_drift()));
    out.push_str(&format!(
        "3b max : {:.6} % du rayon\n",
        report.max_far_half_displacement()
    ));
    out
}
