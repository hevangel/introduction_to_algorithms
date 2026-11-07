//! @title Dijkstra's Algorithm
//! @chapter 24.3 (3e) / 22.3 (4e) / 25.2 (1e)
//! @editions 1,2,3,4
//! @summary Greedy single-source shortest paths for non-negative weights, using a min-priority queue.
//! @order 120
use crate::outln;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub const INF: i64 = i64::MAX / 4;
pub type WeightedAdj = Vec<Vec<(usize, i64)>>;

/// DIJKSTRA(G, w, s) with a binary heap: O((V+E) lg V). All weights must be ≥ 0.
pub fn dijkstra(adj: &WeightedAdj, s: usize) -> (Vec<i64>, Vec<Option<usize>>) {
    let n = adj.len();
    let (mut d, mut pi) = (vec![INF; n], vec![None; n]);
    d[s] = 0;
    let mut q = BinaryHeap::from([Reverse((0i64, s))]);
    while let Some(Reverse((du, u))) = q.pop() {
        if du > d[u] { continue; } // stale entry
        for &(v, w) in &adj[u] {
            debug_assert!(w >= 0);
            if du + w < d[v] {          // RELAX
                d[v] = du + w;
                pi[v] = Some(u);
                q.push(Reverse((d[v], v)));
            }
        }
    }
    (d, pi)
}

/// Θ(V²) array version for dense graphs.
pub fn dijkstra_array(adj: &WeightedAdj, s: usize) -> Vec<i64> {
    let n = adj.len();
    let (mut d, mut done) = (vec![INF; n], vec![false; n]);
    d[s] = 0;
    for _ in 0..n {
        let Some(u) = (0..n).filter(|&v| !done[v] && d[v] < INF).min_by_key(|&v| d[v]) else { break };
        done[u] = true;
        for &(v, w) in &adj[u] { if d[u] + w < d[v] { d[v] = d[u] + w; } }
    }
    d
}

pub fn path(pi: &[Option<usize>], s: usize, v: usize) -> Option<Vec<usize>> {
    let (mut p, mut cur) = (vec![v], v);
    while cur != s { cur = pi[cur]?; p.push(cur); }
    p.reverse();
    Some(p)
}

pub fn demo() -> String {
    let mut o = String::new();
    // Figure 24.6: s=0 t=1 x=2 y=3 z=4
    let e = [(0, 1, 10), (0, 3, 5), (1, 2, 1), (1, 3, 2), (2, 4, 4), (3, 1, 3), (3, 2, 9), (3, 4, 2), (4, 0, 7), (4, 2, 6)];
    let mut adj = vec![vec![]; 5];
    for &(u, v, w) in &e { adj[u].push((v, w)); }
    let (d, pi) = dijkstra(&adj, 0);
    outln!(o, "distances from s: {:?}", d);
    outln!(o, "array version   : {:?}", dijkstra_array(&adj, 0));
    outln!(o, "shortest path s → x: {:?}", path(&pi, 0, 2));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_figure_24_6() {
        let e = [(0, 1, 10), (0, 3, 5), (1, 2, 1), (1, 3, 2), (2, 4, 4), (3, 1, 3), (3, 2, 9), (3, 4, 2), (4, 0, 7), (4, 2, 6)];
        let mut adj = vec![vec![]; 5];
        for &(u, v, w) in &e { adj[u].push((v, w)); }
        assert_eq!(dijkstra(&adj, 0).0, vec![0, 8, 9, 5, 7]);
    }
    #[test]
    fn agrees_with_bellman_ford() {
        let mut r = Rng::new(132);
        for _ in 0..60 {
            let n = r.range(1, 14);
            let edges: Vec<(usize, usize, i64)> = (0..r.range(0, 40)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(0, 20) as i64)).collect();
            let mut adj = vec![vec![]; n];
            for &(u, v, w) in &edges { adj[u].push((v, w)); }
            let s = r.range(0, n - 1);
            let (d, pi) = dijkstra(&adj, s);
            let (bf, _) = super::super::bellman_ford::bellman_ford(n, &edges, s).unwrap();
            assert_eq!(d, bf);
            assert_eq!(dijkstra_array(&adj, s), bf);
            for v in 0..n { if let Some(p) = path(&pi, s, v) { assert_eq!(p[0], s); } }
        }
    }
}
