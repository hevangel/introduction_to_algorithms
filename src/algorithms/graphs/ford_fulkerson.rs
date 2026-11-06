//! @title Ford-Fulkerson & Edmonds-Karp Maximum Flow
//! @chapter 26.1–26.2 (1e–3e) / 24.1–24.2 (4e)
//! @editions 1,2,3,4
//! @summary Augment along residual paths: DFS (Ford-Fulkerson) or BFS shortest paths (Edmonds-Karp, O(VE²)); min-cut extraction.
//! @order 170
use crate::outln;
use std::collections::VecDeque;

/// Flow network with paired residual edges (edge `e` and its reverse `e ^ 1`).
#[derive(Debug, Clone)]
pub struct FlowNetwork {
    pub n: usize,
    to: Vec<usize>,
    /// Residual capacity of every directed edge.
    cap: Vec<i64>,
    adj: Vec<Vec<usize>>,
}

impl FlowNetwork {
    pub fn new(n: usize) -> Self { FlowNetwork { n, to: vec![], cap: vec![], adj: vec![vec![]; n] } }
    /// Add edge u → v with capacity c; returns the edge id (the reverse edge is id + 1).
    pub fn add_edge(&mut self, u: usize, v: usize, c: i64) -> usize {
        let id = self.to.len();
        self.to.push(v); self.cap.push(c); self.adj[u].push(id);
        self.to.push(u); self.cap.push(0); self.adj[v].push(id + 1);
        id
    }
    /// Flow currently pushed through edge `id` (= residual capacity of its reverse edge).
    pub fn flow_on(&self, id: usize) -> i64 { self.cap[id ^ 1] }
    pub fn capacity_of(&self, id: usize) -> i64 { self.cap[id] + self.cap[id ^ 1] }

    /// EDMONDS-KARP: always augment along a shortest (fewest-edge) residual path — O(V·E²).
    pub fn edmonds_karp(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0;
        loop {
            let mut pred = vec![usize::MAX; self.n];
            let mut q = VecDeque::from([s]);
            let mut seen = vec![false; self.n];
            seen[s] = true;
            while let Some(u) = q.pop_front() {
                for &e in &self.adj[u] {
                    let v = self.to[e];
                    if !seen[v] && self.cap[e] > 0 { seen[v] = true; pred[v] = e; q.push_back(v); }
                }
            }
            if !seen[t] { return flow; }
            // bottleneck residual capacity c_f(p)
            let (mut b, mut v) = (i64::MAX, t);
            while v != s { let e = pred[v]; b = b.min(self.cap[e]); v = self.to[e ^ 1]; }
            v = t;
            while v != s { let e = pred[v]; self.cap[e] -= b; self.cap[e ^ 1] += b; v = self.to[e ^ 1]; }
            flow += b;
        }
    }

    /// FORD-FULKERSON with depth-first augmenting paths (may be slow when capacities are huge).
    pub fn ford_fulkerson_dfs(&mut self, s: usize, t: usize) -> i64 {
        fn dfs(g: &mut FlowNetwork, u: usize, t: usize, f: i64, seen: &mut Vec<bool>) -> i64 {
            if u == t { return f; }
            seen[u] = true;
            for i in 0..g.adj[u].len() {
                let e = g.adj[u][i];
                let v = g.to[e];
                if !seen[v] && g.cap[e] > 0 {
                    let d = dfs(g, v, t, f.min(g.cap[e]), seen);
                    if d > 0 { g.cap[e] -= d; g.cap[e ^ 1] += d; return d; }
                }
            }
            0
        }
        let mut flow = 0;
        loop {
            let mut seen = vec![false; self.n];
            let f = dfs(self, s, t, i64::MAX, &mut seen);
            if f == 0 { return flow; }
            flow += f;
        }
    }

    /// After a maximum flow: vertices reachable from s in the residual graph (the source side of a minimum cut).
    pub fn min_cut_source_side(&self, s: usize) -> Vec<bool> {
        let mut seen = vec![false; self.n];
        let mut st = vec![s];
        seen[s] = true;
        while let Some(u) = st.pop() {
            for &e in &self.adj[u] { let v = self.to[e]; if !seen[v] && self.cap[e] > 0 { seen[v] = true; st.push(v); } }
        }
        seen
    }
    /// Capacity of the cut (S, V−S) in the original network.
    pub fn cut_capacity(&self, side: &[bool]) -> i64 {
        (0..self.to.len()).step_by(2).filter(|&e| side[self.to[e ^ 1]] && !side[self.to[e]]).map(|e| self.capacity_of(e)).sum()
    }
    /// Net flow out of vertex u (0 for internal vertices — flow conservation).
    pub fn net_flow(&self, u: usize) -> i64 {
        self.adj[u].iter().map(|&e| if e % 2 == 0 { self.flow_on(e) } else { -self.flow_on(e ^ 1) }).sum()
    }
}

/// Figure 26.1: a shipping network. s=0, v1=1, v2=2, v3=3, v4=4, t=5.
pub fn clrs_network() -> FlowNetwork {
    let mut g = FlowNetwork::new(6);
    for &(u, v, c) in &[(0, 1, 16), (0, 2, 13), (1, 3, 12), (2, 1, 4), (2, 4, 14), (3, 2, 9), (3, 5, 20), (4, 3, 7), (4, 5, 4)] { g.add_edge(u, v, c); }
    g
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = clrs_network();
    outln!(o, "Edmonds-Karp max flow       = {}", a.edmonds_karp(0, 5));
    let side = a.min_cut_source_side(0);
    outln!(o, "min cut: S = {:?}, capacity = {}", (0..6).filter(|&v| side[v]).collect::<Vec<_>>(), a.cut_capacity(&side));
    let mut b = clrs_network();
    outln!(o, "Ford-Fulkerson (DFS) value  = {}", b.ford_fulkerson_dfs(0, 5));
    outln!(o, "flow conservation: net flow at v1..v4 = {:?}", (1..5).map(|v| a.net_flow(v)).collect::<Vec<_>>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    pub fn random_network(r: &mut Rng) -> (FlowNetwork, usize) {
        let n = r.range(2, 9);
        let mut g = FlowNetwork::new(n);
        for _ in 0..r.range(1, 25) {
            let (u, v) = (r.range(0, n - 1), r.range(0, n - 1));
            if u != v { g.add_edge(u, v, r.range(1, 20) as i64); }
        }
        (g, n)
    }
    #[test]
    fn book_value_23() {
        assert_eq!(clrs_network().edmonds_karp(0, 5), 23);
        assert_eq!(clrs_network().ford_fulkerson_dfs(0, 5), 23);
    }
    #[test]
    fn max_flow_min_cut_theorem_and_conservation() {
        let mut r = Rng::new(138);
        for _ in 0..80 {
            let (g, n) = random_network(&mut r);
            let (mut a, mut b) = (g.clone(), g.clone());
            let f = a.edmonds_karp(0, n - 1);
            assert_eq!(f, b.ford_fulkerson_dfs(0, n - 1));
            let side = a.min_cut_source_side(0);
            assert!(!side[n - 1] || f == 0 && false || !side[n - 1]);
            assert_eq!(a.cut_capacity(&side), f);
            for v in 1..n - 1 { assert_eq!(a.net_flow(v), 0); }
            assert_eq!(a.net_flow(0), f);
            for e in (0..a.to.len()).step_by(2) { assert!(a.flow_on(e) >= 0 && a.flow_on(e) <= a.capacity_of(e)); }
        }
    }
}
