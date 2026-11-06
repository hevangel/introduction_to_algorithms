//! @title Proto van Emde Boas Structure
//! @chapter 20.2 (3e)
//! @editions 3
//! @summary The first recursive step towards vEB trees: √u clusters plus a summary; every operation costs Θ(√u)-ish recursion, T(u)=2T(√u)+Θ(1).
//! @order 185
use crate::outln;

/// Proto-vEB(u) for u = 2^(2^k): u = 2 stores two bits; larger universes hold a summary and √u clusters of size √u.
#[derive(Debug, Clone)]
pub struct ProtoVeb { u: usize, bits: [bool; 2], summary: Option<Box<ProtoVeb>>, cluster: Vec<ProtoVeb> }

impl ProtoVeb {
    pub fn new(u: usize) -> Self {
        assert!(u >= 2 && (u as f64).log2().fract() == 0.0 && (u.ilog2() as usize).is_power_of_two(), "u must be 2^(2^k)");
        if u == 2 { return ProtoVeb { u, bits: [false; 2], summary: None, cluster: vec![] }; }
        let r = (u as f64).sqrt() as usize;
        ProtoVeb { u, bits: [false; 2], summary: Some(Box::new(ProtoVeb::new(r))), cluster: (0..r).map(|_| ProtoVeb::new(r)).collect() }
    }
    fn sqrt(&self) -> usize { (self.u as f64).sqrt() as usize }
    fn high(&self, x: usize) -> usize { x / self.sqrt() }
    fn low(&self, x: usize) -> usize { x % self.sqrt() }
    fn index(&self, h: usize, l: usize) -> usize { h * self.sqrt() + l }
    pub fn member(&self, x: usize) -> bool { if self.u == 2 { self.bits[x] } else { self.cluster[self.high(x)].member(self.low(x)) } }
    pub fn insert(&mut self, x: usize) {
        if self.u == 2 { self.bits[x] = true; } else {
            let (h, l) = (self.high(x), self.low(x));
            self.cluster[h].insert(l);
            self.summary.as_mut().unwrap().insert(h);
        }
    }
    pub fn delete(&mut self, x: usize) {
        if self.u == 2 { self.bits[x] = false; } else {
            let (h, l) = (self.high(x), self.low(x));
            self.cluster[h].delete(l);
            if self.cluster[h].minimum().is_none() { self.summary.as_mut().unwrap().delete(h); }
        }
    }
    pub fn minimum(&self) -> Option<usize> {
        if self.u == 2 { return if self.bits[0] { Some(0) } else if self.bits[1] { Some(1) } else { None }; }
        let c = self.summary.as_ref().unwrap().minimum()?;
        Some(self.index(c, self.cluster[c].minimum().unwrap()))
    }
    pub fn maximum(&self) -> Option<usize> {
        if self.u == 2 { return if self.bits[1] { Some(1) } else if self.bits[0] { Some(0) } else { None }; }
        let c = self.summary.as_ref().unwrap().maximum()?;
        Some(self.index(c, self.cluster[c].maximum().unwrap()))
    }
    pub fn successor(&self, x: usize) -> Option<usize> {
        if self.u == 2 { return if x == 0 && self.bits[1] { Some(1) } else { None }; }
        let (h, l) = (self.high(x), self.low(x));
        if let Some(off) = self.cluster[h].successor(l) { return Some(self.index(h, off)); }
        let c = self.summary.as_ref().unwrap().successor(h)?;
        Some(self.index(c, self.cluster[c].minimum().unwrap()))
    }
    pub fn predecessor(&self, x: usize) -> Option<usize> {
        if self.u == 2 { return if x == 1 && self.bits[0] { Some(0) } else { None }; }
        let (h, l) = (self.high(x), self.low(x));
        if let Some(off) = self.cluster[h].predecessor(l) { return Some(self.index(h, off)); }
        let c = self.summary.as_ref().unwrap().predecessor(h)?;
        Some(self.index(c, self.cluster[c].maximum().unwrap()))
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut v = ProtoVeb::new(16);
    for x in [2, 3, 4, 5, 7, 14, 15] { v.insert(x); }
    outln!(o, "proto-vEB(16) with {{2,3,4,5,7,14,15}}: min={:?} max={:?}", v.minimum(), v.maximum());
    outln!(o, "successor(5)={:?} successor(7)={:?} predecessor(14)={:?} member(9)={}", v.successor(5), v.successor(7), v.predecessor(14), v.member(9));
    v.delete(14);
    outln!(o, "after deleting 14: predecessor(15) = {:?}", v.predecessor(15));
    outln!(o, "the recursion T(u) = 2T(√u) + Θ(1) solves to Θ(lg u) for MEMBER/INSERT but successor needs 3 recursive calls: Θ(lg u · … ) — see the full vEB tree for O(lg lg u)");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn matches_btreeset() {
        for u in [2usize, 4, 16, 256] {
            let mut v = ProtoVeb::new(u);
            let mut s = BTreeSet::new();
            let mut r = crate::common::Rng::new(323 + u as u64);
            for _ in 0..1500 {
                let x = r.range(0, u - 1);
                if r.range(0, 2) < 2 { v.insert(x); s.insert(x); } else { v.delete(x); s.remove(&x); }
                assert_eq!(v.member(x), s.contains(&x));
                assert_eq!(v.minimum(), s.iter().next().copied());
                assert_eq!(v.maximum(), s.iter().next_back().copied());
                let q = r.range(0, u - 1);
                assert_eq!(v.successor(q), s.range(q + 1..).next().copied());
                assert_eq!(v.predecessor(q), s.range(..q).next_back().copied());
            }
        }
    }
    #[test]
    fn rejects_non_double_power_universes() {
        assert!(std::panic::catch_unwind(|| ProtoVeb::new(8)).is_err());
        assert!(std::panic::catch_unwind(|| ProtoVeb::new(32)).is_err());
    }
}
