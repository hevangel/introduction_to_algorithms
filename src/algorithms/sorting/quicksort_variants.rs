//! @title Quicksort Variants
//! @chapter Problems 7-2, 7-4, 7-5 (2e–4e)
//! @editions 2,3,4
//! @summary Three-way partitioning for equal keys, tail-recursion elimination with O(lg n) stack, median-of-3 pivots.
//! @order 80
use crate::outln;

/// Problem 7-2: three-way partition (Dutch national flag). Equal keys are never recursed on.
pub fn quicksort_three_way<T: PartialOrd + Clone>(a: &mut [T]) {
    if a.len() <= 1 { return; }
    let x = a[a.len() / 2].clone();
    let (mut lt, mut i, mut gt) = (0, 0, a.len());
    while i < gt {
        if a[i] < x {
            a.swap(lt, i);
            lt += 1;
            i += 1;
        } else if a[i] > x {
            gt -= 1;
            a.swap(i, gt);
        } else {
            i += 1;
        }
    }
    let (left, rest) = a.split_at_mut(lt);
    quicksort_three_way(left);
    quicksort_three_way(&mut rest[gt - lt..]);
}

fn lomuto<T: PartialOrd>(a: &mut [T]) -> usize {
    let r = a.len() - 1;
    let mut i = 0;
    for j in 0..r {
        if a[j] <= a[r] {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, r);
    i
}

/// Problem 7-4: TAIL-RECURSIVE-QUICKSORT — recurse into the smaller part, loop on the larger.
/// Returns the maximum recursion depth reached (O(lg n) guaranteed).
pub fn quicksort_tail_recursive<T: PartialOrd>(a: &mut [T]) -> usize {
    fn go<T: PartialOrd>(mut s: &mut [T], depth: usize, max: &mut usize) {
        *max = (*max).max(depth);
        while s.len() > 1 {
            let q = lomuto(s);
            let (l, r) = s.split_at_mut(q);
            let r = &mut r[1..];
            if l.len() < r.len() {
                go(l, depth + 1, max);
                s = r;
            } else {
                go(r, depth + 1, max);
                s = l;
            }
        }
    }
    let mut max = 0;
    go(a, 1, &mut max);
    max
}

/// Problem 7-5: median-of-3 pivot selection.
pub fn quicksort_median_of_three<T: PartialOrd>(a: &mut [T]) {
    if a.len() <= 1 { return; }
    let (lo, mid, hi) = (0, a.len() / 2, a.len() - 1);
    // move the median of a[lo], a[mid], a[hi] to the pivot position hi
    let m = if (a[lo] <= a[mid]) == (a[mid] <= a[hi]) { mid } else if (a[mid] <= a[lo]) == (a[lo] <= a[hi]) { lo } else { hi };
    a.swap(m, hi);
    let q = lomuto(a);
    let (l, r) = a.split_at_mut(q);
    quicksort_median_of_three(l);
    quicksort_median_of_three(&mut r[1..]);
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = vec![3, 1, 3, 2, 3, 1, 2, 3, 3];
    quicksort_three_way(&mut a);
    outln!(o, "three-way on many duplicates: {:?}", a);
    let mut big: Vec<u32> = (0..100_000).collect();
    let depth = quicksort_tail_recursive(&mut big);
    outln!(o, "tail-recursive quicksort on 100000 sorted keys: max stack depth = {} (lg n ≈ 17)", depth);
    let mut c: Vec<i32> = (0..50).rev().collect();
    quicksort_median_of_three(&mut c);
    outln!(o, "median-of-3 on reverse input sorted: {}", crate::common::is_sorted(&c));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn all_variants_sort() {
        let mut r = Rng::new(31);
        for n in 0..90 {
            let v = r.vec(n, -4, 4);
            let mut e = v.clone();
            e.sort();
            let (mut a, mut b, mut c) = (v.clone(), v.clone(), v);
            quicksort_three_way(&mut a);
            quicksort_tail_recursive(&mut b);
            quicksort_median_of_three(&mut c);
            assert_eq!(a, e);
            assert_eq!(b, e);
            assert_eq!(c, e);
        }
    }
    #[test]
    fn stack_depth_is_logarithmic_on_worst_case_input() {
        let mut v: Vec<u32> = (0..50_000).collect();
        let d = quicksort_tail_recursive(&mut v);
        assert!(d <= 20, "depth {d}");
    }
    #[test]
    fn three_way_handles_all_equal() {
        let mut v = vec![7; 100_000];
        quicksort_three_way(&mut v);
        assert!(v.iter().all(|&x| x == 7));
    }
}
