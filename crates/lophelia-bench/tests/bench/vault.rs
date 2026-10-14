use lophelia_bench::repo_root;
use lophelia_bench::snapshot::snapshot;
use lophelia_bench::vault::write_vault;

use crate::{TempDir, sbm_100};

#[test]
fn a_target_inside_the_repository_is_refused_and_nothing_is_created() {
    let root = repo_root().expect("repo root");
    let target = root
        .join("bench")
        .join(format!("vault-test-{}", std::process::id()));
    let result = write_vault(&sbm_100(), &target, &root);
    assert_eq!(result.map_err(|e| e.code()), Err("inside_repository"));
    assert!(!target.exists());
}

#[test]
fn a_target_that_holds_a_file_is_refused() {
    let dir = TempDir::new("not-empty");
    dir.write("keep.txt", "mine\n");
    let root = repo_root().expect("repo root");
    let result = write_vault(&sbm_100(), dir.path(), &root);
    assert_eq!(result.map_err(|e| e.code()), Err("target_not_empty"));
    assert_eq!(
        std::fs::read_to_string(dir.path().join("keep.txt")).expect("kept"),
        "mine\n"
    );
}

#[test]
fn a_written_vault_snapshots_back_to_the_same_graph() {
    let graph = sbm_100();
    let dir = TempDir::new("vault");
    let target = dir.path().join("vault");
    write_vault(&graph, &target, &repo_root().expect("repo root")).expect("write vault");
    assert!(target.join("node_modules/exclue.md").exists());

    let (read_back, _) = snapshot(&target).expect("snapshot");
    assert_eq!(read_back.ids, graph.ids);
    assert_eq!(read_back.edges, graph.edges);
    assert!(
        read_back
            .ids
            .iter()
            .all(|id| !id.starts_with("node_modules/"))
    );
}
