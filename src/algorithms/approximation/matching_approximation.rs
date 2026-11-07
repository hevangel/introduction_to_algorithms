//! @title Approximating Maximum Matching
//! @chapter Problem 35-4 (3e)
//! @editions 3,4
//! @summary Any maximal matching has at least half the edges of a maximum matching; sorting edges by weight and taking them greedily gives a ½-approximation for maximum-weight matching.
//! @order 52
use crate::outln;

pub type Edge = (usize, usize, i64);

/// Maximal (not maximum) matching: scan the edges once, take each edge whose endpoints are both free. Returns edge indices. ≥ ½ of the maximum cardinality.
pub fn greedy_maximal_matching(n: usize, edges: &[Edge]) -> Vec<usize> {
    let mut used = vec![false; n];
    let mut m = vec![];
    for (i, &(u, v, _)) in edges.iter().enumerate() { if u != v && !used[u] && !used[v] { used[u] = true; used[v] = true; m.push(i); } }
    m
}

/// Greedy for maximum-weight matching: consider edges by decreasing weight. Weight ≥ ½ of the optimum.
pub fn greedy_weighted_matching(n: usize, edges: &[Edge]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..edges.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(edges[i].2));
    let mut used = vec![false; n];
    order.into_iter().filter(|&i| { let (u, v, _) = edges[i]; if u != v && !used[u] && !used[v] { used[u] = true; used[v] = true; true } else { false } }).collect()
}

/// Exact maximum matching by enumeration of edge subsets (≤ 20 edges): returns (max cardinality, max weight).
pub fn optimum(n: usize, edges: &[Edge]) -> (usize, i64) {
    let mut best = (0, 0);
    for m in 0u32..1 << edges.len() {
        let mut used = vec![false; n];
        let (mut ok, mut w) = (true, 0);
        for (i, &(u, v, wt)) in edges.iter().enumerate() {
            if m >> i & 1 == 1 { if u == v || used[u] || used[v] { ok = false; break; } used[u] = true; used[v] = true; w += wt; }
        }
        if ok { best = (best.0.max(m.count_ones() as usize), best.1.max(w)); }
    }
    best
}

pub fn weight(edges: &[Edge], m: &[usize]) -> i64 { m.iter().map(|&i| edges[i].2).sum() }

pub fn demo() -> String {
    let mut o = String::new();
    // a path 0-1-2-3 where the greedy choice of the middle edge halves the matching
    let e: Vec<Edge> = vec![(1, 2, 3), (0, 1, 2), (2, 3, 2)];
    let (opt_c, opt_w) = optimum(4, &e);
    outln!(o, "path 0-1-2-3 with the middle edge listed first: maximal matching {:?}, maximum has {opt_c} edges", greedy_maximal_matching(4, &e));
    outln!(o, "weighted greedy takes weight {} vs optimum {opt_w}", weight(&e, &greedy_weighted_matching(4, &e)));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn half_approximation_guarantees() {
        let mut r = Rng::new(482);
        for _ in 0..400 {
            let n = r.range(2, 8);
            let edges: Vec<Edge> = (0..r.range(1, 14)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(1, 20) as i64)).collect();
            let (opt_c, opt_w) = optimum(n, &edges);
            let mm = greedy_maximal_matching(n, &edges);
            assert!(2 * mm.len() >= opt_c, "maximal matching {} optimum {opt_c}", mm.len());
            let wm = greedy_weighted_matching(n, &edges);
            assert!(2 * weight(&edges, &wm) >= opt_w, "weighted greedy {} optimum {opt_w}", weight(&edges, &wm));
            // validity: no vertex used twice, and the matching is maximal
            for set in [&mm, &wm] {
                let mut used = vec![0; n];
                for &i in set.iter() { used[edges[i].0] += 1; used[edges[i].1] += 1; }
                assert!(used.iter().all(|&c| c <= 1));
                assert!(edges.iter().all(|&(u, v, _)| u == v || used[u] > 0 || used[v] > 0));
            }
        }
    }
    #[test]
    fn the_factor_two_is_tight_on_a_path() {
        let e: Vec<Edge> = vec![(1, 2, 1), (0, 1, 1), (2, 3, 1)];
        assert_eq!(greedy_maximal_matching(4, &e).len(), 1);
        assert_eq!(optimum(4, &e).0, 2);
    }
}
