//! @title Floyd-Warshall All-Pairs Shortest Paths
//! @chapter 25.2–25.3 (3e) / 23.2 (4e) / 26.2 (1e)
//! @editions 1,2,3,4
//! @summary Θ(V³) dynamic program over intermediate vertices; also transitive closure (Warshall).
//! @order 140
use crate::outln;

pub const INF: i64 = i64::MAX / 4;

/// FLOYD-WARSHALL(W): `w[i][j]` = edge weight, INF if none, w[i][i] ≤ 0. Returns (dist, predecessor matrix).
pub fn floyd_warshall(w: &[Vec<i64>]) -> (Vec<Vec<i64>>, Vec<Vec<Option<usize>>>) {
    let n = w.len();
    let mut d = w.to_vec();
    let mut pi: Vec<Vec<Option<usize>>> = (0..n).map(|i| (0..n).map(|j| if i != j && w[i][j] < INF { Some(i) } else { None }).collect()).collect();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][k] < INF && d[k][j] < INF && d[i][k] + d[k][j] < d[i][j] {
                    d[i][j] = d[i][k] + d[k][j];
                    pi[i][j] = pi[k][j];
                }
            }
        }
    }
    (d, pi)
}

/// PRINT-ALL-PAIRS-SHORTEST-PATH
pub fn path(pi: &[Vec<Option<usize>>], i: usize, j: usize) -> Option<Vec<usize>> {
    let mut p = vec![j];
    let mut cur = j;
    while cur != i {
        cur = pi[i][cur]?;
        p.push(cur);
        if p.len() > pi.len() + 1 { return None; }
    }
    p.reverse();
    Some(p)
}

/// A negative-weight cycle exists iff some diagonal entry of the result is negative (Exercise 25.2-6).
pub fn has_negative_cycle(d: &[Vec<i64>]) -> bool { (0..d.len()).any(|i| d[i][i] < 0) }

/// Warshall's transitive closure: t[i][j] iff a path (of ≥ 0 edges) exists.
pub fn transitive_closure(adj: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let n = adj.len();
    let mut t = adj.to_vec();
    for i in 0..n { t[i][i] = true; }
    for k in 0..n { for i in 0..n { for j in 0..n { t[i][j] = t[i][j] || (t[i][k] && t[k][j]); } } }
    t
}

pub fn matrix(n: usize, edges: &[(usize, usize, i64)]) -> Vec<Vec<i64>> {
    let mut w = vec![vec![INF; n]; n];
    for i in 0..n { w[i][i] = 0; }
    for &(u, v, c) in edges { w[u][v] = w[u][v].min(c); }
    w
}

pub fn demo() -> String {
    let mut o = String::new();
    // Figure 25.1: vertices 1..5 -> 0..4
    let e = [(0, 1, 3), (0, 2, 8), (0, 4, -4), (1, 3, 1), (1, 4, 7), (2, 1, 4), (3, 0, 2), (3, 2, -5), (4, 3, 6)];
    let (d, pi) = floyd_warshall(&matrix(5, &e));
    outln!(o, "all-pairs shortest-path matrix:");
    for row in &d { outln!(o, "  {:?}", row); }
    outln!(o, "path 3 → 4 (1-based vertices): {:?}", path(&pi, 2, 3).map(|p| p.iter().map(|x| x + 1).collect::<Vec<_>>()));
    let adj = vec![vec![false, true, false], vec![false, false, true], vec![false, false, false]];
    outln!(o, "transitive closure of 0→1→2:");
    for row in transitive_closure(&adj) { outln!(o, "  {:?}", row); }
    let (neg, _) = floyd_warshall(&matrix(3, &[(0, 1, 1), (1, 2, -3), (2, 0, 1)]));
    outln!(o, "negative cycle detected: {}", has_negative_cycle(&neg));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_figure_25_1() {
        let e = [(0, 1, 3), (0, 2, 8), (0, 4, -4), (1, 3, 1), (1, 4, 7), (2, 1, 4), (3, 0, 2), (3, 2, -5), (4, 3, 6)];
        let (d, pi) = floyd_warshall(&matrix(5, &e));
        assert_eq!(d, vec![vec![0, 1, -3, 2, -4], vec![3, 0, -4, 1, -1], vec![7, 4, 0, 5, 3], vec![2, -1, -5, 0, -2], vec![8, 5, 1, 6, 0]]);
        let p = path(&pi, 0, 2).unwrap();
        assert_eq!(p, vec![0, 4, 3, 2]);
        assert_eq!(p.windows(2).map(|w| matrix(5, &e)[w[0]][w[1]]).sum::<i64>(), -3);
    }
    #[test]
    fn matches_dijkstra_for_nonnegative_weights() {
        let mut r = Rng::new(134);
        for _ in 0..40 {
            let n = r.range(1, 10);
            let edges: Vec<(usize, usize, i64)> = (0..r.range(0, 30)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(0, 15) as i64)).collect();
            let (d, _) = floyd_warshall(&matrix(n, &edges));
            let mut adj = vec![vec![]; n];
            for &(u, v, w) in &edges { adj[u].push((v, w)); }
            for s in 0..n { assert_eq!(d[s], super::super::dijkstra::dijkstra(&adj, s).0); }
            assert!(!has_negative_cycle(&d));
        }
    }
    #[test]
    fn closure_matches_floyd_reachability() {
        let mut r = Rng::new(135);
        let n = 9;
        let adj: Vec<Vec<bool>> = (0..n).map(|u| (0..n).map(|v| u != v && r.f64() < 0.15).collect()).collect();
        let edges: Vec<(usize, usize, i64)> = (0..n).flat_map(|u| (0..n).filter(move |&v| u != v).map(move |v| (u, v, 1))).filter(|&(u, v, _)| adj[u][v]).collect();
        let (d, _) = floyd_warshall(&matrix(n, &edges));
        let t = transitive_closure(&adj);
        for i in 0..n { for j in 0..n { assert_eq!(t[i][j], d[i][j] < INF); } }
    }
}
