//! @title Kruskal's Minimum Spanning Tree
//! @chapter 23.1–23.2 (3e) / 21 (4e) / 24 (1e–2e); Problem 23-1
//! @editions 1,2,3,4
//! @summary Sort edges by weight and add each that joins two components (union-find); second-best MST.
//! @order 80
use crate::outln;

pub type Edge = (usize, usize, i64);

fn find(p: &mut [usize], x: usize) -> usize {
    let mut r = x;
    while p[r] != r { r = p[r]; }
    let mut c = x;
    while p[c] != r { let n = p[c]; p[c] = r; c = n; }
    r
}

/// MST-KRUSKAL: returns (total weight, indices of the chosen edges). On a disconnected graph it returns a minimum spanning forest.
pub fn kruskal(n: usize, edges: &[Edge]) -> (i64, Vec<usize>) {
    let mut order: Vec<usize> = (0..edges.len()).collect();
    order.sort_by_key(|&i| edges[i].2);
    let mut p: Vec<usize> = (0..n).collect();
    let (mut total, mut chosen) = (0, vec![]);
    for i in order {
        let (u, v, w) = edges[i];
        let (a, b) = (find(&mut p, u), find(&mut p, v));
        if a != b {
            p[a] = b;
            total += w;
            chosen.push(i);
        }
    }
    (total, chosen)
}

/// Problem 23-1: weight of a second-best spanning tree (the cheapest tree different from the MST found by Kruskal).
/// Uses the lemma that some second-best tree differs from the MST by a single edge swap: O(V²) after the MST.
pub fn second_best_mst(n: usize, edges: &[Edge]) -> Option<i64> {
    let (total, tree) = kruskal(n, edges);
    if tree.len() + 1 != n { return None; } // graph not connected
    let mut adj = vec![vec![]; n];
    for &i in &tree { let (u, v, w) = edges[i]; adj[u].push((v, w)); adj[v].push((u, w)); }
    // maxe[u][v] = heaviest edge on the tree path between u and v
    let mut maxe = vec![vec![i64::MIN; n]; n];
    for s in 0..n {
        let mut stack = vec![(s, usize::MAX, i64::MIN)];
        while let Some((u, par, m)) = stack.pop() {
            maxe[s][u] = m;
            for &(v, w) in &adj[u] { if v != par { stack.push((v, u, m.max(w))); } }
        }
    }
    let in_tree: std::collections::HashSet<usize> = tree.into_iter().collect();
    (0..edges.len()).filter(|i| !in_tree.contains(i) && edges[*i].0 != edges[*i].1)
        .map(|i| total + edges[i].2 - maxe[edges[i].0][edges[i].1]).min()
}

pub fn demo() -> String {
    let mut o = String::new();
    // Figure 23.4 (vertices a..i = 0..8)
    let e: Vec<Edge> = vec![(0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2), (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7)];
    let (w, tree) = kruskal(9, &e);
    outln!(o, "MST weight = {w}");
    for &i in &tree { outln!(o, "  edge {}-{} (w={})", e[i].0, e[i].1, e[i].2); }
    outln!(o, "second-best spanning tree weight = {:?}", second_best_mst(9, &e));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn spanning_trees(n: usize, edges: &[Edge]) -> Vec<(i64, Vec<usize>)> {
        let m = edges.len();
        let mut out = vec![];
        for mask in 0..1u32 << m {
            if mask.count_ones() as usize != n - 1 { continue; }
            let ids: Vec<usize> = (0..m).filter(|i| mask >> i & 1 == 1).collect();
            let mut p: Vec<usize> = (0..n).collect();
            let ok = ids.iter().all(|&i| { let (a, b) = (find(&mut p, edges[i].0), find(&mut p, edges[i].1)); if a == b { false } else { p[a] = b; true } });
            if ok { out.push((ids.iter().map(|&i| edges[i].2).sum(), ids)); }
        }
        out
    }
    #[test]
    fn book_weight_37() {
        let e: Vec<Edge> = vec![(0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2), (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7)];
        assert_eq!(kruskal(9, &e).0, 37);
    }
    #[test]
    fn mst_and_second_best_match_enumeration() {
        let mut r = Rng::new(128);
        for _ in 0..40 {
            let n = r.range(2, 6);
            let mut edges: Vec<Edge> = (1..n).map(|v| (r.range(0, v - 1), v, r.range(1, 9) as i64)).collect(); // spanning tree
            for _ in 0..r.range(0, 5) { let (a, b) = (r.range(0, n - 1), r.range(0, n - 1)); if a != b { edges.push((a, b, r.range(1, 9) as i64)); } }
            let trees = spanning_trees(n, &edges);
            let best = trees.iter().map(|t| t.0).min().unwrap();
            let (w, mut chosen) = kruskal(n, &edges);
            chosen.sort();
            assert_eq!(w, best);
            let second = trees.iter().filter(|t| t.1 != chosen).map(|t| t.0).min();
            assert_eq!(second_best_mst(n, &edges), second, "edges {edges:?}");
        }
    }
    #[test]
    fn forest_for_disconnected_graph() {
        let (w, t) = kruskal(5, &[(0, 1, 3), (3, 4, 1)]);
        assert_eq!((w, t.len()), (4, 2));
        assert_eq!(second_best_mst(5, &[(0, 1, 3), (3, 4, 1)]), None);
    }
}
