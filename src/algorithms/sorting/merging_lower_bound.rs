//! @title Lower Bounds for Merging
//! @chapter Problem 8-6 (1e–4e)
//! @editions 1,2,3,4
//! @summary Merging two sorted lists of n elements needs 2n − 1 comparisons in the worst case; the information-theoretic bound lg C(2n,n) is only 2n − O(lg n), but an adversary argument shows every adjacent pair must be compared.
//! @order 218
use crate::outln;

/// Standard linear merge. Returns the merged list and the list of (value, value) pairs it compared.
pub fn merge_counting(a: &[u32], b: &[u32]) -> (Vec<u32>, Vec<(u32, u32)>) {
    let (mut i, mut j) = (0, 0);
    let (mut out, mut cmps) = (vec![], vec![]);
    while i < a.len() && j < b.len() {
        cmps.push((a[i], b[j]));
        if a[i] <= b[j] { out.push(a[i]); i += 1; } else { out.push(b[j]); j += 1; }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    (out, cmps)
}

/// lg C(2n, n): the number of comparisons any merging algorithm needs just to tell the C(2n, n) possible interleavings apart.
pub fn information_bound(n: usize) -> f64 { (1..=n).map(|i| ((n + i) as f64).log2() - (i as f64).log2()).sum() }

/// The adversary's input: a₁ < b₁ < a₂ < b₂ < …, which forces every adjacent pair to be compared.
pub fn adversary(n: usize) -> (Vec<u32>, Vec<u32>) { ((0..n as u32).map(|i| 2 * i).collect(), (0..n as u32).map(|i| 2 * i + 1).collect()) }

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "   n | lg C(2n,n) | 2n−1 (merge on the adversary input)");
    for n in [1usize, 2, 4, 8, 16, 64, 256] {
        let (a, b) = adversary(n);
        outln!(o, "{n:>4} | {:>10.2} | {}", information_bound(n), merge_counting(&a, &b).1.len());
    }
    outln!(o, "the information bound falls short of 2n−1 by about ½ lg n, but no algorithm can beat 2n−1: each adjacent pair of the interleaving must be compared");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn adversary_forces_every_adjacent_comparison() {
        for n in 1..=20usize {
            let (a, b) = adversary(n);
            let (out, cmps) = merge_counting(&a, &b);
            assert_eq!(out, (0..2 * n as u32).collect::<Vec<_>>());
            assert_eq!(cmps.len(), 2 * n - 1);
            // every pair of values adjacent in the sorted order was compared directly
            for v in 0..2 * n as u32 - 1 { assert!(cmps.iter().any(|&(x, y)| (x, y) == (v, v + 1) || (x, y) == (v + 1, v))); }
        }
    }
    #[test]
    fn never_more_than_2n_minus_1_and_bound_is_close() {
        let mut r = Rng::new(443);
        for _ in 0..200 {
            let (la, lb) = (r.range(1, 30), r.range(1, 30));
            let mut a: Vec<u32> = (0..la).map(|_| r.range(0, 100) as u32).collect();
            let mut b: Vec<u32> = (0..lb).map(|_| r.range(0, 100) as u32).collect();
            a.sort();
            b.sort();
            let (out, cmps) = merge_counting(&a, &b);
            assert!(crate::common::is_sorted(&out) && cmps.len() < la + lb);
        }
        for n in 1..=200usize {
            let lb = information_bound(n);
            assert!(lb <= (2 * n - 1) as f64 + 1e-9 && (2 * n) as f64 - lb <= 0.5 * (n as f64).log2() + 2.0, "n={n} bound {lb}");
        }
    }
}
