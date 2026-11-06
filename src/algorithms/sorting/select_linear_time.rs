//! @title Selection in Worst-Case Linear Time
//! @chapter 9.3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Median-of-medians pivot guarantees a 30/70 split, so SELECT runs in Θ(n) worst case.
//! @order 180
use crate::outln;

/// SELECT(A, i): the i-th smallest (1-based) of `a`, worst-case Θ(n).
pub fn select<T: Ord + Clone>(a: &[T], i: usize) -> T {
    assert!(i >= 1 && i <= a.len());
    if a.len() <= 5 {
        let mut v = a.to_vec();
        v.sort();
        return v[i - 1].clone();
    }
    // 1–2. split into ⌈n/5⌉ groups of ≤ 5 and take each group's median
    let medians: Vec<T> = a
        .chunks(5)
        .map(|g| {
            let mut v = g.to_vec();
            v.sort();
            v[(v.len() - 1) / 2].clone()
        })
        .collect();
    // 3. median of medians becomes the pivot
    let pivot = select(&medians, (medians.len() + 1) / 2);
    // 4. partition into < pivot, = pivot, > pivot
    let lt: Vec<T> = a.iter().filter(|x| **x < pivot).cloned().collect();
    let eq = a.iter().filter(|x| **x == pivot).count();
    if i <= lt.len() {
        select(&lt, i)
    } else if i <= lt.len() + eq {
        pivot
    } else {
        let gt: Vec<T> = a.iter().filter(|x| **x > pivot).cloned().collect();
        select(&gt, i - lt.len() - eq)
    }
}

/// Median (lower median) via SELECT.
pub fn median<T: Ord + Clone>(a: &[T]) -> T {
    select(a, (a.len() + 1) / 2)
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = crate::common::Rng::new(8);
    let v = r.vec(25, 0, 99);
    let mut s = v.clone();
    s.sort();
    outln!(o, "A      = {:?}", v);
    outln!(o, "sorted = {:?}", s);
    for i in [1, 9, 13, 25] {
        outln!(o, "SELECT(A, {:>2}) = {}", i, select(&v, i));
    }
    outln!(o, "median = {}", median(&v));
    outln!(o, "on 100000 sorted keys, median = {}", median(&(0..100_000).collect::<Vec<i32>>()));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_sorting_including_duplicates() {
        let mut r = Rng::new(42);
        for n in [1, 2, 5, 6, 11, 37, 100] {
            let v = r.vec(n, -8, 8);
            let mut s = v.clone();
            s.sort();
            for i in 1..=n {
                assert_eq!(select(&v, i), s[i - 1], "n={n} i={i}");
            }
        }
    }
    #[test]
    fn adversarial_sorted_input_is_fast() {
        let v: Vec<i32> = (0..200_000).collect();
        assert_eq!(select(&v, 100_000), 99_999);
    }
}
