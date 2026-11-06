//! @title Merge Sort
//! @chapter 2.3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Divide-and-conquer sort: split, sort halves recursively, merge in linear time.
//! @order 40
use crate::outln;

/// MERGE(A, p, q, r): merges sorted A[p..=q] and A[q+1..=r].
pub fn merge<T: PartialOrd + Clone>(a: &mut [T], p: usize, q: usize, r: usize) {
    let left: Vec<T> = a[p..=q].to_vec();
    let right: Vec<T> = a[q + 1..=r].to_vec();
    let (mut i, mut j) = (0, 0);
    for k in p..=r {
        // taking from the left on ties keeps the sort stable
        if j >= right.len() || (i < left.len() && left[i] <= right[j]) {
            a[k] = left[i].clone();
            i += 1;
        } else {
            a[k] = right[j].clone();
            j += 1;
        }
    }
}

fn merge_sort_range<T: PartialOrd + Clone>(a: &mut [T], p: usize, r: usize) {
    if p < r {
        let q = (p + r) / 2;
        merge_sort_range(a, p, q);
        merge_sort_range(a, q + 1, r);
        merge(a, p, q, r);
    }
}

/// MERGE-SORT(A, 1, n)
pub fn merge_sort<T: PartialOrd + Clone>(a: &mut [T]) {
    if a.len() > 1 {
        let r = a.len() - 1;
        merge_sort_range(a, 0, r);
    }
}

/// Bottom-up (iterative) merge sort — no recursion.
pub fn merge_sort_bottom_up<T: PartialOrd + Clone>(a: &mut [T]) {
    let n = a.len();
    let mut w = 1;
    while w < n {
        let mut p = 0;
        while p + w < n {
            let q = p + w - 1;
            let r = (p + 2 * w - 1).min(n - 1);
            merge(a, p, q, r);
            p += 2 * w;
        }
        w *= 2;
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = vec![38, 27, 43, 3, 9, 82, 10];
    outln!(o, "input         : {:?}", a);
    let mut b = a.clone();
    merge_sort(&mut a);
    outln!(o, "merge_sort    : {:?}", a);
    merge_sort_bottom_up(&mut b);
    outln!(o, "bottom-up     : {:?}", b);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn both_variants_sort() {
        let mut r = Rng::new(3);
        for n in 0..70 {
            let v = r.vec(n, -20, 20);
            let mut e = v.clone();
            e.sort();
            let (mut a, mut b) = (v.clone(), v);
            merge_sort(&mut a);
            merge_sort_bottom_up(&mut b);
            assert_eq!(a, e);
            assert_eq!(b, e);
        }
    }
    #[test]
    fn merge_combines_two_sorted_runs() {
        let mut a = vec![1, 4, 9, 2, 3, 10];
        merge(&mut a, 0, 2, 5);
        assert_eq!(a, vec![1, 2, 3, 4, 9, 10]);
    }
}
