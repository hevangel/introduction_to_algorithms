//! @title Bucket Sort
//! @chapter 8.4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Distribute uniform keys in [0,1) into n buckets, sort each by insertion sort: Θ(n) expected.
//! @order 120
use crate::outln;

/// BUCKET-SORT(A): `a` values must lie in `[0, 1)`.
pub fn bucket_sort(a: &[f64]) -> Vec<f64> {
    let n = a.len();
    if n == 0 { return vec![]; }
    let mut buckets: Vec<Vec<f64>> = vec![vec![]; n];
    for &x in a {
        buckets[(n as f64 * x) as usize].push(x);
    }
    let mut out = Vec::with_capacity(n);
    for b in buckets.iter_mut() {
        // insertion sort each bucket
        for j in 1..b.len() {
            let key = b[j];
            let mut i = j;
            while i > 0 && b[i - 1] > key {
                b[i] = b[i - 1];
                i -= 1;
            }
            b[i] = key;
        }
        out.extend_from_slice(b);
    }
    out
}

/// Size of the largest bucket (a proxy for the extra work done by insertion sort).
pub fn max_bucket(a: &[f64]) -> usize {
    let n = a.len();
    let mut c = vec![0usize; n.max(1)];
    for &x in a { c[(n as f64 * x) as usize] += 1; }
    c.into_iter().max().unwrap_or(0)
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = [0.78, 0.17, 0.39, 0.26, 0.72, 0.94, 0.21, 0.12, 0.23, 0.68];
    outln!(o, "input : {:?}", a);
    outln!(o, "sorted: {:?}", bucket_sort(&a));
    let mut r = crate::common::Rng::new(3);
    let big: Vec<f64> = (0..10000).map(|_| r.f64()).collect();
    outln!(o, "10000 uniform keys: largest bucket holds {} keys (expected small)", max_bucket(&big));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn sorts_uniform_floats() {
        let mut r = Rng::new(35);
        for n in [0, 1, 2, 17, 500] {
            let v: Vec<f64> = (0..n).map(|_| r.f64()).collect();
            let mut e = v.clone();
            e.sort_by(|a, b| a.partial_cmp(b).unwrap());
            assert_eq!(bucket_sort(&v), e);
        }
    }
    #[test]
    fn buckets_are_small_for_uniform_input() {
        let mut r = Rng::new(36);
        let v: Vec<f64> = (0..20000).map(|_| r.f64()).collect();
        assert!(max_bucket(&v) < 12);
    }
}
