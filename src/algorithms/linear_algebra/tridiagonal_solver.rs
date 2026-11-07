//! @title Tridiagonal Systems (Thomas Algorithm)
//! @chapter Chapter 28 exercises (LU decomposition of tridiagonal matrices)
//! @editions 2,3,4
//! @summary LU decomposition of a tridiagonal matrix keeps its band structure, so a tridiagonal system A x = d is solved in O(n) time.
//! @order 26
use crate::outln;

/// Solve a tridiagonal system. `sub[i]` = A[i+1][i] (length n−1), `diag[i]` = A[i][i], `sup[i]` = A[i][i+1] (length n−1).
/// Forward elimination then back substitution — Θ(n). Returns None on a zero pivot (use pivoting for general matrices).
pub fn solve_tridiagonal(sub: &[f64], diag: &[f64], sup: &[f64], rhs: &[f64]) -> Option<Vec<f64>> {
    let n = diag.len();
    assert!(sub.len() + 1 == n && sup.len() + 1 == n && rhs.len() == n);
    let (mut c, mut d) = (vec![0.0; n], vec![0.0; n]);
    if diag[0].abs() < 1e-14 { return None; }
    c[0] = if n > 1 { sup[0] / diag[0] } else { 0.0 };
    d[0] = rhs[0] / diag[0];
    for i in 1..n {
        let denom = diag[i] - sub[i - 1] * c[i - 1]; // pivot after eliminating the sub-diagonal entry
        if denom.abs() < 1e-14 { return None; }
        if i < n - 1 { c[i] = sup[i] / denom; }
        d[i] = (rhs[i] - sub[i - 1] * d[i - 1]) / denom;
    }
    let mut x = vec![0.0; n];
    x[n - 1] = d[n - 1];
    for i in (0..n - 1).rev() { x[i] = d[i] - c[i] * x[i + 1]; }
    Some(x)
}

pub fn demo() -> String {
    let mut o = String::new();
    // the 1-D Poisson/heat-equation matrix tridiag(−1, 2, −1) with right-hand side 1
    let n = 8;
    let x = solve_tridiagonal(&vec![-1.0; n - 1], &vec![2.0; n], &vec![-1.0; n - 1], &vec![1.0; n]).unwrap();
    outln!(o, "tridiag(−1,2,−1) x = 1, n = {n}: {:?}", x.iter().map(|v| (v * 1000.0).round() / 1000.0).collect::<Vec<_>>());
    outln!(o, "exact: x_i = i(n+1−i)/2 → {:?}", (1..=n).map(|i| (i * (n + 1 - i)) as f64 / 2.0).collect::<Vec<_>>());
    let big = 200_000;
    let sol = solve_tridiagonal(&vec![-1.0; big - 1], &vec![2.0; big], &vec![-1.0; big - 1], &vec![1.0; big]).unwrap();
    outln!(o, "n = {big} solved in O(n); middle value {:.1} (exact {:.1})", sol[big / 2 - 1], (big / 2 * (big + 1 - big / 2)) as f64 / 2.0);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::linear_algebra::lup_decomposition::solve;
    use crate::common::Rng;
    #[test]
    fn matches_dense_lup_solver_on_diagonally_dominant_systems() {
        let mut r = Rng::new(418);
        for _ in 0..200 {
            let n = r.range(1, 10);
            let sub: Vec<f64> = (0..n - 1).map(|_| r.f64() * 2.0 - 1.0).collect();
            let sup: Vec<f64> = (0..n - 1).map(|_| r.f64() * 2.0 - 1.0).collect();
            let diag: Vec<f64> = (0..n).map(|_| 3.0 + r.f64()).collect();
            let rhs: Vec<f64> = (0..n).map(|_| r.f64() * 10.0 - 5.0).collect();
            let mut a = vec![vec![0.0; n]; n];
            for i in 0..n { a[i][i] = diag[i]; if i + 1 < n { a[i][i + 1] = sup[i]; a[i + 1][i] = sub[i]; } }
            let x = solve_tridiagonal(&sub, &diag, &sup, &rhs).unwrap();
            let y = solve(&a, &rhs).unwrap();
            for i in 0..n { assert!((x[i] - y[i]).abs() < 1e-8, "{x:?} {y:?}"); }
        }
    }
    #[test]
    fn poisson_matrix_has_the_closed_form_solution() {
        let n = 50;
        let x = solve_tridiagonal(&vec![-1.0; n - 1], &vec![2.0; n], &vec![-1.0; n - 1], &vec![1.0; n]).unwrap();
        for i in 1..=n { assert!((x[i - 1] - (i * (n + 1 - i)) as f64 / 2.0).abs() < 1e-8); }
    }
    #[test]
    fn singular_system_detected() {
        assert!(solve_tridiagonal(&[1.0], &[0.0, 1.0], &[1.0], &[1.0, 1.0]).is_none());
    }
}
