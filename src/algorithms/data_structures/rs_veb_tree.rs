//! @title Reduced-Space van Emde Boas Trees
//! @chapter Problem 20-1 (3e)
//! @editions 3,4
//! @summary Replace the √u + 1 pre-allocated sub-trees of a vEB tree by a hash table that holds only non-empty clusters (and a summary allocated on demand): space proportional to the number of stored keys instead of u, still O(lg lg u) expected per operation.
//! @order 30
use crate::outln;
use std::collections::HashMap;

/// The space recurrence of an ordinary vEB tree with universe u = 2^(2^k): P(u) = (√u + 1)·P(√u) + √u, with P(2) = 1.
pub fn plain_veb_space(u: u64) -> u64 { if u <= 2 { 1 } else { let r = (u as f64).sqrt() as u64; (r + 1) * plain_veb_space(r) + r } }

/// A reduced-space vEB tree over the universe {0, …, 2^w − 1}. `min` is stored only at the node (not in any cluster); clusters live in a hash table.
#[derive(Debug, Clone)]
pub struct RsVeb { w: u32, min: Option<u32>, max: Option<u32>, summary: Option<Box<RsVeb>>, cluster: HashMap<u32, RsVeb> }

impl RsVeb {
    /// CREATE-NEW-RS-vEB-TREE(u): O(1) — nothing is allocated except the (empty) hash table.
    pub fn new(w: u32) -> Self { assert!((1..=31).contains(&w)); RsVeb { w, min: None, max: None, summary: None, cluster: HashMap::new() } }
    fn low_bits(&self) -> u32 { self.w / 2 }
    pub fn is_empty(&self) -> bool { self.min.is_none() }
    pub fn minimum(&self) -> Option<u32> { self.min }
    pub fn maximum(&self) -> Option<u32> { self.max }
    fn split(&self, x: u32) -> (u32, u32) { (x >> self.low_bits(), x & ((1u32 << self.low_bits()) - 1)) }
    pub fn member(&self, x: u32) -> bool {
        if self.min == Some(x) || self.max == Some(x) { return true; }
        if self.w == 1 || self.min.is_none() { return false; }
        let (hi, lo) = self.split(x);
        self.cluster.get(&hi).is_some_and(|c| c.member(lo))
    }
    /// RS-VEB-TREE-INSERT: clusters and the summary are created the first time they are needed. O(lg lg u) expected.
    pub fn insert(&mut self, mut x: u32) {
        let Some(min) = self.min else { self.min = Some(x); self.max = Some(x); return };
        if x == min { return; }
        if x < min { self.min = Some(x); x = min; } // the old minimum moves down into a cluster
        if self.w > 1 {
            let (hi, lo) = self.split(x);
            let low = self.low_bits();
            let w_summary = self.w - low;
            let c = self.cluster.entry(hi).or_insert_with(|| RsVeb::new(low));
            if c.is_empty() { self.summary.get_or_insert_with(|| Box::new(RsVeb::new(w_summary))).insert(hi); }
            // (the entry API above borrowed `cluster`; re-fetch to insert)
            self.cluster.get_mut(&hi).unwrap().insert(lo);
        }
        if self.max.is_none_or(|m| x > m) { self.max = Some(x); }
    }
    /// RS-VEB-TREE-SUCCESSOR: smallest stored key strictly greater than x. O(lg lg u) expected.
    pub fn successor(&self, x: u32) -> Option<u32> {
        let min = self.min?;
        if x < min { return Some(min); }
        if self.w == 1 { return (x == 0 && self.max == Some(1)).then_some(1); }
        let (hi, lo) = self.split(x);
        if let Some(c) = self.cluster.get(&hi) { if c.max.is_some_and(|m| lo < m) { return Some((hi << self.low_bits()) | c.successor(lo)?); } }
        let next = self.summary.as_ref()?.successor(hi)?;
        Some((next << self.low_bits()) | self.cluster[&next].min.unwrap())
    }
    /// Number of nodes allocated in the whole structure (the space in "tree nodes"; hash-table slots are proportional to it).
    pub fn nodes(&self) -> usize { 1 + self.cluster.values().map(|c| c.nodes()).sum::<usize>() + self.summary.as_ref().map_or(0, |s| s.nodes()) }
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "space of a plain vEB tree, P(u) = (√u+1)P(√u) + √u:");
    for k in 1..=5u32 { let u = 1u64 << (1 << k); outln!(o, "  u = 2^{:<2}: P(u) = {:>12} = 2u − 3 = {}", 1 << k, plain_veb_space(u), 2 * u - 3); }
    let mut r = crate::common::Rng::new(20);
    let mut t = RsVeb::new(30);
    let keys: Vec<u32> = (0..5000).map(|_| r.range(0, (1 << 30) - 1) as u32).collect();
    for &k in &keys { t.insert(k); }
    outln!(o, "RS-vEB over u = 2^30 holding {} keys uses {} nodes (a plain vEB tree would need ≈ 2^31)", keys.len(), t.nodes());
    outln!(o, "successor(1000000) = {:?}", t.successor(1_000_000));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    use std::collections::BTreeSet;
    #[test]
    fn plain_veb_space_is_two_u_minus_three() {
        for k in 1..=5u32 { let u = 1u64 << (1 << k); if k >= 2 { assert_eq!(plain_veb_space(u), 2 * u - 3, "u = 2^{}", 1 << k); } assert!(plain_veb_space(u) <= 2 * u); }
    }
    #[test]
    fn matches_a_sorted_set_for_member_min_max_successor() {
        let mut r = Rng::new(515);
        for w in [1u32, 2, 3, 5, 8, 12, 20, 31] {
            let (mut t, mut s) = (RsVeb::new(w), BTreeSet::new());
            let max = ((1u64 << w) - 1) as usize;
            for _ in 0..300 {
                let k = r.range(0, max) as u32;
                t.insert(k);
                s.insert(k);
                let q = r.range(0, max) as u32;
                assert_eq!(t.successor(q), s.range(q + 1..).next().copied(), "w = {w}, successor({q})");
                assert_eq!(t.member(q), s.contains(&q));
            }
            assert_eq!((t.minimum(), t.maximum()), (s.iter().next().copied(), s.iter().next_back().copied()));
        }
    }
    #[test]
    fn space_is_proportional_to_the_number_of_keys_not_the_universe() {
        let mut r = Rng::new(516);
        let mut t = RsVeb::new(30);
        let n = 4000;
        for _ in 0..n { t.insert(r.range(0, (1 << 30) - 1) as u32); }
        assert!(t.nodes() <= 6 * n, "{} nodes for {n} keys", t.nodes());
        assert!(RsVeb::new(30).nodes() == 1, "creating an empty tree takes constant time and space");
    }
}
