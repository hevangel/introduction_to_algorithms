//! @title Johnson's Algorithm
//! @chapter 25.3 (3e) / 23.3 (4e) / 26.4 (1e)
//! @editions 1,2,3,4
//! @summary All-pairs shortest paths on sparse graphs: reweight with Bellman-Ford potentials, then run Dijkstra from every vertex.
//! @order 150
use super::bellman_ford::bellman_ford;
use crate::outln;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub const INF: i64 = i64::MAX / 4;

/// JOHNSON(G): `Some(matrix)` of shortest distances (INF for unreachable pairs) or `None` if a negative cycle exists.
pub fn johnson(n: usize, edges: &[(usize, usize, i64)]) -> Option<Vec<Vec<i64>>> {
    // G': add vertex s = n with 0-weight edges to all vertices
    let mut ext = edges.to_vec();
    for v in 0..n { ext.push((n, v, 0)); }
    let (h, _) = bellman_ford(n + 1, &ext, n)?;             // potentials h(v) = δ(s, v)
    // reweight: ŵ(u,v) = w(u,v) + h(u) − h(v) ≥ 0
    let mut adj = vec![vec![]; n];
    for &(u, v, w) in edges { adj[u].push((v, w + h[u] - h[v])); }
    let mut d = vec![vec![INF; n]; n];
    for u in 0..n {
        let mut dist = vec![INF; n];
        dist[u] = 0;
        let mut q = BinaryHeap::from([Reverse((0i64, u))]);
        while let Some(Reverse((du, x))) = q.pop() {
            if du > dist[x] { continue; }
            for &(y, w) in &adj[x] {
                if du + w < dist[y] { dist[y] = du + w; q.push(Reverse((dist[y], y))); }
            }
        }
        for v in 0..n {
            if dist[v] < INF { d[u][v] = dist[v] + h[v] - h[u]; }   // undo the reweighting
        }
    }
    Some(d)
}

pub fn demo() -> String {
    let mut o = String::new();
    let e = [(0, 1, 3), (0, 2, 8), (0, 4, -4), (1, 3, 1), (1, 4, 7), (2, 1, 4), (3, 0, 2), (3, 2, -5), (4, 3, 6)];
    let d = johnson(5, &e).unwrap();
    outln!(o, "Johnson on the Figure 25.1 graph:");
    for row in &d { outln!(o, "  {:?}", row); }
    let fw = super::floyd_warshall::floyd_warshall(&super::floyd_warshall::matrix(5, &e)).0;
    outln!(o, "equals Floyd-Warshall: {}", d == fw);
    outln!(o, "negative cycle graph → {:?}", johnson(3, &[(0, 1, 1), (1, 2, -3), (2, 0, 1)]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_floyd_warshall_with_negative_edges() {
        let mut r = Rng::new(136);
        let mut tested = 0;
        for _ in 0..200 {
            let n = r.range(1, 9);
            let edges: Vec<(usize, usize, i64)> = (0..r.range(0, 25)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(0, 20) as i64 - 4)).collect();
            let (fw, _) = super::super::floyd_warshall::floyd_warshall(&super::super::floyd_warshall::matrix(n, &edges));
            let neg = super::super::floyd_warshall::has_negative_cycle(&fw);
            match johnson(n, &edges) {
                None => assert!(neg),
                Some(d) => { assert!(!neg); assert_eq!(d, fw); tested += 1; }
            }
        }
        assert!(tested > 20);
    }
}
