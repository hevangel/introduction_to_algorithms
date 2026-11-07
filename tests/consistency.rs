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

/// Every `(#/some_id)` link in the docs must point at a catalogued algorithm.
#[test]
fn doc_cross_links_resolve() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let ids: HashSet<&str> = registry::ENTRIES.iter().map(|e| e.id).collect();
    let mut broken = vec![];
    for e in registry::ENTRIES {
        let doc = std::fs::read_to_string(root.join(e.doc)).unwrap();
        let mut rest = doc.as_str();
        while let Some(p) = rest.find("(#/") {
            let tail = &rest[p + 3..];
            let end = tail.find(')').unwrap_or(0);
            let target = &tail[..end];
            if !ids.contains(target) { broken.push(format!("{} -> {}", e.id, target)); }
            rest = &tail[end..];
        }
    }
    assert!(broken.is_empty(), "broken doc links: {broken:?}");
}

/// Every algorithm file declares valid source metadata and mentions its tests.
#[test]
fn every_source_has_tests() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for e in registry::ENTRIES {
        let src = std::fs::read_to_string(root.join(e.source)).unwrap();
        assert!(src.contains("#[cfg(test)]") && src.contains("#[test]"), "{}: no unit tests", e.id);
    }
}
