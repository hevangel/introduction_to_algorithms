//! @title Interval Trees
//! @chapter 14.3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Red-black tree keyed by low endpoints and augmented with the maximum high endpoint: INTERVAL-SEARCH in O(lg n).
//! @order 140
use super::red_black_tree::{RedBlackTree, NIL};
use crate::outln;

/// Closed intervals [lo, hi]. Key = lo, aux = hi, subtree max = max hi.
#[derive(Debug, Clone, Default)]
pub struct IntervalTree {
    rb: RedBlackTree,
}

fn overlaps(a: (i64, i64), b: (i64, i64)) -> bool { a.0 <= b.1 && b.0 <= a.1 }

impl IntervalTree {
    pub fn new() -> Self { IntervalTree { rb: RedBlackTree::new() } }
    pub fn len(&self) -> usize { self.rb.len() }
    pub fn is_empty(&self) -> bool { self.rb.is_empty() }
    pub fn insert(&mut self, lo: i64, hi: i64) {
        assert!(lo <= hi);
        self.rb.insert_with_aux(lo, hi);
    }
    pub fn delete(&mut self, lo: i64, hi: i64) -> bool {
        let z = self.rb.find_where(lo, &|t, x| t.aux(x) == hi);
        if z == NIL { return false; }
        self.rb.delete(z);
        true
    }
    fn interval(&self, x: usize) -> (i64, i64) { (self.rb.key(x), self.rb.aux(x)) }

    /// INTERVAL-SEARCH(T, i): some interval overlapping [lo, hi], in O(lg n).
    pub fn search(&self, lo: i64, hi: i64) -> Option<(i64, i64)> {
        let mut x = self.rb.root();
        while x != NIL && !overlaps(self.interval(x), (lo, hi)) {
            let l = self.rb.left(x);
            x = if l != NIL && self.rb.max_aux(l) >= lo { l } else { self.rb.right(x) };
        }
        if x == NIL { None } else { Some(self.interval(x)) }
    }
    /// Exercise 14.3-5-style: list all overlapping intervals in O(min(n, k lg n)).
    pub fn search_all(&self, lo: i64, hi: i64) -> Vec<(i64, i64)> {
        fn rec(t: &IntervalTree, x: usize, q: (i64, i64), out: &mut Vec<(i64, i64)>) {
            if x == NIL { return; }
            let l = t.rb.left(x);
            if l != NIL && t.rb.max_aux(l) >= q.0 { rec(t, l, q, out); }
            if overlaps(t.interval(x), q) { out.push(t.interval(x)); }
            if t.rb.key(x) <= q.1 { rec(t, t.rb.right(x), q, out); }
        }
        let mut out = vec![];
        rec(self, self.rb.root(), (lo, hi), &mut out);
        out
    }
    pub fn validate(&self) -> bool { self.rb.validate().is_ok() }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = IntervalTree::new();
    for (lo, hi) in [(16, 21), (8, 9), (25, 30), (5, 8), (15, 23), (17, 19), (26, 26), (0, 3), (6, 10), (19, 20)] { t.insert(lo, hi); }
    outln!(o, "intervals stored: {}", t.len());
    outln!(o, "search [22,25]  -> {:?}", t.search(22, 25));
    outln!(o, "search [11,14]  -> {:?}  (no overlap)", t.search(11, 14));
    let mut all = t.search_all(14, 16);
    all.sort();
    outln!(o, "all overlapping [14,16] -> {:?}", all);
    t.delete(15, 23);
    outln!(o, "after deleting (15,23): search [22,25] -> {:?}", t.search(22, 25));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_brute_force() {
        let mut r = Rng::new(74);
        let mut t = IntervalTree::new();
        let mut model: Vec<(i64, i64)> = vec![];
        for step in 0..1500 {
            if r.range(0, 3) < 2 || model.is_empty() {
                let lo = r.range(0, 200) as i64;
                let hi = lo + r.range(0, 30) as i64;
                t.insert(lo, hi);
                model.push((lo, hi));
            } else {
                let i = r.range(0, model.len() - 1);
                let iv = model.swap_remove(i);
                assert!(t.delete(iv.0, iv.1));
            }
            let (qlo, qhi) = (r.range(0, 230) as i64, 0);
            let q = (qlo, qlo + r.range(0, 10) as i64 + qhi);
            let mut want: Vec<_> = model.iter().copied().filter(|&iv| overlaps(iv, q)).collect();
            let mut got = t.search_all(q.0, q.1);
            want.sort();
            got.sort();
            assert_eq!(got, want);
            assert_eq!(t.search(q.0, q.1).is_some(), !want.is_empty());
            if let Some(iv) = t.search(q.0, q.1) { assert!(overlaps(iv, q)); }
            if step % 50 == 0 { assert!(t.validate()); }
        }
    }
    #[test]
    fn delete_missing_interval() {
        let mut t = IntervalTree::new();
        t.insert(1, 5);
        assert!(!t.delete(1, 6));
        assert!(t.delete(1, 5));
        assert!(t.is_empty());
    }
}
