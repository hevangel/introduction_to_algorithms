//! @title Binary Insertion Sort & Insertion/Merge Hybrid
//! @chapter Exercise 2.3-6, Problem 2-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Binary-search the insertion point (Θ(n lg n) comparisons); switch merge sort to insertion sort below a cutoff k.
//! @order 175
use crate::outln;

/// Insertion sort that binary-searches the insertion point. Returns the number of comparisons (Θ(n lg n); moves stay Θ(n²)).
pub fn binary_insertion_sort<T: PartialOrd + Clone>(a: &mut [T]) -> usize {
    let mut comps = 0;
    for j in 1..a.len() {
        let key = a[j].clone();
        let (mut lo, mut hi) = (0, j);
        while lo < hi {
            let mid = (lo + hi) / 2;
            comps += 1;
            if a[mid] <= key { lo = mid + 1; } else { hi = mid; }
        }
        a[lo..=j].rotate_right(1);
    }
    comps
}

fn insertion_counting<T: PartialOrd + Clone>(a: &mut [T]) -> usize {
    let mut comps = 0;
    for j in 1..a.len() {
        let key = a[j].clone();
        let mut i = j;
        while i > 0 {
            comps += 1;
            if a[i - 1] > key { a[i] = a[i - 1].clone(); i -= 1; } else { break; }
        }
        a[i] = key;
    }
    comps
}

/// Problem 2-1: merge sort that sorts subarrays of length ≤ k by insertion sort. Worst case Θ(nk + n lg(n/k)); returns comparisons.
pub fn hybrid_merge_sort<T: PartialOrd + Clone>(a: &mut [T], k: usize) -> usize {
    if a.len() <= k.max(1) { return insertion_counting(a); }
    let mid = a.len() / 2;
    let mut comps = hybrid_merge_sort(&mut a[..mid], k) + hybrid_merge_sort(&mut a[mid..], k);
    let (left, right) = (a[..mid].to_vec(), a[mid..].to_vec());
    let (mut i, mut j) = (0, 0);
    for slot in a.iter_mut() {
        if i < left.len() && j < right.len() { comps += 1; }
        if j >= right.len() || (i < left.len() && left[i] <= right[j]) { *slot = left[i].clone(); i += 1; } else { *slot = right[j].clone(); j += 1; }
    }
    comps
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = crate::common::Rng::new(1);
    let v = r.vec(4096, 0, 1_000_000);
    let mut b = v.clone();
    outln!(o, "n = 4096 random keys: binary insertion sort used {} comparisons (n lg n ≈ {}); plain insertion sort ≈ {}", binary_insertion_sort(&mut b), 4096 * 12, insertion_counting(&mut v.clone()));
    for k in [1, 4, 16, 64, 256] {
        let mut c = v.clone();
        outln!(o, "hybrid merge sort with cutoff k = {:>3}: {} comparisons", k, hybrid_merge_sort(&mut c, k));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn sort_correctly_and_stably() {
        let mut r = Rng::new(300);
        for n in 0..80 {
            let v = r.vec(n, 0, 15);
            let mut e = v.clone();
            e.sort();
            let (mut a, mut b) = (v.clone(), v.clone());
            binary_insertion_sort(&mut a);
            hybrid_merge_sort(&mut b, 8);
            assert_eq!((a, b), (e.clone(), e));
        }
        let mut pairs: Vec<(i32, char)> = vec![(2, 'a'), (1, 'b'), (2, 'c'), (1, 'd')];
        #[derive(Clone, PartialEq)] struct K(i32, char);
        impl PartialOrd for K { fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> { self.0.partial_cmp(&o.0) } }
        let mut ks: Vec<K> = pairs.drain(..).map(|(k, c)| K(k, c)).collect();
        binary_insertion_sort(&mut ks);
        assert_eq!(ks.iter().map(|k| k.1).collect::<String>(), "bdac");
    }
    #[test]
    fn comparison_counts() {
        let mut r = Rng::new(301);
        let v = r.vec(1024, 0, 100000);
        assert!(binary_insertion_sort(&mut v.clone()) <= 1024 * 11);
        assert!(insertion_counting(&mut v.clone()) > 200_000);
    }
}
