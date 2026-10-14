mod evaluate;
mod generate;
mod graph;
mod louvain;
mod metrics;
mod replay;
mod snapshot;
mod stability;
mod vault;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use lophelia_bench::graph::{BenchGraph, read_graph};

pub const SBM_100: &str = include_str!("../../../../bench/graphs/sbm-100.json");
pub const SBM_1000: &str = include_str!("../../../../bench/graphs/sbm-1000.json");
pub const SBM_2500: &str = include_str!("../../../../bench/graphs/sbm-2500.json");
pub const BASE_SCENARIO: &str = include_str!("../../../../bench/scenarios/base.json");

pub fn sbm_100() -> BenchGraph {
    match read_graph(SBM_100) {
        Ok(graph) => graph,
        Err(e) => panic!("sbm-100 refused: {e}"),
    }
}

/// A fresh folder under the system temp dir, removed when dropped.
pub struct TempDir(PathBuf);

impl TempDir {
    pub fn new(tag: &str) -> TempDir {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "lophelia-bench-{tag}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn write(&self, relative: &str, text: &str) {
        let path = self.0.join(relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create parent");
        }
        std::fs::write(path, text).expect("write file");
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
