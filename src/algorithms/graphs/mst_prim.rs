//! @title Prim's Minimum Spanning Tree
//! @chapter 23.2 (3e) / 21 (4e) / 24 (1e–2e)
//! @editions 1,2,3,4
//! @summary Grow one tree from a root, always adding the lightest edge leaving it (min-priority queue).
//! @order 90
use crate::outln;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub type WeightedAdj = Vec<Vec<(usize, i64)>>;

/// MST-PRIM with a binary heap (lazy deletion): O(E lg V). Returns (total weight, parent array) for the component of `root`.
pub fn prim(adj: &WeightedAdj, root: usize) -> (i64, Vec<Option<usize>>) {
    let n = adj.len();
    let (mut key, mut parent, mut in_tree) = (vec![i64::MAX; n], vec![None; n], vec![false; n]);
    key[root] = 0;
    let mut q = BinaryHeap::from([Reverse((0i64, root))]);
    let mut total = 0;
    while let Some(Reverse((k, u))) = q.pop() {
        if in_tree[u] || k > key[u] { continue; }
        in_tree[u] = true;
        total += k;
        for &(v, w) in &adj[u] {
            if !in_tree[v] && w < key[v] {
                key[v] = w;
                parent[v] = Some(u);
                q.push(Reverse((w, v)));
            }
        }
    }
    (total, parent)
}

/// Array-based Prim, Θ(V²) — best for dense graphs.
pub fn prim_array(adj: &WeightedAdj, root: usize) -> i64 {
    let n = adj.len();
    let (mut key, mut in_tree) = (vec![i64::MAX; n], vec![false; n]);
    key[root] = 0;
    let mut total = 0;
    for _ in 0..n {
        let Some(u) = (0..n).filter(|&v| !in_tree[v] && key[v] != i64::MAX).min_by_key(|&v| key[v]) else { break };
        in_tree[u] = true;
        total += key[u];
        for &(v, w) in &adj[u] { if !in_tree[v] && w < key[v] { key[v] = w; } }
    }
    total
}

pub fn adjacency(n: usize, edges: &[(usize, usize, i64)]) -> WeightedAdj {
    let mut a = vec![vec![]; n];
    for &(u, v, w) in edges { a[u].push((v, w)); a[v].push((u, w)); }
    a
}

pub fn demo() -> String {
    let mut o = String::new();
    let e = [(0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2), (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7)];
    let adj = adjacency(9, &e);
    let (w, parent) = prim(&adj, 0);
    outln!(o, "Prim (heap) total weight  = {w}");
    outln!(o, "Prim (array) total weight = {}", prim_array(&adj, 0));
    outln!(o, "parent array (root 0): {:?}", parent);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_kruskal_on_random_connected_graphs() {
        let mut r = Rng::new(129);
        for _ in 0..60 {
            let n = r.range(1, 20);
            let mut edges: Vec<(usize, usize, i64)> = (1..n).map(|v| (r.range(0, v - 1), v, r.range(1, 50) as i64)).collect();
            for _ in 0..r.range(0, 40) { let (a, b) = (r.range(0, n - 1), r.range(0, n - 1)); if a != b { edges.push((a, b, r.range(1, 50) as i64)); } }
            let adj = adjacency(n, &edges);
            let k = super::super::mst_kruskal::kruskal(n, &edges).0;
            let root = r.range(0, n - 1);
            assert_eq!(prim(&adj, root).0, k);
            assert_eq!(prim_array(&adj, root), k);
        }
    }
    #[test]
    fn parent_pointers_form_a_tree() {
        let e = [(0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2), (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7)];
        let (w, parent) = prim(&adjacency(9, &e), 0);
        assert_eq!(w, 37);
        assert_eq!(parent.iter().filter(|p| p.is_none()).count(), 1);
        for v in 0..9 { let mut x = v; let mut steps = 0; while let Some(p) = parent[x] { x = p; steps += 1; assert!(steps < 9); } assert_eq!(x, 0); }
    }
}
