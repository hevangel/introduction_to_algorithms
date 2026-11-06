//! @title Gradient Descent
//! @chapter 33.3 (4e)
//! @editions 4
//! @summary Minimise a differentiable convex function by repeatedly stepping against the gradient; linear regression example.
//! @order 30
use crate::outln;

/// GRADIENT-DESCENT: x_{t+1} = x_t − η ∇f(x_t). Returns the final point and the trace of values `f(x_t)` if `f` is supplied.
pub fn gradient_descent(grad: impl Fn(&[f64]) -> Vec<f64>, x0: &[f64], eta: f64, iters: usize) -> Vec<f64> {
    let mut x = x0.to_vec();
    for _ in 0..iters {
        let g = grad(&x);
        for (xi, gi) in x.iter_mut().zip(&g) { *xi -= eta * gi; }
    }
    x
}

/// Gradient descent with early stopping when ‖∇f‖ < tol. Returns (x, iterations used).
pub fn gradient_descent_tol(grad: impl Fn(&[f64]) -> Vec<f64>, x0: &[f64], eta: f64, tol: f64, max_iters: usize) -> (Vec<f64>, usize) {
    let mut x = x0.to_vec();
    for t in 0..max_iters {
        let g = grad(&x);
        if g.iter().map(|v| v * v).sum::<f64>().sqrt() < tol { return (x, t); }
        for (xi, gi) in x.iter_mut().zip(&g) { *xi -= eta * gi; }
    }
    (x, max_iters)
}

/// Least-squares linear regression y ≈ Xθ by gradient descent on f(θ) = (1/2m)·‖Xθ − y‖²; ∇f = (1/m)·Xᵀ(Xθ − y).
pub fn linear_regression(x: &[Vec<f64>], y: &[f64], eta: f64, iters: usize) -> Vec<f64> {
    let (m, d) = (x.len(), x[0].len());
    gradient_descent(|th| {
        let resid: Vec<f64> = x.iter().zip(y).map(|(row, &yi)| row.iter().zip(th).map(|(a, b)| a * b).sum::<f64>() - yi).collect();
        (0..d).map(|j| (0..m).map(|i| x[i][j] * resid[i]).sum::<f64>() / m as f64).collect()
    }, &vec![0.0; d], eta, iters)
}

pub fn demo() -> String {
    let mut o = String::new();
    // f(x, y) = (x − 3)² + 2(y + 1)²  → minimum at (3, −1)
    let grad = |p: &[f64]| vec![2.0 * (p[0] - 3.0), 4.0 * (p[1] + 1.0)];
    for eta in [0.01, 0.1, 0.4, 0.55] {
        let x = gradient_descent(grad, &[0.0, 0.0], eta, 100);
        outln!(o, "η = {:<4}: after 100 steps x = ({:>10.4}, {:>10.4})", eta, x[0], x[1]);
    }
    let (x, it) = gradient_descent_tol(grad, &[10.0, 10.0], 0.25, 1e-8, 10_000);
    outln!(o, "with a tolerance: converged to ({:.6}, {:.6}) in {} iterations", x[0], x[1], it);
    let xs: Vec<Vec<f64>> = (0..20).map(|i| vec![1.0, i as f64 / 10.0]).collect();
    let ys: Vec<f64> = xs.iter().map(|r| 2.0 + 3.0 * r[1]).collect();
    outln!(o, "linear regression on y = 2 + 3x: θ = {:?}", linear_regression(&xs, &ys, 0.3, 5000).iter().map(|v| (v * 1e4).round() / 1e4).collect::<Vec<_>>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn converges_on_a_convex_quadratic() {
        let grad = |p: &[f64]| vec![2.0 * (p[0] - 3.0), 4.0 * (p[1] + 1.0)];
        let x = gradient_descent(grad, &[0.0, 0.0], 0.1, 500);
        assert!((x[0] - 3.0).abs() < 1e-6 && (x[1] + 1.0).abs() < 1e-6);
        let (x2, it) = gradient_descent_tol(grad, &[10.0, -10.0], 0.2, 1e-9, 100_000);
        assert!((x2[0] - 3.0).abs() < 1e-6 && it < 1000);
    }
    #[test]
    fn step_size_matters() {
        // for f = x² (L = 2) the iteration contracts iff 0 < η < 1; η > 1 diverges
        let grad = |p: &[f64]| vec![2.0 * p[0]];
        assert!(gradient_descent(grad, &[1.0], 0.9, 200)[0].abs() < 1e-6);
        assert!(gradient_descent(grad, &[1.0], 1.1, 200)[0].abs() > 1e3);
    }
    #[test]
    fn regression_matches_normal_equations() {
        let mut r = Rng::new(263);
        let xs: Vec<Vec<f64>> = (0..50).map(|_| vec![1.0, r.f64() * 4.0 - 2.0, r.f64() * 2.0]).collect();
        let ys: Vec<f64> = xs.iter().map(|x| 1.0 - 2.0 * x[1] + 0.5 * x[2] + (r.f64() - 0.5) * 0.1).collect();
        let gd = linear_regression(&xs, &ys, 0.1, 20_000);
        let exact = crate::algorithms::linear_algebra::least_squares::least_squares(&xs, &ys).unwrap();
        for (a, b) in gd.iter().zip(&exact) { assert!((a - b).abs() < 1e-3, "{gd:?} vs {exact:?}"); }
    }
}
