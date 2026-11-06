//! @title Classifying Edges by Breadth-First Search
//! @chapter Problem 22-1 (1e–4e) / 20-1
//! @editions 1,2,3,4
//! @summary BFS produces only tree, back and cross edges (never forward) and d[v] ≤ d[u] + 1; in undirected graphs every non-tree edge joins vertices at distance ≤ 1.
//! @order 25
use crate::outln;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind { Tree, Back, Forward, Cross }

/// Run BFS from s and classify every edge (u, v) whose tail u is reachable. Returns (distances, edge kinds).
pub fn classify(adj: &[Vec<usize>], s: usize) -> (Vec<Option<usize>>, Vec<(usize, usize, Kind)>) {
    let n = adj.len();
    let (mut d, mut parent) = (vec![None; n], vec![None; n]);
    d[s] = Some(0);
    let mut q = VecDeque::from([s]);
    let mut order = vec![];
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &adj[u] { if d[v].is_none() { d[v] = Some(d[u].unwrap() + 1); parent[v] = Some(u); q.push_back(v); } }
    }
    let is_ancestor = |a: usize, mut x: usize| { loop { if x == a { return true; } match parent[x] { Some(p) => x = p, None => return false } } };
    let mut kinds = vec![];
    let mut tree_used = vec![false; n];
    for &u in &order {
        for &v in &adj[u] {
            let k = if parent[v] == Some(u) && !tree_used[v] { tree_used[v] = true; Kind::Tree }
                else if is_ancestor(v, u) { Kind::Back }            // v is an ancestor of u (or u itself)
                else if is_ancestor(u, v) { Kind::Forward }         // u is a proper ancestor of v: cannot happen in BFS
                else { Kind::Cross };
            kinds.push((u, v, k));
        }
    }
    (d, kinds)
}

pub fn demo() -> String {
    let mut o = String::new();
    let adj = vec![vec![1, 2], vec![3], vec![1, 3, 0], vec![0, 4], vec![2]];
    let (d, kinds) = classify(&adj, 0);
    outln!(o, "directed graph {:?}, distances from 0: {:?}", adj, d);
    for (u, v, k) in &kinds { outln!(o, "  edge {u} → {v}: {:?}", k); }
    outln!(o, "forward edges found: {}  (always 0 for BFS)", kinds.iter().filter(|e| e.2 == Kind::Forward).count());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn directed_bfs_properties() {
        let mut r = Rng::new(355);
        for _ in 0..200 {
            let n = r.range(1, 12);
            let adj: Vec<Vec<usize>> = (0..n).map(|u| (0..n).filter(|&v| r.f64() < 0.2 && (u != v || r.f64() < 0.3)).collect()).collect();
            let (d, kinds) = classify(&adj, 0);
            for &(u, v, k) in &kinds {
                assert_ne!(k, Kind::Forward, "BFS has no forward edges");
                assert!(d[v].unwrap() <= d[u].unwrap() + 1, "d[v] ≤ d[u] + 1");
                match k {
                    Kind::Tree => assert_eq!(d[v].unwrap(), d[u].unwrap() + 1),
                    Kind::Back => assert!(d[v].unwrap() <= d[u].unwrap()),
                    _ => {}
                }
            }
            assert_eq!(kinds.iter().filter(|e| e.2 == Kind::Tree).count(), d.iter().flatten().count() - 1);
        }
    }
    #[test]
    fn undirected_bfs_has_only_tree_and_cross_edges() {
        let mut r = Rng::new(356);
        for _ in 0..200 {
            let n = r.range(1, 12);
            let mut adj = vec![vec![]; n];
            for u in 0..n { for v in u + 1..n { if r.f64() < 0.25 { adj[u].push(v); adj[v].push(u); } } }
            let (d, kinds) = classify(&adj, 0);
            for &(u, v, k) in &kinds {
                assert!((d[u].unwrap() as i64 - d[v].unwrap() as i64).abs() <= 1);
                // a "back" edge in an undirected graph can only be the reverse of a tree edge (v is u's parent)
                if k == Kind::Back { assert_eq!(d[v].unwrap() + 1, d[u].unwrap()); }
            }
        }
    }
}
