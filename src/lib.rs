//! Every algorithm from the 1st–4th editions of *Introduction to Algorithms* (CLRS).
//!
//! Layout: `src/algorithms/<category>/<id>.rs` holds one algorithm family (code, tests, demo);
//! `docs/<category>/<id>.md` explains it. `build.rs` discovers the files and generates the
//! module tree plus [`registry::ENTRIES`], which drives `gen_site_data` and the web app.
#![allow(dead_code, clippy::needless_range_loop, clippy::too_many_arguments, clippy::type_complexity,
         clippy::many_single_char_names, clippy::collapsible_else_if, clippy::manual_memcpy)]

pub mod common;

/// One catalogued algorithm family.
#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub id: &'static str,
    pub title: &'static str,
    pub category: &'static str,
    pub chapter: &'static str,
    /// Editions (1–4) in which the material appears.
    pub editions: &'static [u8],
    pub summary: &'static str,
    pub order: u32,
    pub source: &'static str,
    pub doc: &'static str,
    pub demo: fn() -> String,
}

/// (id, title, description) in textbook order.
pub const CATEGORIES: &[(&str, &str, &str)] = &[
    ("foundations", "Foundations", "Sorting basics, divide-and-conquer, recurrences and probabilistic analysis (Part I)."),
    ("sorting", "Sorting & Order Statistics", "Heaps, quicksort, linear-time sorts, sorting networks and selection (Part II)."),
    ("data_structures", "Data Structures", "Elementary structures, hashing, trees, B-trees, heaps and disjoint sets (Part III)."),
    ("advanced_design", "Advanced Design & Analysis", "Dynamic programming, greedy algorithms and amortized analysis (Part IV)."),
    ("graphs", "Graph Algorithms", "Traversals, spanning trees, shortest paths, flows and matchings (Part VI)."),
    ("parallel", "Parallel Algorithms", "Multithreaded / task-parallel algorithms (3rd & 4th editions)."),
    ("pram", "PRAM Algorithms", "Algorithms for parallel computers (1st edition)."),
    ("circuits", "Arithmetic Circuits", "Adders, multipliers and clocked circuits (1st edition)."),
    ("linear_algebra", "Matrix Operations", "LUP decomposition, inversion and least squares."),
    ("linear_programming", "Linear Programming", "Simplex method and duality."),
    ("polynomials_fft", "Polynomials & FFT", "Fast Fourier transform and polynomial arithmetic."),
    ("number_theory", "Number-Theoretic Algorithms", "GCD, modular arithmetic, RSA and primality."),
    ("string_matching", "String Matching", "Pattern searching and suffix arrays."),
    ("computational_geometry", "Computational Geometry", "Segments, convex hulls, closest pair (editions 1–3)."),
    ("np_completeness", "NP-Completeness", "Verification, reductions and exact solvers."),
    ("approximation", "Approximation Algorithms", "Provable near-optimal algorithms for hard problems."),
    ("machine_learning", "Machine Learning", "Clustering, weighted majority, gradient descent (4th edition)."),
    ("online", "Online Algorithms", "Competitive analysis, caching, self-organising lists (4th edition)."),
];

pub mod algorithms {
    include!(concat!(env!("OUT_DIR"), "/algorithm_mods.rs"));
}

pub mod registry {
    use super::{algorithms, Entry};
    include!(concat!(env!("OUT_DIR"), "/registry.rs"));

    /// Entries sorted by category (textbook order), then by `order`, then id.
    pub fn sorted() -> Vec<Entry> {
        let mut v: Vec<Entry> = ENTRIES.to_vec();
        let pos = |c: &str| crate::CATEGORIES.iter().position(|x| x.0 == c).unwrap_or(usize::MAX);
        v.sort_by_key(|e| (pos(e.category), e.order, e.id));
        v
    }
}
