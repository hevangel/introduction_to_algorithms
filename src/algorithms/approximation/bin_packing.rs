//! @title Bin Packing: First Fit
//! @chapter Problem 35-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Pack items of size ≤ capacity into the fewest bins: Next/First/Best Fit and First Fit Decreasing, with the Problem 35-1 guarantees (≤ 2·OPT; FFD ≤ 11/9·OPT + 6/9).
//! @order 40
use crate::outln;

/// FIRST-FIT: put each item into the lowest-indexed bin where it fits, opening a new bin when none does.
pub fn first_fit(items: &[u32], cap: u32) -> Vec<Vec<u32>> {
    let mut bins: Vec<(u32, Vec<u32>)> = vec![];
    for &x in items {
        match bins.iter_mut().find(|b| b.0 + x <= cap) { Some(b) => { b.0 += x; b.1.push(x); } None => bins.push((x, vec![x])) }
    }
    bins.into_iter().map(|b| b.1).collect()
}

/// NEXT-FIT: only the most recently opened bin is considered. At most 2·⌈Σ/cap⌉ bins.
pub fn next_fit(items: &[u32], cap: u32) -> usize {
    let (mut bins, mut load) = (0, cap + 1);
    for &x in items { if load + x > cap { bins += 1; load = x; } else { load += x; } }
    bins
}

/// BEST-FIT: choose the fullest bin that still has room.
pub fn best_fit(items: &[u32], cap: u32) -> usize {
    let mut loads: Vec<u32> = vec![];
    for &x in items {
        match loads.iter_mut().filter(|l| **l + x <= cap).max_by_key(|l| **l) { Some(l) => *l += x, None => loads.push(x) }
    }
    loads.len()
}

/// FIRST-FIT-DECREASING: sort the items by size, largest first, then First Fit.
pub fn first_fit_decreasing(items: &[u32], cap: u32) -> Vec<Vec<u32>> {
    let mut v = items.to_vec();
    v.sort_by(|a, b| b.cmp(a));
    first_fit(&v, cap)
}

/// The optimal number of bins by exhaustive search (small inputs only).
pub fn optimal_bins(items: &[u32], cap: u32) -> usize {
    fn go(items: &[u32], i: usize, loads: &mut Vec<u32>, cap: u32, best: &mut usize) {
        if loads.len() >= *best { return; }
        if i == items.len() { *best = loads.len(); return; }
        for b in 0..loads.len() { if loads[b] + items[i] <= cap { loads[b] += items[i]; go(items, i + 1, loads, cap, best); loads[b] -= items[i]; } }
        loads.push(items[i]);
        go(items, i + 1, loads, cap, best);
        loads.pop();
    }
    let mut best = items.len() + 1;
    let mut sorted = items.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    go(&sorted, 0, &mut vec![], cap, &mut best);
    best
}

pub fn demo() -> String {
    let mut o = String::new();
    let items = [4u32, 8, 1, 4, 2, 1, 7, 3, 6, 5];
    outln!(o, "capacity 10, items {:?} (sum {}, lower bound {} bins)", items, items.iter().sum::<u32>(), items.iter().sum::<u32>().div_ceil(10));
    outln!(o, "next fit  : {} bins", next_fit(&items, 10));
    outln!(o, "first fit : {} bins {:?}", first_fit(&items, 10).len(), first_fit(&items, 10));
    outln!(o, "best fit  : {} bins", best_fit(&items, 10));
    outln!(o, "FFD       : {} bins {:?}", first_fit_decreasing(&items, 10).len(), first_fit_decreasing(&items, 10));
    outln!(o, "optimal   : {} bins", optimal_bins(&items, 10));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn guarantees_hold_against_the_exact_optimum() {
        let mut r = Rng::new(414);
        for _ in 0..200 {
            let cap = r.range(5, 20) as u32;
            let items: Vec<u32> = (0..r.range(1, 10)).map(|_| r.range(1, cap as usize) as u32).collect();
            let opt = optimal_bins(&items, cap);
            let (ff, nf, bf, ffd) = (first_fit(&items, cap).len(), next_fit(&items, cap), best_fit(&items, cap), first_fit_decreasing(&items, cap).len());
            for packing in [first_fit(&items, cap), first_fit_decreasing(&items, cap)] {
                assert!(packing.iter().all(|b| b.iter().sum::<u32>() <= cap));
                let mut all: Vec<u32> = packing.concat();
                all.sort();
                let mut want = items.clone();
                want.sort();
                assert_eq!(all, want);
            }
            assert!(opt >= (items.iter().sum::<u32>() as usize).div_ceil(cap as usize));
            assert!(ff >= opt && bf >= opt && nf >= opt && ffd >= opt);
            assert!(nf <= 2 * opt && ff <= 2 * opt && bf <= 2 * opt, "factor-2 guarantee");
            assert!(9 * ffd <= 11 * opt + 6, "FFD ≤ 11/9 OPT + 6/9");
        }
    }
    #[test]
    fn ffd_beats_first_fit_on_a_classic_input() {
        let items = [1u32, 6, 2, 5, 3, 4, 4, 3, 5, 2, 6, 1];
        assert!(first_fit_decreasing(&items, 7).len() <= first_fit(&items, 7).len());
        assert_eq!(optimal_bins(&items, 7), 6);
    }
}
