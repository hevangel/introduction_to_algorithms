//! @title Perfect Hashing (FKS)
//! @chapter 11.5 (2e, 3e)
//! @editions 2,3
//! @summary Two-level universal hashing gives worst-case O(1) lookup for a static key set in O(n) space.
//! @order 90
use crate::common::Rng;
use crate::outln;

const P: u128 = (1u128 << 61) - 1; // Mersenne prime > any 60-bit key

#[derive(Debug, Clone, Copy)]
struct H { a: u64, b: u64 }

impl H {
    fn random(rng: &mut Rng) -> Self { H { a: 1 + rng.next_u64() % (P as u64 - 1), b: rng.next_u64() % P as u64 } }
    fn apply(&self, k: u64, m: usize) -> usize { (((self.a as u128 * k as u128 + self.b as u128) % P) % m as u128) as usize }
}

#[derive(Debug, Clone)]
struct Bucket { h: H, slots: Vec<Option<u64>> }

/// Static perfect hash table: level one splits n keys into n buckets; bucket j gets n_j² slots and a collision-free hash.
#[derive(Debug, Clone)]
pub struct PerfectHash {
    top: H,
    m: usize,
    buckets: Vec<Bucket>,
    pub top_level_retries: usize,
}

impl PerfectHash {
    pub fn build(keys: &[u64], rng: &mut Rng) -> Self {
        let mut ks = keys.to_vec();
        ks.sort();
        ks.dedup();
        let n = ks.len().max(1);
        let mut retries = 0;
        let (top, groups) = loop {
            let h = H::random(rng);
            let mut groups: Vec<Vec<u64>> = vec![vec![]; n];
            for &k in &ks { groups[h.apply(k, n)].push(k); }
            // Theorem 11.9: E[Σ n_j²] < 2n, so by Markov Σ n_j² < 4n with probability ≥ 1/2
            if groups.iter().map(|g| g.len() * g.len()).sum::<usize>() < 4 * n { break (h, groups); }
            retries += 1;
        };
        let buckets = groups.iter().map(|g| {
            let size = g.len() * g.len();
            if size == 0 { return Bucket { h: H { a: 1, b: 0 }, slots: vec![] }; }
            loop {
                let h = H::random(rng);
                let mut slots = vec![None; size];
                let ok = g.iter().all(|&k| { let i = h.apply(k, size); if slots[i].is_some() { false } else { slots[i] = Some(k); true } });
                if ok { return Bucket { h, slots }; }
            }
        }).collect();
        PerfectHash { top, m: n, buckets, top_level_retries: retries }
    }
    /// Worst-case O(1): two hash evaluations and one comparison.
    pub fn contains(&self, k: u64) -> bool {
        let b = &self.buckets[self.top.apply(k, self.m)];
        !b.slots.is_empty() && b.slots[b.h.apply(k, b.slots.len())] == Some(k)
    }
    pub fn total_slots(&self) -> usize { self.m + self.buckets.iter().map(|b| b.slots.len()).sum::<usize>() }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(2);
    let keys: Vec<u64> = (0..1000).map(|_| r.next_u64() >> 8).collect();
    let ph = PerfectHash::build(&keys, &mut r);
    outln!(o, "built a perfect hash for {} keys; top-level retries = {}", keys.len(), ph.top_level_retries);
    outln!(o, "total slots = {} (≤ 5n = 5000 guaranteed by the analysis)", ph.total_slots());
    outln!(o, "all keys found: {}", keys.iter().all(|&k| ph.contains(k)));
    outln!(o, "absent key found? {}", ph.contains(12345));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_membership_and_linear_space() {
        let mut r = Rng::new(58);
        for n in [1usize, 2, 10, 300, 2000] {
            let keys: Vec<u64> = (0..n).map(|_| r.next_u64() >> 8).collect();
            let ph = PerfectHash::build(&keys, &mut r);
            assert!(keys.iter().all(|&k| ph.contains(k)));
            assert!(ph.total_slots() <= 5 * n + 5, "n={n} slots={}", ph.total_slots());
            for _ in 0..200 {
                let q = r.next_u64() >> 8;
                assert_eq!(ph.contains(q), keys.contains(&q));
            }
        }
    }
    #[test]
    fn empty_and_duplicates() {
        let mut r = Rng::new(59);
        let ph = PerfectHash::build(&[], &mut r);
        assert!(!ph.contains(5));
        let ph = PerfectHash::build(&[3, 3, 3], &mut r);
        assert!(ph.contains(3) && !ph.contains(4));
    }
}
