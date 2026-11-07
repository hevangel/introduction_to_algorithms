//! @title Integer Linear Programming (Branch and Bound)
//! @chapter Problem 29-3 (3e)
//! @editions 3,4
//! @summary Requiring the variables to be integers makes LP NP-hard; branch and bound repeatedly solves the LP relaxation and splits on a fractional variable (x ≤ ⌊v⌋ or x ≥ ⌈v⌉), pruning by the relaxation's bound.
//! @order 35
use super::simplex::{simplex, LpResult};
use crate::outln;

/// maximise cᵀx s.t. Ax ≤ b, x ≥ 0 and x integral. Returns the optimal value and an optimal integral x, or None if infeasible.
/// The system must be bounded (e.g. all constraint coefficients non-negative with every variable appearing in some row).
pub fn ilp_maximize(a: &[Vec<f64>], b: &[f64], c: &[f64]) -> Option<(i64, Vec<i64>)> {
    let mut best: Option<(i64, Vec<i64>)> = None;
    branch(a.to_vec(), b.to_vec(), c, &mut best, 0);
    best
}

fn branch(a: Vec<Vec<f64>>, b: Vec<f64>, c: &[f64], best: &mut Option<(i64, Vec<i64>)>, depth: usize) {
    if depth > 60 { return; }
    let LpResult::Optimal { value, x } = simplex(&a, &b, c) else { return };
    if let Some((bv, _)) = best { if value < *bv as f64 + 1.0 - 1e-6 && c.iter().all(|v| v.fract() == 0.0) { return; } } // integer objective: need a strictly better integer value
    match x.iter().position(|v| (v - v.round()).abs() > 1e-6) {
        None => { // the relaxation is already integral
            let xi: Vec<i64> = x.iter().map(|v| v.round() as i64).collect();
            let val: i64 = c.iter().zip(&xi).map(|(ci, xv)| (ci * *xv as f64).round() as i64).sum();
            if best.as_ref().is_none_or(|(bv, _)| val > *bv) { *best = Some((val, xi)); }
        }
        Some(j) => {
            let n = c.len();
            let mut row = vec![0.0; n];
            row[j] = 1.0;
            let (mut a1, mut b1) = (a.clone(), b.clone()); // x_j ≤ ⌊v⌋
            a1.push(row.clone()); b1.push(x[j].floor());
            branch(a1, b1, c, best, depth + 1);
            let (mut a2, mut b2) = (a, b); // x_j ≥ ⌈v⌉  ⇔  −x_j ≤ −⌈v⌉
            row[j] = -1.0;
            a2.push(row); b2.push(-x[j].ceil());
            branch(a2, b2, c, best, depth + 1);
        }
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    // maximise 5x + 4y  s.t.  6x + 4y ≤ 24,  x + 2y ≤ 6  (LP optimum 21 at (3, 1.5); integer optimum 20)
    let (a, b, c) = (vec![vec![6.0, 4.0], vec![1.0, 2.0]], vec![24.0, 6.0], vec![5.0, 4.0]);
    outln!(o, "LP relaxation: {:?}", simplex(&a, &b, &c));
    outln!(o, "integer optimum: {:?}", ilp_maximize(&a, &b, &c));
    outln!(o, "knapsack as an ILP: {:?}", ilp_maximize(&[vec![3.0, 4.0, 5.0], vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]], &[8.0, 1.0, 1.0, 1.0], &[4.0, 5.0, 6.0]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_enumeration_of_all_integer_points() {
        let mut r = Rng::new(498);
        for _ in 0..150 {
            let (m, n) = (r.range(1, 3), r.range(1, 3));
            let a: Vec<Vec<f64>> = (0..m).map(|_| r.vec(n, 1, 6).into_iter().map(|v| v as f64).collect()).collect();
            let b: Vec<f64> = r.vec(m, 3, 25).into_iter().map(|v| v as f64).collect();
            let c: Vec<f64> = r.vec(n, 1, 9).into_iter().map(|v| v as f64).collect();
            let bound = 26i64;
            let mut best = 0i64; // x = 0 is feasible, so the optimum is at least 0
            let mut x = vec![0i64; n];
            loop {
                if (0..m).all(|i| (0..n).map(|j| a[i][j] * x[j] as f64).sum::<f64>() <= b[i] + 1e-9) { best = best.max((0..n).map(|j| c[j] as i64 * x[j]).sum()); }
                let mut k = 0;
                while k < n { x[k] += 1; if x[k] <= bound { break; } x[k] = 0; k += 1; }
                if k == n { break; }
            }
            let (v, sol) = ilp_maximize(&a, &b, &c).unwrap();
            assert_eq!(v, best, "A={a:?} b={b:?} c={c:?}");
            assert!((0..m).all(|i| (0..n).map(|j| a[i][j] * sol[j] as f64).sum::<f64>() <= b[i] + 1e-9));
        }
    }
    #[test]
    fn integer_optimum_can_be_strictly_worse_than_the_relaxation() {
        let (a, b, c) = (vec![vec![6.0, 4.0], vec![1.0, 2.0]], vec![24.0, 6.0], vec![5.0, 4.0]);
        let LpResult::Optimal { value, .. } = simplex(&a, &b, &c) else { panic!() };
        assert!((value - 21.0).abs() < 1e-6);
        assert_eq!(ilp_maximize(&a, &b, &c).unwrap().0, 20);
    }
}
