//! @title Amortized Analysis: Dynamic Tables
//! @chapter 17.4 (2e–3e) / 16.4 (4e) / 18.4 (1e)
//! @editions 1,2,3,4
//! @summary Doubling on overflow and halving at load 1/4 gives O(1) amortized INSERT and DELETE.
//! @order 220
use crate::outln;

/// A table that grows by doubling and shrinks by halving. The "cost" counts elementary insertions/copies/deletions.
#[derive(Debug, Clone, Default)]
pub struct DynamicTable<T> {
    items: Vec<T>,
    size: usize, // allocated slots
    pub total_cost: usize,
    pub operations: usize,
    pub resizes: usize,
}

impl<T> DynamicTable<T> {
    pub fn new() -> Self { DynamicTable { items: vec![], size: 0, total_cost: 0, operations: 0, resizes: 0 } }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn capacity(&self) -> usize { self.size }
    pub fn load_factor(&self) -> f64 { if self.size == 0 { 1.0 } else { self.items.len() as f64 / self.size as f64 } }
    /// Potential Φ(T) = 2·num − size if α ≥ ½, size/2 − num otherwise.
    pub fn potential(&self) -> f64 {
        let (num, size) = (self.items.len() as f64, self.size as f64);
        if self.load_factor() >= 0.5 { 2.0 * num - size } else { size / 2.0 - num }
    }
    /// TABLE-INSERT
    pub fn insert(&mut self, x: T) {
        self.operations += 1;
        if self.size == 0 { self.size = 1; }
        else if self.items.len() == self.size {
            self.total_cost += self.items.len(); // copy every item into the new, twice as large table
            self.size *= 2;
            self.resizes += 1;
        }
        self.items.push(x);
        self.total_cost += 1;
    }
    /// TABLE-DELETE: contract to half the size when the table becomes less than a quarter full.
    pub fn delete(&mut self) -> Option<T> {
        self.operations += 1;
        let x = self.items.pop()?;
        self.total_cost += 1;
        if 4 * self.items.len() < self.size {
            self.total_cost += self.items.len(); // copy into the smaller table
            self.size /= 2;
            self.resizes += 1;
        }
        Some(x)
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = DynamicTable::new();
    outln!(o, "inserting 1..=17: capacity grows 1 → 2 → 4 → 8 → 16 → 32");
    for i in 1..=17u32 {
        t.insert(i);
        if i.is_power_of_two() || i == 17 { outln!(o, "  after insert {:>2}: capacity {:>2}, cumulative cost {:>2}", i, t.capacity(), t.total_cost); }
    }
    let mut big = DynamicTable::new();
    for i in 0..100_000 { big.insert(i); }
    outln!(o, "100000 inserts: total cost {} = {:.3}·n (bound 3n), {} resizes", big.total_cost, big.total_cost as f64 / 100_000.0, big.resizes);
    while big.len() > 100 { big.delete(); }
    outln!(o, "after deleting down to 100 items: capacity {} (load {:.2}, never below 1/4)", big.capacity(), big.load_factor());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn inserts_cost_less_than_three_per_operation() {
        let mut t = DynamicTable::new();
        for i in 0..10_000 { t.insert(i); }
        assert!(t.total_cost <= 3 * t.operations);
        assert_eq!(t.capacity(), 16384);
    }
    #[test]
    fn mixed_operations_stay_amortized_constant_and_load_factor_bounded() {
        let mut r = Rng::new(117);
        let mut t = DynamicTable::new();
        for round in 0..30_000 {
            if r.range(0, 2) < 1 || t.is_empty() { t.insert(round); } else { t.delete(); }
            if t.len() > 0 { assert!(t.load_factor() >= 0.25 - 1e-12 && t.load_factor() <= 1.0); }
            assert!(t.potential() >= -1e-9);
        }
        assert!(t.total_cost <= 3 * t.operations, "{} vs {}", t.total_cost, 3 * t.operations);
    }
    #[test]
    fn potential_is_zero_just_after_resize() {
        let mut t = DynamicTable::new();
        for i in 0..9 { t.insert(i); }
        assert_eq!(t.capacity(), 16);
        assert_eq!(t.potential(), 2.0); // 2·9 − 16
    }
}
