//! @title Weighted Vertex Cover by LP Rounding
//! @chapter 35.4 (2e–4e)
//! @editions 2,3,4
//! @summary Solve the LP relaxation (x_u + x_v ≥ 1), then round every x_v ≥ ½ up: a 2-approximation for weighted cover.
//! @order 50
use crate::algorithms::linear_programming::simplex::{simplex, LpResult};
use crate::outln;

/// Solve the relaxation  minimise Σ w_v x_v  s.t.  x_u + x_v ≥ 1 for every edge,  0 ≤ x ≤ 1. Returns (LP optimum, fractional x).
pub fn vertex_cover_lp(n: usize, edges: &[(usize, usize)], w: &[f64]) -> (f64, Vec<f64>) {
    // as a maximisation in standard form: max −w·x  s.t.  −x_u − x_v ≤ −1,  x_v ≤ 1,  x ≥ 0
    let mut a: Vec<Vec<f64>> = vec![];
    let mut b: Vec<f64> = vec![];
    for &(u, v) in edges {
        let mut row = vec![0.0; n];
        row[u] = -1.0;
        row[v] = -1.0;
        a.push(row);
        b.push(-1.0);
    }
    for v in 0..n {
        let mut row = vec![0.0; n];
        row[v] = 1.0;
        a.push(row);
        b.push(1.0);
    }
    let c: Vec<f64> = w.iter().map(|x| -x).collect();
    match simplex(&a, &b, &c) {
        LpResult::Optimal { value, x } => (-value, x),
        other => panic!("vertex-cover LP is always feasible and bounded, got {other:?}"),
    }
}

/// APPROX-MIN-WEIGHT-VC(G, w): LP, then C = { v : x̄_v ≥ ½ }. Returns (cover, its weight, LP lower bound).
pub fn approx_min_weight_vc(n: usize, edges: &[(usize, usize)], w: &[f64]) -> (Vec<usize>, f64, f64) {
    let (lp, x) = vertex_cover_lp(n, edges, w);
    let cover: Vec<usize> = (0..n).filter(|&v| x[v] >= 0.5 - 1e-9).collect();
    let weight = cover.iter().map(|&v| w[v]).sum();
    (cover, weight, lp)
}

pub fn optimal_weight(n: usize, edges: &[(usize, usize)], w: &[f64]) -> f64 {
    (0u32..1 << n).filter(|m| edges.iter().all(|&(u, v)| m >> u & 1 == 1 || m >> v & 1 == 1))
        .map(|m| (0..n).filter(|i| m >> i & 1 == 1).map(|i| w[i]).sum()).fold(f64::INFINITY, f64::min)
}

pub fn demo() -> String {
    let mut o = String::new();
    // triangle with weights 1: LP optimum 1.5 (x = ½ each), integral optimum 2, rounding gives 3 vertices of weight 3?
    let edges = [(0, 1), (1, 2), (0, 2)];
    let (lp, x) = vertex_cover_lp(3, &edges, &[1.0, 1.0, 1.0]);
    outln!(o, "triangle, unit weights: LP optimum {:.3}, x = {:?}", lp, x.iter().map(|v| (v * 1000.0).round() / 1000.0).collect::<Vec<_>>());
    let (c, wt, _) = approx_min_weight_vc(3, &edges, &[1.0, 1.0, 1.0]);
    outln!(o, "  rounded cover {:?} weight {} vs optimum {}", c, wt, optimal_weight(3, &edges, &[1.0, 1.0, 1.0]));
    let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 0), (1, 3)];
    let w = [3.0, 1.0, 4.0, 1.0, 5.0];
    let (c, wt, lp) = approx_min_weight_vc(5, &edges, &w);
    outln!(o, "weighted 5-vertex graph: cover {:?}, weight {} (LP bound {:.3}, optimum {})", c, wt, lp, optimal_weight(5, &edges, &w));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn rounding_gives_valid_cover_within_twice_the_lp_bound() {
        let mut r = Rng::new(254);
        for _ in 0..150 {
            let n = r.range(2, 9);
            let edges: Vec<(usize, usize)> = (0..r.range(1, 14)).map(|_| (r.range(0, n - 1), r.range(0, n - 1))).filter(|e| e.0 != e.1).collect();
            if edges.is_empty() { continue; }
            let w: Vec<f64> = (0..n).map(|_| r.range(1, 9) as f64).collect();
            let (cover, weight, lp) = approx_min_weight_vc(n, &edges, &w);
            assert!(edges.iter().all(|&(u, v)| cover.contains(&u) || cover.contains(&v)));
            let opt = optimal_weight(n, &edges, &w);
            assert!(lp <= opt + 1e-6, "LP is a lower bound");
            assert!(weight <= 2.0 * lp + 1e-6, "rounding at most doubles the LP weight");
            assert!(weight <= 2.0 * opt + 1e-6);
        }
    }
    #[test]
    fn triangle_lp_is_half_integral() {
        let (lp, x) = vertex_cover_lp(3, &[(0, 1), (1, 2), (0, 2)], &[1.0, 1.0, 1.0]);
        assert!((lp - 1.5).abs() < 1e-7);
        assert!(x.iter().all(|v| (v - 0.5).abs() < 1e-7));
    }
}
