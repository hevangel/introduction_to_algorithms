//! @title Minimum Mean-Weight Cycle (Karp)
//! @chapter Problem 24-5 (2e–3e) / 22-5 (4e)
//! @editions 2,3,4
//! @summary Karp's O(VE) algorithm: min over v of max over k of (d_n(v) − d_k(v))/(n − k).
//! @order 230
use crate::outln;

/// Karp's algorithm. Returns the minimum mean weight of a directed cycle, or None if the graph is acyclic.
pub fn min_mean_cycle(n: usize, edges: &[(usize, usize, i64)]) -> Option<f64> {
    const INF: i64 = i64::MAX / 4;
    // d[k][v] = minimum weight of a walk with exactly k edges from the virtual source (reaches every vertex for free)
    let mut d = vec![vec![INF; n]; n + 1];
    d[0] = vec![0; n];
    for k in 1..=n {
        for &(u, v, w) in edges {
            if d[k - 1][u] < INF && d[k - 1][u] + w < d[k][v] { d[k][v] = d[k - 1][u] + w; }
        }
    }
    let mut best: Option<f64> = None;
    for v in 0..n {
        if d[n][v] >= INF { continue; }
        let worst = (0..n).filter(|&k| d[k][v] < INF).map(|k| (d[n][v] - d[k][v]) as f64 / (n - k) as f64).fold(f64::NEG_INFINITY, f64::max);
        best = Some(best.map_or(worst, |b: f64| b.min(worst)));
    }
    best
}

pub fn demo() -> String {
    let mut o = String::new();
    let e = [(0, 1, 1), (1, 2, 2), (2, 0, 3), (1, 3, -1), (3, 1, 0)];
    outln!(o, "edges {:?}", e);
    outln!(o, "min mean cycle weight = {:?} (cycle 1→3→1 has weight −1 over 2 edges)", min_mean_cycle(4, &e));
    outln!(o, "DAG → {:?}", min_mean_cycle(3, &[(0, 1, 5), (1, 2, 5)]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    /// Enumerate all simple cycles by DFS from their smallest vertex.
    fn brute(n: usize, edges: &[(usize, usize, i64)]) -> Option<f64> {
        fn go(start: usize, u: usize, w: i64, len: usize, vis: &mut Vec<bool>, edges: &[(usize, usize, i64)], best: &mut Option<f64>) {
            for &(a, b, c) in edges {
                if a != u { continue; }
                if b == start { let m = (w + c) as f64 / (len + 1) as f64; *best = Some(best.map_or(m, |x: f64| x.min(m))); }
                else if b > start && !vis[b] { vis[b] = true; go(start, b, w + c, len + 1, vis, edges, best); vis[b] = false; }
            }
        }
        let mut best = None;
        for s in 0..n { let mut vis = vec![false; n]; vis[s] = true; go(s, s, 0, 0, &mut vis, edges, &mut best); }
        best
    }
    #[test]
    fn matches_cycle_enumeration() {
        let mut r = Rng::new(144);
        for _ in 0..100 {
            let n = r.range(1, 6);
            let edges: Vec<(usize, usize, i64)> = (0..r.range(0, 12)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(0, 14) as i64 - 4)).collect();
            match (min_mean_cycle(n, &edges), brute(n, &edges)) {
                (None, None) => {}
                (Some(a), Some(b)) => assert!((a - b).abs() < 1e-9, "{a} vs {b} for {edges:?}"),
                (a, b) => panic!("{a:?} vs {b:?} for {edges:?}"),
            }
        }
    }
    #[test]
    fn simple_cases() {
        assert_eq!(min_mean_cycle(2, &[(0, 1, 3), (1, 0, 5)]), Some(4.0));
        assert_eq!(min_mean_cycle(1, &[(0, 0, -7)]), Some(-7.0));
        assert_eq!(min_mean_cycle(3, &[(0, 1, 5), (1, 2, 5)]), None);
    }
}
