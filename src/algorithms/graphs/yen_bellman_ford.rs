//! @title Yen's Improvement to Bellman-Ford
//! @chapter Problem 24-1 (1e–4e) / 22-1
//! @editions 1,2,3,4
//! @summary Relax forward edges in increasing vertex order, then backward edges in decreasing order: at most ⌈|V|/2⌉ passes instead of |V|−1.
//! @order 105
use crate::outln;

pub const INF: i64 = i64::MAX / 4;
pub type Edge = (usize, usize, i64);

/// Bellman–Ford with Yen's edge ordering. Returns (distances or None on a reachable negative cycle, passes that changed something).
pub fn yen_bellman_ford(n: usize, edges: &[Edge], s: usize) -> (Option<Vec<i64>>, usize) {
    let mut fwd: Vec<Edge> = edges.iter().copied().filter(|e| e.0 < e.1).collect(); // E_f: edges that go "up" in the vertex order
    let mut bwd: Vec<Edge> = edges.iter().copied().filter(|e| e.0 > e.1).collect(); // E_b: edges that go "down"
    fwd.sort_by_key(|e| e.0);                          // increasing source: a whole increasing run is relaxed in one sweep
    bwd.sort_by_key(|e| std::cmp::Reverse(e.0));       // decreasing source: a whole decreasing run in one sweep
    let mut d = vec![INF; n];
    d[s] = 0;
    let mut passes = 0;
    let relax = |d: &mut Vec<i64>, set: &[Edge]| { let mut ch = false; for &(u, v, w) in set { if d[u] < INF && d[u] + w < d[v] { d[v] = d[u] + w; ch = true; } } ch };
    for _ in 0..n {
        let c1 = relax(&mut d, &fwd);
        let c2 = relax(&mut d, &bwd);
        if !(c1 || c2) { return (Some(d), passes); }
        passes += 1;
    }
    (None, passes) // still changing after n passes: negative cycle
}

/// Number of passes of the *ordinary* Bellman–Ford (arbitrary edge order) until nothing changes.
pub fn plain_passes(n: usize, edges: &[Edge], s: usize) -> usize {
    let mut d = vec![INF; n];
    d[s] = 0;
    let mut passes = 0;
    loop {
        let mut ch = false;
        for &(u, v, w) in edges { if d[u] < INF && d[u] + w < d[v] { d[v] = d[u] + w; ch = true; } }
        if !ch || passes > n { return passes; }
        passes += 1;
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    // a path whose vertex numbers zig-zag, so shortest paths alternate between increasing and decreasing runs
    let n = 9;
    let order = [0usize, 8, 1, 7, 2, 6, 3, 5, 4];
    let edges: Vec<Edge> = order.windows(2).map(|w| (w[0], w[1], 1)).collect();
    let (d, passes) = yen_bellman_ford(n, &edges, 0);
    outln!(o, "zig-zag path of {n} vertices: distances {:?}", d.unwrap());
    outln!(o, "Yen passes that changed something: {passes} (≤ ⌈n/2⌉ = {}); worst-case ordinary Bellman–Ford passes: n−1 = {}", n.div_ceil(2), n - 1);
    outln!(o, "ordinary Bellman–Ford with the edges in the worst order needed {} passes", plain_passes(n, &edges.iter().rev().copied().collect::<Vec<_>>(), 0));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn same_distances_as_bellman_ford_and_few_passes() {
        let mut r = Rng::new(357);
        for _ in 0..200 {
            let n = r.range(2, 12);
            // weights c + h[u] − h[v] with c ≥ 0 may be negative but never create a negative cycle
            let h: Vec<i64> = r.vec(n, 0, 6);
            let edges: Vec<Edge> = (0..r.range(1, 30)).map(|_| { let (u, v) = (r.range(0, n - 1), r.range(0, n - 1)); (u, v, r.range(0, 5) as i64 + h[u] - h[v]) }).filter(|e| e.0 != e.1).collect();
            let s = r.range(0, n - 1);
            let (d, passes) = yen_bellman_ford(n, &edges, s);
            let (bf, _) = super::super::bellman_ford::bellman_ford(n, &edges, s).unwrap();
            assert_eq!(d.unwrap(), bf);
            assert!(passes <= n.div_ceil(2) + 1, "passes {passes} n {n}");
        }
    }
    #[test]
    fn negative_cycle_is_reported_and_zigzag_needs_half_the_passes() {
        assert!(yen_bellman_ford(3, &[(0, 1, 1), (1, 2, -3), (2, 0, 1)], 0).0.is_none());
        let n = 21;
        let mut order: Vec<usize> = vec![];
        let (mut lo, mut hi) = (0, n - 1);
        while lo <= hi { order.push(lo); if lo != hi { order.push(hi); } lo += 1; if hi == 0 { break; } hi -= 1; }
        let edges: Vec<Edge> = order.windows(2).map(|w| (w[0], w[1], 1)).collect();
        let (d, passes) = yen_bellman_ford(n, &edges, order[0]);
        assert!(d.is_some() && passes <= n / 2 + 1, "{passes}");
    }
}
