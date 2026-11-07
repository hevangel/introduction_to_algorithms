//! @title Cubic Spline Interpolation
//! @chapter Problem 28-2 (3e)
//! @editions 3,4
//! @summary A piecewise cubic with continuous first and second derivatives through given points: the spline's second derivatives solve a tridiagonal system, so interpolation takes O(n).
//! @order 28
use super::tridiagonal_solver::solve_tridiagonal;
use crate::outln;

/// Boundary conditions for the two free parameters of a cubic spline.
#[derive(Debug, Clone, Copy)]
pub enum Boundary { /// Second derivative zero at both ends.
    Natural, /// Prescribed first derivatives (y′ at the first and last knot).
    Clamped(f64, f64) }

#[derive(Debug, Clone)]
pub struct Spline { x: Vec<f64>, y: Vec<f64>, m: Vec<f64> } // m[i] = S″(x_i)

impl Spline {
    /// Fit the spline through (x[i], y[i]) with strictly increasing x. The second derivatives M_i satisfy
    /// h_{i−1}M_{i−1} + 2(h_{i−1}+h_i)M_i + h_i M_{i+1} = 6((y_{i+1}−y_i)/h_i − (y_i−y_{i−1})/h_{i−1}).
    pub fn fit(x: &[f64], y: &[f64], bc: Boundary) -> Spline {
        let n = x.len();
        assert!(n >= 2 && y.len() == n);
        let h: Vec<f64> = (0..n - 1).map(|i| x[i + 1] - x[i]).collect();
        let slope: Vec<f64> = (0..n - 1).map(|i| (y[i + 1] - y[i]) / h[i]).collect();
        let (mut sub, mut diag, mut sup, mut rhs) = (vec![0.0; n - 1], vec![0.0; n], vec![0.0; n - 1], vec![0.0; n]);
        for i in 1..n - 1 {
            sub[i - 1] = h[i - 1]; diag[i] = 2.0 * (h[i - 1] + h[i]); sup[i] = h[i];
            rhs[i] = 6.0 * (slope[i] - slope[i - 1]);
        }
        match bc {
            Boundary::Natural => { diag[0] = 1.0; diag[n - 1] = 1.0; }                  // M₀ = M_{n−1} = 0
            Boundary::Clamped(d0, dn) => {
                diag[0] = 2.0 * h[0]; sup[0] = h[0]; rhs[0] = 6.0 * (slope[0] - d0);
                diag[n - 1] = 2.0 * h[n - 2]; sub[n - 2] = h[n - 2]; rhs[n - 1] = 6.0 * (dn - slope[n - 2]);
            }
        }
        let m = solve_tridiagonal(&sub, &diag, &sup, &rhs).expect("a spline system is diagonally dominant");
        Spline { x: x.to_vec(), y: y.to_vec(), m }
    }
    /// Evaluate S(t) (extrapolates with the end cubics outside the data range).
    pub fn eval(&self, t: f64) -> f64 {
        let i = self.x.partition_point(|&xi| xi <= t).clamp(1, self.x.len() - 1) - 1;
        let h = self.x[i + 1] - self.x[i];
        let (a, b) = (self.x[i + 1] - t, t - self.x[i]);
        self.m[i] * a.powi(3) / (6.0 * h) + self.m[i + 1] * b.powi(3) / (6.0 * h) + (self.y[i] / h - self.m[i] * h / 6.0) * a + (self.y[i + 1] / h - self.m[i + 1] * h / 6.0) * b
    }
    /// Second derivative S″(t).
    pub fn second_derivative(&self, t: f64) -> f64 {
        let i = self.x.partition_point(|&xi| xi <= t).clamp(1, self.x.len() - 1) - 1;
        let h = self.x[i + 1] - self.x[i];
        (self.m[i] * (self.x[i + 1] - t) + self.m[i + 1] * (t - self.x[i])) / h
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let ys: Vec<f64> = xs.iter().map(|&x: &f64| x.sin()).collect();
    let s = Spline::fit(&xs, &ys, Boundary::Natural);
    outln!(o, "natural spline through sin(x) at 0..4; compare at the midpoints:");
    for t in [0.5, 1.5, 2.5, 3.5] { outln!(o, "  S({t}) = {:.5}, sin({t}) = {:.5}", s.eval(t), t.sin()); }
    let c = Spline::fit(&xs, &ys, Boundary::Clamped(0f64.cos(), 4f64.cos()));
    outln!(o, "clamped spline: S(2.5) = {:.5} (error {:.1e})", c.eval(2.5), (c.eval(2.5) - 2.5f64.sin()).abs());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn knots(r: &mut Rng, n: usize) -> Vec<f64> { let mut x = vec![r.f64()]; for _ in 1..n { let last = *x.last().unwrap(); x.push(last + 0.2 + r.f64()); } x }
    #[test]
    fn interpolates_and_is_c2_at_the_knots() {
        let mut r = Rng::new(463);
        for _ in 0..100 {
            let n = r.range(3, 10);
            let x = knots(&mut r, n);
            let y: Vec<f64> = (0..n).map(|_| r.f64() * 10.0 - 5.0).collect();
            for bc in [Boundary::Natural, Boundary::Clamped(1.0, -2.0)] {
                let s = Spline::fit(&x, &y, bc);
                for i in 0..n { assert!((s.eval(x[i]) - y[i]).abs() < 1e-9); }
                for i in 1..n - 1 { // value, slope and curvature are continuous across interior knots
                    let e = 1e-6;
                    assert!((s.eval(x[i] - e) - s.eval(x[i] + e)).abs() < 1e-4);
                    assert!(((s.eval(x[i] + e) - s.eval(x[i])) / e - (s.eval(x[i]) - s.eval(x[i] - e)) / e).abs() < 1e-3);
                    let e2 = 1e-9;
                    assert!((s.second_derivative(x[i] - e2) - s.second_derivative(x[i] + e2)).abs() < 1e-4);
                }
            }
            let s = Spline::fit(&x, &y, Boundary::Natural);
            assert!(s.second_derivative(x[0]).abs() < 1e-9 && s.second_derivative(x[n - 1]).abs() < 1e-9);
        }
    }
    #[test]
    fn clamped_spline_reproduces_any_cubic_exactly() {
        let f = |t: f64| 2.0 - 3.0 * t + 0.5 * t * t + 0.25 * t.powi(3);
        let df = |t: f64| -3.0 + t + 0.75 * t * t;
        let x = [0.0, 0.7, 1.5, 2.2, 3.9, 5.0];
        let y: Vec<f64> = x.iter().map(|&t| f(t)).collect();
        let s = Spline::fit(&x, &y, Boundary::Clamped(df(0.0), df(5.0)));
        for k in 0..50 { let t = k as f64 * 0.1; assert!((s.eval(t) - f(t)).abs() < 1e-9, "t = {t}"); }
    }
}
