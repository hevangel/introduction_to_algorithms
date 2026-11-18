//! @title Natural Cubic Spline Interpolation
//! @chapter Problem 28-2 (2e–4e)
//! @editions 2,3,4
//! @summary Fit C² piecewise-cubic polynomials through n+1 knots in O(n) time using the Thomas tridiagonal solver on second derivatives.
//! @order 35
use super::tridiagonal_solver::solve_tridiagonal;
use crate::outln;

/// Natural cubic spline through knots `(x_0, y_0), ..., (x_n, y_n)` with S''(x_0) = S''(x_n) = 0.
#[derive(Debug, Clone)]
pub struct CubicSpline {
    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
    /// Second derivatives S''(x_i) at all n + 1 knots.
    pub d: Vec<f64>,
}

impl CubicSpline {
    /// Fit a natural cubic spline in O(n) time using the Thomas tridiagonal solver.
    pub fn fit(xs: &[f64], ys: &[f64]) -> Self {
        let m = xs.len();
        assert!(m >= 2 && m == ys.len());
        assert!(xs.windows(2).all(|w| w[0] < w[1]), "knots must be strictly increasing");
        let n = m - 1;
        if n == 1 {
            return Self { xs: xs.to_vec(), ys: ys.to_vec(), d: vec![0.0, 0.0] };
        }
        let h: Vec<f64> = (0..n).map(|i| xs[i + 1] - xs[i]).collect();
        let k = n - 1; // number of interior knots 1..=n-1
        let mut sub = vec![0.0; k.saturating_sub(1)];
        let mut diag = vec![0.0; k];
        let mut sup = vec![0.0; k.saturating_sub(1)];
        let mut rhs = vec![0.0; k];

        for row in 0..k {
            let i = row + 1;
            diag[row] = 2.0 * (h[i - 1] + h[i]);
            rhs[row] = 6.0 * ((ys[i + 1] - ys[i]) / h[i] - (ys[i] - ys[i - 1]) / h[i - 1]);
            if row + 1 < k {
                sup[row] = h[i];
                sub[row] = h[i];
            }
        }
        let interior_d = solve_tridiagonal(&sub, &diag, &sup, &rhs).expect("SPD spline system");
        let mut d = Vec::with_capacity(m);
        d.push(0.0);
        d.extend(interior_d);
        d.push(0.0);
        Self { xs: xs.to_vec(), ys: ys.to_vec(), d }
    }

    /// Evaluate `(S(x), S'(x), S''(x))` at any `x` (clamped/extrapolated via nearest end segment outside range).
    pub fn eval(&self, x: f64) -> (f64, f64, f64) {
        let n = self.xs.len() - 1;
        let i = if x <= self.xs[0] {
            0
        } else if x >= self.xs[n] {
            n - 1
        } else {
            let pos = self.xs.partition_point(|&xi| xi <= x);
            (pos - 1).min(n - 1)
        };
        let hi = self.xs[i + 1] - self.xs[i];
        let t = x - self.xs[i];
        let di = self.d[i];
        let di1 = self.d[i + 1];
        let bi = (self.ys[i + 1] - self.ys[i]) / hi - hi * (2.0 * di + di1) / 6.0;
        let ci = di / 2.0;
        let ei = (di1 - di) / (6.0 * hi);

        let val = self.ys[i] + t * (bi + t * (ci + t * ei));
        let deriv1 = bi + t * (2.0 * ci + 3.0 * ei * t);
        let deriv2 = di + (di1 - di) * (t / hi);
        (val, deriv1, deriv2)
    }

    /// Explicitly evaluate left-hand limit inside segment `seg` at its right endpoint `xs[seg + 1]`.
    fn eval_segment(&self, seg: usize, t: f64) -> (f64, f64, f64) {
        let hi = self.xs[seg + 1] - self.xs[seg];
        let di = self.d[seg];
        let di1 = self.d[seg + 1];
        let bi = (self.ys[seg + 1] - self.ys[seg]) / hi - hi * (2.0 * di + di1) / 6.0;
        let ci = di / 2.0;
        let ei = (di1 - di) / (6.0 * hi);
        let val = self.ys[seg] + t * (bi + t * (ci + t * ei));
        let d1 = bi + t * (2.0 * ci + 3.0 * ei * t);
        let d2 = di + (di1 - di) * (t / hi);
        (val, d1, d2)
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let xs = [0.0f64, 1.0, 2.5, 4.0, 5.0];
    let ys: Vec<f64> = xs.iter().map(|&x| (x * 1.1).sin()).collect();
    let sp = CubicSpline::fit(&xs, &ys);
    outln!(o, "natural second derivatives D = {:?}", sp.d.iter().map(|v| (v * 1000.0).round() / 1000.0).collect::<Vec<_>>());
    for probe in [0.5, 1.0, 2.0, 3.25, 4.5] {
        let (v, d1, d2) = sp.eval(probe);
        outln!(o, "  x = {probe:>4.2}: S={v:>7.4}  S'={d1:>7.4}  S''={d2:>7.4}");
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn interpolates_knots_with_c2_continuity_and_natural_boundaries() {
        let mut r = Rng::new(2802);
        for _ in 0..40 {
            let m = r.range(3, 18);
            let mut xs = vec![0.0];
            for _ in 1..m {
                xs.push(xs.last().unwrap() + 0.3 + r.f64() * 2.5);
            }
            let ys: Vec<f64> = (0..m).map(|_| r.f64() * 20.0 - 10.0).collect();
            let sp = CubicSpline::fit(&xs, &ys);

            // Natural boundary conditions
            assert!((sp.eval(xs[0]).2).abs() < 1e-10);
            assert!((sp.eval(xs[m - 1]).2).abs() < 1e-10);

            // Exact interpolation at every knot
            for i in 0..m {
                assert!((sp.eval(xs[i]).0 - ys[i]).abs() < 1e-10);
            }

            // C^0, C^1, C^2 continuity across every interior knot
            for i in 1..(m - 1) {
                let h_prev = xs[i] - xs[i - 1];
                let left = sp.eval_segment(i - 1, h_prev);
                let right = sp.eval_segment(i, 0.0);
                assert!((left.0 - right.0).abs() < 1e-9);
                assert!((left.1 - right.1).abs() < 1e-9);
                assert!((left.2 - right.2).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn reproduces_linear_function_exactly() {
        let xs = [0.0, 0.7, 1.9, 3.3, 5.0];
        let ys: Vec<f64> = xs.iter().map(|&x| 2.5 * x - 1.25).collect();
        let sp = CubicSpline::fit(&xs, &ys);
        assert!(sp.d.iter().all(|v| v.abs() < 1e-11));
        for probe in [0.3, 1.5, 2.8, 4.4] {
            let (v, d1, d2) = sp.eval(probe);
            assert!((v - (2.5 * probe - 1.25)).abs() < 1e-10);
            assert!((d1 - 2.5).abs() < 1e-10);
            assert!(d2.abs() < 1e-10);
        }
    }
}
