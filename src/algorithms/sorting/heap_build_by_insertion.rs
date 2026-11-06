//! @title Building a Heap by Insertion vs. BUILD-MAX-HEAP
//! @chapter Problem 6-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary BUILD-MAX-HEAP′ inserts elements one by one: Θ(n lg n) worst case, and it can produce a different heap than the Θ(n) bottom-up build.
//! @order 25
use crate::outln;

/// BUILD-MAX-HEAP′: repeatedly MAX-HEAP-INSERT (sift each new element up). Returns the number of swaps.
pub fn build_by_insertion(a: &mut [i64]) -> usize {
    let mut swaps = 0;
    for i in 1..a.len() {
        let mut j = i;
        while j > 0 && a[(j - 1) / 2] < a[j] { a.swap(j, (j - 1) / 2); j = (j - 1) / 2; swaps += 1; }
    }
    swaps
}

/// BUILD-MAX-HEAP (Floyd): sift down from the last internal node. Returns the number of swaps (≤ n).
pub fn build_by_heapify(a: &mut [i64]) -> usize {
    let (n, mut swaps) = (a.len(), 0);
    for start in (0..n / 2).rev() {
        let mut i = start;
        loop {
            let (l, r) = (2 * i + 1, 2 * i + 2);
            let mut m = i;
            if l < n && a[l] > a[m] { m = l; }
            if r < n && a[r] > a[m] { m = r; }
            if m == i { break; }
            a.swap(i, m);
            swaps += 1;
            i = m;
        }
    }
    swaps
}

pub fn is_max_heap(a: &[i64]) -> bool { (1..a.len()).all(|i| a[(i - 1) / 2] >= a[i]) }

pub fn demo() -> String {
    let mut o = String::new();
    let (mut x, mut y) = (vec![1, 2, 3], vec![1, 2, 3]);
    build_by_insertion(&mut x);
    build_by_heapify(&mut y);
    outln!(o, "input [1,2,3]: insertion build → {:?}, bottom-up build → {:?} (different heaps!)", x, y);
    for n in [1usize << 8, 1 << 12, 1 << 16] {
        let asc: Vec<i64> = (0..n as i64).collect();
        let (s1, s2) = (build_by_insertion(&mut asc.clone()), build_by_heapify(&mut asc.clone()));
        outln!(o, "n = {:>6} ascending input: insertion {:>8} swaps (≈ n lg n = {}), bottom-up {:>6} swaps (≤ n)", n, s1, n * n.ilog2() as usize, s2);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn both_build_valid_heaps_but_not_always_the_same() {
        let mut r = Rng::new(309);
        let mut differ = 0;
        for n in 0..100 {
            let v = r.vec(n, -50, 50);
            let (mut a, mut b) = (v.clone(), v);
            build_by_insertion(&mut a);
            build_by_heapify(&mut b);
            assert!(is_max_heap(&a) && is_max_heap(&b));
            if a != b { differ += 1; }
        }
        assert!(differ > 10);
        let (mut x, mut y) = (vec![1, 2, 3], vec![1, 2, 3]);
        build_by_insertion(&mut x);
        build_by_heapify(&mut y);
        assert_eq!((x, y), (vec![3, 1, 2], vec![3, 2, 1]));
    }
    #[test]
    fn worst_case_gap() {
        let n = 1 << 12;
        let asc: Vec<i64> = (0..n).collect();
        let (s1, s2) = (build_by_insertion(&mut asc.clone()), build_by_heapify(&mut asc.clone()));
        assert!(s1 > 3 * s2 && s2 <= n as usize, "{s1} {s2}");
    }
}
