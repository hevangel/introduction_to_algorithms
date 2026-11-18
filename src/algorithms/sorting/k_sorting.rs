//! @title Average Sorting (k-Sorted Arrays)
//! @chapter Problem 8-5 (1e–4e)
//! @editions 1,2,3,4
//! @summary Permute an array so every k-length window average is nondecreasing in O(n lg(n/k)) time, plus complete sorting of k-sorted arrays in O(n lg k).
//! @order 130
use crate::outln;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Verify the k-sorted property: for all i, a[i] <= a[i + k] (equivalently, every k-window average is nondecreasing).
pub fn is_k_sorted(a: &[i64], k: usize) -> bool {
    if k == 0 {
        return false;
    }
    (0..a.len().saturating_sub(k)).all(|i| a[i] <= a[i + k])
}

/// k-sort `a` in O(n lg(n/k)) time by sorting each residue class modulo k independently.
pub fn k_sort(a: &mut [i64], k: usize) {
    assert!(k >= 1, "k must be at least 1");
    let n = a.len();
    for r in 0..k.min(n) {
        let mut stride: Vec<i64> = (r..n).step_by(k).map(|i| a[i]).collect();
        stride.sort_unstable();
        for (idx, val) in (r..n).step_by(k).zip(stride) {
            a[idx] = val;
        }
    }
}

/// Sort an already k-sorted array in O(n lg k) time using a k-way min-heap merge.
pub fn merge_k_sorted(a: &[i64], k: usize) -> Vec<i64> {
    assert!(k >= 1, "k must be at least 1");
    let n = a.len();
    let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::with_capacity(k.min(n));
    for r in 0..k.min(n) {
        heap.push(Reverse((a[r], r)));
    }
    let mut out = Vec::with_capacity(n);
    while let Some(Reverse((val, idx))) = heap.pop() {
        out.push(val);
        let next = idx + k;
        if next < n {
            heap.push(Reverse((a[next], next)));
        }
    }
    out
}

pub fn demo() -> String {
    let mut o = String::new();
    let orig = vec![9, 3, 7, 1, 8, 2, 6, 4, 5, 0];
    let mut a = orig.clone();
    k_sort(&mut a, 3);
    outln!(o, "original   = {:?}", orig);
    outln!(o, "3-sorted   = {:?} (is_3_sorted = {})", a, is_k_sorted(&a, 3));
    let avgs: Vec<f64> = a.windows(3).map(|w| w.iter().sum::<i64>() as f64 / 3.0).collect();
    outln!(o, "3-window averages = {:?}", avgs.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>());
    outln!(o, "k-way merged      = {:?}", merge_k_sorted(&a, 3));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn k_sort_window_averages_and_merge() {
        let mut r = Rng::new(8501);
        for _ in 0..100 {
            let n = r.range(1, 120);
            let k = r.range(1, 20);
            let orig: Vec<i64> = r.vec(n, -500, 500);
            let mut a = orig.clone();
            k_sort(&mut a, k);
            assert!(is_k_sorted(&a, k));
            if k <= n {
                let sums: Vec<i64> = a.windows(k).map(|w| w.iter().sum()).collect();
                assert!(sums.windows(2).all(|w| w[0] <= w[1]));
            }
            let merged = merge_k_sorted(&a, k);
            let mut expected = orig;
            expected.sort();
            assert_eq!(merged, expected);
        }
    }

    #[test]
    fn k_equal_1_is_full_sort() {
        let mut a = vec![5, 2, 9, 1, 5, 6];
        k_sort(&mut a, 1);
        assert_eq!(a, vec![1, 2, 5, 5, 6, 9]);
        assert!(is_k_sorted(&a, 1));
    }
}
