//! @title Hash Tables with Chaining
//! @chapter 11.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Resolve collisions with linked lists per slot; expected Θ(1+α) per operation under simple uniform hashing.
//! @order 60
use crate::outln;

/// Hash table with collision resolution by chaining. Keys are `u64`.
#[derive(Debug, Clone)]
pub struct ChainedHashTable<V> {
    slots: Vec<Vec<(u64, V)>>,
    n: usize,
}

impl<V> ChainedHashTable<V> {
    pub fn new(m: usize) -> Self { ChainedHashTable { slots: (0..m).map(|_| Vec::new()).collect(), n: 0 } }
    fn h(&self, k: u64) -> usize {
        // multiplicative mixing then reduction: behaves like simple uniform hashing for our tests
        ((k.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 32) % self.slots.len() as u64) as usize
    }
    pub fn len(&self) -> usize { self.n }
    pub fn is_empty(&self) -> bool { self.n == 0 }
    /// Load factor α = n/m.
    pub fn load_factor(&self) -> f64 { self.n as f64 / self.slots.len() as f64 }
    /// CHAINED-HASH-INSERT: Θ(1) — insert at the head of the chain (no duplicate check).
    pub fn insert(&mut self, k: u64, v: V) {
        let i = self.h(k);
        self.slots[i].insert(0, (k, v));
        self.n += 1;
    }
    /// CHAINED-HASH-SEARCH: expected Θ(1 + α).
    pub fn search(&self, k: u64) -> Option<&V> {
        self.slots[self.h(k)].iter().find(|(key, _)| *key == k).map(|(_, v)| v)
    }
    /// CHAINED-HASH-DELETE
    pub fn delete(&mut self, k: u64) -> Option<V> {
        let i = self.h(k);
        let pos = self.slots[i].iter().position(|(key, _)| *key == k)?;
        self.n -= 1;
        Some(self.slots[i].remove(pos).1)
    }
    pub fn chain_lengths(&self) -> Vec<usize> { self.slots.iter().map(|c| c.len()).collect() }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = ChainedHashTable::new(8);
    for k in 1..=20u64 { t.insert(k * 37, k); }
    outln!(o, "n = {}, m = 8, load factor α = {:.2}", t.len(), t.load_factor());
    outln!(o, "chain lengths: {:?}", t.chain_lengths());
    outln!(o, "search(37*5) = {:?}, search(1) = {:?}", t.search(37 * 5), t.search(1));
    outln!(o, "delete(37*5) = {:?}, then search = {:?}", t.delete(37 * 5), t.search(37 * 5));
    let mut r = crate::common::Rng::new(9);
    for alpha in [0.5, 1.0, 2.0, 4.0] {
        let m = 1000;
        let mut big = ChainedHashTable::new(m);
        let n = (alpha * m as f64) as usize;
        for _ in 0..n { big.insert(r.next_u64(), ()); }
        let lens = big.chain_lengths();
        let max = lens.iter().max().unwrap();
        outln!(o, "α={alpha:>3}: longest chain = {max}, empty slots = {}", lens.iter().filter(|&&l| l == 0).count());
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn behaves_like_a_map() {
        let mut t = ChainedHashTable::new(13);
        let mut m = HashMap::new();
        let mut r = crate::common::Rng::new(53);
        for i in 0..3000u64 {
            let k = r.range(0, 200) as u64;
            match r.range(0, 2) {
                0 => if !m.contains_key(&k) { t.insert(k, i); m.insert(k, i); },
                1 => assert_eq!(t.delete(k), m.remove(&k)),
                _ => assert_eq!(t.search(k), m.get(&k)),
            }
        }
        assert_eq!(t.len(), m.len());
    }
    #[test]
    fn chains_are_short_for_uniform_keys() {
        let mut t = ChainedHashTable::new(1000);
        let mut r = crate::common::Rng::new(54);
        for _ in 0..1000 { t.insert(r.next_u64(), ()); }
        assert!(*t.chain_lengths().iter().max().unwrap() <= 10);
    }
}
