# CLRS Edition Coverage Review & Agent Implementation Guide

This review audits [Introduction to Algorithms in Rust](../README.md) (`clrs`) against all four local CLRS textbook transcriptions in [`learning/textbooks/IntroductionToAlgorithms`](../../textbooks/IntroductionToAlgorithms) and turns every verified coverage gap, edition-metadata error, and test bug into an agent-executable implementation plan.

---

## 1. Executive Summary

- **Current baseline:** [Cargo.toml](../Cargo.toml) (`name = "clrs"`, Rust edition `2021`, zero external crates) auto-discovers **244 algorithm families** across all **18 categories** defined in [`src/lib.rs`](../src/lib.rs#L28-L47), paired with markdown guides under `docs/<category>/<id>.md`, generated logs under `logs/<id>.log`, and [site-data/manifest.json](../site-data/manifest.json):
  - `foundations` (22), `sorting` (29), `data_structures` (35), `advanced_design` (32), `graphs` (42), `parallel` (6), `pram` (5), `circuits` (7), `linear_algebra` (5), `linear_programming` (2), `polynomials_fft` (7), `number_theory` (12), `string_matching` (9), `computational_geometry` (9), `np_completeness` (7), `approximation` (9), `machine_learning` (3), `online` (3).
- **Coverage assessment:** Core chapter algorithms across editions `1e–4e` are remarkably well covered. The remaining work falls into three concrete buckets:
  1. **4 missing section-level algorithms (`CLRS-01`–`CLRS-04`):** `1e §30.4` work-efficient randomized list prefix & `1e §30.5` deterministic $O(\lg^* n)$ symmetry breaking / 6-coloring on EREW PRAM; `1e §26.4` closed-semiring general path framework; `1e §31.3` boolean matrix multiplication via Strassen/$\mathbb{Z}_{n+1}$ embedding and Four-Russians bit-packing; and `4e §27.3.2` deterministic `MARKING` and `RANDOMIZED-MARKING` ($O(\lg k)$-competitive online paging).
  2. **22 high-value chapter-end problems & edition extensions (`CLRS-05`–`CLRS-26`):** including 4e linear-time suffix arrays + Burrows–Wheeler Transform (`Problems 32-2, 32-3`), 4e ML algorithms (`Problems 33-1` Newton, `33-2` Hedge, `33-4` SGD), `4e §24-6 / §24-7 / §25-4` flow & matching problems (widest augmenting path, global min-cut, fractional bipartite matching), `4e §26-3..26-5` parallel LU/scan/stencil, `4e §27-1` cow-path online search, `1e–4e Problem 11-4` Carter–Wegman hashing, `2e Problem 19-1` 2-3-4 heaps, `Problem 26-3 / 24-3` max-weight closure project selection, `Problem 28-2` cubic splines, `Problem 29-2/29-3` complementary slackness & branch-and-bound ILP, `Problems 30-3..30-4 / 31-4` multipoint FFT eval & Tonelli–Shanks quadratic residues, `Problem B-3` tree separators, and `Problems D-1/D-2` Vandermonde / $\text{GF}(2)$ matrices.
  3. **Verified metadata & test bugs in existing files (Section 7):** 8 `-?` placeholders in `//! @chapter`, 8 wrong `4e` chapter numbers (`36`/`37` instead of `34`/`35`), swapped `25.2`/`25.3` between `hungarian.rs` and `stable_matching.rs`, stale `@editions` lists (`max_subarray.rs`, `computational_geometry/*`, `hopcroft_karp.rs`, `odd_even_merge_sort.rs`, `pairing_heap.rs`, `hire_assistant.rs`, `random_permutation.rs`, `approximation/*`), and a non-suboptimal adversarial test in `greedy_set_cover.rs`.

---

## 2. Agent Implementation Protocol & Repository Contract

Every module added or modified in `learning/introduction_to_algorithms` is governed by [`build.rs`](../build.rs), [`src/lib.rs`](../src/lib.rs), [`tests/consistency.rs`](../tests/consistency.rs), and [`src/bin/gen_site_data.rs`](../src/bin/gen_site_data.rs) (also summarized in [`CLAUDE.md`](../CLAUDE.md)).

### 2.1 Zero-Dependency Crate & Registry Discovery
- **Zero external crates:** [`Cargo.toml`](../Cargo.toml) has no `[dependencies]`. Use `std` plus [`crate::common`](../src/common.rs) (`Rng`, `outln!`, `is_sorted`) and internal siblings (`super::...` or `crate::algorithms::...`).
- **Automatic discovery ([`build.rs`](../build.rs#L1-L66)):** `build.rs` scans `src/algorithms/<category>/<id>.rs`, validates the 5 required `//! @key value` lines, generates `$OUT_DIR/algorithm_mods.rs` and `$OUT_DIR/registry.rs`, and panics on duplicate `<id>` across any category. **Never add manual `mod` declarations.**
- **Category allowlist:** `<category>` must match one of the 18 IDs in [`src/lib.rs::CATEGORIES`](../src/lib.rs#L28-L47). Loop-style array indexing is intentionally permitted (`#![allow(clippy::needless_range_loop, ...)]` in [`src/lib.rs`](../src/lib.rs#L6-L7)).

### 2.2 Three-Artifact Contract per Algorithm `<id>`

1. **Source module `src/algorithms/<category>/<id>.rs`** (mirror [`src/algorithms/sorting/heapsort.rs`](../src/algorithms/sorting/heapsort.rs)):
   - First 5 lines **must** provide all 5 metadata keys:
     ```rust
     //! @title Human-Readable Algorithm Title
     //! @chapter 27.3 (4e); Problem 16-5 (3e)
     //! @editions 1,2,3,4
     //! @summary Single-sentence description ending with complexity or core guarantee.
     //! @order 120
     ```
     (`@editions` accepts comma-separated integers in `1..=4`; verified by [`tests/consistency.rs`](../tests/consistency.rs#L13).)
   - Expose `pub fn demo() -> String` building output via `crate::outln!(o, ...)` and deterministic `crate::common::Rng::new(<seed>)`. Must return non-empty output (`every_entry_is_complete`).
   - Include `#[cfg(test)] mod tests` with `#[test]` functions (`every_source_has_tests`). Test against brute force, `std` reference implementations, and exact worked book figures.
   - Write original code and explanations; **never reproduce textbook prose, figures, or exercise solutions**.

2. **Documentation guide `docs/<category>/<id>.md`** (mirror [`docs/sorting/heapsort.md`](../docs/sorting/heapsort.md)):
   - Must start with `# <Title>` (`L1`) and exceed **400 bytes** ([`tests/consistency.rs#L16-L17`](../tests/consistency.rs#L16-L17)).
   - Never leave orphan docs in `docs/<category>/` without a matching `.rs` file (`no_orphan_docs`).
   - Follow the house structure: `## What it is`, `## How it works` (with a ```` ```mermaid ```` diagram where visualizable), `## Complexity`, `## When to use it`, and `## In this implementation`.
   - Use `[link text](#/<other_id>)` for cross-references; every target ID must exist in `registry::ENTRIES` (`doc_cross_links_resolve`).

3. **Generated site artifacts (`logs/<id>.log` & `site-data/manifest.json`):**
   - Regenerate after every code, demo, or metadata change. Do not edit `site-data/manifest.json` by hand.
   - Edit [`site-data/editions.json`](../site-data/editions.json) by hand only when an added algorithm represents an edition-defining chapter/section milestone.

### 2.3 Verification & Site Regeneration Workflow

Run from `learning/introduction_to_algorithms`:

```sh
cargo test --lib <id>            # targeted unit tests for new/edited algorithm
cargo test --test consistency    # validates metadata, >=400B H1 docs, non-empty demo(), links, #[test]
cargo test                       # full lib + consistency suite
```

To regenerate `logs/<id>.log` and `site-data/manifest.json`:

- **POSIX shell / Git Bash:**
  ```sh
  ./scripts/build_site_data.sh
  ```
- **Windows PowerShell equivalent** (mirroring [`scripts/build_site_data.sh`](../scripts/build_site_data.sh)):
  ```powershell
  New-Item -ItemType Directory -Force target | Out-Null
  cargo test --release --lib -- --test-threads=1 | Tee-Object -FilePath target/test-output.txt | Select-Object -Last 5
  cargo run --release --quiet --bin gen_site_data -- target/test-output.txt
  ```

---

## 3. Local Textbook Markdown Corpus Audit & Exact Line-Number Index

All four published CLRS editions exist under `learning/textbooks/IntroductionToAlgorithms/` (listed in [`learning/textbooks/README.md`](../../textbooks/README.md)). Always cross-check `3e` and `4e` clean Markdown headings first, then confirm `1e`-only or `2e`-only sections against `CLRS_1st_Ed_1990.md` and `CLRS_2nd_Ed_2001.md`.

| Edition | Local File | Lines | Structure & Verified Caveats |
| --- | --- | ---: | --- |
| **1st (1990)** | [`CLRS_1st_Ed_1990.md`](../../textbooks/IntroductionToAlgorithms/CLRS_1st_Ed_1990.md) | 26,155 | DjVu OCR; `<!-- page N -->` markers (PDF page ≈ book page + 11); 37 chapters (Math Ch 2–6 inline in Part I; unique Ch 29 Circuits & Ch 30 PRAM); noisy mathematical OCR. |
| **2nd (2001)** | [`CLRS_2nd_Ed_2001.md`](../../textbooks/IntroductionToAlgorithms/CLRS_2nd_Ed_2001.md) | 24,423 | Clean PDF text with `Chapter X:` headers; **caveat:** `Part VII` / `Chapter 27` header text appears twice (`L14505–L14800` vs `L14801–L15224`), and **Appendix D (`Matrices`) is omitted** after `Appendix C` (`L23479–L24423`). |
| **3rd (2009)** | [`CLRS_3rd_Ed_2009.md`](../../textbooks/IntroductionToAlgorithms/CLRS_3rd_Ed_2009.md) | 39,783 | Clean `##` Part and `###` Chapter headings (`L337–L39783`); 35 chapters + Appendices A–D + Bibliography (`L36961`) + Index (`L38311`). |
| **4th (2022)** | [`CLRS_4th_Ed_2022.md`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md) | 41,232 | Clean `##` Part, `###` Chapter, `####` Section headings (`L341–L41232`); 35 chapters + Appendices A–D + Bibliography (`L38087`) + Index (`L39899`). |

### Exact Chapter & Appendix Line Boundaries

| Topic | `CLRS_1st_Ed_1990.md` | `CLRS_2nd_Ed_2001.md` | `CLRS_3rd_Ed_2009.md` | `CLRS_4th_Ed_2022.md` |
| --- | --- | --- | --- | --- |
| Front Matter / TOC | `L1–L250` | `L1–L192` | `L1–L336` (TOC `L59–L192`) | `L1–L340` (TOC `L39–L172`) |
| Role of Algorithms | Ch 1 `L251` (`p.1`) | Ch 1 `L219` | Ch 1 `L363` | Ch 1 `L369` |
| Getting Started / Asymptotics | Ch 2 `L639`, Ch 3 `L1201`, Ch 4 `L1627`, Ch 5 `L2223` | Ch 2 `L377`, Ch 3 `L849` | Ch 2 `L569`, Ch 3 `L1285` | Ch 2 `L595`, Ch 3 `L1555` |
| Divide-and-Conquer / Recurrences | *(in Ch 4)* | Ch 4 `L1419` | Ch 4 `L2163` | Ch 4 `L2583` (`§4.7` Akra-Bazzi `L3941`) |
| Probabilistic / Randomized | Ch 6 `L2763` | Ch 5 `L2115` | Ch 5 `L3941` | Ch 5 `L4417` |
| Heapsort | Ch 7 `L3927` | Ch 6 `L2821` | Ch 6 `L5167` | Ch 6 `L5747` |
| Quicksort | Ch 8 `L4245` | Ch 7 `L3199` | Ch 7 `L5667` | Ch 7 `L6275` |
| Sorting in Linear Time | Ch 9 `L4705` | Ch 8 `L3645` | Ch 8 `L6253` | Ch 8 `L6963` |
| Medians & Order Statistics | Ch 10 `L4999` | Ch 9 `L4069` | Ch 9 `L7007` | Ch 9 `L7639` |
| Elementary Data Structures | Ch 11 `L5313` | Ch 10 `L4421` | Ch 10 `L7589` | Ch 10 `L8321` |
| Hash Tables | Ch 12 `L5801` | Ch 11 `L4825` | Ch 11 `L8051` | Ch 11 `L8751` |
| Binary Search Trees | Ch 13 `L6415` | Ch 12 `L5533` | Ch 12 `L9233` | Ch 12 `L10017` |
| Red-Black Trees | Ch 14 `L6861` | Ch 13 `L5995` | Ch 13 `L9971` | Ch 13 `L10519` |
| Augmenting Data Structures | Ch 15 `L7395` | Ch 14 `L6567` | Ch 14 `L10579` | Ch 17 `L13833` |
| Dynamic Programming | Ch 16 `L7747` | Ch 15 `L6937` | Ch 15 `L10967` | Ch 14 `L11085` |
| Greedy Algorithms | Ch 17 `L8407` | Ch 16 `L7779` | Ch 16 `L12193` | Ch 15 `L12317` |
| Amortized Analysis | Ch 18 `L8977` | Ch 17 `L8513` | Ch 17 `L13071` | Ch 16 `L12997` |
| B-Trees | Ch 19 `L9471` | Ch 18 `L9121` | Ch 18 `L13929` | Ch 18 `L14209` |
| Binomial Heaps | Ch 20 `L9831` | Ch 19 `L9483` | *(Prob `19-2` `L14935`)* | *Removed* |
| Fibonacci Heaps | Ch 21 `L10243` | Ch 20 `L9897` | Ch 19 `L14379` | *Removed* |
| van Emde Boas Trees | *None* | *None* | Ch 20 `L15041` | *Removed* |
| Disjoint Sets | Ch 22 `L10825` | Ch 21 `L10303` | Ch 21 `L15673` | Ch 19 `L14703` |
| Elementary Graph Algorithms | Ch 23 `L11473` | Ch 22 `L10879` | Ch 22 `L16349` | Ch 20 `L15413` |
| Minimum Spanning Trees | Ch 24 `L12311` | Ch 23 `L11565` | Ch 23 `L17101` | Ch 21 `L16169` |
| Single-Source Shortest Paths | Ch 25 `L12625` | Ch 24 `L11897` | Ch 24 `L17471` | Ch 22 `L16541` |
| All-Pairs Shortest Paths | Ch 26 `L13519` | Ch 25 `L12789` | Ch 25 `L18575` | Ch 23 `L17697` |
| Maximum Flow | Ch 27 `L14707` | Ch 26 `L13291` | Ch 26 `L19351` | Ch 24 `L18541` |
| Matchings in Bipartite Graphs | *(in Ch 27.3)* | *(in Ch 26.3)* | *(Ch 26.3 & Prob `26-6`)* | Ch 25 `L20007` |
| Sorting Networks / Parallel / Online | Ch 28 `L15833`, Ch 29 `L16265`, Ch 30 `L18021` | Ch 27 `L14543 / L14837` *(Sorting Nets)* | Ch 27 `L21563` *(Multithreaded)* | Ch 26 `L21083` *(Parallel)*, Ch 27 `L22011` *(Online)* |
| Matrix Operations | Ch 31 `L19205` | Ch 28 `L15225` | Ch 28 `L22463` | Ch 28 `L23053` |
| Linear Programming | *None* | Ch 29 `L16397` | Ch 29 `L23713` | Ch 29 `L24549` |
| Polynomials & FFT | Ch 32 `L21099` | Ch 30 `L17621` | Ch 30 `L25923` | Ch 30 `L25503` |
| Number-Theoretic Algorithms | Ch 33 `L21757` | Ch 31 `L18037` | Ch 31 `L27177` | Ch 31 `L26785` |
| String Matching | Ch 34 `L23363` | Ch 32 `L19573` | Ch 32 `L29029` | Ch 32 `L28583` |
| Computational Geometry / ML | Ch 35 `L24179` *(Geom)* | Ch 33 `L20175` *(Geom)* | Ch 33 `L29717` *(Geom)* | Ch 33 `L29687` *(ML)* |
| NP-Completeness | Ch 36 `L24665` | Ch 34 `L20729` | Ch 34 `L30471` | Ch 34 `L31061` |
| Approximation Algorithms | Ch 37 `L25485` | Ch 35 `L21841` | Ch 35 `L31849` | Ch 35 `L32537` |
| Appendices A–D | *(in Ch 2, 3, 5, 6)* | App A `L22575`, App B `L22907`, App C `L23479` *(no D)* | App A `L32953`, App B `L33845`, App C `L34513`, App D `L36447` | App A `L33531`, App B `L34543`, App C `L35245`, App D `L37507` |

---

## 4. Edition-by-Edition Chapter Crosswalk (`1e` vs `2e` vs `3e` vs `4e`)

| Topic Area | 1e (1990) | 2e (2001) | 3e (2009) | 4e (2022) | Repo Category | Existing Modules | Priority Gaps |
| --- | --- | --- | --- | --- | --- | ---: | --- |
| Foundations, Recurrences & Probabilistic Analysis | Ch 1–6 | Ch 1–5 | Ch 1–5 | Ch 1–5 | `foundations` | 22 | Fix `max_subarray`, Ch 5/Ch 6 notes |
| Heaps, Quicksort, Linear Sorts, Selection, Sorting Nets | Ch 7–10, 28 | Ch 6–9, 27 | Ch 6–9 | Ch 6–9 | `sorting` | 29 | `CLRS-05`, `CLRS-06`, `CLRS-10` |
| Stacks, Hash Tables, BST, RBT, Augmenting, B-Trees, Heaps, vEB, DSU | Ch 11–15, 19–22 | Ch 10–14, 18–21 | Ch 10–14, 18–21 | Ch 10–13, 17–19 | `data_structures` | 35 | `CLRS-07`, `CLRS-09`, fix `pairing_heap` |
| Dynamic Programming, Greedy, Amortized | Ch 16–18 | Ch 15–17 | Ch 15–17 | Ch 14–16 | `advanced_design` | 32 | `CLRS-08`, `CLRS-11` |
| Graph Traversals, MST, SSSP, APSP, Max Flow, Matchings | Ch 23–27 | Ch 22–26 | Ch 22–26 | Ch 20–25 | `graphs` | 42 | **`CLRS-02`**, `CLRS-12`–`CLRS-16` |
| Multithreaded / Task-Parallel Algorithms | — | — | Ch 27 | Ch 26 | `parallel` | 6 | `CLRS-17` |
| PRAM Parallel Algorithms | Ch 30 | — | — | — | `pram` | 5 | **`CLRS-01`** |
| Arithmetic Circuits | Ch 29 | — | — | — | `circuits` | 7 | Fix `carry_select_adder` meta |
| Matrix Operations | Ch 31 | Ch 28 | Ch 28 | Ch 28 | `linear_algebra` | 5 | **`CLRS-03`**, `CLRS-19`, `CLRS-26` |
| Linear Programming | — | Ch 29 | Ch 29 | Ch 29 | `linear_programming` | 2 | `CLRS-20` |
| Polynomials & FFT | Ch 32 | Ch 30 | Ch 30 | Ch 30 | `polynomials_fft` | 7 | `CLRS-21` |
| Number-Theoretic Algorithms | Ch 33 | Ch 31 | Ch 31 | Ch 31 | `number_theory` | 12 | `CLRS-21`, fix `perfect_power` |
| String Matching & Suffix Arrays | Ch 34 | Ch 32 | Ch 32 | Ch 32 | `string_matching` | 9 | **`CLRS-22`** (`4e` linear SA + BWT) |
| Computational Geometry | Ch 35 | Ch 33 | Ch 33 | — | `computational_geometry` | 9 | `CLRS-24`, fix `ed: [1,2,3]` |
| NP-Completeness | Ch 36 | Ch 34 | Ch 34 | Ch 34 | `np_completeness` | 7 | `CLRS-11` (`34-4`), fix Ch `36->34` |
| Approximation Algorithms | Ch 37 | Ch 35 | Ch 35 | Ch 35 | `approximation` | 9 | `CLRS-25`, fix Ch `37->35` & `greedy_set_cover` |
| Machine Learning Algorithms | — | — | — | Ch 33 | `machine_learning` | 3 | **`CLRS-23`** (`Newton`, `Hedge`, `SGD`) |
| Online Algorithms | — | *(17-5)* | *(16-5, 17-5)* | Ch 27 | `online` | 3 | **`CLRS-04`** (`Marking`), `CLRS-18` |
| Appendices (Summations, Sets/Trees, Counting, Matrices) | Ch 2,3,5,6 | App A–C | App A–D | App A–D | `graphs` / `linear_algebra` / docs | — | `CLRS-26` (`B-3`, `D-1`, `D-2`) |

---

## 5. Section- and Problem-Level Coverage Audit by Part

- **Part I — Foundations (Ch 1–5 in 2e–4e; Ch 1–6 in 1e):** All core algorithms (`insertion_sort`, `merge_sort`, `strassen`, `master_theorem`, `akra_bazzi`, `hire_assistant`, `random_permutation`, `online_hiring`, `probabilistic_analysis`) and Problems `2-1..2-4`, `4-5`, `4-6`, `5-1`, `5-2` are implemented. `max_subarray` is `3e` only (`§4.1`); `4e` excised it (`CLRS_4th_Ed_2022.md#L261`). `hire_assistant` and `random_permutation` were introduced in `2e` (`CLRS_2nd_Ed_2001.md#L105`), whereas `online_hiring` descends from the Secretary Problem in `1e §6.6` (`L3877`). Non-algorithmic math sections (`Ch 3` asymptotics, recurrence substitution proofs) belong in explanatory docs rather than Rust modules.
- **Part II — Sorting & Order Statistics (Ch 6–9; Ch 7–10 in 1e; Ch 28 in 1e / Ch 27 in 2e):** All core sorts, order-statistics procedures, and 1e/2e sorting networks (`comparison_network`, `bitonic_sorter`, `odd_even_merge_sort`) are covered. Missing chapter problems: **`CLRS-05`** (`8-2` in-place linear-time sorting), **`CLRS-06`** (`8-5` average sorting / $k$-sorted arrays), and **`CLRS-10`** (`2e 27-1` transposition network & `27-3` Beneš/Waksman permutation networks).
- **Part III & Part V — Data Structures & Advanced Data Structures (`data_structures`, 35 modules):** Every section-level data structure from `1e–4e` is implemented (`stack_queue`, `linked_list`, `chained_hash_table`, `open_addressing`, `perfect_hashing`, `binary_search_tree`, `red_black_tree`, `avl_tree`, `treap`, `order_statistic_tree`, `interval_tree`, `b_tree`, `binomial_heap`, `fibonacci_heap`, `van_emde_boas_tree`, `disjoint_set`, `offline_disjoint_set_apps`, `ackermann_alpha`). Missing problem structures: **`CLRS-07`** (`11-4` $k$-universal hashing & Carter–Wegman authentication) and **`CLRS-09`** (`2e 19-1` / `1e 20-1` mergeable 2-3-4 heaps). *(Note: `pairing_heap` is a companion addition not actually present in `3e Problem 19-2`.)*
- **Part IV — Dynamic Programming, Greedy & Amortized Analysis (`advanced_design`, 32 modules):** Covers all `1e–4e` core sections (`rod_cutting`, `assembly_line_scheduling`, `matrix_chain`, `lcs`, `optimal_bst`, `activity_selection`, `fractional_knapsack`, `huffman`, `matroid_greedy`, `binary_counter`, `multipop_stack`, `dynamic_table`), 4e `§15.4` offline caching (via [`online_caching`](../src/algorithms/online/online_caching.rs)), and nearly all chapter-end DP/greedy/amortized problems. Remaining gaps: **`CLRS-08`** (`4e Problem 16-1` binary reflected Gray code) and **`CLRS-11`** (`2e Problems 15-6, 15-7` checkerboard & max-profit scheduling, paired with `Problem 34-4`).
- **Part VI — Graph Algorithms (`graphs`, 42 modules):** All `1e–4e` traversals, MST, shortest paths, max flow (including `1e–3e` push-relabel), and `4e` Ch 25 bipartite matchings (`hopcroft_karp`, `hungarian`, `stable_matching`) are implemented. Missing section & problems: **`CLRS-02`** (`1e §26.4` closed-semiring shortest-path framework), **`CLRS-12`** (`4e Problem 20-5` planar graph orientation), **`CLRS-13`** (`Problem 26-3 / 24-3` max-weight closure consulting / Space Shuttle), **`CLRS-14`** (`4e Problem 24-6` widest augmenting path), **`CLRS-15`** (`4e Problem 24-7` global min-cut), **`CLRS-16`** (`4e Problems 25-3, 25-4, 25-5` fractional matching rounding & dual labels).
- **Part VII — Selected Topics (11 categories):**
  - `pram` (1e Ch 30): **`CLRS-01`** (`§30.4–30.5` work-efficient randomized list prefix & $O(\lg^* n)$ 6-coloring / symmetry breaking).
  - `parallel` (3e Ch 27 / 4e Ch 26): **`CLRS-17`** (`4e Problems 26-3, 26-4, 26-5` parallel LU, `P-SCAN`, 2D stencil).
  - `online` (4e Ch 27): **`CLRS-04`** (`§27.3.2` deterministic `MARKING` & `RANDOMIZED-MARKING`), **`CLRS-18`** (`4e Problem 27-1` cow-path doubling search).
  - `linear_algebra` (1e–4e Ch 28 / 31): **`CLRS-03`** (`1e §31.3` boolean matrix multiplication via Strassen & Four Russians), **`CLRS-19`** (`Problem 28-2` natural cubic splines).
  - `linear_programming` (2e–4e Ch 29): **`CLRS-20`** (`Problems 29-1, 29-2, 29-3` LP feasibility, complementary slackness, branch-and-bound ILP).
  - `polynomials_fft` (Ch 30 / 32) & `number_theory` (Ch 31 / 33): **`CLRS-21`** (`Problems 30-2..30-4` 2D FFT, all-derivatives, multipoint eval; `31-3` Fibonacci methods; `31-4` quadratic residues & Tonelli–Shanks).
  - `string_matching` (Ch 32 / 34): **`CLRS-22`** (`4e Problems 32-2, 32-3` $O(n)$ suffix array construction & Burrows–Wheeler Transform + inverse LF-mapping).
  - `machine_learning` (4e Ch 33): **`CLRS-23`** (`4e Problems 33-1, 33-2, 33-4` Newton's method, Hedge, Stochastic Gradient Descent).
  - `computational_geometry` (1e Ch 35 / 2e–3e Ch 33): **`CLRS-24`** (`Problem 33-4` picking up sticks).
  - `approximation` (Ch 35 / 37): **`CLRS-25`** (`Problems 35-3, 35-4, 35-6, 35-7` weighted set cover, maximal matching, max spanning tree, 0-1 knapsack FPTAS).
- **Part VIII — Appendices (`A–D`):** Mostly pure mathematical reference (`Summations`, `Sets/Relations/Functions`, `Counting/Probability`, `Matrix properties`), except three algorithmic problems: **`CLRS-26`** (`Problem B-3` tree bisection separator; `Problems D-1, D-2` Vandermonde & $\text{GF}(2)$ matrix permutations).

---

## 6. Prioritized Work-Item Backlog & Module Specifications (`CLRS-01` – `CLRS-26`)

Implement items independently in priority order (**`P1`** → **`P2`**). Every card gives exact file paths, `//! @...` header metadata, textbook line citations, core Rust APIs, and `#[test]` invariants. Remember to write the paired `> 400`-byte `docs/<category>/<id>.md` file for every new `.rs` module.

---

### Track A: Missing Textbook Section-Level Algorithms (`P1`)

#### `CLRS-01` [P1] PRAM Work-Efficient List Prefix & $O(\lg^* n)$ Symmetry Breaking
- **Category & Files:** `src/algorithms/pram/symmetry_breaking.rs` + `docs/pram/symmetry_breaking.md`
- **Metadata Header:**
  ```rust
  //! @title PRAM Work-Efficient Prefix & Symmetry Breaking
  //! @chapter 30.4–30.5 (1e)
  //! @editions 1
  //! @summary Deterministic coin-tossing 6-coloring in O(lg* n) rounds and O(lg n) 3-coloring/maximal independent set on a linked list.
  //! @order 60
  ```
- **Textbook Anchor:** [`CLRS_1st_Ed_1990.md#L18447-L19060`](../../textbooks/IntroductionToAlgorithms/CLRS_1st_Ed_1990.md#L18447-L19060) (`§30.4` `Randomized-List-Prefix` & `§30.5` deterministic symmetry breaking).
- **Algorithm & Rust Design:**
  - `pub fn six_color_list(next: &[Option<usize>]) -> (Vec<usize>, usize)`: starting with node index as initial color $c[u] = u$, repeatedly replace each non-root color by the index of the lowest differing bit `2 * k + bit_k(c[u])` against `c[next[u]]`, returning `(colors_0_to_5, rounds)`.
  - `pub fn three_color_from_six(next: &[Option<usize>], mut c: Vec<usize>) -> Vec<usize>`: shift-down + local recolor passes eliminating colors `3, 4, 5` sequentially in $O(1)$ parallel rounds, yielding a valid 3-coloring (and greedy maximal independent set `MIS`!).
  - `pub fn randomized_list_prefix(next: &[Option<usize>], vals: &[i64], seed: u64) -> Vec<i64>`: simulate EREW independent-set thinning via `crate::common::Rng` until $O(n / \lg n)$ elements remain, run `pointer_jumping`, and splice back.
- **Test Invariants:** Assert zero adjacent nodes share a color across 1..500-node lists/forests, `colors.iter().all(|&x| x < 6)` after $\le \lg^* n + 2$ rounds, `< 3` after reduction, and exact prefix sums matching sequential traversal.

#### `CLRS-02` [P1] Closed-Semiring All-Pairs Path Framework
- **Category & Files:** `src/algorithms/graphs/semiring_paths.rs` + `docs/graphs/semiring_paths.md`
- **Metadata Header:**
  ```rust
  //! @title Closed-Semiring All-Pairs Path Problems
  //! @chapter 26.4 (1e)
  //! @editions 1
  //! @summary Unify transitive closure, all-pairs shortest paths, widest bottleneck paths, and highest-reliability paths over an algebraic closed semiring (S, ⊕, ⊙, 0̄, 1̄, *).
  //! @order 155
  ```
- **Textbook Anchor:** [`CLRS_1st_Ed_1990.md#L14357-L14670`](../../textbooks/IntroductionToAlgorithms/CLRS_1st_Ed_1990.md#L14357-L14670) (`§26.4` *A general framework for solving path problems in directed graphs*).
- **Algorithm & Rust Design:**
  - Define `pub trait ClosedSemiring: Clone + PartialEq { fn zero() -> Self; fn one() -> Self; fn add(&self, rhs: &Self) -> Self; fn mul(&self, rhs: &Self) -> Self; fn closure(&self) -> Self; }`.
  - Implement generic `pub fn semiring_apsp<S: ClosedSemiring>(base: &[Vec<S>]) -> Vec<Vec<S>>` mirroring the Kleene/Floyd-Warshall update $m_{ij}^{(k)} = m_{ij}^{(k-1)} \oplus (m_{ik}^{(k-1)} \odot (m_{kk}^{(k-1)})^* \odot m_{kj}^{(k-1)})$.
  - Provide 4 concrete semiring types: `BooleanClosure` (`||, &&`), `TropicalMinPlus` (`min, +`), `BottleneckMaxMin` (`max, min` for widest capacity path), and `ReliabilityMaxMul` (`max, *` for $p \in [0, 1]$ probabilities).
- **Test Invariants:** Assert `TropicalMinPlus` matches `super::floyd_warshall::floyd_warshall` on Figure 25.1 (`1e` Fig 26.4), `BooleanClosure` matches `transitive_closure`, and `BottleneckMaxMin` matches brute-force widest paths on random DAGs/cyclic graphs.

#### `CLRS-03` [P1] Boolean Matrix Multiplication via Ring Embedding & Four Russians
- **Category & Files:** `src/algorithms/linear_algebra/boolean_matrix_multiply.rs` + `docs/linear_algebra/boolean_matrix_multiply.md`
- **Metadata Header:**
  ```rust
  //! @title Boolean Matrix Multiplication
  //! @chapter 31.3 (1e)
  //! @editions 1
  //! @summary Multiply boolean matrices over the OR-AND quasiring by embedding into Z_{n+1} for Strassen's algorithm, plus packed Four-Russians bitset speedup and triangle detection.
  //! @order 25
  ```
- **Textbook Anchor:** [`CLRS_1st_Ed_1990.md#L19683-L19765`](../../textbooks/IntroductionToAlgorithms/CLRS_1st_Ed_1990.md#L19683-L19765) (`§31.3` *Algebraic number systems and boolean matrix multiplication* + `31.3-6` triangle detection).
- **Algorithm & Rust Design:**
  - Contrast why Strassen fails directly over Boolean OR-AND (quasiring has no additive inverses) vs embedding $\{0, 1\} \hookrightarrow \mathbb{Z}$ (computing integer witness count $C_{ij} \in [0..n]$ via `super::super::foundations::strassen` or integer Strassen block product, then thresholding `(C_{ij} > 0)` in $O(n^{\lg 7})$).
  - Implement word-packed / Four-Russians lookup-block multiplication (`pub fn bool_matmul_four_russians`) and `pub fn has_triangle(adj: &[Vec<bool>]) -> Option<(usize, usize, usize)>` (via $A^2 \land A$).
- **Test Invariants:** Verify naive Boolean `O(n^3)`, Strassen-thresholded, and Four-Russians outputs match bit-for-bit across `n = 1..32`; verify `has_triangle` against brute-force triple enumeration.

#### `CLRS-04` [P1] Randomized & Deterministic Marking for Online Caching
- **Category & Files:** Extend `src/algorithms/online/online_caching.rs` (or add `src/algorithms/online/randomized_marking.rs` + `docs/online/randomized_marking.md`)
- **Metadata Header:**
  ```rust
  //! @title Randomized Marking for Online Paging
  //! @chapter 27.3.2 (4e)
  //! @editions 4
  //! @summary Epoch-based MARKING (k-competitive) and RANDOMIZED-MARKING (2 H_k = O(lg k) expected competitive ratio against an oblivious adversary).
  //! @order 35
  ```
- **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L22571-L22918`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L22571-L22918) (`§27.3.2` *Randomized caching algorithms*).
- **Algorithm & Rust Design:**
  - `pub fn marking_misses(k: usize, requests: &[usize]) -> usize`: maintain `(page, marked: bool)` in cache; unmark all entries when a miss occurs with every cached slot already marked (starting a new epoch), then evict an arbitrary unmarked slot.
  - `pub fn randomized_marking_misses(k: usize, requests: &[usize], seed: u64) -> usize`: pick a uniformly random unmarked cached slot via `crate::common::Rng`.
- **Test Invariants:** On the cyclic $(k+1)$-page adversarial sequence where deterministic `LRU` / `FIFO` suffer $k \cdot \text{OPT}$ misses (`ratio = k`), assert average `randomized_marking_misses` over deterministic seeds stays $\le 2 H_k \cdot \text{belady\_misses}(k, \text{seq})$.

---

### Track B: Sorting, Hashing, Data Structures & Amortized Problems (`P2`)

#### `CLRS-05` [P2] Sorting in Place in Linear Time
- **Category & Files:** `src/algorithms/sorting/in_place_linear_sort.rs` + `docs/sorting/in_place_linear_sort.md`
- **Metadata Header:**
  ```rust
  //! @title Sorting in Place in Linear Time
  //! @chapter Problem 8-2 (1e–4e)
  //! @editions 1,2,3,4
  //! @summary Compare the three O(n) constrained-key trade-offs: O(n) worst-case, stability, and O(1) auxiliary space (in-place counting-sort cycle chasing).
  //! @order 125
  ```
- **Textbook Anchor:** [`CLRS_3rd_Ed_2009.md#L6783-L6804`](../../textbooks/IntroductionToAlgorithms/CLRS_3rd_Ed_2009.md#L6783-L6804), [`CLRS_4th_Ed_2022.md#L7417-L7438`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L7417-L7438) (`Problem 8-2`).
- **Rust Design:** Implement all three algorithmic variants for keys $\{0, \dots, k-1\}$ with satellite payloads: (a) stable $O(n)$ (auxiliary array), (b) $O(1)$-space unstable cycle-permuting counting sort for small $k$, and (c) $O(1)$-space bit-partition radix/0-1 sort for `b`-bit keys. Assert stability/in-place invariants against `slice::sort_by_key`.

#### `CLRS-06` [P2] $k$-Sorting / Average Sorting
- **Category & Files:** `src/algorithms/sorting/k_sorting.rs` + `docs/sorting/k_sorting.md`
- **Metadata Header:**
  ```rust
  //! @title Average Sorting (k-Sorted Arrays)
  //! @chapter Problem 8-5 (1e–4e)
  //! @editions 1,2,3,4
  //! @summary Permute an array so every k-length window average is nondecreasing in O(n lg(n/k)) time, plus complete sorting of k-sorted arrays in O(n lg k).
  //! @order 130
  ```
- **Textbook Anchor:** [`CLRS_3rd_Ed_2009.md#L6833-L6868`](../../textbooks/IntroductionToAlgorithms/CLRS_3rd_Ed_2009.md#L6833-L6868), [`CLRS_4th_Ed_2022.md#L7467-L7502`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L7467-L7502) (`Problem 8-5`).
- **Rust Design:** `pub fn k_sort(a: &mut [i64], k: usize)` (partition into `k` strided subsequences mod `k` and sort each in $O((n/k)\lg(n/k))$ total $O(n\lg(n/k))$) and `pub fn merge_k_sorted(a: &[i64], k: usize) -> Vec<i64>` via `k`-way min-heap merge ($O(n\lg k)$). Verify window-average monotonicity `sum(a[i..i+k]) <= sum(a[i+1..i+1+k])` iff $a[i] \le a[i+k]$.

#### `CLRS-07` [P2] $k$-Universal Hashing & Carter–Wegman Authentication
- **Category & Files:** `src/algorithms/data_structures/universal_auth_hash.rs` + `docs/data_structures/universal_auth_hash.md`
- **Metadata Header:**
  ```rust
  //! @title k-Universal Hashing and Carter-Wegman Authentication
  //! @chapter Problem 11-4 (1e–4e)
  //! @editions 1,2,3,4
  //! @summary Construct strongly 2-universal and k-universal degree-(k-1) polynomial hash families over GF(p) and one-time Carter-Wegman message authentication tags.
  //! @order 75
  ```
- **Textbook Anchor:** [`CLRS_3rd_Ed_2009.md#L9171-L9232`](../../textbooks/IntroductionToAlgorithms/CLRS_3rd_Ed_2009.md#L9171-L9232), [`CLRS_4th_Ed_2022.md#L9945-L9968`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L9945-L9968) (`Problem 11-4`).
- **Rust Design:** Evaluate $h_a(x) = (\sum_{j=0}^{k-1} a_j x^j) \bmod p$ with Horner's rule (`u128` modular arithmetic for prime $p$), show pairwise uniformity on small $\mathbb{Z}_p$ (`p = 11` exact $1/p^2$ joint distribution check!) and XOR/OTP-masked multi-message Carter–Wegman tags.

#### `CLRS-08` [P2] Binary Reflected Gray Code (4e Amortized)
- **Category & Files:** `src/algorithms/advanced_design/gray_code.rs` + `docs/advanced_design/gray_code.md`
- **Metadata Header:**
  ```rust
  //! @title Binary Reflected Gray Code
  //! @chapter Problem 16-1 (4e)
  //! @editions 4
  //! @summary Generate all n-bit words changing one bit per step via recursive reflection, G(i) = i ^ (i >> 1), and amortized O(1) bit-position iteration.
  //! @order 245
  ```
- **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L13637-L13658`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L13637-L13658) (`Problem 16-1`).
- **Rust Design:** Compare recursive reflect construction, closed-form `gray_encode(u64)` / `gray_decode(u64)`, and single-bit transition index (`trailing_zeros(i + 1)`), proving total bit flips across $2^n$ steps is $\Theta(2^n)$ ($O(1)$ amortized per step).

#### `CLRS-09` [P2] 2-3-4 Heaps (Mergeable O(lg n) Main-Memory Heaps)
- **Category & Files:** `src/algorithms/data_structures/two_three_four_heap.rs` + `docs/data_structures/two_three_four_heap.md`
- **Metadata Header:**
  ```rust
  //! @title 2-3-4 Heaps
  //! @chapter Problem 20-1 (1e); Problem 19-1 (2e–3e)
  //! @editions 1,2,3
  //! @summary Store keys only in leaves with subtree minima cached in internal 2-3-4 nodes for O(lg n) minimum, insert, decrease-key, extract-min, delete, and meld.
  //! @order 192
  ```
- **Textbook Anchor:** [`CLRS_2nd_Ed_2001.md#L9853-L9870`](../../textbooks/IntroductionToAlgorithms/CLRS_2nd_Ed_2001.md#L9853-L9870), [`CLRS_3rd_Ed_2009.md#L15010-L15030`](../../textbooks/IntroductionToAlgorithms/CLRS_3rd_Ed_2009.md#L15010-L15030) (`Problem 19-1` in `2e`/`3e`).
- **Rust Design:** Maintain leaf-only keys, `small` subtree min in internal nodes (`degree in 2..=4`), uniform leaf depth (`height`), and `union` splicing at matching depth, verifying all 6 operations against a `BTreeMap` model over 2,000 randomized operations.

#### `CLRS-10` [P2] Transposition & Beneš / Waksman Permutation Networks
- **Category & Files:** `src/algorithms/sorting/permutation_network.rs` + `docs/sorting/permutation_network.md`
- **Metadata Header:**
  ```rust
  //! @title Transposition & Permutation Networks
  //! @chapter Problems 28-1 & 28-3 (1e); Problems 27-1 & 27-3 (2e)
  //! @editions 1,2
  //! @summary Odd-even transposition sorting in Θ(n) depth and recursive 2-switch Beneš permutation routing in O(lg n) depth and O(n lg n) switches.
  //! @order 105
  ```
- **Textbook Anchor:** [`CLRS_2nd_Ed_2001.md#L15157-L15224`](../../textbooks/IntroductionToAlgorithms/CLRS_2nd_Ed_2001.md#L15157-L15224) (`Problems 27-1` and `27-3`).
- **Rust Design:** Implement both odd-even transposition brick sort (`O(n)` parallel layers) and recursive bipartite 2-coloring decomposition of any $n = 2^k$ permutation onto a Beneš switch network ($2\lg n - 1$ stages), asserting exact routing of random permutations sampled by `Rng`.

---

### Track C: Dynamic Programming, Graphs, Flows & Matchings (`P1`/`P2`)

#### `CLRS-11` [P2] Checkerboard Path DP & Profit-Deadline Scheduling
- **Category & Files:** `src/algorithms/advanced_design/checkerboard_deadline_dp.rs` + `docs/advanced_design/checkerboard_deadline_dp.md`
- **Metadata Header:**
  ```rust
  //! @title Checkerboard Optimization & Profit-Deadline Scheduling
  //! @chapter Problems 15-6 & 15-7 (2e); Problem 34-4 (1e–4e)
  //! @editions 1,2,3,4
  //! @summary Bottom-up maximum-profit checkerboard traversal and pseudopolynomial O(nD) scheduling with profits, deadlines, and durations (contrasted with NP-completeness).
  //! @order 145
  ```
- **Textbook Anchor:** [`CLRS_2nd_Ed_2001.md#L7747-L7778`](../../textbooks/IntroductionToAlgorithms/CLRS_2nd_Ed_2001.md#L7747-L7778) (`Problems 15-6` checkerboard; `15-7` max-profit scheduling); [`CLRS_4th_Ed_2022.md#L32511-L32536`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L32511-L32536) (`Problem 34-4`).
- **Rust Design:** Solve (a) row-by-row 3-way move checkerboard DP reconstructing optimal path and (b) sorting jobs by deadline $d_i$ then computing `dp[t]` = max profit achievable ending by time $t \le D$, asserting exact agreement with exhaustive $O(2^n)$ subset enumeration on random small instances.

#### `CLRS-12` [P2] Dynamic Planar Graph Orientation (`O(1)` Adjacency Query)
- **Category & Files:** `src/algorithms/graphs/planar_graph_orientation.rs` + `docs/graphs/planar_graph_orientation.md`
- **Metadata Header:**
  ```rust
  //! @title Dynamic Planar Graph Low-Outdegree Orientation
  //! @chapter Problem 20-5 (4e)
  //! @editions 4
  //! @summary Orient edges dynamically during incremental planar vertex insertion so out-degree stays ≤ 5 and adjacency queries run in O(1) time.
  //! @order 55
  ```
- **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L16149-L16168`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L16149-L16168) (`Problem 20-5` *Inserting and querying vertices in planar graphs*).
- **Rust Design:** Support `add_vertex_and_edges(neighbors)` using Euler's $E \le 3V - 6$ property and Chrobak-Eppstein orientation maintenance (degeneracy $\le 5$ / edge reversal invariant or bounded-outdegree reorientation on static/incremental planar graphs), verifying `out_degree(v) <= 5` (static 5-degeneracy orientation) / bounded amortized update and exact symmetric `has_edge(u, v)`.

#### `CLRS-13` [P1] Maximum-Weight Closure / Project Selection via Min-Cut
- **Category & Files:** `src/algorithms/graphs/project_selection_closure.rs` + `docs/graphs/project_selection_closure.md`
- **Metadata Header:**
  ```rust
  //! @title Project Selection & Maximum-Weight Closure
  //! @chapter Problem 26-3 (1e–3e); Problem 24-3 (4e)
  //! @editions 1,2,3,4
  //! @summary Reduce profit maximization with prerequisite costs (algorithmic consulting / Space Shuttle payloads) to s-t minimum cut in O(VE²) or push-relabel time.
  //! @order 195
  ```
- **Textbook Anchor:** [`CLRS_2nd_Ed_2001.md#L14361-L14410`](../../textbooks/IntroductionToAlgorithms/CLRS_2nd_Ed_2001.md#L14361-L14410) (`26-3` Space Shuttle), [`CLRS_3rd_Ed_2009.md#L21381-L21404`](../../textbooks/IntroductionToAlgorithms/CLRS_3rd_Ed_2009.md#L21381-L21404) (`26-3`), [`CLRS_4th_Ed_2022.md#L19811-L19834`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L19811-L19834) (`24-3`).
- **Rust Design:** Build flow network with `s -> job` (capacity = reward $p_i > 0$), `prerequisite -> t` (capacity = cost $c_k$), `job -> prerequisite` (`INF`), reuse `super::ford_fulkerson`, extract the source-side residual closure `S`, and assert `total_profit(S) == sum(p_i) - max_flow` equals brute-force $2^n$ closure enumeration!

#### `CLRS-14` [P2] Maximum Flow by Widest Augmenting Path
- **Category & Files:** `src/algorithms/graphs/widest_augmenting_path.rs` + `docs/graphs/widest_augmenting_path.md`
- **Metadata Header:**
  ```rust
  //! @title Widest Augmenting Path Max Flow
  //! @chapter Problem 24-6 (4e)
  //! @editions 4
  //! @summary Choose the maximum-bottleneck s-t augmenting path per step via modified Dijkstra in O(E lg C) augmentations.
  //! @order 185
  ```
- **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L19873-L19898`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L19873-L19898) (`Problem 24-6`).
- **Rust Design:** Implement max-bottleneck `BinaryHeap` Dijkstra over positive residual capacities (`cap_bot[v] = max(cap_bot[v], min(cap_bot[u], resid[u][v]))`), tracking augmentation count vs Edmonds–Karp and asserting identical `max_flow` to `super::ford_fulkerson`.

#### `CLRS-15` [P1] Undirected Global Minimum Cut (Stoer–Wagner + V-1 Flows)
- **Category & Files:** `src/algorithms/graphs/global_min_cut.rs` + `docs/graphs/global_min_cut.md`
- **Metadata Header:**
  ```rust
  //! @title Undirected Global Minimum Cut
  //! @chapter Problem 24-7 (4e)
  //! @editions 4
  //! @summary Find a minimum-weight nonempty vertex bipartition via both the V−1 s-t max-flow reduction and Stoer–Wagner minimum-cut phase contractions.
  //! @order 190
  ```
- **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L19899-L19928`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L19899-L19928) (`Problem 24-7`).
- **Rust Design:** Implement (1) CLRS's $|V|-1$ fixed-source $s$-$t$ max-flow baseline (`min_{t != 0} max_flow(0, t)`) reusing `super::ford_fulkerson` and (2) `stoer_wagner` maximum adjacency ordering contractions. Verify both agree with exhaustive $2^{n-1}-1$ cut enumeration on random weighted undirected graphs.

#### `CLRS-16` [P2] Fractional Matching Rounding & Optimal Dual Vertex Labels
- **Category & Files:** `src/algorithms/graphs/fractional_bipartite_matching.rs` + `docs/graphs/fractional_bipartite_matching.md`
- **Metadata Header:**
  ```rust
  //! @title Fractional Bipartite Matching & Vertex Labels
  //! @chapter Problems 25-4 & 25-5 (4e)
  //! @editions 4
  //! @summary Round a fractional bipartite matching to an integral matching of ≥ weight in O(E) time via alternating cycles/paths, and recover optimal Egerváry vertex labels from a matching.
  //! @order 225
  ```
- **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L20985-L21044`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L20985-L21044) (`Problems 25-4` & `25-5`).
- **Rust Design:** Iteratively eliminate cycles of strictly fractional edges (`0 < x_e < 1`) shifting odd/even cycle edges by $\min(\varepsilon^+, \varepsilon^-)$ so at least one edge hits `0` or `1` without lowering total weight, plus path rounding for degree-1 endpoints; also reconstruct feasible non-negative dual labels `(u, v)` from an optimal matching via shortest augmenting paths (`Problem 25-5`).

---

### Track D: Parallel, Online, Numerical, FFT & Number Theory (`P2`)

#### `CLRS-17` [P2] Parallel Prefix Scan, 2D Stencil & Parallel LU
- **Category & Files:** `src/algorithms/parallel/p_scan_stencil_lu.rs` + `docs/parallel/p_scan_stencil_lu.md`
- **Metadata Header:**
  ```rust
  //! @title Parallel Scan, 2D Stencil & Parallel LU
  //! @chapter Problems 27-3..27-5 (3e); Problems 26-3..26-5 (4e)
  //! @editions 3,4
  //! @summary Work-efficient two-pass parallel prefix scan, divide-and-conquer quadrant stencil DP, and parallel block LU decomposition.
  //! @order 60
  ```
- **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L21847-L21968`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L21847-L21968) (`Problems 26-3`, `26-4`, `26-5`).
- **Rust Design:** Model deterministic spawn/sync work & critical-path span counters alongside (a) `p_scan_up_down` ($O(n)$ work, $O(\lg n)$ span), (b) quadrant-recursive 2D stencil wavefront (e.g., edit distance / LCS grid), and (c) recursive block LU/triangular solve. Verify identical outputs to serial scan, LCS, and `lup_decomposition`.

#### `CLRS-18` [P2] The Cow-Path Problem (Online Unbounded Search)
- **Category & Files:** `src/algorithms/online/cow_path.rs` + `docs/online/cow_path.md`
- **Metadata Header:**
  ```rust
  //! @title The Cow-Path Problem (Online Search)
  //! @chapter Problem 27-1 (4e)
  //! @editions 4
  //! @summary Doubling spiral search along a 1D line finds a target at unknown signed distance d in ≤ 9|d| total steps, optimal among deterministic algorithms.
  //! @order 40
  ```
- **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L22955-L22964`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L22955-L22964) (`Problem 27-1` + `27-1` extension to arbitrary factor $r > 1$).
- **Rust Design:** Implement `cow_path_search(target: i64, base: f64)` measuring total walk distance vs `|target|`, demonstrating why $r = 2$ achieves $\le 9|d|$ and verifying competitive ratio bounds numerically for $d \in [-10^4, 10^4] \setminus \{0\}$.

#### `CLRS-19` [P2] Natural Cubic Splines via Tridiagonal Solve
- **Category & Files:** `src/algorithms/linear_algebra/cubic_splines.rs` + `docs/linear_algebra/cubic_splines.md`
- **Metadata Header:**
  ```rust
  //! @title Natural Cubic Spline Interpolation
  //! @chapter Problem 28-2 (2e–4e)
  //! @editions 2,3,4
  //! @summary Fit C² piecewise-cubic polynomials through n+1 knots in O(n) time using the Thomas tridiagonal solver on second derivatives.
  //! @order 35
  ```
- **Textbook Anchor:** [`CLRS_2nd_Ed_2001.md#L16339-L16378`](../../textbooks/IntroductionToAlgorithms/CLRS_2nd_Ed_2001.md#L16339-L16378), [`CLRS_4th_Ed_2022.md#L24463-L24470`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L24463-L24470) (`Problem 28-2`).
- **Rust Design:** Assemble the symmetric diagonally dominant tridiagonal system for interior second derivatives $D_i = S''(x_i)$ (`h_i` spacing supported for non-uniform knots too), solve in $O(n)$ via `super::tridiagonal_solver`, and evaluate $S(x)$, $S'(x)$, $S''(x)$. Verify $C^0, C^1, C^2$ continuity at every interior knot and $S''(x_0) = S''(x_n) = 0$.

#### `CLRS-20` [P2] LP Feasibility, Complementary Slackness & Branch-and-Bound ILP
- **Category & Files:** `src/algorithms/linear_programming/ilp_complementary_slackness.rs` + `docs/linear_programming/ilp_complementary_slackness.md`
- **Metadata Header:**
  ```rust
  //! @title LP Feasibility, Complementary Slackness & Integer LP
  //! @chapter Problems 29-1..29-3 (2e–4e)
  //! @editions 2,3,4
  //! @summary Solve Ax ≤ b feasibility via Simplex, verify primal/dual optimality certificates via complementary slackness, and solve small integer LPs with branch-and-bound.
  //! @order 30
  ```
- **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L25407-L25468`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L25407-L25468) (`Problems 29-1, 29-2, 29-3`).
- **Rust Design:** Reuse `super::simplex` to implement (a) `check_lp_feasibility(a, b)` (maximize $0$ subject to $Ax \le b$), (b) `verify_complementary_slackness(a, b, c, x, y)` checking $x_j (c_j - \sum a_{ij} y_i) = 0$ and $y_i (b_i - \sum a_{ij} x_j) = 0$, and (c) LP-relaxation branch-and-bound solver finding integral optima where LP fails (`29-3` duality gap witness).

#### `CLRS-21` [P2] Multidimensional FFT, All-Derivatives, Multipoint Eval & Quadratic Residues
Split cleanly across their two natural categories:
1. **`src/algorithms/polynomials_fft/multipoint_eval_derivatives.rs`** (`docs/polynomials_fft/multipoint_eval_derivatives.md`)
   - **Header:** `@title All Derivatives & Fast Multipoint Evaluation` | `@chapter Problems 30-2..30-4 (4e); Problems 30-3..30-5 (1e–3e)` | `@editions 1,2,3,4`
   - **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L26609-L26744`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L26609-L26744).
   - **Rust Design:** Reuse `super::fft_recursive` and `super::polynomial_multiplication`: (a) 2D FFT via row+column 1D FFT passes, (b) compute $A^{(t)}(x_0)$ for all $t=0..n-1$ in $O(n \lg n)$ via convolution, and (c) subproduct-tree polynomial remainder recursion evaluating $A(x_0), \dots, A(x_{n-1})$ in $O(n \lg^2 n)$ vs Horner brute force.
2. **`src/algorithms/number_theory/quadratic_residues.rs`** (`docs/number_theory/quadratic_residues.md`)
   - **Header:** `@title Quadratic Residues & Tonelli-Shanks` | `@chapter Problem 31-4 (2e–4e); Problem 33-4 (1e)` | `@editions 1,2,3,4`
   - **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L28515-L28520`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L28515-L28520).
   - **Rust Design:** Compute Legendre symbol via Euler's criterion $a^{(p-1)/2} \bmod p$ using `super::modular_exponentiation`, and find $r$ with $r^2 \equiv a \pmod p$ via Tonelli–Shanks, verifying $(p-r)^2 \equiv r^2 \equiv a$ for primes $p \in [3..1000]$.

---

### Track E: 4e String Matching, Machine Learning, Geometry, Approximation & Appendices (`P1`/`P2`)

#### `CLRS-22` [P1] Linear-Time Suffix Array & Burrows–Wheeler Transform (4e Ch 32)
- **Category & Files:** `src/algorithms/string_matching/burrows_wheeler_linear_sa.rs` + `docs/string_matching/burrows_wheeler_linear_sa.md`
- **Metadata Header:**
  ```rust
  //! @title Linear-Time Suffix Array & Burrows-Wheeler Transform
  //! @chapter Problems 32-2 & 32-3 (4e)
  //! @editions 4
  //! @summary O(n) skew (DC3) or SA-IS suffix array construction plus invertible Burrows-Wheeler Transform and O(n) LF-mapping reconstruction.
  //! @order 70
  ```
- **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L29549-L29666`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L29549-L29666) (`Problems 32-2` & `32-3`).
- **Rust Design:** Implement Kärkkäinen–Sanders DC3/Skew recursion ($i \bmod 3 \neq 0$ suffixes, radix-sort triples, recursive solve if duplicates, merge $0 \bmod 3$ with $1,2 \bmod 3$) in $O(n)$, plus `bwt_encode(text)` (last column of sorted rotations) and `bwt_decode(bwt)` via $C$-counts and rank/`LF` inverse walk.
- **Test Invariants:** Assert DC3 matches `super::suffix_array::suffix_array` across random alphabets + all Fibonacci/periodic strings; assert `bwt_decode(&bwt_encode(t)) == t` for edge lengths `0..120`.

#### `CLRS-23` [P1] 4e ML Extensions: Newton's Method, Hedge & Stochastic Gradient Descent
- **Category & Files:** `src/algorithms/machine_learning/newton_hedge_sgd.rs` + `docs/machine_learning/newton_hedge_sgd.md`
- **Metadata Header:**
  ```rust
  //! @title Newton's Method, Hedge & Stochastic Gradient Descent
  //! @chapter Problems 33-1, 33-2 & 33-4 (4e)
  //! @editions 4
  //! @summary Second-order Newton root/minimization convergence, continuous Hedge multiplicative weights over bounded losses, and stochastic mini-batch gradient descent.
  //! @order 40
  ```
- **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L30957-L31044`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L30957-L31044) (`Problems 33-1`, `33-2`, `33-4`).
- **Rust Design:** Implement (a) multi-dimensional / 1D `newtons_method` ($x \leftarrow x - f'(x)/f''(x)$ quadratic convergence vs linear GD), (b) `hedge(losses, epsilon)` updating $w_i \leftarrow w_i (1 - \epsilon)^{\ell_{t,i}}$ verifying the $\le (1+\epsilon)\text{OPT} + \frac{\ln N}{\epsilon}$ regret bound, and (c) `sgd` using deterministic shuffled batches via `Rng` compared against full-batch `super::gradient_descent`.

#### `CLRS-24` [P2] Picking Up Sticks (Sweep-Line Occlusion Topological Sort)
- **Category & Files:** `src/algorithms/computational_geometry/picking_up_sticks.rs` + `docs/computational_geometry/picking_up_sticks.md`
- **Metadata Header:**
  ```rust
  //! @title Picking Up Sticks
  //! @chapter Problem 35-4 (1e); Problem 33-4 (2e–3e)
  //! @editions 1,2,3
  //! @summary Determine in O(n lg n) time whether n overlapping 3D projection sticks can be lifted sequentially without obstruction, and output a valid removal order.
  //! @order 85
  ```
- **Textbook Anchor:** [`CLRS_3rd_Ed_2009.md#L30427-L30436`](../../textbooks/IntroductionToAlgorithms/CLRS_3rd_Ed_2009.md#L30427-L30436) (`Problem 33-4`).
- **Rust Design:** Given line segments whose vertical `(below, above)` constraints form a partial order, build the segment intersection DAG using `super::segment_intersection` (or $O(n \lg n)$ sweep neighbor checks) and extract a topological pick-up order (`None` if cyclic).

#### `CLRS-25` [P2] Approximation Problems: Weighted Set Cover, Maximal Matching, Max Spanning Tree & Knapsack FPTAS
- **Category & Files:** `src/algorithms/approximation/weighted_cover_knapsack_ptas.rs` + `docs/approximation/weighted_cover_knapsack_ptas.md`
- **Metadata Header:**
  ```rust
  //! @title Weighted Set Cover, Maximal Matching & 0-1 Knapsack FPTAS
  //! @chapter Problems 35-3, 35-4, 35-6 & 35-7 (2e–4e)
  //! @editions 1,2,3,4
  //! @summary Classic approximation algorithms from Chapter 35 problems: H(n)-approx weighted greedy set cover, 2-approx maximal matching, 2-approx max spanning tree, and (1−ε) 0-1 knapsack FPTAS.
  //! @order 75
  ```
- **Textbook Anchor:** [`CLRS_3rd_Ed_2009.md#L32773-L32930`](../../textbooks/IntroductionToAlgorithms/CLRS_3rd_Ed_2009.md#L32773-L32930), [`CLRS_4th_Ed_2022.md#L33353-L33468`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L33353-L33468) (`Problems 35-3, 35-4, 35-6, 35-7`).
- **Rust Design:** Implement all four clean short procedures and test (a) weighted cover $\le H(d)\text{OPT}$ vs brute force, (b) arbitrary maximal matching size $\ge \text{OPT}/2$ vs `hopcroft_karp`, (c) Prim/Kruskal max-spanning-tree on complete/general graphs, and (d) scaled-value DP guaranteeing $(1-\varepsilon)\text{OPT} \le \text{FPTAS} \le \text{OPT}$ vs exact `knapsack_01`.

#### `CLRS-26` [P2] Appendix Algorithms: Tree Bisection Separator & Vandermonde / $\text{GF}(2)$ Matrices
Implement in two cleanly separated files matching category boundaries:
1. **`src/algorithms/graphs/tree_bisection.rs`** (`docs/graphs/tree_bisection.md`)
   - **Header:** `@title Tree Bisection (Balanced Separator)` | `@chapter Problem B-3 (1e–4e)` | `@editions 1,2,3,4`
   - **Textbook Anchor:** [`CLRS_3rd_Ed_2009.md#L34491-L34512`](../../textbooks/IntroductionToAlgorithms/CLRS_3rd_Ed_2009.md#L34491-L34512), [`CLRS_4th_Ed_2022.md#L35225-L35244`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L35225-L35244) (`Problem B-3`).
   - **Rust Design:** Compute centroid / balanced edge-removal partition slicing a binary or bounded-degree tree into two sets of size $\le \frac{3}{4} n$ (for binary trees, $\le \frac{2}{3} n$ at a single centroid edge!) in $O(n)$ DFS time.
2. **`src/algorithms/linear_algebra/vandermonde_gf2.rs`** (`docs/linear_algebra/vandermonde_gf2.md`)
   - **Header:** `@title Vandermonde Matrices & GF(2) Linear Permutations` | `@chapter Problems D-1 & D-2 (3e–4e)` | `@editions 3,4`
   - **Textbook Anchor:** [`CLRS_4th_Ed_2022.md#L37937-L38048`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L37937-L38048) (`Problems D-1` & `D-2`).
   - **Rust Design:** Exact rational determinant verifying $\det V = \prod_{0 \le j < k < n}(x_k - x_j)$ plus invertible $b \times b$ $\text{GF}(2)$ matrix bit-permutation maps (including bit-reversal matrix $J$ and Gray-code matrix) verifying nonsingular $M \in \text{GL}_b(\mathbb{F}_2)$ induces a bijection on $\{0..2^b-1\}$!

---

## 7. Immediate Metadata & Bug-Fix Checklist (Existing Files)

Applying the fixes below makes every existing `@chapter`, `@editions`, doc claim, and test assertion accurate against all four local CLRS texts. After editing these files, run `./scripts/build_site_data.sh` (or the PowerShell equivalent in Section 2.3) and `cargo test`.

### Group 1: Resolve All 8 `-?` Chapter Placeholders

| Source File | Current `//! @chapter` (and `@editions`) | Verified Replacement |
| --- | --- | --- |
| [`acyclic_subgraphs.rs`](../src/algorithms/advanced_design/acyclic_subgraphs.rs) | `Problem 17-2 (1e); 16-? (2e)` (`1,2`) | `Problem 17-2 (1e); Problem 16-3 (2e–3e)` (`1,2,3` — `2e L8475`, `3e L13011`) |
| [`nesting_boxes.rs`](../src/algorithms/graphs/nesting_boxes.rs) | `Problem 15-? (1e–2e); 24-2 (3e) / 22-2 (4e)` (`2,3,4`) | `Problem 24-2 (2e–3e) / 22-2 (4e)` (`2e L12655`, `3e L18393`, `4e L17483`) |
| [`min_cost_flow.rs`](../src/algorithms/graphs/min_cost_flow.rs) | `29.2 (3e) / 29 (4e), Problem 29-? ; 26 exercises` (`2,3,4`) | `29.2 (2e–4e); Problem 29-5 (3e–4e)` (`2e L16769`, `3e L24283, L25891`, `4e L24881, L25471`) |
| [`p_quicksort.rs`](../src/algorithms/parallel/p_quicksort.rs) | `Problem 27-? / 27.3 exercises (3e), 26 (4e)` (`3,4`) | `27.3, Problem 27-6 (3e) / 26.3, Problem 26-6 (4e)` (`3e L22429`, `4e L21969`) |
| [`carry_select_adder.rs`](../src/algorithms/circuits/carry_select_adder.rs) | `Problem 29-? / 29.2 (1e)` (`1`) | `29.2 (1e) (companion extension)` (`1e` `29-1` is division circuits `L17933`, not carry-select) |
| [`tridiagonal_solver.rs`](../src/algorithms/linear_algebra/tridiagonal_solver.rs) | `Exercise 28.2-? / 28.3 (tridiagonal LU)` (`2,3,4`) | `Problem 28-1 (2e–4e); Ch 31 exercises (1e)` (`1,2,3,4` — `2e L16325`, `3e L23609`, `4e L24437`, `1e L20985`) |
| [`perfect_power.rs`](../src/algorithms/number_theory/perfect_power.rs) | `Exercise 31.1-? / Problem 31-? (integer powers)` (`1,2,3,4`) | `Exercise 31.1-8 (2e–4e)` (`2,3,4` — `2e L18251`, `3e L27375`, `4e L26995`) |
| [`k_center.rs`](../src/algorithms/approximation/k_center.rs) | `Problem 35-? / Section 35.2 (clustering)` (`3,4`) | `Companion extension (Ch 35; related to 4e §33.1 clustering)` *(neither `3e` nor `4e` has a k-center problem in Ch 35)* |

### Group 2: Fix Wrong `4e` Chapter `36` / `37` Citations (8 Files)

In `1e`, NP-Completeness is Chapter `36` and Approximation is Chapter `37`. In `2e`, `3e`, AND **`4e`**, they are **Chapter `34`** (`L31061`) and **Chapter `35`** (`L32537`). All 8 files below erroneously label `36`/`37` as `(4e)`:

| Source File | Current `//! @chapter` & `@editions` | Corrected `//! @chapter` & `@editions` |
| --- | --- | --- |
| [`np_verification.rs`](../src/algorithms/np_completeness/np_verification.rs) | `34.2 (1e–3e) / 36.2 (4e)` (`1,2,3,4`) | `36.2 (1e) / 34.2 (2e–4e)` (`1,2,3,4`) |
| [`hamiltonian_cycle.rs`](../src/algorithms/np_completeness/hamiltonian_cycle.rs) | `34.2, 34.5.4 (1e–3e) / 36 (4e)` (`1,2,3,4`) | `36.2, 36.5.4 (1e) / 34.2, 34.5.4 (2e–4e)` (`1,2,3,4`) |
| [`cnf_sat.rs`](../src/algorithms/np_completeness/cnf_sat.rs) | `34.4 (1e–3e) / 36.4 (4e)` (`1,2,3,4`) | `36.4, Exercise 36.4-7 (1e) / 34.4 (2e–4e)` (`1,2,3,4`) |
| [`np_reductions.rs`](../src/algorithms/np_completeness/np_reductions.rs) | `34.3, 34.5 (1e–3e) / 36.3, 36.5 (4e)` (`1,2,3,4`) | `36.3, 36.5 (1e) / 34.3, 34.5 (2e–4e)` (`1,2,3,4`) |
| [`subset_sum_np.rs`](../src/algorithms/np_completeness/subset_sum_np.rs) | `34.5.5 (1e–3e) / 36.5.5 (4e); 35.5 (3e)` (`1,2,3,4`) | `36.5.5 (1e) / 34.5.5 (2e–4e)` (`1,2,3,4`) *(exact decision problem; optimization PTAS is 37.4/35.5)* |
| [`approx_vertex_cover.rs`](../src/algorithms/approximation/approx_vertex_cover.rs) | `35.1 (3e) / 37.1 (4e) / 37.1 (2e)` (`2,3,4`) | `37.1 (1e) / 35.1 (2e–4e)` (`1,2,3,4` — `1e L25495`) |
| [`approx_tsp.rs`](../src/algorithms/approximation/approx_tsp.rs) | `35.2 (3e) / 37.2 (4e)` (`2,3,4`) | `37.2 (1e) / 35.2 (2e–4e)` (`1,2,3,4` — `1e L237`) |
| [`randomized_approximation.rs`](../src/algorithms/approximation/randomized_approximation.rs) | `35.4 (3e) / 37.4 (4e); Problem 35-3/37` (`2,3,4`) | `35.4 (2e–4e)` (`2,3,4` — randomized/LP approx arrived in `2e`; `35-3` is weighted set cover) |

### Group 3: Swapped `4e` Sections in Bipartite Matching (2 Files)

| Source File | Current `//! @chapter` | Corrected `//! @chapter` |
| --- | --- | --- |
| [`hungarian.rs`](../src/algorithms/graphs/hungarian.rs) | `25.2 (4e)` | `25.3 (4e)` ([`CLRS_4th_Ed_2022.md#L20483`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L20483)) |
| [`stable_matching.rs`](../src/algorithms/graphs/stable_matching.rs) | `25.3 (4e)` | `25.2 (4e)` ([`CLRS_4th_Ed_2022.md#L20337`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L20337)) |

### Group 4: Other Verified `@editions` / `@chapter` / Doc Mismatches

| Source / Doc File | Observed Bug | Fix |
| --- | --- | --- |
| [`max_subarray.rs`](../src/algorithms/foundations/max_subarray.rs) | Claims `ed: [3,4]` and `Exercise 4.1-5 (4e)` | Change to `@editions 3` and `@chapter 4.1, Exercise 4.1-5 (3e)` (`4e` dropped max subarray [`L261`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L261); `4e` only has `4.1-1..4.1-4` [`L2837–L2847`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L2837-L2847)). |
| [`hopcroft_karp.rs`](../src/algorithms/graphs/hopcroft_karp.rs) | Claims `ed: [4]`, `25.1 (4e)` | Change to `@editions 3,4` and `@chapter 25.1 (4e); Problem 26-6 (3e)` ([`3e L21445`](../../textbooks/IntroductionToAlgorithms/CLRS_3rd_Ed_2009.md#L21445)). |
| [`convex_layers.rs`](../src/algorithms/computational_geometry/convex_layers.rs), [`maximal_layers.rs`](../src/algorithms/computational_geometry/maximal_layers.rs), [`ghostbusters.rs`](../src/algorithms/computational_geometry/ghostbusters.rs), [`point_in_polygon.rs`](../src/algorithms/computational_geometry/point_in_polygon.rs) | Claim `@editions 1,2,3,4` (`1e–4e`) | Remove `4` (`4e` removed Ch 33 Computational Geometry [`L261`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L261)): change to `@editions 2,3` for Problems `33-1, 33-2, 33-4` (`1e` Ch 35 lacked those problem numbers) and `@editions 1,2,3` (`35.1 (1e) / 33.1 (2e–3e)`) for `point_in_polygon.rs`. |
| [`greedy_set_cover.rs`](../src/algorithms/approximation/greedy_set_cover.rs), [`approx_subset_sum.rs`](../src/algorithms/approximation/approx_subset_sum.rs) | Claim `@editions 2,3,4` (`35.3`, `35.5`) | Both exist in `1e` (`§37.3`, `§37.4` [`L237-L245`](../../textbooks/IntroductionToAlgorithms/CLRS_1st_Ed_1990.md#L237-L245)): change to `@editions 1,2,3,4` (`37.3 (1e) / 35.3 (2e–4e)` and `37.4 (1e) / 35.5 (2e–4e)`). |
| [`odd_even_merge_sort.rs`](../src/algorithms/sorting/odd_even_merge_sort.rs) | Cites `27.4 / Problem 27-5 (1e, 2e)` | Update to `Problem 28-2 (1e) / Problem 27-2 (2e)` ([`2e L15175`](../../textbooks/IntroductionToAlgorithms/CLRS_2nd_Ed_2001.md#L15175)); 2e Ch 27 ends at `Problem 27-3`. |
| [`pairing_heap.rs`](../src/algorithms/data_structures/pairing_heap.rs) + [`docs/.../pairing_heap.md`](../docs/data_structures/pairing_heap.md#L4) | Cites `Problem 19-2 (1e–3e)` (`3e` doc claim) | `3e Problem 19-2` is *Binomial trees and binomial heaps* ([`3e L14935`](../../textbooks/IntroductionToAlgorithms/CLRS_3rd_Ed_2009.md#L14935)); update header/doc to clarify pairing heaps are a companion Ch 19 (`20` in `1e/2e`) priority-queue alternative, not a `1e–3e` problem. |
| [`hire_assistant.rs`](../src/algorithms/foundations/hire_assistant.rs), [`random_permutation.rs`](../src/algorithms/foundations/random_permutation.rs), [`online_hiring.rs`](../src/algorithms/foundations/online_hiring.rs) | Cite `5.1–5.4.4 (1e–4e)` (`1,2,3,4`) | Clarify `1e`: Hiring Problem & `PERMUTE` were added in `2e Ch 5` (`hire_assistant`, `random_permutation` → `editions 2,3,4`); `online_hiring` descends from `1e §6.6` Secretary Problem (`L3877`) + `2e–4e §5.4.4`. |
| [`online_caching.rs`](../src/algorithms/online/online_caching.rs) | Cites `27.3 (4e); Problem 16-5 (3e)` | Add `15.4 (4e)` ([`4e L12799`](../../textbooks/IntroductionToAlgorithms/CLRS_4th_Ed_2022.md#L12799)) for Belady's offline `furthest-in-future` cache policy, which is explicitly implemented in `belady_misses` (`L50`). |

### Group 5: Broken Adversarial Test Case in `greedy_set_cover.rs`

- In [`src/algorithms/approximation/greedy_set_cover.rs`](../src/algorithms/approximation/greedy_set_cover.rs#L39-L74), `demo()` states `"greedy takes the 3 big sets while 2 sets suffice"` (`L39`), yet `bad` (`L40, L72`) puts 7 elements `{0..6}` in set 0 and leaves only 3 uncovered elements `{7, 8, 9}`, which `set 1` (`{0,1,2,7,8,9}`) covers in 1 more step — so `greedy_set_cover(10, &bad).unwrap().len() == 2` (`L74`), tying the optimum!
- **Fix:** Replace `bad` with a genuine counterexample where greedy picks strictly more sets than OPT (e.g., for universe `0..14`: optimal disjoint half sets `S0 = 0..7` and `S1 = 7..14` of size 7 each (`OPT = 2`), while greedy picks size-8 `{0..4, 7..11}` first and is forced to take multiple smaller sets afterward to finish `0..14`).

---

## 8. Review Caveats

- **Transcription fidelity:** `CLRS_1st_Ed_1990.md` is a DjVu OCR transcription with symbol and whitespace noise; `CLRS_2nd_Ed_2001.md` duplicates `Chapter 27` (`L14505–L14800` / `L14801–L15224`) and omits `Appendix D`. Line citations for `3e` and `4e` come from clean `##` / `###` / `####` headings.
- **Execution status:** No `cargo test`, `cargo clippy`, or `./scripts/build_site_data.sh` commands were run during this review. The 244-algorithm count, category totals, `@chapter`/`@editions` metadata, and source/doc snippets were audited directly from `site-data/manifest.json`, `site-data/editions.json`, `src/algorithms/`, `docs/`, `build.rs`, `src/lib.rs`, and `tests/consistency.rs`.
