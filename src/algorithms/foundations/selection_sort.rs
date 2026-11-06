//! @title Selection Sort
//! @chapter Exercise 2.2-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Repeatedly select the minimum of the unsorted suffix and swap it forward.
//! @order 30
use crate::outln;

/// SELECTION-SORT(A): after round i, A[0..=i] holds the i+1 smallest elements in order.
pub fn selection_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    if n < 2 {
        return;
    }
    for i in 0..n - 1 {
        let mut m = i;
        for j in i + 1..n {
            if a[j] < a[m] {
                m = j;
            }
        }
        a.swap(i, m);
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = vec![64, 25, 12, 22, 11];
    outln!(o, "before: {:?}", a);
    selection_sort(&mut a);
    outln!(o, "after : {:?}", a);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_std_sort() {
        let mut r = Rng::new(2);
        for n in 0..30 {
            let mut v = r.vec(n, -9, 9);
            let mut e = v.clone();
            e.sort();
            selection_sort(&mut v);
            assert_eq!(v, e);
        }
    }
}
