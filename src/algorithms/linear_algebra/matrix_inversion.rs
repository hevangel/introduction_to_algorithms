//! @title Matrix Inversion
//! @chapter 28.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Invert a matrix by solving n systems with one LUP factorisation; equivalent in difficulty to matrix multiplication.
//! @order 20
use super::lup_decomposition::{lup_decomposition, lup_solve, mat_mul, Matrix};
use crate::outln;

/// Compute A⁻¹ column by column: solve A·xⱼ = eⱼ for each unit vector (one LUP factorisation, n solves).
pub fn invert(a: &Matrix) -> Option<Matrix> {
    let n = a.len();
    let (lu, pi) = lup_decomposition(a)?;
    let mut inv = vec![vec![0.0; n]; n];
    for j in 0..n {
        let mut e = vec![0.0; n];
        e[j] = 1.0;
        let col = lup_solve(&lu, &pi, &e);
        for i in 0..n { inv[i][j] = col[i]; }
    }
    Some(inv)
}

pub fn identity(n: usize) -> Matrix { (0..n).map(|i| (0..n).map(|j| (i == j) as u8 as f64).collect()).collect() }

/// Max-norm of A·B − I (used to check an inverse).
pub fn identity_error(a: &Matrix, b: &Matrix) -> f64 {
    let p = mat_mul(a, b);
    let n = a.len();
    (0..n).flat_map(|i| (0..n).map(move |j| (i, j))).map(|(i, j)| (p[i][j] - (i == j) as u8 as f64).abs()).fold(0.0, f64::max)
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = vec![vec![4.0, 7.0], vec![2.0, 6.0]];
    let inv = invert(&a).unwrap();
    outln!(o, "A      = {:?}", a);
    outln!(o, "A⁻¹    = {:?}", inv.iter().map(|r| r.iter().map(|v| (v * 1e6).round() / 1e6).collect::<Vec<_>>()).collect::<Vec<_>>());
    outln!(o, "‖A·A⁻¹ − I‖ = {:.2e}", identity_error(&a, &inv));
    let mut r = crate::common::Rng::new(3);
    let big: Matrix = (0..30).map(|_| (0..30).map(|_| r.f64() - 0.5).collect()).collect();
    outln!(o, "random 30×30: error {:.2e}", identity_error(&big, &invert(&big).unwrap()));
    outln!(o, "singular matrix has no inverse: {:?}", invert(&vec![vec![1.0, 2.0], vec![2.0, 4.0]]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn two_by_two() {
        let inv = invert(&vec![vec![4.0, 7.0], vec![2.0, 6.0]]).unwrap();
        let want = [[0.6, -0.7], [-0.2, 0.4]];
        for i in 0..2 { for j in 0..2 { assert!((inv[i][j] - want[i][j]).abs() < 1e-12); } }
    }
    #[test]
    fn random_inverses() {
        let mut r = Rng::new(181);
        for n in 1..=15 {
            let a: Matrix = (0..n).map(|_| (0..n).map(|_| r.f64() * 4.0 - 2.0).collect()).collect();
            let inv = invert(&a).unwrap();
            assert!(identity_error(&a, &inv) < 1e-6, "n={n}");
            assert!(identity_error(&inv, &a) < 1e-6);
        }
        assert!(invert(&identity(5)).unwrap() == identity(5));
    }
}
