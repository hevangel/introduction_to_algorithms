# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repo is

Every algorithm from editions 1–4 of CLRS (*Introduction to Algorithms*) implemented in dependency-free Rust, each with unit tests, a demo that produces a run log, and a markdown explanation. `index.html` is a single-page web app that loads all of that **at runtime** (nothing is hard-coded in the page). ~281 algorithm "families", 18 topic areas, ~637 unit tests.

## Commands

```bash
cargo test                          # unit tests (lib) + tests/consistency.rs (docs/source/link checks)
cargo test --lib heapsort           # one algorithm: filter by id (module name == file name)
cargo test --lib algorithms::graphs # one category
cargo test --test consistency       # docs/registry invariants only (~10 s: runs every demo())
./scripts/build_site_data.sh        # run tests, then regenerate site-data/manifest.json and logs/*.log
python3 -m http.server 8000         # serve the repo root, open http://localhost:8000/ (file:// cannot fetch())
```

`logs/` and `site-data/manifest.json` are **committed generated files**; rerun `scripts/build_site_data.sh` after adding/changing an algorithm, its demo, or its tests. `site-data/editions.json` (edition comparison shown on the home page) is hand-written.

## Architecture (the part that spans several files)

* **One algorithm family = three artefacts** sharing an id: `src/algorithms/<category>/<id>.rs`, `docs/<category>/<id>.md`, and (generated) `logs/<id>.log`. The category directory names must appear in `CATEGORIES` in `src/lib.rs` (this list also fixes textbook order and the titles shown in the app).
* **`build.rs` is the registry.** It scans `src/algorithms/*/*.rs`, reads the `//! @title / @chapter / @editions / @summary / @order` header from each file (all five are required, `@editions` is a comma list of 1–4), and generates (a) the module tree via `#[path]` attributes and (b) `registry::ENTRIES`. **Never edit module lists by hand and never add `mod` lines** — dropping a correctly headed file into a category directory is enough. The id is the file stem and must be unique across categories.
* **`src/bin/gen_site_data.rs`** iterates the registry: calls every `demo()` to write `logs/<id>.log` (demo output plus, if given a `cargo test` transcript, per-algorithm test results parsed from `test algorithms::<cat>::<id>::tests::…` lines) and emits `site-data/manifest.json`.
* **`index.html`** fetches `site-data/manifest.json`, `site-data/editions.json`, then per algorithm the `.md`, `.rs` and `.log`; renders markdown with marked, diagrams from ```` ```mermaid ```` blocks with mermaid, highlighting with highlight.js (all from cdnjs). Routing is `#/<id>[?tab=source|log]`.
* **Cross-references between docs** are `[text](#/other_id)` links; `tests/consistency.rs::doc_cross_links_resolve` fails on dangling ids. Algorithm modules also reuse each other through `super::sibling::…` (e.g. graphs use `bellman_ford`, `dijkstra`, `floyd_warshall`; circuits share `Circuit`; `np_completeness` shares `hamiltonian_cycle::traveling_salesman_exact`), so renaming a file means fixing both Rust paths and doc links.

## Conventions that the tooling enforces

* Every algorithm file exposes `pub fn demo() -> String` (build the text with `crate::outln!(buf, …)`) and contains `#[cfg(test)] mod tests` with `#[test]` fns (checked by `every_source_has_tests`). Tests compare against brute force / `std` / the book's worked examples (figure numbers are cited in test names or comments).
* Docs must start with `# Title`, be > 400 chars, and follow the house layout: *What it is · How it works (with a mermaid diagram when useful) · Complexity · When to use it · In this implementation*.
* No external crates. Randomness only via `crate::common::Rng` (seeded xorshift) so demos, logs and tests are reproducible; keep randomised tests statistically safe with fixed seeds.
* Editions metadata reflects where the material appears in the book (e.g. `fibonacci_heap` = `1,2,3`, `suffix_array` = `4`); update `site-data/editions.json` too if a new algorithm is edition-defining.
* `src/lib.rs` allows textbook-style index loops (`clippy::needless_range_loop` etc.); algorithms are written to mirror the book's pseudocode, with CLRS procedure names in comments.
* `scripts/split_bundle.py` is only an authoring helper (splits a file of `=== FILE path ===` sections into files); it is not part of the build.
