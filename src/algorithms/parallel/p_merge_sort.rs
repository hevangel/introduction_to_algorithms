//! @title Parallel Merge Sort
//! @chapter 27.3 (3e) / 26.3 (4e)
//! @editions 3,4
//! @summary Parallel merge via binary search on the median of the longer array: span Θ(lg³ n), parallelism Θ(n/lg² n).
//! @order 30
use crate::outln;
use std::thread;

const CUTOFF: usize = 2048;

fn serial_merge<T: Ord + Copy>(a: &[T], b: &[T], out: &mut [T]) {
    let (mut i, mut j) = (0, 0);
    for o in out.iter_mut() {
        if j >= b.len() || (i < a.len() && a[i] <= b[j]) { *o = a[i]; i += 1; } else { *o = b[j]; j += 1; }
    }
}

/// P-MERGE(T, p1, r1, p2, r2, A, p3): merge sorted `a` and `b` into `out` (len a + len b) recursively in parallel.
pub fn p_merge<T: Ord + Copy + Send + Sync>(a: &[T], b: &[T], out: &mut [T]) {
    let (a, b) = if a.len() >= b.len() { (a, b) } else { (b, a) }; // a is the longer array
    if a.is_empty() { return; }
    if out.len() <= CUTOFF { return serial_merge(a, b, out); }
    let ma = a.len() / 2;
    let x = a[ma];                               // median of the longer array
    let mb = b.partition_point(|&y| y < x);       // BINARY-SEARCH: first element of b that is ≥ x
    let mid = ma + mb;
    out[mid] = x;
    let (lo, rest) = out.split_at_mut(mid);
    let hi = &mut rest[1..];
    thread::scope(|s| {
        s.spawn(|| p_merge(&a[..ma], &b[..mb], lo));  // spawn
        p_merge(&a[ma + 1..], &b[mb..], hi);
    });                                               // sync
}

/// P-MERGE-SORT(A, p, r)
pub fn p_merge_sort<T: Ord + Copy + Send + Sync>(a: &mut [T]) {
    let n = a.len();
    if n <= CUTOFF {
        crate::algorithms::foundations::merge_sort::merge_sort(a);
        return;
    }
    let mid = n / 2;
    {
        let (l, r) = a.split_at_mut(mid);
        thread::scope(|s| {
            s.spawn(|| p_merge_sort(l));
            p_merge_sort(r);
        });
    }
    let mut tmp = a.to_vec();
    let (l, r) = a.split_at(mid);
    p_merge(l, r, &mut tmp);
    a.copy_from_slice(&tmp);
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = crate::common::Rng::new(77);
    let mut v = r.vec(200_000, -1_000_000, 1_000_000);
    let mut e = v.clone();
    e.sort();
    p_merge_sort(&mut v);
    outln!(o, "sorted 200000 random integers in parallel; equals std sort: {}", v == e);
    let (a, b) = ((0..10).map(|x| x * 2).collect::<Vec<i64>>(), (0..8).map(|x| x * 3).collect::<Vec<i64>>());
    let mut out = vec![0; 18];
    p_merge(&a, &b, &mut out);
    outln!(o, "p_merge of {:?} and {:?} = {:?}", a, b, out);
    outln!(o, "analysis: work Θ(n lg n), span Θ(lg³ n) with parallel merge (vs Θ(n) with a serial MERGE)");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn merge_matches_sorted_concatenation() {
        let mut r = Rng::new(152);
        for (na, nb) in [(0, 0), (0, 5), (7, 0), (3000, 5000), (10_000, 100), (5000, 5000)] {
            let (mut a, mut b) = (r.vec(na, -50, 50), r.vec(nb, -50, 50));
            a.sort();
            b.sort();
            let mut e: Vec<i64> = a.iter().chain(&b).copied().collect();
            e.sort();
            let mut out = vec![0; na + nb];
            p_merge(&a, &b, &mut out);
            assert_eq!(out, e, "{na} {nb}");
        }
    }
    #[test]
    fn sort_matches_std() {
        let mut r = Rng::new(153);
        for n in [0, 1, 100, 2048, 2049, 50_000] {
            let mut v = r.vec(n, -1000, 1000);
            let mut e = v.clone();
            e.sort();
            p_merge_sort(&mut v);
            assert_eq!(v, e);
        }
    }
}
