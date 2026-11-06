//! @title Open Addressing
//! @chapter 11.4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Store all keys in the table itself; linear probing, quadratic probing and double hashing.
//! @order 80
use crate::outln;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Probe { Linear, Quadratic, Double }

#[derive(Debug, Clone, Copy, PartialEq)]
enum Slot { Empty, Deleted, Full(u64) }

/// Open-addressing table; `m` is a power of two so the probe sequences below visit every slot.
#[derive(Debug, Clone)]
pub struct OpenAddressTable {
    slots: Vec<Slot>,
    probe: Probe,
    n: usize,
}

impl OpenAddressTable {
    pub fn new(m: usize, probe: Probe) -> Self {
        assert!(m.is_power_of_two());
        OpenAddressTable { slots: vec![Slot::Empty; m], probe, n: 0 }
    }
    fn h1(&self, k: u64) -> usize { ((k.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 29) as usize) & (self.slots.len() - 1) }
    fn h2(&self, k: u64) -> usize { (((k.wrapping_mul(0xC2B2_AE3D_27D4_EB4F) >> 31) as usize) | 1) & (self.slots.len() - 1) } // odd ⇒ coprime to m
    /// Probe sequence h(k, i).
    fn at(&self, k: u64, i: usize) -> usize {
        let m = self.slots.len();
        match self.probe {
            Probe::Linear => (self.h1(k) + i) & (m - 1),
            Probe::Quadratic => (self.h1(k) + (i + i * i) / 2) & (m - 1), // c1 = c2 = ½ visits all slots when m = 2^p
            Probe::Double => (self.h1(k) + i * self.h2(k)) & (m - 1),
        }
    }
    pub fn len(&self) -> usize { self.n }
    pub fn is_empty(&self) -> bool { self.n == 0 }
    pub fn load_factor(&self) -> f64 { self.n as f64 / self.slots.len() as f64 }
    /// HASH-INSERT: returns the number of probes used (Err on overflow). Duplicate keys are ignored.
    pub fn insert(&mut self, k: u64) -> Result<usize, &'static str> {
        if self.search(k).is_some() { return Ok(0); }
        for i in 0..self.slots.len() {
            let j = self.at(k, i);
            if matches!(self.slots[j], Slot::Empty | Slot::Deleted) {
                self.slots[j] = Slot::Full(k);
                self.n += 1;
                return Ok(i + 1);
            }
        }
        Err("hash table overflow")
    }
    /// HASH-SEARCH: (slot, probes) if present.
    pub fn search(&self, k: u64) -> Option<(usize, usize)> {
        for i in 0..self.slots.len() {
            let j = self.at(k, i);
            match self.slots[j] {
                Slot::Empty => return None,
                Slot::Full(x) if x == k => return Some((j, i + 1)),
                _ => {}
            }
        }
        None
    }
    /// Deletion marks the slot DELETED (not NIL) so later searches do not stop early.
    pub fn delete(&mut self, k: u64) -> bool {
        if let Some((j, _)) = self.search(k) {
            self.slots[j] = Slot::Deleted;
            self.n -= 1;
            true
        } else {
            false
        }
    }
}

/// Average probes of successful searches after filling to load factor `alpha`.
pub fn average_successful_probes(probe: Probe, m: usize, alpha: f64, seed: u64) -> f64 {
    let mut r = crate::common::Rng::new(seed);
    let mut t = OpenAddressTable::new(m, probe);
    let keys: Vec<u64> = (0..(alpha * m as f64) as usize).map(|_| r.next_u64()).collect();
    for &k in &keys { t.insert(k).unwrap(); }
    keys.iter().map(|&k| t.search(k).unwrap().1 as f64).sum::<f64>() / keys.len() as f64
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "average probes for a successful search (m = 4096)");
    outln!(o, "  α   | linear | quadratic | double | uniform-hashing theory (1/α)ln(1/(1−α))");
    for alpha in [0.5f64, 0.75, 0.9] {
        let th = (1.0 / alpha) * (1.0 / (1.0 - alpha)).ln();
        outln!(o, " {:.2} | {:>6.2} | {:>9.2} | {:>6.2} | {:.2}",
            alpha,
            average_successful_probes(Probe::Linear, 4096, alpha, 1),
            average_successful_probes(Probe::Quadratic, 4096, alpha, 1),
            average_successful_probes(Probe::Double, 4096, alpha, 1), th);
    }
    let mut t = OpenAddressTable::new(8, Probe::Linear);
    for k in [10, 18, 26] { t.insert(k).unwrap(); }
    outln!(o, "deleted flag keeps later keys reachable: delete(10)={}, search(26)={:?}", t.delete(10), t.search(26).is_some());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn all_strategies_behave_like_a_set() {
        for probe in [Probe::Linear, Probe::Quadratic, Probe::Double] {
            let mut t = OpenAddressTable::new(256, probe);
            let mut s = HashSet::new();
            let mut r = crate::common::Rng::new(57);
            for _ in 0..4000 {
                let k = r.range(0, 400) as u64;
                match r.range(0, 2) {
                    0 => if s.len() < 200 { t.insert(k).unwrap(); s.insert(k); },
                    1 => assert_eq!(t.delete(k), s.remove(&k)),
                    _ => assert_eq!(t.search(k).is_some(), s.contains(&k)),
                }
            }
            assert_eq!(t.len(), s.len());
        }
    }
    #[test]
    fn table_can_be_filled_completely_and_then_overflows() {
        for probe in [Probe::Linear, Probe::Quadratic, Probe::Double] {
            let mut t = OpenAddressTable::new(64, probe);
            for k in 0..64u64 { t.insert(k * 7919).unwrap(); }
            assert_eq!(t.insert(999_983), Err("hash table overflow"));
            for k in 0..64u64 { assert!(t.search(k * 7919).is_some()); }
        }
    }
    #[test]
    fn double_hashing_beats_linear_at_high_load() {
        let lin = average_successful_probes(Probe::Linear, 4096, 0.9, 3);
        let dbl = average_successful_probes(Probe::Double, 4096, 0.9, 3);
        assert!(dbl < lin);
    }
}
