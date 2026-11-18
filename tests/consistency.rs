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
fn topic_docs_are_complete() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for (cid, _, _) in CATEGORIES {
        let path = root.join(format!("docs/{cid}.md"));
        let doc = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("{cid}: missing topic doc {}", path.display()));
        assert!(doc.starts_with("# "), "{cid}: topic doc must start with an H1");
        assert!(doc.len() > 400, "{cid}: topic doc too short");
    }
}

#[test]
fn no_orphan_docs() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for entry in std::fs::read_dir(root.join("docs")).unwrap().flatten() {
        let p = entry.path();
        if p.is_file() {
            let cid = p.file_stem().unwrap().to_string_lossy().to_string();
            assert!(CATEGORIES.iter().any(|c| c.0 == cid), "orphan topic doc {}", p.display());
            continue;
        }
        if !p.is_dir() { continue; }
        for f in std::fs::read_dir(&p).unwrap().flatten() {
            let id = f.path().file_stem().unwrap().to_string_lossy().to_string();
            assert!(registry::ENTRIES.iter().any(|e| e.id == id), "orphan doc {}", f.path().display());
        }
    }
}

/// Every `(#/some_id)` or `(#/topic/cat_id)` link in algorithm and topic docs must resolve.
#[test]
fn doc_cross_links_resolve() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let ids: HashSet<&str> = registry::ENTRIES.iter().map(|e| e.id).collect();
    let cats: HashSet<&str> = CATEGORIES.iter().map(|c| c.0).collect();
    let mut broken = vec![];
    let mut check = |owner: &str, doc: &str| {
        let mut rest = doc;
        while let Some(p) = rest.find("(#/") {
            let tail = &rest[p + 3..];
            let end = tail.find(')').unwrap_or(0);
            let target = &tail[..end];
            let ok = if let Some(cat) = target.strip_prefix("topic/") {
                cats.contains(cat)
            } else {
                ids.contains(target)
            };
            if !ok { broken.push(format!("{owner} -> {target}")); }
            rest = &tail[end..];
        }
    };
    for e in registry::ENTRIES {
        let doc = std::fs::read_to_string(root.join(e.doc)).unwrap();
        check(e.id, &doc);
    }
    for (cid, _, _) in CATEGORIES {
        let doc = std::fs::read_to_string(root.join(format!("docs/{cid}.md"))).unwrap();
        check(&format!("topic/{cid}"), &doc);
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
