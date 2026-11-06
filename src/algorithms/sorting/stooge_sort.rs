//! @title Stooge Sort
//! @chapter Problem 7-3 (2e–4e)
//! @editions 2,3,4
//! @summary A deliberately slow recursive sort: Θ(n^(lg 3/lg 1.5)) ≈ Θ(n^2.71).
//! @order 90
use crate::outln;

/// STOOGE-SORT(A, i, j) on the whole slice.
pub fn stooge_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    if n < 2 { return; }
    if a[0] > a[n - 1] {
        a.swap(0, n - 1);
    }
    if n >= 3 {
        let k = n / 3; // ⌊n/3⌋ — the book uses ⌈2n/3⌉ as the overlapping size
        let size = n - k;
        stooge_sort(&mut a[..size]);
        stooge_sort(&mut a[k..]);
        stooge_sort(&mut a[..size]);
    }
}

/// Number of recursive calls for input size n (grows as n^2.71).
pub fn calls(n: usize) -> u64 {
    if n < 3 { return 1; }
    1 + 3 * calls(n - n / 3)
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = vec![8, 3, 9, 1, 7, 2, 6];
    stooge_sort(&mut a);
    outln!(o, "sorted: {:?}", a);
    for n in [10, 100, 1000, 10000] {
        outln!(o, "n={:>5}: {:>12} recursive calls (n^2.71 ≈ {:.0})", n, calls(n), (n as f64).powf(2.7095));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn sorts_every_size() {
        let mut r = Rng::new(32);
        for n in 0..45 {
            let mut v = r.vec(n, -10, 10);
            let mut e = v.clone();
            e.sort();
            stooge_sort(&mut v);
            assert_eq!(v, e, "n={n}");
        }
    }
    #[test]
    fn call_count_grows_superquadratically() {
        assert!(calls(2000) as f64 / calls(1000) as f64 > 6.0);
    }
}
