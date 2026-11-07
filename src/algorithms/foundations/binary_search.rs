//! @title Binary Search
//! @chapter Exercise 2.3-5 / 2.3-7 (1e–4e)
//! @editions 1,2,3,4
//! @summary Halve a sorted search interval each step: Θ(lg n). Includes the two-sum-in-sorted-order idea.
//! @order 60
use crate::outln;

/// Iterative binary search on a sorted slice; returns an index of `v` if present.
pub fn binary_search<T: Ord>(a: &[T], v: &T) -> Option<usize> {
    let (mut lo, mut hi) = (0usize, a.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match a[mid].cmp(v) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}

/// Recursive version over `a[lo..hi]`.
pub fn binary_search_rec<T: Ord>(a: &[T], v: &T, lo: usize, hi: usize) -> Option<usize> {
    if lo >= hi {
        return None;
    }
    let mid = lo + (hi - lo) / 2;
    if a[mid] == *v {
        Some(mid)
    } else if a[mid] < *v {
        binary_search_rec(a, v, mid + 1, hi)
    } else {
        binary_search_rec(a, v, lo, mid)
    }
}

/// Exercise 2.3-7: do two distinct elements of `s` sum to `x`? Θ(n lg n) — sort, then sweep.
pub fn has_pair_with_sum(s: &[i64], x: i64) -> bool {
    let mut a = s.to_vec();
    a.sort();
    if a.len() < 2 {
        return false;
    }
    let (mut i, mut j) = (0, a.len() - 1);
    while i < j {
        match (a[i] + a[j]).cmp(&x) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j -= 1,
        }
    }
    false
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = [2, 3, 5, 7, 11, 13, 17, 19];
    outln!(o, "A = {:?}", a);
    for v in [11, 4, 19, 2] {
        outln!(o, "search {:>2}: iterative={:?} recursive={:?}", v, binary_search(&a, &v), binary_search_rec(&a, &v, 0, a.len()));
    }
    outln!(o, "pair with sum 24 exists? {}", has_pair_with_sum(&a, 24));
    outln!(o, "pair with sum 100 exists? {}", has_pair_with_sum(&a, 100));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_every_element_and_rejects_absent() {
        let a: Vec<i32> = (0..50).map(|x| x * 2).collect();
        for (i, x) in a.iter().enumerate() {
            assert_eq!(binary_search(&a, x), Some(i));
            assert_eq!(binary_search_rec(&a, x, 0, a.len()), Some(i));
            assert_eq!(binary_search(&a, &(x + 1)), None);
        }
        assert_eq!(binary_search::<i32>(&[], &3), None);
    }
    #[test]
    fn two_sum() {
        assert!(has_pair_with_sum(&[1, 5, 9, 14], 15));
        assert!(!has_pair_with_sum(&[1, 5, 9, 14], 16));
        assert!(!has_pair_with_sum(&[7], 14)); // needs two distinct elements
    }
}
