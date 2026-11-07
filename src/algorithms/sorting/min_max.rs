//! @title Simultaneous Minimum and Maximum
//! @chapter 9.1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Find both extremes with at most 3⌊n/2⌋ comparisons by pairing elements.
//! @order 160
use crate::outln;

/// MINIMUM(A) — n−1 comparisons, which is optimal.
pub fn minimum<T: PartialOrd + Copy>(a: &[T]) -> Option<T> {
    let mut it = a.iter();
    let mut m = *it.next()?;
    for &x in it {
        if x < m { m = x; }
    }
    Some(m)
}

/// Returns (min, max, comparisons) using the pairing trick: ≤ 3⌊n/2⌋ comparisons.
pub fn min_and_max<T: PartialOrd + Copy>(a: &[T]) -> Option<(T, T, usize)> {
    if a.is_empty() { return None; }
    let mut cmp = 0;
    let (mut mn, mut mx, start);
    if a.len() % 2 == 1 {
        mn = a[0];
        mx = a[0];
        start = 1;
    } else {
        cmp += 1;
        if a[0] < a[1] { mn = a[0]; mx = a[1]; } else { mn = a[1]; mx = a[0]; }
        start = 2;
    }
    let mut i = start;
    while i + 1 < a.len() {
        let (small, large);
        cmp += 1;
        if a[i] < a[i + 1] { small = a[i]; large = a[i + 1]; } else { small = a[i + 1]; large = a[i]; }
        cmp += 2;
        if small < mn { mn = small; }
        if large > mx { mx = large; }
        i += 2;
    }
    Some((mn, mx, cmp))
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = [3, 8, 1, 9, 4, 7, 2, 6, 5];
    let (mn, mx, c) = min_and_max(&a).unwrap();
    outln!(o, "A = {:?}", a);
    outln!(o, "min={mn} max={mx} using {c} comparisons (naive: {}, bound 3⌊n/2⌋ = {})", 2 * (a.len() - 1), 3 * (a.len() / 2));
    let b: Vec<i32> = (0..1000).rev().collect();
    let (_, _, c) = min_and_max(&b).unwrap();
    outln!(o, "n=1000: {c} comparisons (naive {}, bound {})", 2 * 999, 1500);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn correct_and_within_bound() {
        let mut r = Rng::new(40);
        for n in 1..100 {
            let v = r.vec(n, -50, 50);
            let (mn, mx, c) = min_and_max(&v).unwrap();
            assert_eq!(mn, *v.iter().min().unwrap());
            assert_eq!(mx, *v.iter().max().unwrap());
            let bound = if n % 2 == 1 { 3 * (n / 2) } else { 3 * n / 2 - 2 };
            assert!(c <= bound.max(0) + if n == 1 { 0 } else { 0 }, "n={n} c={c} bound={bound}");
            assert_eq!(minimum(&v), Some(mn));
        }
        assert!(min_and_max::<i32>(&[]).is_none());
    }
}
