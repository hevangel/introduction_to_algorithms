//! @title Parallel Randomized Quicksort
//! @chapter Problem 27-? / 27.3 exercises (3e), 26 (4e)
//! @editions 3,4
//! @summary Spawn the two recursive calls after a serial partition: work Θ(n lg n), span Θ(n) expected for a serial partition — a parallel partition would give Θ(lg² n).
//! @order 40
use crate::common::Rng;
use crate::outln;
use std::thread;

fn partition(a: &mut [i64], rng: &mut Rng) -> usize {
    let p = rng.range(0, a.len() - 1);
    a.swap(p, a.len() - 1);
    let pivot = a[a.len() - 1];
    let mut i = 0;
    for j in 0..a.len() - 1 { if a[j] < pivot { a.swap(i, j); i += 1; } }
    let last = a.len() - 1;
    a.swap(i, last);
    i
}

/// P-QUICKSORT: the two sub-arrays are disjoint (`split_at_mut`), so the recursive calls run in parallel without races.
/// Sub-problems smaller than `cutoff` are sorted serially. Returns the recursion depth reached (a measure of span).
pub fn p_quicksort(a: &mut [i64], seed: u64, cutoff: usize) -> usize {
    if a.len() <= 1 { return 0; }
    if a.len() <= cutoff { a.sort(); return 0; }
    let mut rng = Rng::new(seed);
    let q = partition(a, &mut rng);
    let (left, rest) = a.split_at_mut(q);
    let right = &mut rest[1..];
    let (dl, dr) = thread::scope(|s| {
        let h = s.spawn(|| p_quicksort(left, seed.wrapping_mul(3).wrapping_add(1), cutoff)); // spawn
        let dr = p_quicksort(right, seed.wrapping_mul(5).wrapping_add(7), cutoff);
        (h.join().unwrap(), dr)                                                               // sync
    });
    1 + dl.max(dr)
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(9);
    for n in [1_000usize, 100_000] {
        let mut a = r.vec(n, 0, 1_000_000);
        let mut expect = a.clone();
        expect.sort();
        let depth = p_quicksort(&mut a, 1, 2_000);
        outln!(o, "n = {n:>7}: sorted correctly {}, parallel recursion depth {depth} (≈ 2·lg n = {:.0})", a == expect, 2.0 * (n as f64).log2());
    }
    outln!(o, "with a serial partition the span is Θ(n) (the first partition alone costs n); parallelising the partition gives Θ(lg² n)");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sorts_random_sorted_reversed_and_duplicate_heavy_inputs() {
        let mut r = Rng::new(431);
        for _ in 0..40 {
            let n = r.range(0, 3000);
            let mut a = r.vec(n, 0, 50);
            let mut e = a.clone();
            e.sort();
            p_quicksort(&mut a, 7, 100);
            assert_eq!(a, e);
        }
        let mut s: Vec<i64> = (0..5000).collect();
        p_quicksort(&mut s, 3, 64);
        assert_eq!(s, (0..5000).collect::<Vec<_>>());
        let mut d: Vec<i64> = (0..5000).rev().collect();
        p_quicksort(&mut d, 3, 64);
        assert_eq!(d, (0..5000).collect::<Vec<_>>());
    }
    #[test]
    fn depth_is_logarithmic_with_high_probability() {
        let mut r = Rng::new(432);
        let mut a = r.vec(50_000, 0, 1 << 30);
        let depth = p_quicksort(&mut a, 11, 500);
        assert!(depth < 80, "depth {depth}");
    }
}
