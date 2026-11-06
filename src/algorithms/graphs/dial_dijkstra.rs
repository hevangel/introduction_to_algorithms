//! @title Dijkstra with a Bucket Queue (Dial's Algorithm)
//! @chapter Exercises 24.3-7, 24.3-8 (1e–4e)
//! @editions 1,2,3,4
//! @summary For integer weights ≤ W the priority queue can be an array of buckets: O(E + W·V) — O(E) for constant W.
//! @order 122
use crate::outln;

pub const INF: i64 = i64::MAX / 4;

/// Dijkstra with a bucket queue indexed by distance; weights must be non-negative integers. Unreachable vertices get INF.
/// Buckets are scanned in increasing order, so EXTRACT-MIN costs O(1) amortised.
pub fn dial(adj: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
    let n = adj.len();
    let mut d = vec![INF; n];
    d[s] = 0;
    let mut buckets: Vec<Vec<usize>> = vec![vec![s]];
    let mut done = vec![false; n];
    let mut cur = 0usize;
    while cur < buckets.len() {
        while let Some(u) = buckets[cur].pop() {
            if done[u] || d[u] != cur as i64 { continue; } // stale entry
            done[u] = true;
            for &(v, w) in &adj[u] {
                let nd = cur as i64 + w;
                if nd < d[v] {
                    d[v] = nd;
                    if buckets.len() <= nd as usize { buckets.resize(nd as usize + 1, vec![]); }
                    buckets[nd as usize].push(v);
                }
            }
        }
        cur += 1;
    }
    d
}

pub fn demo() -> String {
    let mut o = String::new();
    let e = [(0, 1, 2), (0, 2, 5), (1, 2, 1), (1, 3, 4), (2, 3, 1), (3, 4, 3)];
    let mut adj = vec![vec![]; 5];
    for &(u, v, w) in &e { adj[u].push((v, w)); }
    outln!(o, "Dial distances from 0: {:?}", dial(&adj, 0));
    let mut r = crate::common::Rng::new(1);
    let n = 20_000;
    let mut big = vec![vec![]; n];
    for u in 0..n { for _ in 0..4 { big[u].push((r.range(0, n - 1), r.range(0, 10) as i64)); } }
    let d = dial(&big, 0);
    let (dd, _) = super::dijkstra::dijkstra(&big, 0);
    outln!(o, "n = {n}, E = {}, W = 10: bucket queue equals binary-heap Dijkstra: {}", 4 * n, d == dd);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn equals_heap_dijkstra_including_zero_weights_and_unreachable_vertices() {
        let mut r = Rng::new(362);
        for _ in 0..200 {
            let n = r.range(1, 20);
            let maxw = r.range(0, 12) as i64;
            let mut adj = vec![vec![]; n];
            for _ in 0..r.range(0, 50) { adj[r.range(0, n - 1)].push((r.range(0, n - 1), r.range(0, maxw as usize) as i64)); }
            let s = r.range(0, n - 1);
            assert_eq!(dial(&adj, s), super::super::dijkstra::dijkstra(&adj, s).0);
        }
    }
}
