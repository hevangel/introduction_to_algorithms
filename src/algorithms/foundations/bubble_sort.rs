//! @title Bubble Sort
//! @chapter Problem 2-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Swap adjacent out-of-order pairs; simple, correct, and Θ(n²).
//! @order 50
use crate::outln;

/// BUBBLESORT(A) from Problem 2-2.
pub fn bubble_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in 0..n {
        for j in (i + 1..n).rev() {
            if a[j] < a[j - 1] {
                a.swap(j, j - 1);
            }
        }
    }
}

/// Early-exit variant: stops when a full pass makes no swap.
pub fn bubble_sort_early_exit<T: PartialOrd>(a: &mut [T]) -> usize {
    let mut passes = 0;
    loop {
        passes += 1;
        let mut swapped = false;
        for j in 1..a.len() {
            if a[j] < a[j - 1] {
                a.swap(j, j - 1);
                swapped = true;
            }
        }
        if !swapped {
            return passes;
        }
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = vec![5, 1, 4, 2, 8];
    outln!(o, "before: {:?}", a);
    bubble_sort(&mut a);
    outln!(o, "after : {:?}", a);
    let mut s = vec![1, 2, 3, 4];
    outln!(o, "early exit on sorted input used {} pass(es)", bubble_sort_early_exit(&mut s));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn sorts() {
        let mut r = Rng::new(4);
        for n in 0..30 {
            let v = r.vec(n, 0, 30);
            let mut e = v.clone();
            e.sort();
            let (mut a, mut b) = (v.clone(), v);
            bubble_sort(&mut a);
            bubble_sort_early_exit(&mut b);
            assert_eq!(a, e);
            assert_eq!(b, e);
        }
    }
    #[test]
    fn early_exit_is_one_pass_when_sorted() {
        assert_eq!(bubble_sort_early_exit(&mut [1, 2, 3]), 1);
    }
}
