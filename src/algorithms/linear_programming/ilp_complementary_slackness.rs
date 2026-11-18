//! @title LP Feasibility, Complementary Slackness & Integer LP
//! @chapter Problems 29-1..29-3 (2e–4e)
//! @editions 2,3,4
//! @summary Solve Ax ≤ b feasibility via Simplex, verify primal/dual optimality certificates via complementary slackness, and solve small integer LPs with branch-and-bound.
//! @order 30
use super::simplex::{dual, simplex, LpResult};
use crate::outln;

/// Problem 29-1: Decide feasibility of `{ x >= 0 : A x <= b }` by maximizing c = 0 via Simplex Phase I/II.
pub fn check_lp_feasibility(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    let n = a[0].len();
    match simplex(a, b, &vec![0.0; n]) {
        LpResult::Optimal { x, .. } => Some(x),
        LpResult::Unbounded => Some(vec![0.0; n]),
        LpResult::Infeasible => None,
    }
}

/// Problem 29-2: Verify strong-duality complementary slackness certificate for primal (x) and dual (y):
/// 1. Primal feasible: x >= 0, Ax <= b
/// 2. Dual feasible:   y >= 0, A^T y >= c
/// 3. Complementary slackness: x_j * (sum_i a_ij y_i - c_j) == 0 and y_i * (b_i - sum_j a_ij x_j) == 0.
pub fn verify_complementary_slackness(
    a: &[Vec<f64>],
    b: &[f64],
    c: &[f64],
    x: &[f64],
    y: &[f64],
    tol: f64,
) -> bool {
    let m = b.len();
    let n = c.len();
    if x.iter().any(|&v| v < -tol) || y.iter().any(|&v| v < -tol) {
        return false;
    }
    for i in 0..m {
        let ax_i: f64 = (0..n).map(|j| a[i][j] * x[j]).sum();
        if ax_i > b[i] + tol {
            return false;
        }
        let slack = b[i] - ax_i;
        if (y[i] * slack).abs() > tol {
            return false;
        }
    }
    for j in 0..n {
        let aty_j: f64 = (0..m).map(|i| a[i][j] * y[i]).sum();
        if aty_j < c[j] - tol {
            return false;
        }
        let d_slack = aty_j - c[j];
        if (x[j] * d_slack).abs() > tol {
            return false;
        }
    }
    true
}

/// Extract dual multipliers `y` by solving the dual LP with `simplex::dual`.
pub fn solve_primal_dual_pair(a: &[Vec<f64>], b: &[f64], c: &[f64]) -> Option<(f64, Vec<f64>, Vec<f64>)> {
    let LpResult::Optimal { value, x } = simplex(a, b, c) else {
        return None;
    };
    let (da, db, dc) = dual(a, b, c);
    let LpResult::Optimal { x: y, .. } = simplex(&da, &db, &dc) else {
        return None;
    };
    Some((value, x, y))
}

/// Problem 29-3: Solve Integer Linear Program `max c^T x s.t. A x <= b, x >= 0, x in Z^n`
/// using LP-relaxation branch-and-bound backed by `simplex`.
pub fn solve_ilp_branch_and_bound(a: &[Vec<f64>], b: &[f64], c: &[f64]) -> Option<(f64, Vec<i64>)> {
    fn rec(
        a: &[Vec<f64>],
        b: &[f64],
        c: &[f64],
        best_val: &mut f64,
        best_x: &mut Option<Vec<i64>>,
        depth: usize,
    ) {
        if depth > 40 {
            return;
        }
        let LpResult::Optimal { value, x } = simplex(a, b, c) else {
            return;
        };
        if value <= *best_val + 1e-7 {
            return;
        }
        let frac = x.iter().enumerate().find(|(_, &v)| (v - v.round()).abs() > 1e-6);
        let Some((j, &val)) = frac else {
            *best_val = value;
            *best_x = Some(x.iter().map(|v| v.round() as i64).collect());
            return;
        };

        let n = c.len();
        let fl = val.floor();
        let cl = val.ceil();

        // Branch 1: x_j <= floor(val)
        let mut a1 = a.to_vec();
        let mut b1 = b.to_vec();
        let mut row1 = vec![0.0; n];
        row1[j] = 1.0;
        a1.push(row1);
        b1.push(fl);
        rec(&a1, &b1, c, best_val, best_x, depth + 1);

        // Branch 2: -x_j <= -ceil(val)  (i.e. x_j >= ceil(val))
        let mut a2 = a.to_vec();
        let mut b2 = b.to_vec();
        let mut row2 = vec![0.0; n];
        row2[j] = -1.0;
        a2.push(row2);
        b2.push(-cl);
        rec(&a2, &b2, c, best_val, best_x, depth + 1);
    }

    let mut best_val = f64::NEG_INFINITY;
    let mut best_x = None;
    rec(a, b, c, &mut best_val, &mut best_x, 0);
    best_x.map(|x| (best_val, x))
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = vec![vec![1.0, 1.0, 3.0], vec![2.0, 2.0, 5.0], vec![4.0, 1.0, 2.0]];
    let (b, c) = ([30.0, 24.0, 36.0], [3.0, 1.0, 2.0]);
    let (val, x, y) = solve_primal_dual_pair(&a, &b, &c).unwrap();
    let ok = verify_complementary_slackness(&a, &b, &c, &x, &y, 1e-6);
    outln!(o, "primal/dual opt = {val:.2}, complementary slackness holds? {ok}");

    // ILP integrality gap example: max x1 + x2 s.t. 2x1 + 2x2 <= 3, x1,x2 >= 0 -> LP=1.5, ILP=1
    let ia = vec![vec![2.0, 2.0]];
    let (ib, ic) = ([3.0], [1.0, 1.0]);
    let LpResult::Optimal { value: lp_v, .. } = simplex(&ia, &ib, &ic) else { unreachable!() };
    let (ilp_v, ilp_x) = solve_ilp_branch_and_bound(&ia, &ib, &ic).unwrap();
    outln!(o, "integrality gap: LP relaxation = {lp_v:.2}, ILP opt = {ilp_v:.2} at {:?}", ilp_x);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn complementary_slackness_holds_for_random_bounded_lps() {
        let mut r = Rng::new(2902);
        let mut checked = 0;
        for _ in 0..60 {
            let m = r.range(2, 4);
            let mut a: Vec<Vec<f64>> = (0..m).map(|_| (0..2).map(|_| r.range(0, 5) as f64).collect()).collect();
            let mut b: Vec<f64> = (0..m).map(|_| r.range(1, 15) as f64).collect();
            a.push(vec![1.0, 0.0]);
            b.push(10.0);
            a.push(vec![0.0, 1.0]);
            b.push(10.0);
            let c: Vec<f64> = (0..2).map(|_| r.range(1, 8) as f64).collect();
            if let Some((val, x, y)) = solve_primal_dual_pair(&a, &b, &c) {
                assert!(verify_complementary_slackness(&a, &b, &c, &x, &y, 1e-5));
                let dual_val: f64 = b.iter().zip(&y).map(|(bi, yi)| bi * yi).sum();
                assert!((val - dual_val).abs() < 1e-5);
                // Disrupting slackness fails verification.
                let mut bad_y = y.clone();
                bad_y[0] += 100.0;
                assert!(!verify_complementary_slackness(&a, &b, &c, &x, &bad_y, 1e-5));
                checked += 1;
            }
        }
        assert!(checked >= 20);
    }

    #[test]
    fn feasibility_and_ilp_match_grid_brute_force() {
        assert!(check_lp_feasibility(&[vec![1.0, 1.0], vec![-1.0, -1.0]], &[5.0, -2.0]).is_some());
        assert!(check_lp_feasibility(&[vec![1.0], vec![-1.0]], &[1.0, -2.0]).is_none());

        let a = vec![vec![2.0, 3.0], vec![4.0, 1.0]];
        let b = vec![7.0, 6.0];
        let c = vec![5.0, 4.0];
        let (ilp_v, ilp_x) = solve_ilp_branch_and_bound(&a, &b, &c).unwrap();
        let mut brute_v = i64::MIN;
        for x0 in 0..=6 {
            for x1 in 0..=6 {
                if 2 * x0 + 3 * x1 <= 7 && 4 * x0 + x1 <= 6 {
                    brute_v = brute_v.max(5 * x0 + 4 * x1);
                }
            }
        }
        assert!((ilp_v - brute_v as f64).abs() < 1e-7);
        assert_eq!(5 * ilp_x[0] + 4 * ilp_x[1], brute_v);
    }
}
