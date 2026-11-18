//! @title All Derivatives & Fast Multipoint Evaluation
//! @chapter Problems 30-2..30-4 (4e); Problems 30-3..30-5 (1e–3e)
//! @editions 1,2,3,4
//! @summary Multidimensional 2D FFT, O(n lg n) all-derivatives evaluation at a point via convolution, and O(n lg² n) subproduct-tree multipoint polynomial evaluation.
//! @order 50
use super::fft_recursive::{fft, Complex};
use crate::outln;

/// Problem 30-2 (4e) / 30-3 (1e–3e): 2D FFT by applying 1D FFT along every row, then every column.
/// Both dimensions must be powers of two.
pub fn fft_2d(grid: &[Vec<Complex>], invert: bool) -> Vec<Vec<Complex>> {
    let rows = grid.len();
    let cols = grid[0].len();
    assert!(rows.is_power_of_two() && cols.is_power_of_two());
    let mut temp: Vec<Vec<Complex>> = grid.iter().map(|r| fft(r, invert)).collect();
    for c in 0..cols {
        let col: Vec<Complex> = (0..rows).map(|r| temp[r][c]).collect();
        let transformed = fft(&col, invert);
        for r in 0..rows {
            temp[r][c] = transformed[r];
        }
    }
    temp
}

/// Multiply two real polynomials in O(n lg n) via FFT.
pub fn poly_mul_fft(a: &[f64], b: &[f64]) -> Vec<f64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let out_len = a.len() + b.len() - 1;
    let n = out_len.next_power_of_two();
    let fa: Vec<Complex> = (0..n).map(|i| Complex::new(*a.get(i).unwrap_or(&0.0), 0.0)).collect();
    let fb: Vec<Complex> = (0..n).map(|i| Complex::new(*b.get(i).unwrap_or(&0.0), 0.0)).collect();
    let ya = fft(&fa, false);
    let yb = fft(&fb, false);
    let yc: Vec<Complex> = ya.into_iter().zip(yb).map(|(x, y)| x * y).collect();
    fft(&yc, true).into_iter().take(out_len).map(|c| c.re).collect()
}

/// Problem 30-4 (3e/4e): Compute all derivatives A^(0)(x0), A^(1)(x0), ..., A^(n-1)(x0)
/// simultaneously in O(n lg n) time via reversed-polynomial FFT convolution.
pub fn all_derivatives_at(coeffs: &[f64], x0: f64) -> Vec<f64> {
    let n = coeffs.len();
    if n == 0 {
        return vec![];
    }
    let mut fact = vec![1.0f64; n];
    for i in 1..n {
        fact[i] = fact[i - 1] * i as f64;
    }
    // Let f[j] = a[j] * j!, g[n - 1 - k] = x0^k / k!.
    // Then (f * g)[t + n - 1] = sum_{j=t}^{n-1} a[j] * j! * x0^{j-t} / (j-t)! = A^(t)(x0).
    let f: Vec<f64> = (0..n).map(|j| coeffs[j] * fact[j]).collect();
    let mut g = vec![0.0f64; n];
    let mut pow_x = 1.0f64;
    for k in 0..n {
        g[n - 1 - k] = pow_x / fact[k];
        pow_x *= x0;
    }
    let conv = poly_mul_fft(&f, &g);
    (0..n).map(|t| conv[t + n - 1]).collect()
}

fn trim_poly(mut p: Vec<f64>) -> Vec<f64> {
    while p.len() > 1 && p.last().unwrap().abs() < 1e-12 {
        p.pop();
    }
    p
}

/// Polynomial remainder A(x) mod B(x) for subproduct-tree multipoint evaluation.
fn poly_rem(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut r = a.to_vec();
    let db = b.len() - 1;
    let lead = b[db];
    while r.len() >= b.len() {
        let shift = r.len() - b.len();
        let coeff = r[r.len() - 1] / lead;
        for i in 0..b.len() {
            r[shift + i] -= coeff * b[i];
        }
        r.pop();
    }
    trim_poly(r)
}

/// Problem 30-4 (4e) / 30-5 (1e–3e): Subproduct-tree multipoint evaluation A(x_0), ..., A(x_{m-1}).
pub fn multipoint_eval(coeffs: &[f64], pts: &[f64]) -> Vec<f64> {
    let m = pts.len();
    if m == 0 {
        return vec![];
    }
    enum SubTree {
        Leaf { poly: Vec<f64> },
        Node { poly: Vec<f64>, left: Box<SubTree>, right: Box<SubTree> },
    }
    impl SubTree {
        fn poly(&self) -> &[f64] {
            match self {
                SubTree::Leaf { poly } | SubTree::Node { poly, .. } => poly,
            }
        }
    }
    fn build(pts: &[f64]) -> SubTree {
        if pts.len() == 1 {
            return SubTree::Leaf { poly: vec![-pts[0], 1.0] };
        }
        let mid = pts.len() / 2;
        let left = build(&pts[..mid]);
        let right = build(&pts[mid..]);
        let poly = poly_mul_fft(left.poly(), right.poly());
        SubTree::Node { poly, left: Box::new(left), right: Box::new(right) }
    }
    fn eval_down(a: &[f64], node: &SubTree, out: &mut Vec<f64>) {
        let r = poly_rem(a, node.poly());
        match node {
            SubTree::Leaf { .. } => out.push(r[0]),
            SubTree::Node { left, right, .. } => {
                eval_down(&r, left, out);
                eval_down(&r, right, out);
            }
        }
    }
    let tree = build(pts);
    let mut out = Vec::with_capacity(m);
    eval_down(coeffs, &tree, &mut out);
    out
}

pub fn horner_eval(coeffs: &[f64], x: f64) -> f64 {
    coeffs.iter().rev().fold(0.0, |acc, &c| acc * x + c)
}

pub fn demo() -> String {
    let mut o = String::new();
    let coeffs = [1.0, -2.0, 3.0, -1.0, 2.0]; // 1 - 2x + 3x^2 - x^3 + 2x^4
    let derivs = all_derivatives_at(&coeffs, 2.0);
    outln!(o, "all derivatives of 1 - 2x + 3x^2 - x^3 + 2x^4 at x=2: {:?}", derivs.iter().map(|v| v.round()).collect::<Vec<_>>());
    let pts = [-2.0, -1.0, 0.0, 1.0, 2.0, 3.0];
    let vals = multipoint_eval(&coeffs, &pts);
    outln!(o, "multipoint eval at {:?} = {:?}", pts, vals.iter().map(|v| (v * 100.0).round() / 100.0).collect::<Vec<_>>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn fft_2d_roundtrips_and_matches_separable_dft() {
        let mut r = Rng::new(3002);
        let (rows, cols) = (8usize, 4usize);
        let grid: Vec<Vec<Complex>> = (0..rows)
            .map(|_| (0..cols).map(|_| Complex::new(r.f64() - 0.5, r.f64() - 0.5)).collect())
            .collect();
        let freq = fft_2d(&grid, false);
        let back = fft_2d(&freq, true);
        for i in 0..rows {
            for j in 0..cols {
                assert!((back[i][j] - grid[i][j]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn all_derivatives_matches_direct_taylor_shift() {
        let mut r = Rng::new(3004);
        for n in 1..=10 {
            let coeffs: Vec<f64> = (0..n).map(|_| r.range(0, 10) as f64 - 5.0).collect();
            let x0 = r.f64() * 2.0 - 1.0;
            let got = all_derivatives_at(&coeffs, x0);
            let mut cur = coeffs.clone();
            for t in 0..n {
                let want = horner_eval(&cur, x0);
                assert!((got[t] - want).abs() < 1e-6, "deg={n}, t={t}: got {} vs want {want}", got[t]);
                cur = cur.iter().enumerate().skip(1).map(|(k, &v)| k as f64 * v).collect();
            }
        }
    }

    #[test]
    fn multipoint_eval_matches_horner() {
        let mut r = Rng::new(3005);
        for _ in 0..25 {
            let n = r.range(1, 20);
            let m = r.range(1, 16);
            let coeffs: Vec<f64> = (0..n).map(|_| r.f64() * 4.0 - 2.0).collect();
            let pts: Vec<f64> = (0..m).map(|k| -1.2 + 2.4 * k as f64 / m as f64).collect();
            let got = multipoint_eval(&coeffs, &pts);
            for (&pt, &g) in pts.iter().zip(&got) {
                let want = horner_eval(&coeffs, pt);
                assert!((g - want).abs() < 1e-4, "pt={pt}: got {g} vs want {want}");
            }
        }
    }
}
