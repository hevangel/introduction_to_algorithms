//! Generates the data the web app reads at runtime:
//!   * `site-data/manifest.json` — categories + algorithm metadata (from the registry)
//!   * `logs/<id>.log`           — the demo output (and unit-test results, if a `cargo test`
//!                                 transcript is passed as the first argument)
//!
//! Usage: `cargo run --release --bin gen_site_data [-- target/test-output.txt]`
//! (or just `scripts/build_site_data.sh`).
use clrs::{registry, CATEGORIES};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn esc(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let logs = root.join("logs");
    fs::create_dir_all(&logs).unwrap();
    fs::create_dir_all(root.join("site-data")).unwrap();

    // id -> [(test name, status)]
    let mut tests: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    if let Some(p) = std::env::args().nth(1) {
        let txt = fs::read_to_string(&p).unwrap_or_default();
        for line in txt.lines() {
            let Some(rest) = line.strip_prefix("test algorithms::") else { continue };
            let Some((name, status)) = rest.split_once(" ... ") else { continue };
            let parts: Vec<&str> = name.split("::").collect();
            if parts.len() >= 4 {
                tests.entry(parts[1].to_string()).or_default().push((parts[3..].join("::"), status.to_string()));
            }
        }
    }

    let entries = registry::sorted();
    let mut json = String::from("{\n  \"categories\": [\n");
    for (ci, (cid, ctitle, cdesc)) in CATEGORIES.iter().enumerate() {
        json.push_str(&format!("    {{ \"id\": {}, \"title\": {}, \"description\": {}, \"algorithms\": [\n", esc(cid), esc(ctitle), esc(cdesc)));
        let in_cat: Vec<_> = entries.iter().filter(|e| e.category == *cid).collect();
        for (i, e) in in_cat.iter().enumerate() {
            let demo = (e.demo)();
            let mut log = format!("$ cargo run --bin gen_site_data   # runs {}::demo()\n\n{}\n", e.id, demo.trim_end());
            if let Some(ts) = tests.get(e.id) {
                log.push_str(&format!("\n$ cargo test {}\n", e.id));
                for (n, s) in ts {
                    log.push_str(&format!("test {n} ... {s}\n"));
                }
                let ok = ts.iter().filter(|t| t.1 == "ok").count();
                log.push_str(&format!("\ntest result: {} passed; {} failed\n", ok, ts.len() - ok));
            }
            fs::write(logs.join(format!("{}.log", e.id)), log).unwrap();
            let eds: Vec<String> = e.editions.iter().map(|x| x.to_string()).collect();
            json.push_str(&format!(
                "      {{ \"id\": {}, \"title\": {}, \"chapter\": {}, \"editions\": [{}], \"summary\": {}, \"source\": {}, \"doc\": {}, \"log\": {} }}{}\n",
                esc(e.id), esc(e.title), esc(e.chapter), eds.join(","), esc(e.summary), esc(e.source), esc(e.doc),
                esc(&format!("logs/{}.log", e.id)), if i + 1 < in_cat.len() { "," } else { "" }
            ));
        }
        json.push_str(&format!("    ] }}{}\n", if ci + 1 < CATEGORIES.len() { "," } else { "" }));
    }
    json.push_str("  ]\n}\n");
    fs::write(root.join("site-data/manifest.json"), json).unwrap();
    println!("wrote {} logs and site-data/manifest.json ({} algorithms)", entries.len(), entries.len());
}
