//! @title Heapsort
//! @chapter 6.1–6.4 (1e–4e)
//! @editions 1,2,3,4
//! @summary MAX-HEAPIFY, BUILD-MAX-HEAP and in-place Θ(n lg n) heapsort.
//! @order 10
use crate::outln;

#[inline]
fn left(i: usize) -> usize { 2 * i + 1 }
#[inline]
fn right(i: usize) -> usize { 2 * i + 2 }

/// MAX-HEAPIFY(A, i): float A[i] down so the subtree rooted at i is a max-heap
/// (assumes both child subtrees already are). `heap_size` bounds the heap inside `a`.
pub fn max_heapify<T: PartialOrd>(a: &mut [T], mut i: usize, heap_size: usize) {
    loop {
        let (l, r) = (left(i), right(i));
        let mut largest = i;
        if l < heap_size && a[l] > a[largest] {
            largest = l;
        }
        if r < heap_size && a[r] > a[largest] {
            largest = r;
        }
        if largest == i {
            return;
        }
        a.swap(i, largest);
        i = largest;
    }
}

/// BUILD-MAX-HEAP(A): Θ(n) — heapify every internal node bottom-up.
pub fn build_max_heap<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in (0..n / 2).rev() {
        max_heapify(a, i, n);
    }
}

/// HEAPSORT(A): build a max-heap, then repeatedly move the maximum to the end.
pub fn heapsort<T: PartialOrd>(a: &mut [T]) {
    build_max_heap(a);
    let mut size = a.len();
    while size > 1 {
        a.swap(0, size - 1);
        size -= 1;
        max_heapify(a, 0, size);
    }
}

pub fn is_max_heap<T: PartialOrd>(a: &[T]) -> bool {
    (1..a.len()).all(|i| a[(i - 1) / 2] >= a[i])
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    outln!(o, "input          : {:?}", a);
    build_max_heap(&mut a);
    outln!(o, "BUILD-MAX-HEAP : {:?}  (heap? {})", a, is_max_heap(&a));
    heapsort(&mut a);
    outln!(o, "HEAPSORT       : {:?}", a);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn sorts_and_builds_heaps() {
        let mut r = Rng::new(20);
        for n in 0..80 {
            let v = r.vec(n, -30, 30);
            let mut h = v.clone();
            build_max_heap(&mut h);
            assert!(is_max_heap(&h));
            let mut e = v.clone();
            e.sort();
            let mut s = v;
            heapsort(&mut s);
            assert_eq!(s, e);
        }
    }
    #[test]
    fn book_figure_6_2() {
        let mut a = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        max_heapify(&mut a, 1, 10);
        assert_eq!(a, vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);
    }
}
