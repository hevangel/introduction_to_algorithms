//! @title Randomized Quicksort
//! @chapter 7.3–7.4 (2e–4e; 1e had a variant in 8.x)
//! @editions 1,2,3,4
//! @summary Pick the pivot uniformly at random: expected Θ(n lg n) on every input.
//! @order 70
use crate::common::Rng;
use crate::outln;

/// RANDOMIZED-PARTITION: swap a random element into the pivot slot, then PARTITION.
pub fn randomized_partition<T: PartialOrd>(a: &mut [T], rng: &mut Rng) -> usize {
    let r = a.len() - 1;
    let i = rng.range(0, r);
    a.swap(i, r);
    super::quicksort::partition(a)
}

/// RANDOMIZED-QUICKSORT
pub fn randomized_quicksort<T: PartialOrd>(a: &mut [T], rng: &mut Rng) {
    if a.len() > 1 {
        let q = randomized_partition(a, rng);
        let (l, r) = a.split_at_mut(q);
        randomized_quicksort(l, rng);
        randomized_quicksort(&mut r[1..], rng);
    }
}

/// Count comparisons made by randomized quicksort (for the expected-cost experiment).
pub fn comparisons(n: usize, rng: &mut Rng, sorted_input: bool) -> usize {
    fn go(a: &mut [usize], rng: &mut Rng, c: &mut usize) {
        if a.len() > 1 {
            let r = a.len() - 1;
            let i = rng.range(0, r);
            a.swap(i, r);
            let mut k = 0;
            for j in 0..r {
                *c += 1;
                if a[j] <= a[r] { a.swap(k, j); k += 1; }
            }
            a.swap(k, r);
            let (l, rr) = a.split_at_mut(k);
            go(l, rng, c);
            go(&mut rr[1..], rng, c);
        }
    }
    let mut v: Vec<usize> = (0..n).collect();
    if !sorted_input { rng.shuffle(&mut v); }
    let mut c = 0;
    go(&mut v, rng, &mut c);
    c
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(5);
    let mut a = vec![9, 4, 7, 1, 8, 2, 6, 3, 5];
    randomized_quicksort(&mut a, &mut r);
    outln!(o, "sorted: {:?}", a);
    for n in [100usize, 1000, 10000] {
        let c = comparisons(n, &mut r, true);
        outln!(o, "n={:>5}: comparisons on already-sorted input = {:>7}  (2n ln n ≈ {:.0}, n²/2 = {})", n, c, 2.0 * n as f64 * (n as f64).ln(), n * n / 2);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sorts() {
        let mut r = Rng::new(29);
        for n in 0..80 {
            let mut v = r.vec(n, -30, 30);
            let mut e = v.clone();
            e.sort();
            randomized_quicksort(&mut v, &mut r);
            assert_eq!(v, e);
        }
    }
    #[test]
    fn sorted_input_is_not_quadratic() {
        let mut r = Rng::new(30);
        let n = 5000;
        let c = comparisons(n, &mut r, true);
        assert!((c as f64) < 4.0 * n as f64 * (n as f64).ln(), "{c}");
    }
}
