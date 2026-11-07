//! @title Shortest Paths in ε-Dense Graphs
//! @chapter Problem 25-2 (2e–4e)
//! @editions 2,3,4
//! @summary With E = Θ(V^(1+ε)) edges, a d-ary heap with d = ⌈V^ε⌉ makes Dijkstra run in O(E) and Johnson's algorithm in O(VE), matching the Floyd–Warshall bound for dense graphs and beating it for sparser ones.
//! @order 148
use super::dijkstra::INF;
use crate::outln;

/// Min d-ary heap of (key, vertex) with a position index, supporting DECREASE-KEY. `ops` counts key comparisons.
pub struct DaryMinHeap { d: usize, a: Vec<(i64, usize)>, pos: Vec<usize>, pub ops: usize }

impl DaryMinHeap {
    pub fn new(d: usize, n: usize) -> Self { DaryMinHeap { d: d.max(2), a: vec![], pos: vec![usize::MAX; n], ops: 0 } }
    fn swap(&mut self, i: usize, j: usize) { self.a.swap(i, j); self.pos[self.a[i].1] = i; self.pos[self.a[j].1] = j; }
    fn up(&mut self, mut i: usize) {
        while i > 0 { let p = (i - 1) / self.d; self.ops += 1; if self.a[i].0 < self.a[p].0 { self.swap(i, p); i = p; } else { break; } }
    }
    fn down(&mut self, mut i: usize) {
        loop {
            let first = self.d * i + 1;
            if first >= self.a.len() { return; }
            let last = (first + self.d).min(self.a.len());
            let mut m = first;
            for c in first + 1..last { self.ops += 1; if self.a[c].0 < self.a[m].0 { m = c; } }
            self.ops += 1;
            if self.a[m].0 < self.a[i].0 { self.swap(i, m); i = m; } else { return; }
        }
    }
    pub fn push_or_decrease(&mut self, v: usize, key: i64) {
        if self.pos[v] == usize::MAX { self.a.push((key, v)); let i = self.a.len() - 1; self.pos[v] = i; self.up(i); }
        else { let i = self.pos[v]; self.a[i].0 = key; self.up(i); }
    }
    pub fn pop(&mut self) -> Option<(i64, usize)> {
        if self.a.is_empty() { return None; }
        let last = self.a.len() - 1;
        self.swap(0, last);
        let top = self.a.pop().unwrap();
        self.pos[top.1] = usize::MAX - 1; // finished
        if !self.a.is_empty() { self.down(0); }
        Some(top)
    }
}

/// Dijkstra with a d-ary heap: O((V·d + E)·log_d V). Returns (distances, heap comparisons).
pub fn dijkstra_dary(adj: &[Vec<(usize, i64)>], s: usize, d: usize) -> (Vec<i64>, usize) {
    let n = adj.len();
    let mut dist = vec![INF; n];
    dist[s] = 0;
    let mut h = DaryMinHeap::new(d, n);
    h.push_or_decrease(s, 0);
    while let Some((du, u)) = h.pop() {
        for &(v, w) in &adj[u] {
            if du + w < dist[v] && h.pos[v] != usize::MAX - 1 { dist[v] = du + w; h.push_or_decrease(v, dist[v]); }
        }
    }
    (dist, h.ops)
}

/// The arity the problem prescribes: d = ⌈V^ε⌉ where E = V^(1+ε), i.e. d ≈ E/V.
pub fn best_arity(n: usize, edges: usize) -> usize { (edges as f64 / n as f64).ceil().max(2.0) as usize }

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = crate::common::Rng::new(25);
    let n = 400;
    outln!(o, "n = {n}; random graphs with E = n^(1+ε) edges, total heap comparisons for all-pairs Dijkstra (n runs):");
    for eps in [0.25f64, 0.5, 0.75] {
        let m = (n as f64).powf(1.0 + eps) as usize;
        let mut adj = vec![vec![]; n];
        for _ in 0..m { adj[r.range(0, n - 1)].push((r.range(0, n - 1), r.range(1, 100) as i64)); }
        let d = best_arity(n, m);
        let (mut ops_bin, mut ops_d) = (0, 0);
        for s in 0..n { ops_bin += dijkstra_dary(&adj, s, 2).1; ops_d += dijkstra_dary(&adj, s, d).1; }
        outln!(o, "ε = {eps}: E = {m}, d = {d}: binary heap {ops_bin} comparisons, d-ary heap {ops_d}  (VE = {})", n * m);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_dijkstra_for_every_arity() {
        let mut r = Rng::new(460);
        for _ in 0..150 {
            let n = r.range(1, 25);
            let mut adj = vec![vec![]; n];
            for _ in 0..r.range(0, 90) { adj[r.range(0, n - 1)].push((r.range(0, n - 1), r.range(0, 20) as i64)); }
            let s = r.range(0, n - 1);
            let want = super::super::dijkstra::dijkstra(&adj, s).0;
            for d in [2, 3, 5, 16] { assert_eq!(dijkstra_dary(&adj, s, d).0, want, "d = {d}"); }
        }
    }
    #[test]
    fn work_with_the_prescribed_arity_is_linear_in_e() {
        let mut r = Rng::new(461);
        let n = 300;
        for eps in [0.5f64, 0.8] {
            let m = (n as f64).powf(1.0 + eps) as usize;
            let mut adj = vec![vec![]; n];
            for _ in 0..m { adj[r.range(0, n - 1)].push((r.range(0, n - 1), r.range(1, 1000) as i64)); }
            let d = best_arity(n, m);
            let ops = dijkstra_dary(&adj, 0, d).1 as f64;
            let bound = 4.0 * (m as f64 + (n * d) as f64 * ((n as f64).ln() / (d as f64).ln() + 1.0));
            assert!(ops <= bound, "ops {ops} bound {bound}");
        }
    }
}
