//! @title van Emde Boas Trees
//! @chapter 20 (3e)
//! @editions 3
//! @summary Priority queue on integers 0..u with insert, delete, member, min/max, successor and predecessor in O(lg lg u).
//! @order 190
use crate::outln;

/// van Emde Boas tree over the universe {0, …, u−1}, u a power of two ≥ 2.
/// Min and max are stored explicitly; the minimum is *not* stored in any cluster (the lazy-insertion trick of Section 20.3).
#[derive(Debug, Clone)]
pub struct VebTree {
    u: usize,
    min: Option<usize>,
    max: Option<usize>,
    summary: Option<Box<VebTree>>,
    cluster: Vec<VebTree>,
}

impl VebTree {
    pub fn new(u: usize) -> Self {
        assert!(u >= 2 && u.is_power_of_two());
        if u == 2 { return VebTree { u, min: None, max: None, summary: None, cluster: vec![] }; }
        let k = u.trailing_zeros() as usize;
        let lower = 1usize << (k / 2); // ⌊√u⌋ — size of each cluster
        let upper = 1usize << (k - k / 2); // ⌈√u⌉ — number of clusters
        VebTree {
            u,
            min: None,
            max: None,
            summary: Some(Box::new(VebTree::new(upper))),
            cluster: (0..upper).map(|_| VebTree::new(lower)).collect(),
        }
    }
    fn lower_bits(&self) -> usize { (self.u.trailing_zeros() / 2) as usize }
    fn high(&self, x: usize) -> usize { x >> self.lower_bits() }
    fn low(&self, x: usize) -> usize { x & ((1 << self.lower_bits()) - 1) }
    fn index(&self, h: usize, l: usize) -> usize { (h << self.lower_bits()) | l }

    pub fn universe(&self) -> usize { self.u }
    pub fn minimum(&self) -> Option<usize> { self.min }
    pub fn maximum(&self) -> Option<usize> { self.max }
    pub fn is_empty(&self) -> bool { self.min.is_none() }

    /// VEB-TREE-MEMBER
    pub fn member(&self, x: usize) -> bool {
        if Some(x) == self.min || Some(x) == self.max { return true; }
        if self.u == 2 { return false; }
        self.cluster[self.high(x)].member(self.low(x))
    }
    /// VEB-TREE-SUCCESSOR
    pub fn successor(&self, x: usize) -> Option<usize> {
        if self.u == 2 {
            return if x == 0 && self.max == Some(1) { Some(1) } else { None };
        }
        if self.min.is_some_and(|m| x < m) { return self.min; }
        let (h, l) = (self.high(x), self.low(x));
        if self.cluster[h].max.is_some_and(|m| l < m) {
            let off = self.cluster[h].successor(l)?;
            Some(self.index(h, off))
        } else {
            let sc = self.summary.as_ref().unwrap().successor(h)?;
            Some(self.index(sc, self.cluster[sc].min.unwrap()))
        }
    }
    /// VEB-TREE-PREDECESSOR
    pub fn predecessor(&self, x: usize) -> Option<usize> {
        if self.u == 2 {
            return if x == 1 && self.min == Some(0) { Some(0) } else { None };
        }
        if self.max.is_some_and(|m| x > m) { return self.max; }
        let (h, l) = (self.high(x), self.low(x));
        if self.cluster[h].min.is_some_and(|m| l > m) {
            let off = self.cluster[h].predecessor(l)?;
            Some(self.index(h, off))
        } else {
            match self.summary.as_ref().unwrap().predecessor(h) {
                None => if self.min.is_some_and(|m| x > m) { self.min } else { None },
                Some(pc) => Some(self.index(pc, self.cluster[pc].max.unwrap())),
            }
        }
    }
    /// VEB-TREE-INSERT (no-op if already present).
    pub fn insert(&mut self, x: usize) {
        assert!(x < self.u);
        if !self.member(x) { self.insert_new(x); }
    }
    fn insert_new(&mut self, mut x: usize) {
        if self.min.is_none() {
            self.min = Some(x); // VEB-EMPTY-TREE-INSERT: O(1)
            self.max = Some(x);
            return;
        }
        if x < self.min.unwrap() { std::mem::swap(&mut x, self.min.as_mut().unwrap()); }
        if self.u > 2 {
            let (h, l) = (self.high(x), self.low(x));
            if self.cluster[h].min.is_none() {
                self.summary.as_mut().unwrap().insert_new(h);
                self.cluster[h].min = Some(l);
                self.cluster[h].max = Some(l);
            } else {
                self.cluster[h].insert_new(l);
            }
        }
        if x > self.max.unwrap() { self.max = Some(x); }
    }
    /// VEB-TREE-DELETE (no-op if absent).
    pub fn delete(&mut self, x: usize) {
        if x < self.u && self.member(x) { self.delete_present(x); }
    }
    fn delete_present(&mut self, mut x: usize) {
        if self.min == self.max {
            self.min = None;
            self.max = None;
        } else if self.u == 2 {
            self.min = Some(1 - x);
            self.max = self.min;
        } else {
            if Some(x) == self.min {
                let first = self.summary.as_ref().unwrap().min.unwrap();
                x = self.index(first, self.cluster[first].min.unwrap());
                self.min = Some(x);
            }
            let (h, l) = (self.high(x), self.low(x));
            self.cluster[h].delete_present(l);
            if self.cluster[h].min.is_none() {
                self.summary.as_mut().unwrap().delete_present(h);
                if Some(x) == self.max {
                    self.max = match self.summary.as_ref().unwrap().max {
                        None => self.min,
                        Some(sm) => Some(self.index(sm, self.cluster[sm].max.unwrap())),
                    };
                }
            } else if Some(x) == self.max {
                self.max = Some(self.index(h, self.cluster[h].max.unwrap()));
            }
        }
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut v = VebTree::new(16);
    for x in [2, 3, 4, 5, 7, 14, 15] { v.insert(x); }
    outln!(o, "universe 16, stored {{2,3,4,5,7,14,15}}: min={:?} max={:?}", v.minimum(), v.maximum());
    outln!(o, "successor(5)={:?} successor(7)={:?} successor(15)={:?}", v.successor(5), v.successor(7), v.successor(15));
    outln!(o, "predecessor(7)={:?} predecessor(2)={:?} predecessor(14)={:?}", v.predecessor(7), v.predecessor(2), v.predecessor(14));
    v.delete(7);
    v.delete(2);
    outln!(o, "after deleting 7 and 2: member(7)={} min={:?} successor(5)={:?}", v.member(7), v.minimum(), v.successor(5));
    let mut big = VebTree::new(1 << 16);
    for x in (0..65536).step_by(97) { big.insert(x); }
    outln!(o, "u = 2^16: successor(1000) = {:?} found with ~lg lg u = 4 recursive levels", big.successor(1000));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn matches_btreeset_for_various_universes() {
        for k in [1, 2, 3, 4, 5, 8, 10] {
            let u = 1usize << k;
            let mut v = VebTree::new(u);
            let mut s = BTreeSet::new();
            let mut r = crate::common::Rng::new(90 + k as u64);
            for _ in 0..3000 {
                let x = r.range(0, u - 1);
                match r.range(0, 3) {
                    0 | 1 => { v.insert(x); s.insert(x); }
                    _ => { v.delete(x); s.remove(&x); }
                }
                assert_eq!(v.member(x), s.contains(&x));
                assert_eq!(v.minimum(), s.iter().next().copied());
                assert_eq!(v.maximum(), s.iter().next_back().copied());
                let q = r.range(0, u - 1);
                assert_eq!(v.successor(q), s.range(q + 1..).next().copied(), "succ({q}) u={u}");
                assert_eq!(v.predecessor(q), s.range(..q).next_back().copied(), "pred({q}) u={u}");
            }
        }
    }
    #[test]
    fn empty_tree() {
        let v = VebTree::new(16);
        assert!(v.is_empty());
        assert_eq!(v.successor(3), None);
        assert_eq!(v.predecessor(3), None);
        assert!(!v.member(0));
    }
}
