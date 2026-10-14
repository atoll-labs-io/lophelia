use std::path::Path;
use std::process::ExitCode;

use lophelia_bench::evaluate::{evaluate, format_evaluation, record_reference};
use lophelia_bench::generate::{SPECS, generate};
use lophelia_bench::graph::{BenchGraph, read_graph, write_graph};
use lophelia_bench::louvain::modularity;
use lophelia_bench::snapshot::snapshot;
use lophelia_bench::stability::{format_report, read_scenario, run_stability};
use lophelia_bench::vault::write_vault;
use lophelia_bench::{BenchError, read_file, repo_root, write_file};

const USAGE: &str = "usage :
  lophelia-bench <graphe.json> [--record]
  lophelia-bench generate
  lophelia-bench stability <graphe.json> <scénario.json>
  lophelia-bench snapshot <dossier-vault>
  lophelia-bench vault <graphe.json> <dossier>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let result = match args.as_slice() {
        ["generate"] => run_generate(),
        ["stability", graph, scenario] => run_stability_command(graph, scenario),
        ["snapshot", vault] => run_snapshot(vault),
        ["vault", graph, dir] => run_vault(graph, dir),
        [graph] if !is_subcommand(graph) => run_evaluate(graph, false),
        [graph, "--record"] if !is_subcommand(graph) => run_evaluate(graph, true),
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("erreur : {}: {e}", e.code());
            ExitCode::from(1)
        }
    }
}

fn is_subcommand(arg: &str) -> bool {
    matches!(arg, "generate" | "stability" | "snapshot" | "vault") || arg.starts_with("--")
}

fn load_graph(path: &str) -> Result<BenchGraph, BenchError> {
    read_graph(&read_file(Path::new(path))?)
}

fn stem(path: &str) -> String {
    let name = Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    name.strip_suffix(".json").unwrap_or(&name).to_string()
}

fn run_generate() -> Result<(), BenchError> {
    let dir = repo_root()?.join("bench").join("graphs");
    for spec in &SPECS {
        let path = dir.join(format!("{}.json", spec.name));
        write_file(&path, &write_graph(&generate(spec)))?;
        println!("{}", path.display());
    }
    Ok(())
}

fn run_evaluate(path: &str, record: bool) -> Result<(), BenchError> {
    let graph = load_graph(path)?;
    let stem = stem(path);
    let references = repo_root()?.join("bench").join("references");
    let evaluation = evaluate(&graph, &references, &stem)?;
    print!("{}", format_evaluation(&stem, &evaluation));
    if record {
        let written = record_reference(&references, &stem, evaluation.engine_silhouette)?;
        println!("référence enregistrée : {}", written.display());
    }
    Ok(())
}

fn run_stability_command(graph: &str, scenario: &str) -> Result<(), BenchError> {
    let graph = load_graph(graph)?;
    let steps = read_scenario(&read_file(Path::new(scenario))?)?;
    print!("{}", format_report(&run_stability(&graph, &steps)?));
    Ok(())
}

fn run_snapshot(vault: &str) -> Result<(), BenchError> {
    let vault = Path::new(vault);
    let canonical = vault
        .canonicalize()
        .map_err(|e| BenchError::io(vault, &e))?;
    let name = canonical
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "vault".to_string());
    let (graph, warnings) = snapshot(&canonical)?;
    let path = repo_root()?
        .join("bench")
        .join("local")
        .join(format!("{name}.json"));
    write_file(&path, &write_graph(&graph))?;
    for warning in &warnings {
        eprintln!("avertissement : {warning}");
    }
    println!(
        "instantané : {} ({} nœuds, {} arêtes, {} communautés, modularité {:.6})",
        path.display(),
        graph.ids.len(),
        graph.edges.len(),
        graph.community_count(),
        modularity(graph.ids.len(), &graph.edges, &graph.communities),
    );
    Ok(())
}

fn run_vault(graph: &str, dir: &str) -> Result<(), BenchError> {
    let graph = load_graph(graph)?;
    write_vault(&graph, Path::new(dir), &repo_root()?)?;
    println!("vault écrit : {dir} ({} notes)", graph.ids.len());
    Ok(())
}
