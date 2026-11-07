//! @title Average Sorting (k-Sorted Arrays)
//! @chapter Problem 8-5 (3e) / 8-5 (4e)
//! @editions 3,4
//! @summary An array is k-sorted if A[i] ≤ A[i+k] for all i: sort each residue class in O(n lg(n/k)); fully sort a k-sorted array in O(n lg k) with a k-way merge, which is optimal.
//! @order 216
use crate::outln;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Is the array k-sorted, i.e. does the average of every window of k consecutive elements not exceed that of the next window (⇔ A[i] ≤ A[i+k])?
pub fn is_k_sorted(a: &[i64], k: usize) -> bool { (0..a.len().saturating_sub(k)).all(|i| a[i] <= a[i + k]) }

/// Problem 8-5(d): k-sort an array in O(n lg(n/k)) by sorting each of the k subsequences A[r], A[r+k], A[r+2k], …
pub fn k_sort(a: &mut [i64], k: usize) {
    for r in 0..k.min(a.len()) {
        let mut cls: Vec<i64> = a.iter().skip(r).step_by(k).copied().collect();
        cls.sort();
        for (slot, v) in a.iter_mut().skip(r).step_by(k).zip(cls) { *slot = v; }
    }
}

/// Problem 8-5(e): fully sort an array that is already k-sorted in O(n lg k) comparisons: a k-way merge of its k sorted residue classes with a heap of size k.
/// Returns the sorted array and the number of heap comparisons-by-pop (a proxy for comparisons).
pub fn sort_k_sorted(a: &[i64], k: usize) -> Vec<i64> {
    assert!(is_k_sorted(a, k));
    let mut heap: BinaryHeap<Reverse<(i64, usize)>> = a.iter().take(k).enumerate().map(|(i, &v)| Reverse((v, i))).collect();
    let mut out = Vec::with_capacity(a.len());
    while let Some(Reverse((v, i))) = heap.pop() {
        out.push(v);
        if i + k < a.len() { heap.push(Reverse((a[i + k], i + k))); } // the next element of the same residue class
    }
    out
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = vec![9i64, 3, 7, 1, 8, 2, 6, 4, 5, 0];
    outln!(o, "array {:?}, 3-sorted? {}", a, is_k_sorted(&a, 3));
    k_sort(&mut a, 3);
    outln!(o, "after 3-sorting: {:?}, 3-sorted? {}", a, is_k_sorted(&a, 3));
    outln!(o, "then fully sorted by a 3-way merge: {:?}", sort_k_sorted(&a, 3));
    outln!(o, "a 1-sorted array is just a sorted array; a k-sorted array with k ≥ n is any array");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn k_sort_produces_a_k_sorted_permutation() {
        let mut r = Rng::new(450);
        for _ in 0..200 {
            let n = r.range(0, 60);
            let k = r.range(1, 12);
            let orig = r.vec(n, 0, 30);
            let mut a = orig.clone();
            k_sort(&mut a, k);
            assert!(is_k_sorted(&a, k));
            let (mut x, mut y) = (a.clone(), orig.clone());
            x.sort();
            y.sort();
            assert_eq!(x, y);
        }
    }
    #[test]
    fn merging_a_k_sorted_array_sorts_it() {
        let mut r = Rng::new(451);
        for _ in 0..200 {
            let (n, k) = (r.range(0, 80), r.range(1, 10));
            let mut a = r.vec(n, 0, 50);
            k_sort(&mut a, k);
            let mut e = a.clone();
            e.sort();
            assert_eq!(sort_k_sorted(&a, k), e);
        }
        assert!(is_k_sorted(&[1, 5, 2, 6, 3, 7], 2) && !is_k_sorted(&[3, 1, 2], 1));
    }
}
