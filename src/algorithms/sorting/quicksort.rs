//! @title Quicksort
//! @chapter 7.1–7.2 (2e–4e; 7.1–7.2 with Hoare in 1e)
//! @editions 1,2,3,4
//! @summary Partition around a pivot (Lomuto scheme) and recurse: Θ(n lg n) expected, Θ(n²) worst case.
//! @order 50
use crate::outln;

/// PARTITION(A, p, r) with pivot A[r] (Lomuto). Returns the pivot's final index in `0..a.len()`.
pub fn partition<T: PartialOrd>(a: &mut [T]) -> usize {
    let r = a.len() - 1;
    let mut i = 0; // a[..i] <= pivot, a[i..j] > pivot
    for j in 0..r {
        if a[j] <= a[r] {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, r);
    i
}

/// QUICKSORT(A, p, r)
pub fn quicksort<T: PartialOrd>(a: &mut [T]) {
    if a.len() > 1 {
        let q = partition(a);
        let (left, right) = a.split_at_mut(q);
        quicksort(left);
        quicksort(&mut right[1..]);
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = vec![2, 8, 7, 1, 3, 5, 6, 4];
    outln!(o, "input            : {:?}", a);
    let mut b = a.clone();
    let q = partition(&mut b);
    outln!(o, "after PARTITION  : {:?} (pivot 4 now at index {})", b, q);
    quicksort(&mut a);
    outln!(o, "quicksort result : {:?}", a);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn partition_invariant() {
        let mut a = vec![2, 8, 7, 1, 3, 5, 6, 4];
        let q = partition(&mut a);
        assert_eq!(q, 3);
        assert!(a[..q].iter().all(|&x| x <= a[q]) && a[q + 1..].iter().all(|&x| x > a[q]));
    }
    #[test]
    fn sorts_random_and_adversarial_inputs() {
        let mut r = Rng::new(26);
        for n in 0..100 {
            let mut v = r.vec(n, -20, 20);
            let mut e = v.clone();
            e.sort();
            quicksort(&mut v);
            assert_eq!(v, e);
        }
        let mut sorted: Vec<i32> = (0..2000).collect();
        quicksort(&mut sorted); // worst case recursion, still correct
        assert!(crate::common::is_sorted(&sorted));
    }
}
