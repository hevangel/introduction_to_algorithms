//! @title Minimum-Cost Circulation by Cycle Canceling
//! @chapter Problem 29-5 (3e)
//! @editions 3,4
//! @summary A feasible circulation of least cost has no negative-cost cycle in its residual graph: start from the zero flow and repeatedly cancel a negative cycle found with Bellman–Ford.
//! @order 226
use crate::outln;

/// Directed edge with capacity and per-unit cost (costs may be negative).
pub type Edge = (usize, usize, i64, i64);

/// Returns (minimum cost, flow on each edge). Residual edges: forward (cap − f, cost c) and backward (f, −c). Cancels negative cycles until none remains.
pub fn min_cost_circulation(n: usize, edges: &[Edge]) -> (i64, Vec<i64>) {
    let mut flow = vec![0i64; edges.len()];
    loop {
        // residual arcs as (from, to, residual capacity, cost, edge index, is_forward)
        let mut arcs = vec![];
        for (i, &(u, v, cap, cost)) in edges.iter().enumerate() {
            if flow[i] < cap { arcs.push((u, v, cap - flow[i], cost, i, true)); }
            if flow[i] > 0 { arcs.push((v, u, flow[i], -cost, i, false)); }
        }
        // Bellman–Ford from a virtual source (all distances 0); an n-th round relaxation exposes a negative cycle
        let (mut d, mut pred) = (vec![0i64; n], vec![usize::MAX; n]);
        let mut last = None;
        for _ in 0..n {
            last = None;
            for (k, &(u, v, _, c, _, _)) in arcs.iter().enumerate() { if d[u] + c < d[v] { d[v] = d[u] + c; pred[v] = k; last = Some(v); } }
            if last.is_none() { break; }
        }
        let Some(mut x) = last else { break };
        for _ in 0..n { x = arcs[pred[x]].0; } // walk back n steps to land on the cycle
        let (mut cycle, start) = (vec![], x);
        loop { let k = pred[x]; cycle.push(k); x = arcs[k].0; if x == start { break; } }
        let delta = cycle.iter().map(|&k| arcs[k].2).min().unwrap();
        for &k in &cycle { let (_, _, _, _, i, fwd) = arcs[k]; if fwd { flow[i] += delta; } else { flow[i] -= delta; } }
    }
    (edges.iter().zip(&flow).map(|(e, &f)| e.3 * f).sum(), flow)
}

pub fn demo() -> String {
    let mut o = String::new();
    // a triangle 0→1→2→0 whose total cost is −1 (profitable to circulate) plus an expensive chord
    let edges: Vec<Edge> = vec![(0, 1, 4, 2), (1, 2, 3, -5), (2, 0, 5, 1), (0, 2, 2, 6), (2, 1, 1, 1)];
    let (cost, flow) = min_cost_circulation(3, &edges);
    outln!(o, "edges (u, v, cap, cost) {:?}", edges);
    outln!(o, "minimum-cost circulation: cost {cost}, flows {:?}", flow);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::linear_programming::simplex::{simplex, LpResult};
    use crate::common::Rng;
    fn lp_cost(n: usize, edges: &[Edge]) -> f64 {
        let m = edges.len();
        let (mut a, mut b) = (vec![], vec![]);
        for v in 0..n { // conservation: Σ out − Σ in = 0 as two inequalities
            let row: Vec<f64> = edges.iter().map(|&(x, y, _, _)| (x == v) as i32 as f64 - (y == v) as i32 as f64).collect();
            a.push(row.clone()); b.push(0.0);
            a.push(row.iter().map(|v| -v).collect()); b.push(0.0);
        }
        for (i, e) in edges.iter().enumerate() { let mut row = vec![0.0; m]; row[i] = 1.0; a.push(row); b.push(e.2 as f64); }
        match simplex(&a, &b, &edges.iter().map(|e| -(e.3 as f64)).collect::<Vec<_>>()) { LpResult::Optimal { value, .. } => -value, _ => f64::NAN }
    }
    #[test]
    fn matches_the_linear_program_and_is_a_valid_circulation() {
        let mut r = Rng::new(499);
        for _ in 0..150 {
            let n = r.range(2, 6);
            let edges: Vec<Edge> = (0..r.range(1, 10)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(1, 4) as i64, r.range(0, 14) as i64 - 5)).filter(|e| e.0 != e.1).collect();
            if edges.is_empty() { continue; }
            let (cost, flow) = min_cost_circulation(n, &edges);
            for v in 0..n { assert_eq!(edges.iter().zip(&flow).map(|(e, &f)| if e.0 == v { f } else if e.1 == v { -f } else { 0 }).sum::<i64>(), 0, "conservation at {v}"); }
            assert!(edges.iter().zip(&flow).all(|(e, &f)| (0..=e.2).contains(&f)));
            assert!((cost as f64 - lp_cost(n, &edges)).abs() < 1e-6, "cost {cost} vs LP {} for {edges:?}", lp_cost(n, &edges));
        }
    }
    #[test]
    fn a_profitable_cycle_is_filled_to_capacity() {
        let (cost, flow) = min_cost_circulation(3, &[(0, 1, 4, 1), (1, 2, 3, 1), (2, 0, 5, -5)]);
        assert_eq!((cost, flow), (-9, vec![3, 3, 3])); // 3 units around the cycle of cost −3 each
    }
}
