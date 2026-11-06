//! @title Comparison-Sort Lower Bound
//! @chapter 8.1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Any comparison sort needs ⌈lg n!⌉ comparisons in the worst case: measure real sorts against the decision-tree bound.
//! @order 125
use crate::outln;
use std::cell::Cell;
use std::cmp::Ordering;

thread_local! { static COUNT: Cell<usize> = const { Cell::new(0) }; }

/// A key whose every ordering comparison is counted — turns any generic sort into a decision-tree probe.
#[derive(Clone, Debug, PartialEq)]
pub struct Counted(pub usize);
impl PartialOrd for Counted {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> { COUNT.with(|c| c.set(c.get() + 1)); Some(self.0.cmp(&o.0)) }
}

pub fn all_permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 0 { return vec![vec![]]; }
    let mut out = vec![];
    for p in all_permutations(n - 1) { for pos in 0..=p.len() { let mut q = p.clone(); q.insert(pos, n - 1); out.push(q); } }
    out
}

/// ⌈lg(n!)⌉: the minimum height of a binary tree with n! leaves.
pub fn lower_bound(n: usize) -> usize {
    let f: u128 = (1..=n as u128).product();
    (128 - (f.max(1) - 1).leading_zeros()) as usize * (f > 1) as usize
}

/// (worst-case comparisons, average comparisons) of `sort` over all n! input orders.
pub fn measure(sort: fn(&mut [Counted]), n: usize) -> (usize, f64) {
    let perms = all_permutations(n);
    let (mut worst, mut total) = (0, 0usize);
    for p in &perms {
        let mut v: Vec<Counted> = p.iter().map(|&x| Counted(x)).collect();
        COUNT.with(|c| c.set(0));
        sort(&mut v);
        let c = COUNT.with(|c| c.get());
        assert!(v.windows(2).all(|w| w[0].0 < w[1].0));
        worst = worst.max(c);
        total += c;
    }
    (worst, total as f64 / perms.len() as f64)
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "  n | ⌈lg n!⌉ | insertion w/avg | merge w/avg | heapsort w/avg | quicksort w/avg");
    for n in 2..=7 {
        let ins = measure(|v| super::super::foundations::insertion_sort::insertion_sort(v), n);
        let mrg = measure(|v| super::super::foundations::merge_sort::merge_sort(v), n);
        let hp = measure(|v| super::heapsort::heapsort(v), n);
        let qs = measure(|v| super::quicksort::quicksort(v), n);
        outln!(o, "{:>3} | {:>7} | {:>6} {:>7.2} | {:>5} {:>6.2} | {:>6} {:>7.2} | {:>7} {:>7.2}", n, lower_bound(n), ins.0, ins.1, mrg.0, mrg.1, hp.0, hp.1, qs.0, qs.1);
    }
    outln!(o, "lg(n!) = n lg n − n lg e + O(lg n)  ⇒  Ω(n lg n); 2^h ≥ n! forces height h ≥ lg n!");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bound_values() {
        assert_eq!((1..=8).map(lower_bound).collect::<Vec<_>>(), vec![0, 1, 3, 5, 7, 10, 13, 16]);
    }
    #[test]
    fn no_comparison_sort_beats_the_decision_tree_bound() {
        let sorts: [fn(&mut [Counted]); 4] = [
            |v| super::super::super::foundations::insertion_sort::insertion_sort(v),
            |v| super::super::super::foundations::merge_sort::merge_sort(v),
            |v| super::super::heapsort::heapsort(v),
            |v| super::super::quicksort::quicksort(v),
        ];
        for n in 2..=7 {
            let lg_fact: f64 = (1..=n).map(|i| (i as f64).log2()).sum();
            for s in sorts {
                let (worst, avg) = measure(s, n);
                assert!(worst >= lower_bound(n), "worst case n={n}");
                assert!(avg >= lg_fact - 1e-9, "average case n={n}: {avg} < {lg_fact}");
            }
        }
    }
    #[test]
    fn merge_sort_is_tight_for_four_elements() {
        assert_eq!(measure(|v| super::super::super::foundations::merge_sort::merge_sort(v), 4).0, 5);
    }
}
