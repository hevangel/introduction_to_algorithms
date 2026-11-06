//! @title Insertion Sort
//! @chapter 2.1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Builds a sorted prefix by inserting each element into place; the book's first algorithm.
//! @order 10
use crate::outln;

/// INSERTION-SORT(A): sorts in place into non-decreasing order.
pub fn insertion_sort<T: PartialOrd + Clone>(a: &mut [T]) {
    for j in 1..a.len() {
        let key = a[j].clone();
        let mut i = j;
        while i > 0 && a[i - 1] > key {
            a[i] = a[i - 1].clone();
            i -= 1;
        }
        a[i] = key;
    }
}

/// Exercise 2.1-2: non-increasing order.
pub fn insertion_sort_desc<T: PartialOrd + Clone>(a: &mut [T]) {
    for j in 1..a.len() {
        let key = a[j].clone();
        let mut i = j;
        while i > 0 && a[i - 1] < key {
            a[i] = a[i - 1].clone();
            i -= 1;
        }
        a[i] = key;
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = vec![5, 2, 4, 6, 1, 3];
    outln!(o, "input      : {:?}", a);
    insertion_sort(&mut a);
    outln!(o, "ascending  : {:?}", a);
    insertion_sort_desc(&mut a);
    outln!(o, "descending : {:?}", a);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn sorts_random_inputs() {
        let mut r = Rng::new(1);
        for n in 0..40 {
            let mut v = r.vec(n, -50, 50);
            let mut e = v.clone();
            e.sort();
            insertion_sort(&mut v);
            assert_eq!(v, e);
            insertion_sort_desc(&mut v);
            e.reverse();
            assert_eq!(v, e);
        }
    }
    #[test]
    fn stable_on_equal_keys() {
        let mut v = vec![(1, 'a'), (0, 'b'), (1, 'c'), (0, 'd')];
        // compare only by key via a wrapper
        #[derive(Clone, PartialEq)]
        struct K(i32, char);
        impl PartialOrd for K {
            fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> { self.0.partial_cmp(&o.0) }
        }
        let mut w: Vec<K> = v.drain(..).map(|(k, c)| K(k, c)).collect();
        insertion_sort(&mut w);
        let s: String = w.iter().map(|k| k.1).collect();
        assert_eq!(s, "bdac");
    }
}
