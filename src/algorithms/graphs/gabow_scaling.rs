//! @title Gabow's Scaling Algorithm
//! @chapter Problem 24-4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Single-source shortest paths with integer weights ≤ W in O(E lg W) by solving the problem on the high-order bits first and reweighting with the result.
//! @order 127
use super::dial_dijkstra::dial;
use crate::outln;

/// Gabow scaling. `edges` = (u, v, w) with integer w ≥ 0. Returns distances (None = unreachable).
pub fn gabow(n: usize, edges: &[(usize, usize, i64)], s: usize) -> Vec<Option<i64>> {
    let k = 64 - (edges.iter().map(|e| e.2).max().unwrap_or(0).max(1) as u64).leading_zeros() as usize; // bits of the largest weight
    // reachability: vertices unreachable from s keep no distance
    let mut reach = vec![false; n];
    let mut adj0 = vec![vec![]; n];
    for &(u, v, _) in edges { adj0[u].push(v); }
    let (mut st, _) = (vec![s], 0);
    reach[s] = true;
    while let Some(u) = st.pop() { for &v in &adj0[u] { if !reach[v] { reach[v] = true; st.push(v); } } }
    let mut delta = vec![0i64; n]; // δ₀: with all weights truncated to 0 every reachable vertex is at distance 0
    for i in 1..=k {
        let shift = k - i;
        // w_i(u,v) = ⌊w / 2^{k−i}⌋ = 2·w_{i−1}(u,v) + (next bit);  reweighted ŵ(u,v) = w_i + 2δ_{i−1}(u) − 2δ_{i−1}(v) ≥ 0
        let mut adj = vec![vec![]; n];
        for &(u, v, w) in edges {
            if reach[u] { adj[u].push((v, (w >> shift) + 2 * delta[u] - 2 * delta[v])); }
        }
        debug_assert!(adj.iter().flatten().all(|e| e.1 >= 0));
        let dhat = dial(&adj, s); // small integers: the shortest-path distances in the reweighted graph are at most |E|
        for v in 0..n { if reach[v] { delta[v] = 2 * delta[v] + dhat[v]; } }
    }
    (0..n).map(|v| if reach[v] { Some(delta[v]) } else { None }).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    let e = [(0, 1, 7), (0, 2, 12), (1, 2, 2), (1, 3, 9), (2, 4, 10), (3, 4, 4), (3, 5, 1), (4, 5, 5)];
    outln!(o, "edges {:?}", e);
    outln!(o, "Gabow scaling from 0: {:?}", gabow(6, &e, 0));
    let mut adj = vec![vec![]; 6];
    for &(u, v, w) in &e { adj[u].push((v, w)); }
    outln!(o, "binary-heap Dijkstra  : {:?}", super::dijkstra::dijkstra(&adj, 0).0);
    outln!(o, "{} scaling phases (one per bit of the largest weight 12), each solved with small integer keys", 4);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_dijkstra() {
        let mut r = Rng::new(363);
        for _ in 0..200 {
            let n = r.range(1, 15);
            let maxw = [1usize, 7, 100, 5000][r.range(0, 3)];
            let edges: Vec<(usize, usize, i64)> = (0..r.range(0, 40)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(0, maxw) as i64)).collect();
            let s = r.range(0, n - 1);
            let mut adj = vec![vec![]; n];
            for &(u, v, w) in &edges { adj[u].push((v, w)); }
            let (dj, _) = super::super::dijkstra::dijkstra(&adj, s);
            let want: Vec<Option<i64>> = dj.iter().map(|&d| if d >= super::super::dijkstra::INF { None } else { Some(d) }).collect();
            assert_eq!(gabow(n, &edges, s), want, "{edges:?} s={s}");
        }
    }
    #[test]
    fn single_vertex_and_zero_weights() {
        assert_eq!(gabow(1, &[], 0), vec![Some(0)]);
        assert_eq!(gabow(3, &[(0, 1, 0), (1, 2, 0)], 0), vec![Some(0), Some(0), Some(0)]);
    }
}
