//! @title Multithreaded Randomized Select
//! @chapter Problem 27-6 (3e)
//! @editions 3,4
//! @summary RANDOMIZED-SELECT with a parallel partition (threads classify chunks, then the pieces are concatenated): expected work Θ(n) and — with a span-Θ(lg n) partition — expected span Θ(lg² n).
//! @order 42
use crate::common::Rng;
use crate::outln;
use std::thread;

/// Partition `a` around `pivot` into (less, equal, greater) using up to `workers` threads, each classifying one contiguous chunk.
pub fn p_partition(a: &[i64], pivot: i64, workers: usize) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let chunk = a.len().div_ceil(workers.max(1)).max(1);
    let parts: Vec<(Vec<i64>, Vec<i64>, Vec<i64>)> = thread::scope(|s| {
        let handles: Vec<_> = a.chunks(chunk).map(|c| s.spawn(move || {
            let (mut l, mut e, mut g) = (vec![], vec![], vec![]);
            for &x in c { if x < pivot { l.push(x); } else if x == pivot { e.push(x); } else { g.push(x); } }
            (l, e, g)
        })).collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut out = (vec![], vec![], vec![]);
    for (l, e, g) in parts { out.0.extend(l); out.1.extend(e); out.2.extend(g); } // concatenation in chunk order (a prefix sum of the chunk sizes in a real implementation)
    out
}

/// P-RANDOMIZED-SELECT: the i-th smallest (1-based) of `a`; also returns the number of recursion rounds (a proxy for the span in partition steps).
pub fn p_randomized_select(a: &[i64], i: usize, rng: &mut Rng, workers: usize) -> (i64, usize) {
    assert!((1..=a.len()).contains(&i));
    let mut cur = a.to_vec();
    let (mut i, mut rounds) = (i, 0);
    loop {
        rounds += 1;
        if cur.len() == 1 { return (cur[0], rounds); }
        let pivot = cur[rng.range(0, cur.len() - 1)];
        let (less, equal, greater) = p_partition(&cur, pivot, if cur.len() > 2000 { workers } else { 1 });
        if i <= less.len() { cur = less; }
        else if i <= less.len() + equal.len() { return (pivot, rounds); }
        else { i -= less.len() + equal.len(); cur = greater; }
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut rng = Rng::new(6);
    let a = rng.vec(200_000, 0, 1_000_000_000);
    let mut s = a.clone();
    s.sort();
    for i in [1usize, 100, 100_000, 200_000] {
        let (v, rounds) = p_randomized_select(&a, i, &mut rng, 4);
        outln!(o, "i = {i:>7}: value {v} (correct: {}), {rounds} recursion rounds (expected ≈ 2·ln n = {:.0})", v == s[i - 1], 2.0 * (a.len() as f64).ln());
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_sorting_for_any_worker_count_and_duplicates() {
        let mut rng = Rng::new(517);
        for _ in 0..100 {
            let n = rng.range(1, 3000);
            let hi = if rng.range(0, 1) == 0 { 20 } else { 1_000_000 };
            let a = rng.vec(n, 0, hi);
            let mut s = a.clone();
            s.sort();
            let i = rng.range(1, n);
            for workers in [1usize, 3, 8] { assert_eq!(p_randomized_select(&a, i, &mut rng, workers).0, s[i - 1], "n = {n}, i = {i}"); }
        }
    }
    #[test]
    fn partition_pieces_are_correct_and_recursion_depth_is_logarithmic() {
        let mut rng = Rng::new(518);
        let a = rng.vec(10_000, 0, 100);
        let (l, e, g) = p_partition(&a, 50, 5);
        assert_eq!(l.len() + e.len() + g.len(), a.len());
        assert!(l.iter().all(|&x| x < 50) && e.iter().all(|&x| x == 50) && g.iter().all(|&x| x > 50));
        let big = rng.vec(100_000, 0, 1 << 30);
        let (_, rounds) = p_randomized_select(&big, 50_000, &mut rng, 4);
        assert!(rounds < 80, "rounds {rounds}");
    }
}
