//! @title LP Feasibility, Farkas' Lemma and Complementary Slackness
//! @chapter Problems 29-1, 29-2, 29-4 (3e)
//! @editions 3,4
//! @summary Decide whether Ax ≤ b has a solution with the simplex method; if not, produce a Farkas certificate y ≥ 0 with yᵀA = 0 and yᵀb < 0; and check that optimal primal/dual pairs satisfy complementary slackness.
//! @order 30
use super::simplex::{dual, simplex, LpResult};
use crate::outln;

/// A point x (free variables, possibly negative) with Ax ≤ b, or None if the system is infeasible. Variables are split x = x⁺ − x⁻ for the simplex.
pub fn feasible_point(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    let n = a[0].len();
    let a2: Vec<Vec<f64>> = a.iter().map(|row| row.iter().copied().chain(row.iter().map(|v| -v)).collect()).collect();
    match simplex(&a2, b, &vec![0.0; 2 * n]) {
        LpResult::Optimal { x, .. } => Some((0..n).map(|j| x[j] - x[n + j]).collect()),
        _ => None,
    }
}

/// Farkas' lemma: exactly one of "∃x: Ax ≤ b" and "∃y ≥ 0: yᵀA = 0, yᵀb < 0" holds. Returns the certificate y of infeasibility, found by the LP
/// maximise −bᵀy subject to Aᵀy = 0, Σy ≤ 1, y ≥ 0 (positive optimum ⇔ infeasible).
pub fn farkas_certificate(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    let (m, n) = (a.len(), a[0].len());
    let mut rows: Vec<Vec<f64>> = vec![];
    let mut rhs: Vec<f64> = vec![];
    for j in 0..n { // Aᵀy = 0 as two inequalities
        rows.push((0..m).map(|i| a[i][j]).collect()); rhs.push(0.0);
        rows.push((0..m).map(|i| -a[i][j]).collect()); rhs.push(0.0);
    }
    rows.push(vec![1.0; m]); rhs.push(1.0); // normalisation Σ y ≤ 1
    let c: Vec<f64> = b.iter().map(|v| -v).collect();
    match simplex(&rows, &rhs, &c) {
        LpResult::Optimal { value, x } if value > 1e-9 => Some(x),
        _ => None,
    }
}

/// Complementary slackness (Problem 29-2): for optimal x of  max cᵀx, Ax ≤ b, x ≥ 0  and optimal y of the dual  min bᵀy, Aᵀy ≥ c, y ≥ 0:
/// x_j > 0 ⇒ (Aᵀy)_j = c_j and y_i > 0 ⇒ (Ax)_i = b_i. Returns (primal value, dual value, whether the conditions hold), or None if not solvable.
pub fn check_complementary_slackness(a: &[Vec<f64>], b: &[f64], c: &[f64]) -> Option<(f64, f64, bool)> {
    let (LpResult::Optimal { value: pv, x }, (da, db, dc)) = (simplex(a, b, c), dual(a, b, c)) else { return None };
    let LpResult::Optimal { value: dv, x: y } = simplex(&da, &db, &dc) else { return None };
    let (m, n) = (b.len(), c.len());
    let tol = 1e-6;
    let ax: Vec<f64> = (0..m).map(|i| (0..n).map(|j| a[i][j] * x[j]).sum()).collect();
    let aty: Vec<f64> = (0..n).map(|j| (0..m).map(|i| a[i][j] * y[i]).sum()).collect();
    let holds = (0..n).all(|j| x[j] <= tol || (aty[j] - c[j]).abs() <= tol) && (0..m).all(|i| y[i] <= tol || (ax[i] - b[i]).abs() <= tol);
    Some((pv, -dv, holds))
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = vec![vec![1.0, 1.0], vec![-1.0, 0.0], vec![0.0, -1.0]];
    outln!(o, "x + y ≤ 4, x ≥ 0, y ≥ 0 → feasible point {:?}", feasible_point(&a, &[4.0, 0.0, 0.0]));
    let infeasible_a = vec![vec![1.0], vec![-1.0]];
    let b = [1.0, -3.0]; // x ≤ 1 and x ≥ 3
    outln!(o, "x ≤ 1 and x ≥ 3 → feasible point {:?}, Farkas certificate {:?}", feasible_point(&infeasible_a, &b), farkas_certificate(&infeasible_a, &b));
    let (pa, pb, pc) = (vec![vec![1.0, 1.0, 3.0], vec![2.0, 2.0, 5.0], vec![4.0, 1.0, 2.0]], vec![30.0, 24.0, 36.0], vec![3.0, 1.0, 2.0]);
    outln!(o, "CLRS Section 29.3 LP: (primal value, dual value, complementary slackness) = {:?}", check_complementary_slackness(&pa, &pb, &pc));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn exactly_one_of_a_solution_or_a_farkas_certificate() {
        let mut r = Rng::new(496);
        let (mut feas, mut infeas) = (0, 0);
        for _ in 0..300 {
            let (m, n) = (r.range(1, 5), r.range(1, 3));
            let a: Vec<Vec<f64>> = (0..m).map(|_| r.vec(n, -3, 3).into_iter().map(|v| v as f64).collect()).collect();
            let b: Vec<f64> = r.vec(m, -4, 6).into_iter().map(|v| v as f64).collect();
            let point = feasible_point(&a, &b);
            let cert = farkas_certificate(&a, &b);
            match (point, cert) {
                (Some(x), None) => { feas += 1; for i in 0..m { assert!((0..n).map(|j| a[i][j] * x[j]).sum::<f64>() <= b[i] + 1e-6, "{a:?} {b:?} {x:?}"); } }
                (None, Some(y)) => {
                    infeas += 1;
                    assert!(y.iter().all(|&v| v >= -1e-9));
                    for j in 0..n { assert!((0..m).map(|i| y[i] * a[i][j]).sum::<f64>().abs() < 1e-6); }
                    assert!((0..m).map(|i| y[i] * b[i]).sum::<f64>() < -1e-9);
                }
                other => panic!("Farkas' lemma violated for {a:?} {b:?}: {other:?}"),
            }
        }
        assert!(feas > 30 && infeas > 30, "both outcomes should occur: {feas} {infeas}");
    }
    #[test]
    fn strong_duality_and_complementary_slackness_hold() {
        let mut r = Rng::new(497);
        for _ in 0..150 {
            let (m, n) = (r.range(1, 4), r.range(1, 4));
            let a: Vec<Vec<f64>> = (0..m).map(|_| r.vec(n, 1, 6).into_iter().map(|v| v as f64).collect()).collect(); // positive ⇒ bounded
            let b: Vec<f64> = r.vec(m, 5, 30).into_iter().map(|v| v as f64).collect();
            let c: Vec<f64> = r.vec(n, 1, 8).into_iter().map(|v| v as f64).collect();
            let (pv, dv, cs) = check_complementary_slackness(&a, &b, &c).expect("bounded and feasible");
            assert!((pv - dv).abs() < 1e-6, "primal {pv} dual {dv}");
            assert!(cs, "complementary slackness must hold at optimal solutions");
        }
    }
}
