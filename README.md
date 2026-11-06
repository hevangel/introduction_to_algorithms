# Introduction to Algorithms in Rust

Every algorithm from the **1st (1990), 2nd (2001), 3rd (2009) and 4th (2022) editions** of *Introduction to Algorithms* (CLRS, "the white book"), implemented in dependency-free Rust — **236 algorithm families, ~530 unit tests** — each with a markdown explanation (diagrams in Mermaid), the source code, and a recorded run log. A single-page web app lets you browse and learn them.

```bash
cargo test                      # run all unit tests
./scripts/build_site_data.sh    # (re)generate run logs + manifest for the web app
python3 -m http.server 8000     # then open http://localhost:8000/
```

## Layout

| Path | Contents |
|---|---|
| `src/algorithms/<topic>/<id>.rs` | implementation + tests + `demo()` for one algorithm family |
| `docs/<topic>/<id>.md` | what it is, how it works, complexity, when to use it |
| `logs/<id>.log` | demo output and unit-test results (generated) |
| `site-data/` | `manifest.json` (generated from the Rust registry), `editions.json` (edition comparison) |
| `index.html` | the explorer: loads all of the above at runtime with `fetch` |
| `build.rs` | discovers algorithm files and generates the module tree and registry |
| `tests/consistency.rs` | every algorithm has source, docs, a demo, tests; all doc links resolve |

## What is covered

Foundations (insertion/merge sort, recurrences, Strassen, probabilistic analysis) · Sorting & order statistics (heaps, quicksort variants, linear-time sorts, sorting networks, selection) · Data structures (hashing incl. perfect hashing, BST/red-black/AVL/treap/B-tree, order-statistic & interval trees, binomial/Fibonacci heaps, van Emde Boas, union-find) · Dynamic programming, greedy (Huffman, matroids), amortized analysis · Graphs (BFS/DFS, SCC, MST, shortest paths, all-pairs, max flow incl. push-relabel, matchings, Hungarian, stable marriage) · Multithreaded algorithms · PRAM algorithms and arithmetic circuits (1st edition) · Matrix operations · Simplex · FFT · Number theory (RSA, Miller–Rabin, Pollard rho) · String matching incl. suffix arrays · Computational geometry · NP-completeness reductions · Approximation algorithms · Machine learning and online algorithms (4th edition).

Material that only appears in some editions (e.g. Fibonacci heaps, sorting networks, Boyer–Moore, computational geometry, PRAM, arithmetic circuits for 1st–3rd; k-means, online caching, Hopcroft–Karp, suffix arrays for the 4th) is included and tagged with the editions it belongs to.
