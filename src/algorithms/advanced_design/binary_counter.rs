//! @title Amortized Analysis: Binary Counter
//! @chapter 17.1–17.3 (2e–3e) / 16 (4e) / 18 (1e)
//! @editions 1,2,3,4
//! @summary k-bit counter INCREMENT: aggregate, accounting and potential methods give 2 amortized flips per increment.
//! @order 200
use crate::outln;

/// A k-bit binary counter (least-significant bit first).
#[derive(Debug, Clone)]
pub struct BinaryCounter { bits: Vec<bool>, pub total_cost: usize }

impl BinaryCounter {
    pub fn new(k: usize) -> Self { BinaryCounter { bits: vec![false; k], total_cost: 0 } }
    pub fn value(&self) -> u128 { self.bits.iter().enumerate().map(|(i, &b)| (b as u128) << i).sum() }
    /// Number of 1 bits — the potential Φ.
    pub fn ones(&self) -> usize { self.bits.iter().filter(|&&b| b).count() }
    /// INCREMENT: returns the actual cost (bits flipped).
    pub fn increment(&mut self) -> usize {
        let mut i = 0;
        while i < self.bits.len() && self.bits[i] { self.bits[i] = false; i += 1; }
        if i < self.bits.len() { self.bits[i] = true; i += 1; }
        self.total_cost += i;
        i
    }
    /// DECREMENT (Exercise 17.1-3 setting): returns the flips; wraps at 0.
    pub fn decrement(&mut self) -> usize {
        let mut i = 0;
        while i < self.bits.len() && !self.bits[i] { self.bits[i] = true; i += 1; }
        if i < self.bits.len() { self.bits[i] = false; i += 1; }
        self.total_cost += i;
        i
    }
}

/// Potential-method amortized cost of one increment: actual + ΔΦ.
pub fn amortized_cost(c: &mut BinaryCounter) -> i64 {
    let before = c.ones() as i64;
    let actual = c.increment() as i64;
    actual + c.ones() as i64 - before
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut c = BinaryCounter::new(8);
    outln!(o, " n | value bits | actual cost | total | Φ (ones) | amortized");
    for n in 1..=16 {
        let ones_before = c.ones();
        let cost = c.increment();
        outln!(o, "{:>2} | {:>10} | {:>11} | {:>5} | {:>8} | {}", n, c.value(), cost, c.total_cost, c.ones(), cost as i64 + c.ones() as i64 - ones_before as i64);
    }
    let mut big = BinaryCounter::new(32);
    for _ in 0..100_000 { big.increment(); }
    outln!(o, "100000 increments on a 32-bit counter cost {} flips (< 2n = 200000)", big.total_cost);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn amortized_cost_is_at_most_two() {
        let mut c = BinaryCounter::new(20);
        for _ in 0..5000 { assert!(amortized_cost(&mut c) <= 2); }
    }
    #[test]
    fn total_flips_formula() {
        let mut c = BinaryCounter::new(20);
        for n in 1..=3000u64 {
            c.increment();
            assert_eq!(c.value(), n as u128);
            assert_eq!(c.total_cost as u64, 2 * n - n.count_ones() as u64);
        }
    }
    #[test]
    fn decrement_undoes_increment() {
        let mut c = BinaryCounter::new(10);
        for _ in 0..37 { c.increment(); }
        for _ in 0..37 { c.decrement(); }
        assert_eq!(c.value(), 0);
        // alternating increment/decrement at 2^k−1 ↔ 2^k is the worst case: Θ(k) per op
        let mut d = BinaryCounter::new(10);
        for _ in 0..511 { d.increment(); }
        let before = d.total_cost;
        for _ in 0..10 { d.increment(); d.decrement(); }
        assert!(d.total_cost - before >= 10 * 2 * 9);
    }
}
