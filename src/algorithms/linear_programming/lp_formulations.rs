//! @title Shortest Paths, Max Flow & Min-Cost Flow as Linear Programs
//! @chapter 29.2 (2e–4e)
//! @editions 2,3,4
//! @summary Section 29.2's formulations solved with this repository's simplex and checked against Dijkstra, Edmonds-Karp and successive shortest paths.
//! @order 20
use super::simplex::{simplex, LpResult};
use crate::outln;

pub type Edge = (usize, usize, i64);

/// Build "maximise c·x subject to the equalities A_eq x = b_eq and inequalities A x ≤ b" in the simplex's standard form
/// by splitting every equality into two inequalities.
fn solve(n_vars: usize, eq: &[(Vec<f64>, f64)], le: &[(Vec<f64>, f64)], c: &[f64]) -> LpResult {
    let mut a = vec![];
    let mut b = vec![];
    for (row, rhs) in eq { a.push(row.clone()); b.push(*rhs); a.push(row.iter().map(|x| -x).collect()); b.push(-rhs); }
    for (row, rhs) in le { a.push(row.clone()); b.push(*rhs); }
    let _ = n_vars;
    simplex(&a, &b, c)
}

/// Flow-conservation row for vertex v: Σ out − Σ in.
fn conservation(n: usize, edges: &[Edge], v: usize) -> Vec<f64> {
    let _ = n;
    edges.iter().map(|&(a, b, _)| (a == v) as i32 as f64 - (b == v) as i32 as f64).collect()
}

/// Shortest-path weight s ⇝ t as an LP: minimise Σ w_e x_e with one unit of flow from s to t (x ≥ 0).
pub fn shortest_path_lp(n: usize, edges: &[Edge], s: usize, t: usize) -> Option<f64> {
    let eq: Vec<(Vec<f64>, f64)> = (0..n).map(|v| (conservation(n, edges, v), if v == s { 1.0 } else if v == t { -1.0 } else { 0.0 })).collect();
    let c: Vec<f64> = edges.iter().map(|e| -(e.2 as f64)).collect();
    match solve(edges.len(), &eq, &[], &c) { LpResult::Optimal { value, .. } => Some(-value), _ => None }
}

/// Maximum flow as an LP: maximise the flow out of s subject to capacities and conservation at inner vertices.
pub fn max_flow_lp(n: usize, edges: &[Edge], s: usize, t: usize) -> f64 {
    let eq: Vec<(Vec<f64>, f64)> = (0..n).filter(|&v| v != s && v != t).map(|v| (conservation(n, edges, v), 0.0)).collect();
    let le: Vec<(Vec<f64>, f64)> = edges.iter().enumerate().map(|(i, e)| { let mut row = vec![0.0; edges.len()]; row[i] = 1.0; (row, e.2 as f64) }).collect();
    let c: Vec<f64> = (0..edges.len()).map(|i| (edges[i].0 == s) as i32 as f64 - (edges[i].1 == s) as i32 as f64).collect();
    match solve(edges.len(), &eq, &le, &c) { LpResult::Optimal { value, .. } => value, _ => f64::NAN }
}

/// Minimum-cost flow of value `d`: edges are (u, v, capacity); `cost[i]` is the unit cost of edge i.
pub fn min_cost_flow_lp(n: usize, edges: &[Edge], cost: &[i64], s: usize, t: usize, d: i64) -> Option<f64> {
    let eq: Vec<(Vec<f64>, f64)> = (0..n).map(|v| (conservation(n, edges, v), if v == s { d as f64 } else if v == t { -(d as f64) } else { 0.0 })).collect();
    let le: Vec<(Vec<f64>, f64)> = edges.iter().enumerate().map(|(i, e)| { let mut row = vec![0.0; edges.len()]; row[i] = 1.0; (row, e.2 as f64) }).collect();
    let c: Vec<f64> = cost.iter().map(|&x| -(x as f64)).collect();
    match solve(edges.len(), &eq, &le, &c) { LpResult::Optimal { value, .. } => Some(-value), _ => None }
}

pub fn demo() -> String {
    let mut o = String::new();
    let sp: Vec<Edge> = vec![(0, 1, 10), (0, 3, 5), (1, 2, 1), (1, 3, 2), (2, 4, 4), (3, 1, 3), (3, 2, 9), (3, 4, 2), (4, 0, 7), (4, 2, 6)];
    outln!(o, "shortest path 0 ⇝ 2 as an LP: {:?}  (Dijkstra: 9)", shortest_path_lp(5, &sp, 0, 2));
    let fl: Vec<Edge> = vec![(0, 1, 16), (0, 2, 13), (1, 3, 12), (2, 1, 4), (2, 4, 14), (3, 2, 9), (3, 5, 20), (4, 3, 7), (4, 5, 4)];
    outln!(o, "max flow of the CLRS network as an LP: {}  (Edmonds–Karp: 23)", max_flow_lp(6, &fl, 0, 5));
    let net: Vec<Edge> = vec![(0, 1, 2), (0, 2, 1), (1, 2, 1), (1, 3, 1), (2, 3, 2)];
    outln!(o, "min-cost flow of 3 units as an LP: {:?}", min_cost_flow_lp(4, &net, &[1, 4, 1, 5, 1], 0, 3, 3));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn shortest_paths_match_dijkstra() {
        let mut r = Rng::new(380);
        for _ in 0..60 {
            let n = r.range(2, 6);
            let edges: Vec<Edge> = (0..r.range(1, 12)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(1, 9) as i64)).filter(|e| e.0 != e.1).collect();
            let mut adj = vec![vec![]; n];
            for &(u, v, w) in &edges { adj[u].push((v, w)); }
            let (d, _) = crate::algorithms::graphs::dijkstra::dijkstra(&adj, 0);
            let t = n - 1;
            match shortest_path_lp(n, &edges, 0, t) {
                Some(v) => { assert!(d[t] < crate::algorithms::graphs::dijkstra::INF); assert!((v - d[t] as f64).abs() < 1e-6, "{v} vs {}", d[t]); }
                None => assert!(d[t] >= crate::algorithms::graphs::dijkstra::INF || t == 0),
            }
        }
    }
    #[test]
    fn max_flow_matches_edmonds_karp() {
        let mut r = Rng::new(381);
        for _ in 0..60 {
            let n = r.range(2, 6);
            let edges: Vec<Edge> = (0..r.range(1, 12)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(1, 9) as i64)).filter(|e| e.0 != e.1).collect();
            if edges.is_empty() { continue; }
            let mut g = crate::algorithms::graphs::ford_fulkerson::FlowNetwork::new(n);
            for &(u, v, c) in &edges { g.add_edge(u, v, c); }
            let want = g.edmonds_karp(0, n - 1) as f64;
            assert!((max_flow_lp(n, &edges, 0, n - 1) - want).abs() < 1e-6);
        }
        let fl: Vec<Edge> = vec![(0, 1, 16), (0, 2, 13), (1, 3, 12), (2, 1, 4), (2, 4, 14), (3, 2, 9), (3, 5, 20), (4, 3, 7), (4, 5, 4)];
        assert!((max_flow_lp(6, &fl, 0, 5) - 23.0).abs() < 1e-6);
    }
    #[test]
    fn min_cost_flow_matches_successive_shortest_paths() {
        let mut r = Rng::new(382);
        for _ in 0..60 {
            let n = r.range(2, 6);
            let edges: Vec<Edge> = (0..r.range(1, 12)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(1, 5) as i64)).filter(|e| e.0 != e.1).collect();
            if edges.is_empty() { continue; }
            let cost: Vec<i64> = edges.iter().map(|_| r.range(0, 9) as i64).collect();
            let d = r.range(1, 4) as i64;
            let mut g = crate::algorithms::graphs::min_cost_flow::CostNetwork::new(n);
            for (i, &(u, v, c)) in edges.iter().enumerate() { g.add_edge(u, v, c, cost[i]); }
            let (sent, c) = g.min_cost_flow(0, n - 1, d);
            let lp = min_cost_flow_lp(n, &edges, &cost, 0, n - 1, d);
            if sent == d { assert!((lp.expect("feasible") - c as f64).abs() < 1e-6, "{lp:?} vs {c}"); } else { assert!(lp.is_none(), "flow {d} infeasible"); }
        }
    }
}
