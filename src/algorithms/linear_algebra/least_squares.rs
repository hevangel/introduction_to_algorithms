//! @title Symmetric Positive-Definite Matrices & Least Squares
//! @chapter 28.3–28.4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Cholesky factorisation of SPD matrices and curve fitting by the normal equations AᵀA c = Aᵀy.
//! @order 30
use super::lup_decomposition::{solve, Matrix};
use crate::outln;

pub fn transpose(a: &Matrix) -> Matrix {
    (0..a[0].len()).map(|j| (0..a.len()).map(|i| a[i][j]).collect()).collect()
}
fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    (0..a.len()).map(|i| (0..b[0].len()).map(|j| (0..b.len()).map(|k| a[i][k] * b[k][j]).sum()).collect()).collect()
}

/// Cholesky factorisation A = L·Lᵀ of a symmetric positive-definite matrix. None if A is not SPD.
pub fn cholesky(a: &Matrix) -> Option<Matrix> {
    let n = a.len();
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let s: f64 = (0..j).map(|k| l[i][k] * l[j][k]).sum();
            if i == j {
                let d = a[i][i] - s;
                if d <= 1e-12 { return None; }
                l[i][i] = d.sqrt();
            } else {
                l[i][j] = (a[i][j] - s) / l[j][j];
            }
        }
    }
    Some(l)
}

/// Solve the normal equations: the least-squares solution c of the over-determined system A·c ≈ y.
pub fn least_squares(a: &Matrix, y: &[f64]) -> Option<Vec<f64>> {
    let at = transpose(a);
    let ata = mul(&at, a);
    let aty: Vec<f64> = at.iter().map(|row| row.iter().zip(y).map(|(p, q)| p * q).sum()).collect();
    solve(&ata, &aty)
}

/// Fit a polynomial c₀ + c₁x + … + c_d x^d to the points by least squares.
pub fn polynomial_fit(xs: &[f64], ys: &[f64], degree: usize) -> Option<Vec<f64>> {
    let a: Matrix = xs.iter().map(|&x| (0..=degree).map(|j| x.powi(j as i32)).collect()).collect();
    least_squares(&a, ys)
}

pub fn demo() -> String {
    let mut o = String::new();
    // Section 28.3: the five data points and a quadratic fit
    let (xs, ys) = ([-1.0, 1.0, 2.0, 3.0, 5.0], [2.0, 1.0, 1.0, 0.0, 3.0]);
    let c = polynomial_fit(&xs, &ys, 2).unwrap();
    outln!(o, "points: {:?}", xs.iter().zip(&ys).collect::<Vec<_>>());
    outln!(o, "best quadratic F(x) = {:.3} + {:.3}x + {:.3}x²", c[0], c[1], c[2]);
    let sse: f64 = xs.iter().zip(&ys).map(|(&x, &y)| (c[0] + c[1] * x + c[2] * x * x - y).powi(2)).sum();
    outln!(o, "sum of squared errors = {:.3}", sse);
    let spd = vec![vec![4.0, 12.0, -16.0], vec![12.0, 37.0, -43.0], vec![-16.0, -43.0, 98.0]];
    outln!(o, "Cholesky factor of an SPD matrix: {:?}", cholesky(&spd));
    outln!(o, "indefinite matrix → {:?}", cholesky(&vec![vec![1.0, 2.0], vec![2.0, 1.0]]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn book_quadratic_fit() {
        let c = polynomial_fit(&[-1.0, 1.0, 2.0, 3.0, 5.0], &[2.0, 1.0, 1.0, 0.0, 3.0], 2).unwrap();
        assert!((c[0] - 1.2).abs() < 1e-9 && (c[1] + 0.757142857).abs() < 1e-6 && (c[2] - 0.214285714).abs() < 1e-6, "{c:?}");
    }
    #[test]
    fn exact_polynomials_are_recovered() {
        let xs: Vec<f64> = (-5..=5).map(|x| x as f64).collect();
        let ys: Vec<f64> = xs.iter().map(|&x| 3.0 - 2.0 * x + 0.5 * x * x * x).collect();
        let c = polynomial_fit(&xs, &ys, 3).unwrap();
        for (got, want) in c.iter().zip([3.0, -2.0, 0.0, 0.5]) { assert!((got - want).abs() < 1e-6); }
    }
    #[test]
    fn cholesky_reconstructs_and_rejects_non_spd() {
        let a = vec![vec![4.0, 12.0, -16.0], vec![12.0, 37.0, -43.0], vec![-16.0, -43.0, 98.0]];
        let l = cholesky(&a).unwrap();
        let back = mul(&l, &transpose(&l));
        for i in 0..3 { for j in 0..3 { assert!((back[i][j] - a[i][j]).abs() < 1e-9); } }
        assert_eq!(l[0][0], 2.0);
        assert!(cholesky(&vec![vec![1.0, 2.0], vec![2.0, 1.0]]).is_none());
    }
}
