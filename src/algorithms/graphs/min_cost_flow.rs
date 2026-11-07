//! @title Minimum-Cost Flow
//! @chapter 29.2 (3e) / 29 (4e), Problem 29-? ; 26 exercises
//! @editions 2,3,4
//! @summary Send a required amount of flow at minimum total cost by successive shortest augmenting paths in the residual graph (with negative reverse costs).
//! @order 225
use crate::outln;

#[derive(Debug, Clone)]
struct Edge { to: usize, cap: i64, cost: i64 }

/// Min-cost flow network with paired residual edges (edge e and its reverse e ^ 1, which has cost −cost).
#[derive(Debug, Clone)]
pub struct CostNetwork { n: usize, e: Vec<Edge>, adj: Vec<Vec<usize>> }

impl CostNetwork {
    pub fn new(n: usize) -> Self { CostNetwork { n, e: vec![], adj: vec![vec![]; n] } }
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64, cost: i64) -> usize {
        let id = self.e.len();
        self.e.push(Edge { to: v, cap, cost });
        self.adj[u].push(id);
        self.e.push(Edge { to: u, cap: 0, cost: -cost });
        self.adj[v].push(id + 1);
        id
    }
    pub fn flow_on(&self, id: usize) -> i64 { self.e[id ^ 1].cap }
    /// Push up to `want` units from s to t, each along a cheapest residual path (Bellman–Ford handles the negative reverse edges).
    /// Returns (flow sent, total cost). Costs along successive shortest paths never decrease, which makes the result optimal.
    pub fn min_cost_flow(&mut self, s: usize, t: usize, want: i64) -> (i64, i64) {
        let (mut flow, mut cost) = (0, 0);
        while flow < want {
            let inf = i64::MAX / 4;
            let (mut d, mut pred) = (vec![inf; self.n], vec![usize::MAX; self.n]);
            d[s] = 0;
            for _ in 0..self.n {
                let mut ch = false;
                for u in 0..self.n {
                    if d[u] >= inf { continue; }
                    for &id in &self.adj[u] {
                        let ed = &self.e[id];
                        if ed.cap > 0 && d[u] + ed.cost < d[ed.to] { d[ed.to] = d[u] + ed.cost; pred[ed.to] = id; ch = true; }
                    }
                }
                if !ch { break; }
            }
            if d[t] >= inf { break; }
            let (mut b, mut v) = (want - flow, t);
            while v != s { let id = pred[v]; b = b.min(self.e[id].cap); v = self.e[id ^ 1].to; }
            v = t;
            while v != s { let id = pred[v]; self.e[id].cap -= b; self.e[id ^ 1].cap += b; v = self.e[id ^ 1].to; }
            flow += b;
            cost += b * d[t];
        }
        (flow, cost)
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut g = CostNetwork::new(4);
    // s=0, a=1, b=2, t=3: (capacity, cost)
    g.add_edge(0, 1, 2, 1); g.add_edge(0, 2, 1, 4); g.add_edge(1, 2, 1, 1); g.add_edge(1, 3, 1, 5); g.add_edge(2, 3, 2, 1);
    outln!(o, "send 3 units from s to t: (flow, cost) = {:?}", g.clone().min_cost_flow(0, 3, 3));
    outln!(o, "send 1 unit  : {:?}", g.clone().min_cost_flow(0, 3, 1));
    outln!(o, "send 5 units (only 3 fit): {:?}", g.clone().min_cost_flow(0, 3, 5));
    // assignment problem as min-cost flow
    let cost = [[9, 2, 7], [6, 4, 3], [5, 8, 1]];
    let mut a = CostNetwork::new(8);
    for i in 0..3 { a.add_edge(6, i, 1, 0); a.add_edge(3 + i, 7, 1, 0); for j in 0..3 { a.add_edge(i, 3 + j, 1, cost[i][j]); } }
    outln!(o, "3×3 assignment as a flow problem: {:?} (Hungarian: {})", a.min_cost_flow(6, 7, 3), super::hungarian::hungarian(&cost.iter().map(|r| r.to_vec()).collect::<Vec<_>>()).0);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn assignment_via_flow_equals_hungarian() {
        let mut r = Rng::new(372);
        for _ in 0..80 {
            let n = r.range(1, 6);
            let c: Vec<Vec<i64>> = (0..n).map(|_| r.vec(n, 0, 30)).collect();
            let mut g = CostNetwork::new(2 * n + 2);
            let (s, t) = (2 * n, 2 * n + 1);
            for i in 0..n { g.add_edge(s, i, 1, 0); g.add_edge(n + i, t, 1, 0); for j in 0..n { g.add_edge(i, n + j, 1, c[i][j]); } }
            let (f, cost) = g.min_cost_flow(s, t, n as i64);
            assert_eq!(f, n as i64);
            assert_eq!(cost, super::super::hungarian::hungarian(&c).0);
        }
    }
    #[test]
    fn small_network_and_flow_conservation() {
        let mut g = CostNetwork::new(4);
        let ids = [g.add_edge(0, 1, 2, 1), g.add_edge(0, 2, 1, 4), g.add_edge(1, 2, 1, 1), g.add_edge(1, 3, 1, 5), g.add_edge(2, 3, 2, 1)];
        assert_eq!(g.min_cost_flow(0, 3, 3), (3, 3 + 5 + 6)); // paths s-a-b-t (3), s-b-t (5), s-a-t (6)
        let out0: i64 = [ids[0], ids[1]].iter().map(|&i| g.flow_on(i)).sum();
        assert_eq!(out0, 3);
    }
}
