//! @title Amortized Analysis: Stack with MULTIPOP
//! @chapter 17.1–17.3 (2e–3e) / 16 (4e) / 18 (1e)
//! @editions 1,2,3,4
//! @summary PUSH, POP and MULTIPOP(k) all cost O(1) amortized: a stack object pays for its own removal.
//! @order 210
use crate::outln;

/// Stack with the extra MULTIPOP operation. Costs count elementary push/pop steps.
#[derive(Debug, Clone, Default)]
pub struct MultipopStack<T> { items: Vec<T>, pub total_cost: usize, pub operations: usize }

impl<T> MultipopStack<T> {
    pub fn new() -> Self { MultipopStack { items: vec![], total_cost: 0, operations: 0 } }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn push(&mut self, x: T) { self.items.push(x); self.total_cost += 1; self.operations += 1; }
    pub fn pop(&mut self) -> Option<T> { self.operations += 1; self.total_cost += 1; self.items.pop() }
    /// MULTIPOP(S, k): pop min(|S|, k) objects, actual cost = number popped.
    pub fn multipop(&mut self, k: usize) -> usize {
        self.operations += 1;
        let mut c = 0;
        while c < k && self.items.pop().is_some() { c += 1; }
        self.total_cost += c;
        c
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut s = MultipopStack::new();
    let mut r = crate::common::Rng::new(4);
    let n = 10_000;
    let mut worst_single = 0;
    for _ in 0..n {
        match r.range(0, 2) {
            0 | 1 => s.push(0u8),
            _ => { let c = s.multipop(r.range(1, 50)); worst_single = worst_single.max(c); }
        }
    }
    outln!(o, "{} operations: total actual cost = {}, average = {:.3} (≤ 2 by amortized analysis)", s.operations, s.total_cost, s.total_cost as f64 / s.operations as f64);
    outln!(o, "most expensive single MULTIPOP = {worst_single}, yet the average is O(1)");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn total_cost_at_most_twice_the_operations() {
        let mut r = Rng::new(116);
        for _ in 0..20 {
            let mut s = MultipopStack::new();
            for i in 0..2000 {
                match r.range(0, 3) { 0 | 1 => s.push(i), 2 => { s.pop(); } _ => { s.multipop(r.range(0, 30)); } }
            }
            assert!(s.total_cost <= 2 * s.operations, "{} > 2·{}", s.total_cost, s.operations);
        }
    }
    #[test]
    fn multipop_semantics() {
        let mut s = MultipopStack::new();
        for i in 0..5 { s.push(i); }
        assert_eq!(s.multipop(3), 3);
        assert_eq!(s.multipop(10), 2);
        assert!(s.is_empty());
        assert_eq!(s.pop(), None);
    }
}
