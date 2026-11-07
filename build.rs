//! Discovers every `src/algorithms/<category>/<id>.rs`, reads its `//! @key value`
//! metadata header, and generates (1) the module tree and (2) the registry table.
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out = std::env::var("OUT_DIR").unwrap();
    let alg = Path::new(&root).join("src/algorithms");
    println!("cargo:rerun-if-changed=src/algorithms");
    println!("cargo:rerun-if-changed=docs");

    let mut cats: Vec<String> = fs::read_dir(&alg)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().into_string().unwrap())
        .collect();
    cats.sort();

    let mut mods = String::new();
    let mut entries = String::new();
    let mut seen = BTreeSet::new();

    for cat in &cats {
        let mut files: Vec<_> = fs::read_dir(alg.join(cat))
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map_or(false, |x| x == "rs"))
            .collect();
        files.sort();
        writeln!(mods, "pub mod {cat} {{").unwrap();
        for path in files {
            let id = path.file_stem().unwrap().to_str().unwrap().to_string();
            assert!(seen.insert(id.clone()), "duplicate algorithm id {id}");
            let src = fs::read_to_string(&path).unwrap();
            let mut meta: BTreeMap<String, String> = BTreeMap::new();
            for line in src.lines() {
                let Some(rest) = line.strip_prefix("//! @") else { continue };
                let (k, v) = rest.split_once(' ').unwrap_or((rest, ""));
                meta.insert(k.to_string(), v.trim().to_string());
            }
            for k in ["title", "chapter", "editions", "summary", "order"] {
                assert!(meta.contains_key(k), "{} is missing //! @{k}", path.display());
            }
            let editions: Vec<String> =
                meta["editions"].split(',').map(|s| s.trim().to_string()).collect();
            writeln!(mods, "    #[path = {:?}]\n    pub mod {id};", path.display().to_string()).unwrap();
            writeln!(
                entries,
                "    Entry {{ id: {id:?}, title: {:?}, category: {cat:?}, chapter: {:?}, editions: &[{}], summary: {:?}, order: {}, source: \"src/algorithms/{cat}/{id}.rs\", doc: \"docs/{cat}/{id}.md\", demo: algorithms::{cat}::{id}::demo }},",
                meta["title"], meta["chapter"], editions.join(", "), meta["summary"], meta["order"]
            ).unwrap();
        }
        writeln!(mods, "}}").unwrap();
    }
    fs::write(Path::new(&out).join("algorithm_mods.rs"), mods).unwrap();
    fs::write(
        Path::new(&out).join("registry.rs"),
        format!("pub static ENTRIES: &[Entry] = &[\n{entries}];\n"),
    )
    .unwrap();
}
