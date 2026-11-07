//! @title Bellman-Ford Single-Source Shortest Paths
//! @chapter 24.1 (3e) / 22.1 (4e) / 25.3 (1e)
//! @editions 1,2,3,4
//! @summary Relax every edge V−1 times; detects reachable negative-weight cycles. O(VE).
//! @order 100
use crate::outln;

pub const INF: i64 = i64::MAX / 4;
pub type Edge = (usize, usize, i64);

/// BELLMAN-FORD(G, w, s): `Some((dist, pred))` or `None` if a negative-weight cycle is reachable from `s`.
pub fn bellman_ford(n: usize, edges: &[Edge], s: usize) -> Option<(Vec<i64>, Vec<Option<usize>>)> {
    let (mut d, mut pi) = (vec![INF; n], vec![None; n]);
    d[s] = 0;
    for _ in 1..n {
        let mut changed = false;
        for &(u, v, w) in edges {
            if d[u] < INF && d[u] + w < d[v] {   // RELAX
                d[v] = d[u] + w;
                pi[v] = Some(u);
                changed = true;
            }
        }
        if !changed { break; }
    }
    for &(u, v, w) in edges {
        if d[u] < INF && d[u] + w < d[v] { return None; }
    }
    Some((d, pi))
}

/// Shortest path s ⇝ v from the predecessor array.
pub fn path(pi: &[Option<usize>], s: usize, v: usize) -> Option<Vec<usize>> {
    let (mut p, mut cur) = (vec![v], v);
    while cur != s {
        cur = pi[cur]?;
        p.push(cur);
        if p.len() > pi.len() + 1 { return None; }
    }
    p.reverse();
    Some(p)
}

/// Exercise 24.1-6: find a negative-weight cycle anywhere in the graph (as a vertex list), or None.
pub fn find_negative_cycle(n: usize, edges: &[Edge]) -> Option<Vec<usize>> {
    // start with d = 0 everywhere (equivalent to a virtual source with 0-weight edges)
    let (mut d, mut pi) = (vec![0i64; n], vec![usize::MAX; n]);
    let mut last = None;
    for _ in 0..n {
        last = None;
        for &(u, v, w) in edges {
            if d[u] + w < d[v] { d[v] = d[u] + w; pi[v] = u; last = Some(v); }
        }
        last?;
    }
    let mut x = last?;
    for _ in 0..n { x = pi[x]; } // step back n times to land inside the cycle
    let mut cycle = vec![x];
    let mut y = pi[x];
    while y != x { cycle.push(y); y = pi[y]; }
    cycle.reverse();
    Some(cycle)
}

pub fn demo() -> String {
    let mut o = String::new();
    // Figure 24.4: s=0 t=1 x=2 y=3 z=4
    let e: Vec<Edge> = vec![(0, 1, 6), (0, 3, 7), (1, 2, 5), (1, 3, 8), (1, 4, -4), (2, 1, -2), (3, 2, -3), (3, 4, 9), (4, 0, 2), (4, 2, 7)];
    let (d, pi) = bellman_ford(5, &e, 0).unwrap();
    outln!(o, "distances from s: {:?}", d);
    outln!(o, "path s → x: {:?}", path(&pi, 0, 2));
    let mut neg = e.clone();
    neg[4].2 = -4;
    neg.push((2, 4, -2)); // creates the negative cycle z → ... via x
    outln!(o, "after adding edge x→z of weight −2: {:?}", bellman_ford(5, &neg, 0).map(|x| x.0));
    outln!(o, "negative cycle: {:?}", find_negative_cycle(5, &neg));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_figure_24_4() {
        let e: Vec<Edge> = vec![(0, 1, 6), (0, 3, 7), (1, 2, 5), (1, 3, 8), (1, 4, -4), (2, 1, -2), (3, 2, -3), (3, 4, 9), (4, 0, 2), (4, 2, 7)];
        let (d, _) = bellman_ford(5, &e, 0).unwrap();
        assert_eq!(d, vec![0, 2, 4, 7, -2]);
    }
    fn cycle_weight(c: &[usize], edges: &[Edge]) -> i64 {
        (0..c.len()).map(|i| edges.iter().filter(|e| e.0 == c[i] && e.1 == c[(i + 1) % c.len()]).map(|e| e.2).min().unwrap()).sum()
    }
    #[test]
    fn negative_cycles_detected_consistently_with_floyd_warshall() {
        let mut r = Rng::new(130);
        for _ in 0..80 {
            let n = r.range(1, 7);
            let edges: Vec<Edge> = (0..r.range(0, 14)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(0, 12) as i64 - 3)).collect();
            let mut d = vec![vec![INF; n]; n];
            for i in 0..n { d[i][i] = 0; }
            for &(u, v, w) in &edges { d[u][v] = d[u][v].min(w); }
            for k in 0..n { for i in 0..n { for j in 0..n { if d[i][k] < INF && d[k][j] < INF { d[i][j] = d[i][j].min(d[i][k] + d[k][j]); } } } }
            let any_neg = (0..n).any(|i| d[i][i] < 0);
            let cyc = find_negative_cycle(n, &edges);
            assert_eq!(cyc.is_some(), any_neg, "{edges:?}");
            if let Some(c) = cyc { assert!(cycle_weight(&c, &edges) < 0); }
            // from source 0: distances agree when no negative cycle is reachable
            let reach_neg = (0..n).any(|k| d[0][k] < INF && d[k][k] < 0);
            match bellman_ford(n, &edges, 0) {
                None => assert!(reach_neg),
                Some((dist, _)) => { assert!(!reach_neg); for v in 0..n { assert_eq!(dist[v], d[0][v]); } }
            }
        }
    }
}
