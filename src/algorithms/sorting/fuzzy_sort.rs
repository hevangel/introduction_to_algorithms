//! @title Fuzzy Sorting of Intervals
//! @chapter Problem 7-6 (2e–4e)
//! @editions 2,3,4
//! @summary Order intervals so that points c₁ ≤ … ≤ cₙ with cᵢ ∈ Iᵢ exist: quicksort around the intersection of the pivot's overlapping intervals.
//! @order 200
use crate::common::Rng;
use crate::outln;

pub type Interval = (i64, i64);

/// FUZZY-SORT: returns a permutation of the intervals such that some non-decreasing selection of points exists.
pub fn fuzzy_sort(a: &[Interval], rng: &mut Rng) -> Vec<Interval> {
    if a.len() <= 1 { return a.to_vec(); }
    let pivot = a[rng.range(0, a.len() - 1)];
    // FIND-INTERSECTION: grow the common intersection [lo, hi] over every interval that overlaps it
    let (mut lo, mut hi) = pivot;
    for &(x, y) in a { if x <= hi && y >= lo { lo = lo.max(x); hi = hi.min(y); } }
    let (mut left, mut mid, mut right) = (vec![], vec![], vec![]);
    for &(x, y) in a {
        if y < lo { left.push((x, y)); } else if x > hi { right.push((x, y)); } else { mid.push((x, y)); } // mid intervals all contain [lo, hi]
    }
    let mut out = fuzzy_sort(&left, rng);
    out.extend(mid);
    out.extend(fuzzy_sort(&right, rng));
    out
}

/// Is there a non-decreasing choice of points c_i ∈ I_i? (Greedy: take the smallest feasible point each time.)
pub fn is_fuzzy_sorted(a: &[Interval]) -> bool {
    let mut c = i64::MIN;
    for &(x, y) in a { c = c.max(x); if c > y { return false; } }
    true
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(2);
    let ivs: Vec<Interval> = vec![(8, 10), (1, 3), (4, 6), (2, 5), (9, 12), (5, 7), (0, 1)];
    let s = fuzzy_sort(&ivs, &mut r);
    outln!(o, "intervals : {:?}", ivs);
    outln!(o, "fuzzy sort: {:?}  (valid: {})", s, is_fuzzy_sorted(&s));
    let same: Vec<Interval> = (0..100_000).map(|i| (i % 7, 100 + i % 5)).collect();
    outln!(o, "100000 mutually overlapping intervals: all land in the middle bucket after one pass, valid = {}", is_fuzzy_sorted(&fuzzy_sort(&same, &mut r)));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn output_is_a_permutation_and_fuzzy_sorted() {
        let mut r = Rng::new(310);
        for _ in 0..500 {
            let n = r.range(0, 30);
            let ivs: Vec<Interval> = (0..n).map(|_| { let a = r.range(0, 60) as i64; (a, a + r.range(0, 12) as i64) }).collect();
            let s = fuzzy_sort(&ivs, &mut r);
            assert!(is_fuzzy_sorted(&s), "{ivs:?} -> {s:?}");
            let (mut x, mut y) = (ivs.clone(), s.clone());
            x.sort();
            y.sort();
            assert_eq!(x, y);
        }
    }
    #[test]
    fn detector_and_degenerate_cases() {
        assert!(!is_fuzzy_sorted(&[(5, 6), (1, 2)]));
        assert!(is_fuzzy_sorted(&[(1, 5), (2, 6), (3, 4)]));
        let mut r = Rng::new(1);
        assert!(fuzzy_sort(&[], &mut r).is_empty());
        let all_overlap: Vec<Interval> = (0..1000).map(|i| (i % 10, 50 + i % 3)).collect();
        assert!(is_fuzzy_sorted(&fuzzy_sort(&all_overlap, &mut r)));
    }
}
