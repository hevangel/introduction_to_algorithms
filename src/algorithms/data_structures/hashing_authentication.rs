//! @title Hashing and Authentication
//! @chapter Problem 11-4 (3e)
//! @editions 3,4
//! @summary A secret key selects h from a universal family; the tag h(m) authenticates m: for the family h_{a,b}(m) = (a·m + b) mod p an attacker who has seen one (message, tag) pair can forge another with probability only 1/p.
//! @order 60
use crate::outln;

/// The pairwise-independent family h_{a,b}(m) = (a·m + b) mod p, p prime, a ≠ 0 allowed too (a, b ∈ Z_p).
#[derive(Debug, Clone, Copy)]
pub struct Key { pub a: u64, pub b: u64, pub p: u64 }

impl Key {
    /// Tag of a single-block message m ∈ Z_p.
    pub fn tag(&self, m: u64) -> u64 { (self.a * (m % self.p) + self.b) % self.p }
    /// Tag of a multi-block message (m₁, …, m_k) over Z_p: Σ m_i·a^i + b — a polynomial family; two distinct messages of ≤ k blocks collide
    /// for at most k − 1 values of a (a non-zero polynomial of degree ≤ k − 1 has at most k − 1 roots).
    pub fn tag_blocks(&self, blocks: &[u64]) -> u64 {
        let (mut acc, mut pow) = (0u64, 1u64);
        for &m in blocks { pow = pow * self.a % self.p; acc = (acc + (m % self.p) * pow) % self.p; }
        (acc + self.b) % self.p
    }
}

/// Number of keys (a, b) that are consistent with having seen tag t for message m **and** would give tag t2 for message m2.
/// Returns (keys consistent with the first observation, of those, keys that also forge (m2, t2)).
pub fn forgery_counts(p: u64, m: u64, t: u64, m2: u64, t2: u64) -> (usize, usize) {
    let mut seen = 0;
    let mut forged = 0;
    for a in 0..p { for b in 0..p {
        let k = Key { a, b, p };
        if k.tag(m) == t { seen += 1; if k.tag(m2) == t2 { forged += 1; } }
    } }
    (seen, forged)
}

/// For every pair of distinct multi-block messages, the number of values of a for which the (b-less) polynomial hashes collide.
pub fn collisions_over_a(p: u64, x: &[u64], y: &[u64]) -> usize { (0..p).filter(|&a| Key { a, b: 0, p }.tag_blocks(x) == Key { a, b: 0, p }.tag_blocks(y)).count() }

pub fn demo() -> String {
    let mut o = String::new();
    let p = 31;
    let (seen, forged) = forgery_counts(p, 5, 17, 9, 3);
    outln!(o, "p = {p}: after seeing tag 17 for m = 5 there are {seen} consistent keys; {forged} of them also tag m′ = 9 with 3 → forgery probability {forged}/{seen} = 1/{}", seen / forged.max(1));
    let (x, y) = ([3u64, 1, 4], [3u64, 1, 5]);
    outln!(o, "multi-block messages {:?} vs {:?}: collide for {} of {p} values of a (bound k − 1 = 2 … k = {})", x, y, collisions_over_a(p, &x, &y), x.len());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn the_linear_family_is_pairwise_independent() {
        let p = 23u64;
        let mut r = Rng::new(492);
        for _ in 0..100 {
            let (m, m2) = (r.range(0, p as usize - 1) as u64, r.range(0, p as usize - 1) as u64);
            if m == m2 { continue; }
            let (t, t2) = (r.range(0, p as usize - 1) as u64, r.range(0, p as usize - 1) as u64);
            let (seen, forged) = forgery_counts(p, m, t, m2, t2);
            assert_eq!(seen as u64, p, "exactly p keys are consistent with one observed (m, t)");
            assert_eq!(forged, 1, "and exactly one of them forges any second pair: probability 1/p");
        }
    }
    #[test]
    fn distinct_messages_of_k_blocks_collide_for_at_most_k_minus_one_values_of_a() {
        let mut r = Rng::new(493);
        let p = 31u64;
        for _ in 0..300 {
            let k = r.range(1, 5);
            let x: Vec<u64> = (0..k).map(|_| r.range(0, 30) as u64).collect();
            let y: Vec<u64> = (0..k).map(|_| r.range(0, 30) as u64).collect();
            if x == y { continue; }
            assert!(collisions_over_a(p, &x, &y) <= k, "k = {k}"); // degree ≤ k polynomial a·(...) has the root a = 0 plus at most k − 1 others
        }
    }
}
