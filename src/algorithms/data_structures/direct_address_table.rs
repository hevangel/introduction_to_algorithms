//! @title Direct-Address Tables & Bit Vectors
//! @chapter 11.1 (1e–4e)
//! @editions 1,2,3,4
//! @summary An array indexed by key: O(1) search/insert/delete when the universe is small; bit-vector sets.
//! @order 50
use crate::outln;

/// DIRECT-ADDRESS table over the universe U = {0, …, m−1}.
#[derive(Debug, Clone)]
pub struct DirectAddressTable<V> {
    slots: Vec<Option<V>>,
}

impl<V> DirectAddressTable<V> {
    pub fn new(universe: usize) -> Self { DirectAddressTable { slots: (0..universe).map(|_| None).collect() } }
    pub fn search(&self, k: usize) -> Option<&V> { self.slots[k].as_ref() }
    pub fn insert(&mut self, k: usize, v: V) { self.slots[k] = Some(v); }
    pub fn delete(&mut self, k: usize) -> Option<V> { self.slots[k].take() }
}

/// Exercise 11.1-2: a dynamic set of distinct integers from {0..m−1} stored as a bit vector.
#[derive(Debug, Clone)]
pub struct BitVectorSet {
    words: Vec<u64>,
    m: usize,
}

impl BitVectorSet {
    pub fn new(m: usize) -> Self { BitVectorSet { words: vec![0; m.div_ceil(64)], m } }
    pub fn insert(&mut self, k: usize) { self.words[k / 64] |= 1 << (k % 64); }
    pub fn delete(&mut self, k: usize) { self.words[k / 64] &= !(1 << (k % 64)); }
    pub fn contains(&self, k: usize) -> bool { self.words[k / 64] >> (k % 64) & 1 == 1 }
    /// Largest element, found by scanning words from the top (Exercise 11.1-1).
    pub fn maximum(&self) -> Option<usize> {
        for (w, &word) in self.words.iter().enumerate().rev() {
            if word != 0 { return Some(w * 64 + 63 - word.leading_zeros() as usize); }
        }
        None
    }
    pub fn len(&self) -> usize { self.words.iter().map(|w| w.count_ones() as usize).sum() }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    pub fn universe(&self) -> usize { self.m }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = DirectAddressTable::new(10);
    t.insert(3, "three");
    t.insert(7, "seven");
    outln!(o, "search(3) = {:?}, search(4) = {:?}", t.search(3), t.search(4));
    outln!(o, "delete(3) = {:?}, search(3) = {:?}", t.delete(3), t.search(3));
    let mut s = BitVectorSet::new(1000);
    for k in [5, 63, 64, 999, 500] { s.insert(k); }
    outln!(o, "bit-vector set holds {} elements, maximum = {:?}, universe {} bits in {} words", s.len(), s.maximum(), s.universe(), 1000usize.div_ceil(64));
    s.delete(999);
    outln!(o, "after deleting 999, maximum = {:?}", s.maximum());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn table_operations() {
        let mut t = DirectAddressTable::new(5);
        assert!(t.search(2).is_none());
        t.insert(2, 'x');
        assert_eq!(t.search(2), Some(&'x'));
        assert_eq!(t.delete(2), Some('x'));
        assert_eq!(t.delete(2), None);
    }
    #[test]
    fn bit_vector_matches_btreeset() {
        let mut s = BitVectorSet::new(300);
        let mut b = BTreeSet::new();
        let mut r = crate::common::Rng::new(52);
        for _ in 0..3000 {
            let k = r.range(0, 299);
            if r.range(0, 2) == 0 { s.insert(k); b.insert(k); } else { s.delete(k); b.remove(&k); }
            assert_eq!(s.maximum(), b.iter().next_back().copied());
            assert_eq!(s.contains(k), b.contains(&k));
        }
        assert_eq!(s.len(), b.len());
    }
}
