//! @title Pointer Jumping (Finding Roots in a Forest)
//! @chapter 30.1 (1e), Exercises
//! @editions 1
//! @summary Every node repeatedly replaces its parent by its grandparent: all nodes find their tree's root in ⌈lg h⌉ synchronous rounds with O(n) processors.
//! @order 30
use crate::outln;

/// FIND-ROOTS: `parent[v] == v` marks a root. Synchronous rounds of `parent[v] ← parent[parent[v]]` until nothing changes.
/// Returns (root of each node, number of rounds that changed something).
pub fn find_roots(parent: &[usize]) -> (Vec<usize>, usize) {
    let mut p = parent.to_vec();
    let mut rounds = 0;
    loop {
        let next: Vec<usize> = (0..p.len()).map(|v| p[p[v]]).collect(); // all processors read the old array, then write
        if next == p { return (p, rounds); }
        p = next;
        rounds += 1;
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let chain: Vec<usize> = (0usize..16).map(|v| v.saturating_sub(1)).collect(); // 0 is the root, v → v−1
    let (roots, rounds) = find_roots(&chain);
    outln!(o, "chain of 16 nodes: roots {:?} after {rounds} rounds (⌈lg 15⌉ = 4)", roots);
    let forest = vec![0, 0, 1, 2, 4, 4, 5, 6, 8];
    outln!(o, "forest {:?} → {:?}", forest, find_roots(&forest));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn finds_every_root_in_logarithmic_rounds() {
        let mut r = Rng::new(434);
        for _ in 0..300 {
            let n = r.range(1, 60);
            let parent: Vec<usize> = (0..n).map(|v| if v == 0 || r.f64() < 0.1 { v } else { r.range(0, v - 1) }).collect();
            let walk = |mut v: usize| { let mut depth = 0; while parent[v] != v { v = parent[v]; depth += 1; } (v, depth) };
            let height = (0..n).map(|v| walk(v).1).max().unwrap();
            let (roots, rounds) = find_roots(&parent);
            for v in 0..n { assert_eq!(roots[v], walk(v).0); }
            assert!(rounds <= (height.max(1) as f64).log2().ceil() as usize + 1, "rounds {rounds} height {height}");
        }
    }
    #[test]
    fn chain_needs_exactly_lg_rounds() {
        let chain: Vec<usize> = (0usize..65).map(|v| v.saturating_sub(1)).collect();
        assert_eq!(find_roots(&chain).1, 6); // depth 64 = 2⁶
    }
}
