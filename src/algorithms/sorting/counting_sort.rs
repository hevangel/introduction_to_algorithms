//! @title Counting Sort
//! @chapter 8.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Stable Θ(n+k) sort for integer keys in 0..=k, using a prefix-sum table.
//! @order 100
use crate::outln;

/// COUNTING-SORT(A, B, k): stable sort of values in `0..=k`.
pub fn counting_sort(a: &[usize], k: usize) -> Vec<usize> {
    counting_sort_by_key(a, k, |&x| x)
}

/// Stable counting sort of arbitrary records by an integer key in `0..=k`.
pub fn counting_sort_by_key<T: Clone>(a: &[T], k: usize, key: impl Fn(&T) -> usize) -> Vec<T> {
    let mut c = vec![0usize; k + 1];
    for x in a { c[key(x)] += 1; }                 // c[i] = #elements equal to i
    for i in 1..=k { c[i] += c[i - 1]; }           // c[i] = #elements ≤ i
    let mut b: Vec<Option<T>> = vec![None; a.len()];
    for x in a.iter().rev() {                      // reverse scan keeps it stable
        let kx = key(x);
        c[kx] -= 1;
        b[c[kx]] = Some(x.clone());
    }
    b.into_iter().map(|x| x.unwrap()).collect()
}

/// Exercise 8.2-4: after Θ(n+k) preprocessing, count elements in [lo, hi] in O(1).
pub struct RangeCounter {
    c: Vec<usize>,
}
impl RangeCounter {
    pub fn new(a: &[usize], k: usize) -> Self {
        let mut c = vec![0; k + 2];
        for &x in a { c[x + 1] += 1; }
        for i in 1..c.len() { c[i] += c[i - 1]; }
        RangeCounter { c }
    }
    /// Number of elements x with lo ≤ x ≤ hi.
    pub fn count(&self, lo: usize, hi: usize) -> usize {
        self.c[hi + 1] - self.c[lo]
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = [2, 5, 3, 0, 2, 3, 0, 3];
    outln!(o, "A = {:?}", a);
    outln!(o, "B = {:?}", counting_sort(&a, 5));
    let rc = RangeCounter::new(&a, 5);
    outln!(o, "elements in [2,3] : {}", rc.count(2, 3));
    let recs = [(2, 'a'), (1, 'b'), (2, 'c'), (1, 'd')];
    outln!(o, "stability check   : {:?}", counting_sort_by_key(&recs, 2, |r| r.0));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn sorts_and_is_stable() {
        let mut r = Rng::new(33);
        let v: Vec<usize> = r.vec(300, 0, 20).into_iter().map(|x| x as usize).collect();
        let mut e = v.clone();
        e.sort();
        assert_eq!(counting_sort(&v, 20), e);
        let recs: Vec<(usize, usize)> = v.iter().cloned().enumerate().map(|(i, x)| (x, i)).collect();
        let out = counting_sort_by_key(&recs, 20, |r| r.0);
        assert!(out.windows(2).all(|w| w[0].0 < w[1].0 || (w[0].0 == w[1].0 && w[0].1 < w[1].1)));
    }
    #[test]
    fn range_counter() {
        let a = [2, 5, 3, 0, 2, 3, 0, 3];
        let rc = RangeCounter::new(&a, 5);
        assert_eq!(rc.count(0, 5), 8);
        assert_eq!(rc.count(2, 3), 5);
        assert_eq!(rc.count(4, 4), 0);
        assert_eq!(rc.count(0, 0), 2);
    }
}
