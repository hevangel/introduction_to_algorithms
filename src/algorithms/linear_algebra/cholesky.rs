//! @title Cholesky Decomposition (Symmetric Positive-Definite Matrices)
//! @chapter 28.3–28.5 (2e–4e)
//! @editions 2,3,4
//! @summary A = L·Lᵀ with L lower triangular for symmetric positive-definite A: half the work of LU, no pivoting, and a built-in definiteness test.
//! @order 25
use crate::outln;

pub type Matrix = Vec<Vec<f64>>;

/// Cholesky factor L (A = L·Lᵀ). Returns None if A is not (numerically) symmetric positive-definite. Θ(n³/3) flops.
pub fn cholesky(a: &Matrix) -> Option<Matrix> {
    let n = a.len();
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let s: f64 = (0..j).map(|k| l[i][k] * l[j][k]).sum();
            if i == j {
                let d = a[i][i] - s;
                if d <= 1e-12 { return None; } // a non-positive pivot means A is not positive definite
                l[i][i] = d.sqrt();
            } else {
                l[i][j] = (a[i][j] - s) / l[j][j];
            }
        }
    }
    Some(l)
}

/// Solve A x = b with A = L·Lᵀ by forward and back substitution.
pub fn cholesky_solve(l: &Matrix, b: &[f64]) -> Vec<f64> {
    let n = l.len();
    let mut y = vec![0.0; n];
    for i in 0..n { y[i] = (b[i] - (0..i).map(|k| l[i][k] * y[k]).sum::<f64>()) / l[i][i]; }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() { x[i] = (y[i] - (i + 1..n).map(|k| l[k][i] * x[k]).sum::<f64>()) / l[i][i]; }
    x
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = vec![vec![4.0, 12.0, -16.0], vec![12.0, 37.0, -43.0], vec![-16.0, -43.0, 98.0]];
    let l = cholesky(&a).unwrap();
    outln!(o, "A = {:?}", a);
    outln!(o, "L = {:?}", l);
    let x = cholesky_solve(&l, &[1.0, 2.0, 3.0]);
    outln!(o, "solution of A x = (1,2,3): {:?}", x.iter().map(|v| (v * 1e6).round() / 1e6).collect::<Vec<_>>());
    outln!(o, "indefinite matrix [[1,2],[2,1]]: {:?}", cholesky(&vec![vec![1.0, 2.0], vec![2.0, 1.0]]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn mul(a: &Matrix, b: &Matrix) -> Matrix { let n = a.len(); (0..n).map(|i| (0..n).map(|j| (0..n).map(|k| a[i][k] * b[k][j]).sum()).collect()).collect() }
    fn transpose(a: &Matrix) -> Matrix { let n = a.len(); (0..n).map(|i| (0..n).map(|j| a[j][i]).collect()).collect() }
    fn random_spd(r: &mut Rng, n: usize) -> Matrix {
        let b: Matrix = (0..n).map(|_| (0..n).map(|_| r.f64() * 2.0 - 1.0).collect()).collect();
        let mut a = mul(&transpose(&b), &b); // BᵀB is positive semi-definite; adding I makes it definite
        for i in 0..n { a[i][i] += 1.0; }
        a
    }
    #[test]
    fn reconstructs_and_solves_random_spd_systems() {
        let mut r = Rng::new(417);
        for _ in 0..100 {
            let n = r.range(1, 8);
            let a = random_spd(&mut r, n);
            let l = cholesky(&a).expect("SPD");
            let back = mul(&l, &transpose(&l));
            for i in 0..n { for j in 0..n { assert!((back[i][j] - a[i][j]).abs() < 1e-8); assert!(j <= i || l[i][j] == 0.0); } }
            let b: Vec<f64> = (0..n).map(|_| r.f64() * 10.0).collect();
            let x = cholesky_solve(&l, &b);
            for i in 0..n { assert!((a[i].iter().zip(&x).map(|(p, q)| p * q).sum::<f64>() - b[i]).abs() < 1e-7); }
        }
    }
    #[test]
    fn rejects_non_positive_definite_matrices() {
        assert!(cholesky(&vec![vec![1.0, 2.0], vec![2.0, 1.0]]).is_none());
        assert!(cholesky(&vec![vec![0.0]]).is_none());
        assert!(cholesky(&vec![vec![-1.0, 0.0], vec![0.0, 1.0]]).is_none());
    }
    #[test]
    fn classic_example() {
        let l = cholesky(&vec![vec![4.0, 12.0, -16.0], vec![12.0, 37.0, -43.0], vec![-16.0, -43.0, 98.0]]).unwrap();
        assert!((l[0][0] - 2.0).abs() < 1e-12 && (l[1][0] - 6.0).abs() < 1e-12 && (l[1][1] - 1.0).abs() < 1e-12 && (l[2][0] + 8.0).abs() < 1e-12 && (l[2][1] - 5.0).abs() < 1e-12 && (l[2][2] - 3.0).abs() < 1e-12);
    }
}
