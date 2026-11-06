//! @title Counting Inversions
//! @chapter Problem 2-4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Count out-of-order pairs in Θ(n lg n) by augmenting merge sort.
//! @order 80
use crate::outln;

/// Brute-force Θ(n²) count of pairs i<j with a[i] > a[j].
pub fn count_inversions_naive<T: PartialOrd>(a: &[T]) -> usize {
    let mut c = 0;
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if a[i] > a[j] {
                c += 1;
            }
        }
    }
    c
}

/// Θ(n lg n) count; sorts a copy while merging.
pub fn count_inversions<T: PartialOrd + Clone>(a: &[T]) -> usize {
    let mut v = a.to_vec();
    let n = v.len();
    sort_count(&mut v, 0, n)
}

fn sort_count<T: PartialOrd + Clone>(a: &mut [T], lo: usize, hi: usize) -> usize {
    if hi - lo < 2 {
        return 0;
    }
    let mid = (lo + hi) / 2;
    let mut inv = sort_count(a, lo, mid) + sort_count(a, mid, hi);
    let left = a[lo..mid].to_vec();
    let right = a[mid..hi].to_vec();
    let (mut i, mut j) = (0, 0);
    for k in lo..hi {
        if j >= right.len() || (i < left.len() && left[i] <= right[j]) {
            a[k] = left[i].clone();
            i += 1;
        } else {
            // right[j] jumps ahead of every remaining left element: each is an inversion
            inv += left.len() - i;
            a[k] = right[j].clone();
            j += 1;
        }
    }
    inv
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = [2, 3, 8, 6, 1];
    outln!(o, "A = {:?}", a);
    outln!(o, "naive inversions     = {}", count_inversions_naive(&a));
    outln!(o, "merge-sort inversions = {}", count_inversions(&a));
    let r: Vec<i32> = (0..10).rev().collect();
    outln!(o, "reverse of 0..10 has {} inversions (= 10·9/2)", count_inversions(&r));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_example() {
        assert_eq!(count_inversions(&[2, 3, 8, 6, 1]), 5);
    }
    #[test]
    fn matches_naive() {
        let mut r = Rng::new(5);
        for n in 0..60 {
            let v = r.vec(n, 0, 15);
            assert_eq!(count_inversions(&v), count_inversions_naive(&v));
        }
    }
    #[test]
    fn extremes() {
        let n = 20;
        assert_eq!(count_inversions(&(0..n).collect::<Vec<_>>()), 0);
        assert_eq!(count_inversions(&(0..n).rev().collect::<Vec<_>>()), (n * (n - 1) / 2) as usize);
    }
}
