//! @title LU and LUP Decomposition
//! @chapter 28.1–28.2 (1e–4e; Ch 31 in 1e–2e)
//! @editions 1,2,3,4
//! @summary Solve Ax = b by factoring PA = LU with partial pivoting, then forward/back substitution in Θ(n³).
//! @order 10
use crate::outln;

pub type Matrix = Vec<Vec<f64>>;
const EPS: f64 = 1e-12;

/// LU-DECOMPOSITION (no pivoting): A = L·U with L unit lower triangular. None if a zero pivot is met.
pub fn lu_decomposition(a: &Matrix) -> Option<(Matrix, Matrix)> {
    let n = a.len();
    let (mut l, mut u) = (vec![vec![0.0; n]; n], vec![vec![0.0; n]; n]);
    let mut m = a.clone();
    for k in 0..n {
        if m[k][k].abs() < EPS { return None; }
        u[k][k] = m[k][k];
        l[k][k] = 1.0;
        for i in k + 1..n {
            l[i][k] = m[i][k] / m[k][k];
            u[k][i] = m[k][i];
        }
        for i in k + 1..n { for j in k + 1..n { m[i][j] -= l[i][k] * u[k][j]; } } // Schur complement
    }
    Some((l, u))
}

/// LUP-DECOMPOSITION: returns (combined LU matrix in place, permutation π) with P·A = L·U. None if A is singular.
pub fn lup_decomposition(a: &Matrix) -> Option<(Matrix, Vec<usize>)> {
    let n = a.len();
    let mut a = a.clone();
    let mut pi: Vec<usize> = (0..n).collect();
    for k in 0..n {
        let kp = (k..n).max_by(|&i, &j| a[i][k].abs().partial_cmp(&a[j][k].abs()).unwrap())?;
        if a[kp][k].abs() < EPS { return None; } // singular matrix
        pi.swap(k, kp);
        a.swap(k, kp);
        for i in k + 1..n {
            a[i][k] /= a[k][k];
            for j in k + 1..n { a[i][j] -= a[i][k] * a[k][j]; }
        }
    }
    Some((a, pi))
}

/// Split the in-place LU matrix into explicit L (unit diagonal) and U.
pub fn split_lu(lu: &Matrix) -> (Matrix, Matrix) {
    let n = lu.len();
    let (mut l, mut u) = (vec![vec![0.0; n]; n], vec![vec![0.0; n]; n]);
    for i in 0..n { for j in 0..n {
        if i > j { l[i][j] = lu[i][j]; } else { u[i][j] = lu[i][j]; }
    } l[i][i] = 1.0; }
    (l, u)
}

/// LUP-SOLVE(L, U, π, b): forward substitution for y, back substitution for x.
pub fn lup_solve(lu: &Matrix, pi: &[usize], b: &[f64]) -> Vec<f64> {
    let n = lu.len();
    let mut y = vec![0.0; n];
    for i in 0..n { y[i] = b[pi[i]] - (0..i).map(|j| lu[i][j] * y[j]).sum::<f64>(); }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() { x[i] = (y[i] - (i + 1..n).map(|j| lu[i][j] * x[j]).sum::<f64>()) / lu[i][i]; }
    x
}

/// Solve A·x = b; None if A is singular.
pub fn solve(a: &Matrix, b: &[f64]) -> Option<Vec<f64>> {
    let (lu, pi) = lup_decomposition(a)?;
    Some(lup_solve(&lu, &pi, b))
}

/// det(A) = (−1)^swaps · Π U_ii.
pub fn determinant(a: &Matrix) -> f64 {
    match lup_decomposition(a) {
        None => 0.0,
        Some((lu, pi)) => {
            let n = pi.len();
            let mut visited = vec![false; n];
            let mut swaps = 0;
            for i in 0..n { if !visited[i] { let (mut j, mut len) = (i, 0); while !visited[j] { visited[j] = true; j = pi[j]; len += 1; } swaps += len - 1; } }
            let prod: f64 = (0..n).map(|i| lu[i][i]).product();
            if swaps % 2 == 0 { prod } else { -prod }
        }
    }
}

pub fn mat_mul(a: &Matrix, b: &Matrix) -> Matrix {
    let (n, m, p) = (a.len(), b.len(), b[0].len());
    (0..n).map(|i| (0..p).map(|j| (0..m).map(|k| a[i][k] * b[k][j]).sum()).collect()).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    // Section 28.2's example
    let a = vec![vec![1.0, 2.0, 0.0], vec![3.0, 4.0, 4.0], vec![5.0, 6.0, 3.0]];
    let b = [3.0, 7.0, 8.0];
    let (lu, pi) = lup_decomposition(&a).unwrap();
    let (l, u) = split_lu(&lu);
    outln!(o, "π = {:?}", pi);
    outln!(o, "L = {:?}", l);
    outln!(o, "U = {:?}", u);
    let x = lup_solve(&lu, &pi, &b);
    outln!(o, "solution x = {:?} (expected [-1.4, 2.2, 0.6])", x.iter().map(|v| (v * 1e6).round() / 1e6).collect::<Vec<_>>());
    outln!(o, "det(A) = {:.4}", determinant(&a));
    outln!(o, "plain LU (no pivoting) works here? {}", lu_decomposition(&a).is_some());
    outln!(o, "singular matrix → {:?}", lup_decomposition(&vec![vec![1.0, 2.0], vec![2.0, 4.0]]).is_none());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn rand_matrix(n: usize, r: &mut Rng) -> Matrix { (0..n).map(|_| (0..n).map(|_| r.f64() * 10.0 - 5.0).collect()).collect() }
    #[test]
    fn book_example() {
        let a = vec![vec![1.0, 2.0, 0.0], vec![3.0, 4.0, 4.0], vec![5.0, 6.0, 3.0]];
        let x = solve(&a, &[3.0, 7.0, 8.0]).unwrap();
        for (got, want) in x.iter().zip([-1.4, 2.2, 0.6]) { assert!((got - want).abs() < 1e-9); }
    }
    #[test]
    fn pa_equals_lu_and_residual_small() {
        let mut r = Rng::new(180);
        for n in 1..=12 {
            let a = rand_matrix(n, &mut r);
            let b: Vec<f64> = (0..n).map(|_| r.f64() * 10.0).collect();
            let (lu, pi) = lup_decomposition(&a).unwrap();
            let (l, u) = split_lu(&lu);
            let pa: Matrix = pi.iter().map(|&i| a[i].clone()).collect();
            let prod = mat_mul(&l, &u);
            for i in 0..n { for j in 0..n { assert!((pa[i][j] - prod[i][j]).abs() < 1e-8); } }
            let x = lup_solve(&lu, &pi, &b);
            for i in 0..n { assert!((a[i].iter().zip(&x).map(|(p, q)| p * q).sum::<f64>() - b[i]).abs() < 1e-7); }
        }
    }
    #[test]
    fn determinant_singular_and_lu_without_pivoting() {
        assert!((determinant(&vec![vec![0.0, 1.0], vec![1.0, 0.0]]) + 1.0).abs() < 1e-12);
        assert!((determinant(&vec![vec![2.0, 0.0], vec![0.0, 3.0]]) - 6.0).abs() < 1e-12);
        assert_eq!(determinant(&vec![vec![1.0, 2.0], vec![2.0, 4.0]]), 0.0);
        assert!(lu_decomposition(&vec![vec![0.0, 1.0], vec![1.0, 0.0]]).is_none()); // needs pivoting
        let (l, u) = lu_decomposition(&vec![vec![4.0, 3.0], vec![6.0, 3.0]]).unwrap();
        assert_eq!(mat_mul(&l, &u), vec![vec![4.0, 3.0], vec![6.0, 3.0]]);
    }
}
