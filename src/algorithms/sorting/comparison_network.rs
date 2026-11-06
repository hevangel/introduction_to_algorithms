//! @title Comparison Networks & the 0-1 Principle
//! @chapter 27.1 (1e, 2e); 0-1 principle 27.1
//! @editions 1,2
//! @summary Fixed sequences of compare-exchange gates; test any network exhaustively on 0-1 inputs.
//! @order 130
use crate::outln;

/// A comparison network: `comparators[k] = (i, j)` with `i < j` puts the min on wire i and the max on wire j.
#[derive(Clone, Debug)]
pub struct Network {
    pub wires: usize,
    pub comparators: Vec<(usize, usize)>,
}

impl Network {
    pub fn new(wires: usize) -> Self { Network { wires, comparators: vec![] } }
    pub fn add(&mut self, i: usize, j: usize) {
        assert!(i < j && j < self.wires);
        self.comparators.push((i, j));
    }
    pub fn apply<T: PartialOrd>(&self, a: &mut [T]) {
        for &(i, j) in &self.comparators {
            if a[i] > a[j] { a.swap(i, j); }
        }
    }
    pub fn size(&self) -> usize { self.comparators.len() }
    /// Depth = longest chain of comparators (parallel running time).
    pub fn depth(&self) -> usize {
        let mut t = vec![0usize; self.wires];
        for &(i, j) in &self.comparators {
            let d = t[i].max(t[j]) + 1;
            t[i] = d;
            t[j] = d;
        }
        t.into_iter().max().unwrap_or(0)
    }
    /// 0-1 principle: a network sorts all inputs iff it sorts all 2ⁿ 0-1 sequences (n ≤ 22).
    pub fn is_sorting_network(&self) -> bool {
        assert!(self.wires <= 22);
        (0u32..1 << self.wires).all(|bits| {
            let mut v: Vec<u8> = (0..self.wires).map(|b| ((bits >> b) & 1) as u8).collect();
            self.apply(&mut v);
            v.windows(2).all(|w| w[0] <= w[1])
        })
    }
}

/// Insertion sort as a comparison network (depth 2n−3, size n(n−1)/2).
pub fn insertion_network(n: usize) -> Network {
    let mut net = Network::new(n);
    for i in 1..n {
        for j in (0..i).rev() { net.add(j, j + 1); }
    }
    net
}

/// Odd-even transposition network: n rounds of alternating adjacent comparators (depth n).
pub fn odd_even_transposition(n: usize) -> Network {
    let mut net = Network::new(n);
    for round in 0..n {
        for i in (round % 2..n.saturating_sub(1)).step_by(2) { net.add(i, i + 1); }
    }
    net
}

pub fn demo() -> String {
    let mut o = String::new();
    for n in [4, 6, 8] {
        let ins = insertion_network(n);
        let oet = odd_even_transposition(n);
        outln!(o, "n={n}: insertion net size={:>2} depth={:>2} sorts={}   odd-even transposition size={:>2} depth={:>2} sorts={}",
            ins.size(), ins.depth(), ins.is_sorting_network(), oet.size(), oet.depth(), oet.is_sorting_network());
    }
    let mut bad = Network::new(3);
    bad.add(0, 1);
    outln!(o, "one comparator on 3 wires sorts? {}", bad.is_sorting_network());
    let mut v = [9, 4, 7, 1];
    insertion_network(4).apply(&mut v);
    outln!(o, "apply to [9,4,7,1] -> {:?}", v);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn known_networks_sort() {
        for n in 1..=9 {
            assert!(insertion_network(n).is_sorting_network(), "insertion {n}");
            assert!(odd_even_transposition(n).is_sorting_network(), "oet {n}");
        }
    }
    #[test]
    fn depth_and_size() {
        let n = insertion_network(8);
        assert_eq!(n.size(), 28);
        assert_eq!(n.depth(), 13);
        assert_eq!(odd_even_transposition(8).depth(), 8);
    }
    #[test]
    fn zero_one_principle_agrees_with_integers() {
        let net = odd_even_transposition(8);
        let mut r = Rng::new(37);
        for _ in 0..200 {
            let mut v = r.vec(8, -100, 100);
            let mut e = v.clone();
            e.sort();
            net.apply(&mut v);
            assert_eq!(v, e);
        }
        let mut bad = Network::new(4);
        bad.add(0, 1);
        bad.add(2, 3);
        assert!(!bad.is_sorting_network());
    }
}
