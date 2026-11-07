//! @title Randomized Select
//! @chapter 9.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Quickselect: find the i-th smallest element in expected Θ(n) time.
//! @order 170
use crate::common::Rng;
use crate::outln;

/// RANDOMIZED-SELECT(A, p, r, i): the i-th smallest (1-based) element of `a`. Reorders `a`.
pub fn randomized_select<T: PartialOrd + Clone>(a: &mut [T], i: usize, rng: &mut Rng) -> T {
    assert!(i >= 1 && i <= a.len());
    if a.len() == 1 { return a[0].clone(); }
    let q = super::randomized_quicksort::randomized_partition(a, rng);
    let k = q + 1; // rank of the pivot within a
    if i == k {
        a[q].clone()
    } else if i < k {
        randomized_select(&mut a[..q], i, rng)
    } else {
        randomized_select(&mut a[q + 1..], i - k, rng)
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(11);
    let v = vec![33, 7, 21, 4, 50, 18, 9, 41, 2, 27];
    let mut sorted = v.clone();
    sorted.sort();
    outln!(o, "A (sorted view) = {:?}", sorted);
    for i in [1, 5, 10] {
        let mut w = v.clone();
        outln!(o, "RANDOMIZED-SELECT i={:>2} -> {}", i, randomized_select(&mut w, i, &mut r));
    }
    let mut w = v.clone();
    outln!(o, "median (i=(n+1)/2 lower) -> {}", randomized_select(&mut w, 5, &mut r));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_sorted_order_for_every_rank() {
        let mut r = Rng::new(41);
        for _ in 0..30 {
            let v = r.vec(40, -25, 25);
            let mut s = v.clone();
            s.sort();
            for i in 1..=v.len() {
                let mut w = v.clone();
                assert_eq!(randomized_select(&mut w, i, &mut r), s[i - 1]);
            }
        }
    }
    #[test]
    fn singleton() {
        let mut r = Rng::new(1);
        assert_eq!(randomized_select(&mut [5], 1, &mut r), 5);
    }
}
