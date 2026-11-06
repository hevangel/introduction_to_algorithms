//! @title Second Smallest via a Tournament
//! @chapter Exercise 9.1-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Find the second smallest of n elements with n + ⌈lg n⌉ − 2 comparisons using a knockout tournament.
//! @order 220
use crate::outln;

/// Returns (second smallest, comparisons used). Needs at least two elements.
pub fn second_smallest(a: &[i64]) -> (i64, usize) {
    assert!(a.len() >= 2);
    let mut comps = 0;
    // each round pairs up the survivors; every winner remembers the elements it has beaten directly
    fn tournament(a: &[i64], comps: &mut usize) -> (i64, Vec<i64>) {
        if a.len() == 1 { return (a[0], vec![]); }
        let (l, r) = a.split_at(a.len() / 2);
        let ((lw, mut lbeat), (rw, mut rbeat)) = (tournament(l, comps), tournament(r, comps));
        *comps += 1;
        if lw <= rw { lbeat.push(rw); (lw, lbeat) } else { rbeat.push(lw); (rw, rbeat) }
    }
    let (_, beaten) = tournament(a, &mut comps);
    // the runner-up lost directly to the champion: at most ⌈lg n⌉ candidates
    let mut best = beaten[0];
    for &x in &beaten[1..] { comps += 1; if x < best { best = x; } }
    (best, comps)
}

pub fn demo() -> String {
    let mut o = String::new();
    let v = [27, 4, 91, 15, 8, 63, 2, 40];
    let (s, c) = second_smallest(&v);
    outln!(o, "A = {:?}: second smallest = {s} using {c} comparisons (n + ⌈lg n⌉ − 2 = {})", v, 8 + 3 - 2);
    for n in [10usize, 100, 1000, 100_000] {
        let a: Vec<i64> = (0..n as i64).rev().collect();
        let (s, c) = second_smallest(&a);
        outln!(o, "n = {:>6}: second smallest {s}, comparisons {c} ≤ n + ⌈lg n⌉ − 2 = {}", n, n + (n as f64).log2().ceil() as usize - 2);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn correct_and_within_the_optimal_bound() {
        let mut r = Rng::new(314);
        for n in 2..200usize {
            let v = r.vec(n, -100, 100);
            let mut s = v.clone();
            s.sort();
            let (x, c) = second_smallest(&v);
            assert_eq!(x, s[1], "n={n}");
            assert!(c <= n + (n as f64).log2().ceil() as usize - 2, "n={n} comps={c}");
        }
    }
    #[test]
    fn power_of_two_is_exact() {
        let v: Vec<i64> = (0..16).rev().collect();
        assert_eq!(second_smallest(&v), (1, 16 + 4 - 2));
    }
}
