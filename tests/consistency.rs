//! Repository-wide invariants: every catalogued algorithm has source, docs, a demo and valid metadata.
use clrs::{registry, CATEGORIES};
use std::collections::HashSet;
use std::path::Path;

#[test]
fn every_entry_is_complete() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut ids = HashSet::new();
    for e in registry::ENTRIES {
        assert!(ids.insert(e.id), "duplicate id {}", e.id);
        assert!(CATEGORIES.iter().any(|c| c.0 == e.category), "{}: unknown category {}", e.id, e.category);
        assert!(!e.editions.is_empty() && e.editions.iter().all(|x| (1..=4).contains(x)), "{}: bad editions", e.id);
        assert!(root.join(e.source).is_file(), "{}: missing source", e.id);
        let doc = std::fs::read_to_string(root.join(e.doc)).unwrap_or_else(|_| panic!("{}: missing doc {}", e.id, e.doc));
        assert!(doc.starts_with("# "), "{}: doc must start with an H1", e.id);
        assert!(doc.len() > 400, "{}: doc too short", e.id);
        let out = (e.demo)();
        assert!(!out.trim().is_empty(), "{}: empty demo", e.id);
    }
}

#[test]
fn no_orphan_docs() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for cat in std::fs::read_dir(root.join("docs")).unwrap().flatten() {
        if !cat.path().is_dir() { continue; }
        for f in std::fs::read_dir(cat.path()).unwrap().flatten() {
            let id = f.path().file_stem().unwrap().to_string_lossy().to_string();
            assert!(registry::ENTRIES.iter().any(|e| e.id == id), "orphan doc {}", f.path().display());
        }
    }
}
