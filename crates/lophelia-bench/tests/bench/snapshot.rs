use lophelia_bench::snapshot::snapshot;

use crate::TempDir;

fn edges_by_id(vault: &TempDir) -> (Vec<String>, Vec<(String, String)>, Vec<String>) {
    let (graph, warnings) = snapshot(vault.path()).expect("snapshot");
    let edges = graph
        .edges
        .iter()
        .map(|&(a, b)| (graph.ids[a as usize].clone(), graph.ids[b as usize].clone()))
        .collect();
    (graph.ids, edges, warnings)
}

fn pair(a: &str, b: &str) -> (String, String) {
    (a.to_string(), b.to_string())
}

#[test]
fn links_resolve_through_alias_heading_embed_and_path() {
    let vault = TempDir::new("links");
    vault.write(
        "A.md",
        "[[B]] and [[b|alias]]\n![[B#titre]]\n[[dir/C.md]] [[absente]] [[\n]]\n",
    );
    vault.write("B.md", "# B\n");
    vault.write("dir/C.md", "# C\n");
    let (_, edges, _) = edges_by_id(&vault);
    assert_eq!(edges, vec![pair("A.md", "B.md"), pair("A.md", "dir/C.md")]);
}

#[test]
fn excluded_and_hidden_notes_are_left_out_and_regex_filters_warned() {
    let vault = TempDir::new("filters");
    vault.write(
        ".obsidian/app.json",
        "{\"userIgnoreFilters\":[\"priv/\",\"/x/\"],\"other\":true}",
    );
    vault.write("a.md", "[[n]]\n");
    vault.write("priv/n.md", "# n\n");
    vault.write(".cache/n.md", "# n\n");
    let (ids, edges, warnings) = edges_by_id(&vault);
    assert_eq!(ids, vec!["a.md".to_string()]);
    assert!(edges.is_empty());
    assert_eq!(warnings.len(), 1, "{warnings:?}");
    assert!(warnings[0].contains("/x/"), "{warnings:?}");
}

#[test]
fn a_shared_name_resolves_to_the_shortest_path() {
    let vault = TempDir::new("shared-name");
    // Byte order alone would pick "A/x.md" ('A' < 'x').
    vault.write("x.md", "# x\n");
    vault.write("A/x.md", "# x\n");
    vault.write("from.md", "[[x]]\n");
    let (_, edges, _) = edges_by_id(&vault);
    assert_eq!(edges, vec![pair("from.md", "x.md")]);
}
