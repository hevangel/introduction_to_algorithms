//! @title x-Fast and y-Fast Tries
//! @chapter Problem 20-2 (3e)
//! @editions 3,4
//! @summary x-fast trie: hash tables of all prefixes give predecessor/successor in O(lg lg u) but O(n lg u) space; y-fast trie: store only one representative per bucket of ~lg u keys in the x-fast trie to get O(n) space and O(lg lg u) amortised operations.
//! @order 25
use crate::outln;
use std::collections::{BTreeSet, HashMap};

/// x-fast trie over w-bit keys. `levels[d]` maps every length-d prefix of a stored key to the (min, max) key below it; `next`/`prev` link the keys in order.
pub struct XFastTrie { w: u32, levels: Vec<HashMap<u32, (u32, u32)>>, next: HashMap<u32, u32>, prev: HashMap<u32, u32>, len: usize }

impl XFastTrie {
    pub fn new(w: u32) -> Self { assert!((1..=31).contains(&w)); XFastTrie { w, levels: vec![HashMap::new(); w as usize + 1], next: HashMap::new(), prev: HashMap::new(), len: 0 } }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn contains(&self, x: u32) -> bool { self.levels[self.w as usize].contains_key(&x) }
    fn prefix(&self, x: u32, d: u32) -> u32 { x >> (self.w - d) }
    /// (predecessor-or-equal, successor-or-equal) of x among the stored keys, by binary search on the prefix length: O(lg w) = O(lg lg u) hash lookups.
    fn neighbours(&self, x: u32) -> (Option<u32>, Option<u32>) {
        if self.len == 0 { return (None, None); }
        if self.contains(x) { return (Some(x), Some(x)); }
        let (mut lo, mut hi) = (0u32, self.w); // the root (d = 0) is always present, the leaf (d = w) is not
        while hi - lo > 1 { let mid = (lo + hi) / 2; if self.levels[mid as usize].contains_key(&self.prefix(x, mid)) { lo = mid; } else { hi = mid; } }
        let (min_max_child, bit) = (|child: u32| self.levels[lo as usize + 1][&child], (x >> (self.w - lo - 1)) & 1);
        let p = self.prefix(x, lo);
        if bit == 0 { let succ = min_max_child(p * 2 + 1).0; (self.prev.get(&succ).copied(), Some(succ)) } // left child missing: everything below is larger
        else { let pred = min_max_child(p * 2).1; (Some(pred), self.next.get(&pred).copied()) }
    }
    pub fn predecessor(&self, x: u32) -> Option<u32> { self.neighbours(x).0 }
    pub fn successor(&self, x: u32) -> Option<u32> { self.neighbours(x).1 }
    pub fn insert(&mut self, x: u32) -> bool {
        if self.contains(x) { return false; }
        let (p, s) = self.neighbours(x);
        if let Some(p) = p { self.next.insert(p, x); self.prev.insert(x, p); }
        if let Some(s) = s { self.prev.insert(s, x); self.next.insert(x, s); }
        for d in 0..=self.w { let key = self.prefix(x, d); let e = self.levels[d as usize].entry(key).or_insert((x, x)); e.0 = e.0.min(x); e.1 = e.1.max(x); }
        self.len += 1;
        true
    }
    pub fn remove(&mut self, x: u32) -> bool {
        if !self.contains(x) { return false; }
        let (p, s) = (self.prev.remove(&x), self.next.remove(&x));
        match (p, s) { (Some(p), Some(s)) => { self.next.insert(p, s); self.prev.insert(s, p); } (Some(p), None) => { self.next.remove(&p); } (None, Some(s)) => { self.prev.remove(&s); } _ => {} }
        for d in 0..=self.w {
            let key = self.prefix(x, d);
            let e = self.levels[d as usize][&key];
            if e.0 == x && e.1 == x { self.levels[d as usize].remove(&key); }
            else { let ne = (if e.0 == x { s.unwrap() } else { e.0 }, if e.1 == x { p.unwrap() } else { e.1 }); self.levels[d as usize].insert(key, ne); }
        }
        self.len -= 1;
        true
    }
    /// Number of hash-table entries (the O(n·w) space of an x-fast trie).
    pub fn space(&self) -> usize { self.levels.iter().map(|l| l.len()).sum() }
}

/// y-fast trie: buckets of Θ(w) keys in balanced trees; one representative per bucket lives in an x-fast trie.
/// Bucket r holds the keys in (previous representative, r] (the last bucket also keeps larger keys). Sizes are kept between w/2 and 2w.
pub struct YFastTrie { w: usize, reps: XFastTrie, buckets: HashMap<u32, BTreeSet<u32>>, len: usize }

impl YFastTrie {
    pub fn new(w: u32) -> Self { YFastTrie { w: w as usize, reps: XFastTrie::new(w), buckets: HashMap::new(), len: 0 } }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    fn rep_for(&self, x: u32) -> Option<u32> { self.reps.successor(x).or_else(|| self.reps.predecessor((1u32 << self.reps.w) - 1)) }
    pub fn contains(&self, x: u32) -> bool { self.rep_for(x).is_some_and(|r| self.buckets[&r].contains(&x)) }
    pub fn insert(&mut self, x: u32) -> bool {
        let Some(r) = self.rep_for(x) else { self.reps.insert(x); self.buckets.insert(x, BTreeSet::from([x])); self.len = 1; return true; };
        if !self.buckets.get_mut(&r).unwrap().insert(x) { return false; }
        self.len += 1;
        self.maybe_split(r);
        true
    }
    pub fn remove(&mut self, x: u32) -> bool {
        let Some(r) = self.rep_for(x) else { return false };
        if !self.buckets.get_mut(&r).unwrap().remove(&x) { return false; }
        self.len -= 1;
        if self.buckets[&r].is_empty() { self.buckets.remove(&r); self.reps.remove(r); return true; }
        if self.buckets[&r].len() < self.w / 2 { // too small: merge with the next bucket (or the previous one if this is the last)
            if let Some(nr) = self.reps.successor(r + 1) {
                let small = self.buckets.remove(&r).unwrap();
                self.reps.remove(r);
                self.buckets.get_mut(&nr).unwrap().extend(small);
                self.maybe_split(nr);
            } else if r > 0 {
                if let Some(pr) = self.reps.predecessor(r - 1) {
                    let prev_set = self.buckets.remove(&pr).unwrap();
                    self.reps.remove(pr);
                    self.buckets.get_mut(&r).unwrap().extend(prev_set);
                    self.maybe_split(r);
                }
            }
        }
        true
    }
    /// Split an oversized bucket in half. Both halves get their maximum as representative, so every key lies at or below its bucket's representative
    /// (except in the last bucket, where larger keys are allowed).
    fn maybe_split(&mut self, r: u32) {
        if self.buckets[&r].len() <= 2 * self.w { return; }
        let set = self.buckets.get_mut(&r).unwrap();
        let mid = *set.iter().nth(set.len() / 2).unwrap();
        let upper = set.split_off(&mid);
        let lower = std::mem::take(set);
        self.buckets.remove(&r);
        self.reps.remove(r);
        for half in [lower, upper] {
            let rep = *half.iter().next_back().unwrap();
            self.reps.insert(rep);
            self.buckets.insert(rep, half);
        }
    }
    /// Smallest key ≥ x.
    pub fn successor(&self, x: u32) -> Option<u32> {
        let r = self.rep_for(x)?;
        if let Some(&k) = self.buckets[&r].range(x..).next() { return Some(k); }
        let nr = self.reps.successor(r + 1)?;
        self.buckets[&nr].iter().next().copied()
    }
    /// Largest key ≤ x.
    pub fn predecessor(&self, x: u32) -> Option<u32> {
        let r = self.rep_for(x)?;
        if let Some(&k) = self.buckets[&r].range(..=x).next_back() { return Some(k); }
        if r == 0 { return None; }
        let pr = self.reps.predecessor(r - 1)?;
        self.buckets[&pr].iter().next_back().copied()
    }
    /// (number of buckets, largest bucket size).
    pub fn shape(&self) -> (usize, usize) { (self.buckets.len(), self.buckets.values().map(|b| b.len()).max().unwrap_or(0)) }
    pub fn space(&self) -> usize { self.reps.space() + self.len }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = crate::common::Rng::new(20);
    let w = 24;
    let (mut x, mut y) = (XFastTrie::new(w), YFastTrie::new(w));
    for _ in 0..20_000 { let k = r.range(0, (1 << w) - 1) as u32; x.insert(k); y.insert(k); }
    outln!(o, "w = {w}, n = {}: x-fast trie stores {} hash entries (≈ n·w), y-fast trie {} words (≈ n) in {} buckets (largest {})", x.len(), x.space(), y.space(), y.shape().0, y.shape().1);
    let q = 1_000_000u32;
    outln!(o, "successor({q}): x-fast {:?}, y-fast {:?}; predecessor: {:?} / {:?}", x.successor(q), y.successor(q), x.predecessor(q), y.predecessor(q));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn x_fast_trie_matches_btreeset_including_deletions() {
        let mut r = Rng::new(503);
        let w = 10;
        for _ in 0..30 {
            let (mut t, mut s) = (XFastTrie::new(w), BTreeSet::new());
            for _ in 0..400 {
                let k = r.range(0, (1 << w) - 1) as u32;
                if r.range(0, 2) < 2 { assert_eq!(t.insert(k), s.insert(k)); } else { assert_eq!(t.remove(k), s.remove(&k)); }
                let q = r.range(0, (1 << w) - 1) as u32;
                assert_eq!(t.successor(q), s.range(q..).next().copied(), "successor({q})");
                assert_eq!(t.predecessor(q), s.range(..=q).next_back().copied(), "predecessor({q})");
                assert_eq!(t.contains(k), s.contains(&k));
            }
            assert_eq!(t.len(), s.len());
        }
    }
    #[test]
    fn y_fast_trie_matches_btreeset_and_keeps_buckets_bounded() {
        let mut r = Rng::new(504);
        let w = 12;
        for round in 0..20 {
            let (mut t, mut s) = (YFastTrie::new(w), BTreeSet::new());
            for step in 0..1500 {
                let k = r.range(0, (1 << w) - 1) as u32;
                let insert = if round % 2 == 0 { r.range(0, 3) != 0 } else { step < 800 || r.range(0, 1) == 0 };
                if insert { assert_eq!(t.insert(k), s.insert(k)); } else { assert_eq!(t.remove(k), s.remove(&k)); }
                if step % 7 == 0 {
                    let q = r.range(0, (1 << w) - 1) as u32;
                    assert_eq!(t.successor(q), s.range(q..).next().copied(), "successor({q})");
                    assert_eq!(t.predecessor(q), s.range(..=q).next_back().copied(), "predecessor({q})");
                    assert_eq!(t.contains(q), s.contains(&q));
                }
            }
            assert_eq!(t.len(), s.len());
            assert!(t.shape().1 <= 2 * w as usize, "bucket size {} exceeds 2w", t.shape().1);
        }
    }
    #[test]
    fn y_fast_uses_linear_space_where_x_fast_needs_n_w() {
        let mut r = Rng::new(505);
        let w = 24;
        let (mut x, mut y) = (XFastTrie::new(w), YFastTrie::new(w));
        for _ in 0..5000 { let k = r.range(0, (1 << w) - 1) as u32; x.insert(k); y.insert(k); }
        assert!(x.space() > 8 * x.len(), "x-fast space {} for n = {}", x.space(), x.len());
        assert!(y.space() < 3 * y.len(), "y-fast space {} for n = {}", y.space(), y.len());
    }
}
