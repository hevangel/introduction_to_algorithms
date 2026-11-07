//! @title Median of Two Sorted Arrays
//! @chapter Exercise 9.3-8 (1e–4e)
//! @editions 1,2,3,4
//! @summary Find the median of 2n elements held in two sorted arrays in O(lg n) by discarding a quarter of the data at each step.
//! @order 225
use crate::outln;

/// The k-th smallest (1-based) element of the union of two sorted slices, in O(lg(min(|a|,|b|)))-ish steps.
pub fn kth_of_two(a: &[i64], b: &[i64], k: usize) -> i64 {
    let (a, b) = if a.len() > b.len() { (b, a) } else { (a, b) }; // a is the shorter
    if a.is_empty() { return b[k - 1]; }
    if k == 1 { return a[0].min(b[0]); }
    let i = (k / 2).min(a.len());
    let j = k - i;
    // if a[i] ≤ b[j], a[1..i] has fewer than k elements below it, so all of it lies among the k smallest: discard it
    if a[i - 1] <= b[j - 1] { kth_of_two(&a[i..], b, k - i) } else { kth_of_two(a, &b[j..], k - j) }
}

/// Lower median (the n-th smallest) of two sorted arrays of equal length n.
pub fn median_of_two(a: &[i64], b: &[i64]) -> i64 {
    assert_eq!(a.len(), b.len());
    kth_of_two(a, b, a.len())
}

pub fn demo() -> String {
    let mut o = String::new();
    let (a, b) = ([1, 4, 7, 10, 13], [2, 3, 8, 9, 20]);
    outln!(o, "A = {:?}, B = {:?}", a, b);
    outln!(o, "median of the 10 values (5th smallest) = {}", median_of_two(&a, &b));
    let big_a: Vec<i64> = (0..1_000_000).map(|x| 2 * x).collect();
    let big_b: Vec<i64> = (0..1_000_000).map(|x| 2 * x + 1).collect();
    outln!(o, "two sorted arrays of 10⁶ elements: median = {} found in about lg n = 20 halving steps", median_of_two(&big_a, &big_b));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_merging_for_every_rank() {
        let mut r = Rng::new(315);
        for _ in 0..300 {
            let (na, nb) = (r.range(0, 20), r.range(1, 20));
            let (mut a, mut b) = (r.vec(na, -30, 30), r.vec(nb, -30, 30));
            a.sort();
            b.sort();
            let mut m: Vec<i64> = a.iter().chain(&b).copied().collect();
            m.sort();
            for k in 1..=m.len() { assert_eq!(kth_of_two(&a, &b, k), m[k - 1], "{a:?} {b:?} k={k}"); }
        }
    }
    #[test]
    fn equal_length_median() {
        let mut r = Rng::new(316);
        for n in 1..60 {
            let (mut a, mut b) = (r.vec(n, 0, 100), r.vec(n, 0, 100));
            a.sort();
            b.sort();
            let mut m: Vec<i64> = a.iter().chain(&b).copied().collect();
            m.sort();
            assert_eq!(median_of_two(&a, &b), m[n - 1]);
        }
    }
}
