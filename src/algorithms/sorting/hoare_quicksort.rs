//! @title Hoare Partition Quicksort
//! @chapter 7.1 (1e); Problem 7-1 (2e–4e)
//! @editions 1,2,3,4
//! @summary The original two-pointer partition scheme: fewer swaps than Lomuto.
//! @order 60
use crate::outln;

/// HOARE-PARTITION: pivot is a[0]. Returns j with a[..=j] ≤ pivot ≤ a[j+1..] and j < len−1.
pub fn hoare_partition<T: PartialOrd + Clone>(a: &mut [T]) -> usize {
    let x = a[0].clone();
    let (mut i, mut j) = (-1isize, a.len() as isize);
    loop {
        loop {
            j -= 1;
            if a[j as usize] <= x { break; }
        }
        loop {
            i += 1;
            if a[i as usize] >= x { break; }
        }
        if i < j {
            a.swap(i as usize, j as usize);
        } else {
            return j as usize;
        }
    }
}

/// QUICKSORT using HOARE-PARTITION (recurse on A[p..q] and A[q+1..r]).
pub fn hoare_quicksort<T: PartialOrd + Clone>(a: &mut [T]) {
    if a.len() > 1 {
        let q = hoare_partition(a);
        let (l, r) = a.split_at_mut(q + 1);
        hoare_quicksort(l);
        hoare_quicksort(r);
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = vec![13, 19, 9, 5, 12, 8, 7, 4, 11, 2, 6, 21];
    outln!(o, "input    : {:?}", a);
    let mut b = a.clone();
    let q = hoare_partition(&mut b);
    outln!(o, "partition: {:?} split after index {} (pivot value 13)", b, q);
    hoare_quicksort(&mut a);
    outln!(o, "sorted   : {:?}", a);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn partition_splits_correctly() {
        let mut r = Rng::new(27);
        for n in 2..50 {
            let mut v = r.vec(n, -10, 10);
            let pivot = v[0];
            let q = hoare_partition(&mut v);
            assert!(q < n - 1);
            let (lmax, rmin) = (v[..=q].iter().max().unwrap(), v[q + 1..].iter().min().unwrap());
            assert!(lmax <= rmin && *lmax >= pivot.min(*lmax));
        }
    }
    #[test]
    fn sorts() {
        let mut r = Rng::new(28);
        for n in 0..80 {
            let mut v = r.vec(n, -5, 5); // many duplicates
            let mut e = v.clone();
            e.sort();
            hoare_quicksort(&mut v);
            assert_eq!(v, e);
        }
    }
}
