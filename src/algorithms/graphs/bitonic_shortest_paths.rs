//! @title Bitonic Shortest Paths
//! @chapter Problem 24-6 (1e–4e)
//! @editions 1,2,3,4
//! @summary Shortest paths whose edge weights first increase then decrease: sort the edges once and relax them in ascending, then descending, order — O(E lg E).
//! @order 128
use crate::outln;

pub const INF: i64 = i64::MAX / 4;
pub type Edge = (usize, usize, i64);

/// Shortest *bitonic* path lengths from `s` (edge weights along the path increase, then decrease), assuming distinct non-negative weights.
pub fn bitonic_shortest_paths(n: usize, edges: &[Edge], s: usize) -> Vec<i64> {
    let mut sorted = edges.to_vec();
    sorted.sort_by_key(|e| e.2);
    // phase 1: relax in ascending weight order — any path whose edges are used in this order has increasing weights
    let mut inc = vec![INF; n];
    inc[s] = 0;
    for &(u, v, w) in &sorted { if inc[u] < INF && inc[u] + w < inc[v] { inc[v] = inc[u] + w; } }
    // phase 2: continue from those distances, relaxing in descending order — the remaining edges have decreasing weights
    let mut best = inc;
    for &(u, v, w) in sorted.iter().rev() { if best[u] < INF && best[u] + w < best[v] { best[v] = best[u] + w; } }
    best
}

pub fn is_bitonic(w: &[i64]) -> bool {
    let peak = w.iter().enumerate().max_by_key(|&(_, &x)| x).map_or(0, |(i, _)| i);
    w[..=peak].windows(2).all(|p| p[0] < p[1]) && w[peak..].windows(2).all(|p| p[0] > p[1])
}

pub fn demo() -> String {
    let mut o = String::new();
    let e: Vec<Edge> = vec![(0, 1, 5), (1, 2, 9), (2, 3, 4), (0, 3, 20), (1, 3, 7), (3, 4, 2), (0, 2, 12), (2, 4, 3)];
    outln!(o, "edges (u, v, weight) {:?}", e);
    let d = bitonic_shortest_paths(5, &e, 0);
    outln!(o, "bitonic shortest paths from 0: {:?}", d.iter().map(|&x| if x >= INF { -1 } else { x }).collect::<Vec<_>>());
    outln!(o, "path 0→1→2→3→4 has weights 5,9,4,2: increasing then decreasing = {}", is_bitonic(&[5, 9, 4, 2]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    /// Exhaustive search over simple paths whose weight sequence is bitonic.
    fn brute(n: usize, edges: &[Edge], s: usize) -> Vec<i64> {
        fn go(edges: &[Edge], u: usize, len: i64, ws: &mut Vec<i64>, vis: &mut Vec<bool>, best: &mut Vec<i64>) {
            if ws.is_empty() || is_bitonic(ws) { best[u] = best[u].min(len); } else { return; }
            for &(a, v, w) in edges { if a == u && !vis[v] { vis[v] = true; ws.push(w); go(edges, v, len + w, ws, vis, best); ws.pop(); vis[v] = false; } }
        }
        let mut best = vec![INF; n];
        let mut vis = vec![false; n];
        vis[s] = true;
        go(edges, s, 0, &mut vec![], &mut vis, &mut best);
        best
    }
    #[test]
    fn matches_exhaustive_enumeration() {
        let mut r = Rng::new(364);
        for _ in 0..300 {
            let n = r.range(2, 7);
            let mut ws: Vec<i64> = (1..=40).collect();
            r.shuffle(&mut ws);
            let edges: Vec<Edge> = (0..r.range(1, 14)).map(|i| (r.range(0, n - 1), r.range(0, n - 1), ws[i])).filter(|e| e.0 != e.1).collect();
            let s = r.range(0, n - 1);
            assert_eq!(bitonic_shortest_paths(n, &edges, s), brute(n, &edges, s), "{edges:?}");
        }
    }
    #[test]
    fn bitonic_predicate() {
        assert!(is_bitonic(&[1, 4, 9, 6, 2]) && is_bitonic(&[1, 2, 3]) && is_bitonic(&[3, 2, 1]) && is_bitonic(&[5]));
        assert!(!is_bitonic(&[1, 5, 2, 6]));
    }
}
