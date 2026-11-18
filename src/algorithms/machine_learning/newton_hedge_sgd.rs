//! @title Newton's Method, Hedge & Stochastic Gradient Descent
//! @chapter Problems 33-1, 33-2 & 33-4 (4e)
//! @editions 4
//! @summary Second-order Newton root/minimization convergence, continuous Hedge multiplicative weights over bounded losses, and stochastic mini-batch gradient descent.
//! @order 40
use crate::common::Rng;
use crate::outln;

/// Problem 33-4 / Root & Second-Order Optimization: 1D Newton's method for minimizing f (`x <- x - f'(x)/f''(x)`).
/// Returns `(x_opt, iterations)`.
pub fn newton_minimize_1d(
    d1: impl Fn(f64) -> f64,
    d2: impl Fn(f64) -> f64,
    mut x: f64,
    tol: f64,
    max_iters: usize,
) -> (f64, usize) {
    for it in 0..max_iters {
        let g = d1(x);
        if g.abs() < tol {
            return (x, it);
        }
        let h = d2(x);
        assert!(h.abs() > 1e-15, "zero second derivative in Newton step");
        x -= g / h;
    }
    (x, max_iters)
}

/// Multi-dimensional Newton's method: solves H(x) Δx = -∇f(x) and sets x <- x + Δx.
pub fn newton_minimize(
    grad: impl Fn(&[f64]) -> Vec<f64>,
    hessian: impl Fn(&[f64]) -> Vec<Vec<f64>>,
    x0: &[f64],
    tol: f64,
    max_iters: usize,
) -> (Vec<f64>, usize) {
    let mut x = x0.to_vec();
    for it in 0..max_iters {
        let g = grad(&x);
        if g.iter().map(|v| v * v).sum::<f64>().sqrt() < tol {
            return (x, it);
        }
        let h = hessian(&x);
        let neg_g: Vec<f64> = g.iter().map(|&v| -v).collect();
        let step = crate::algorithms::linear_algebra::lup_decomposition::solve(&h, &neg_g)
            .expect("singular Hessian in Newton step");
        for (xi, si) in x.iter_mut().zip(&step) {
            *xi += si;
        }
    }
    (x, max_iters)
}

/// Problem 33-2: Continuous Hedge multiplicative weights algorithm over bounded losses `losses[t][i] in [0, 1]`.
/// Updates `w[i] <- w[i] * (1 - eps)^{losses[t][i]}` and returns `(learner_expected_loss, best_expert_loss, final_probs)`.
pub fn hedge(losses: &[Vec<f64>], eps: f64) -> (f64, f64, Vec<f64>) {
    assert!(!losses.is_empty() && eps > 0.0 && eps < 1.0);
    let n = losses[0].len();
    let mut w = vec![1.0f64; n];
    let mut learner_loss = 0.0f64;
    let mut expert_losses = vec![0.0f64; n];

    for row in losses {
        let sum_w: f64 = w.iter().sum();
        for (i, &l) in row.iter().enumerate() {
            let p = w[i] / sum_w;
            learner_loss += p * l;
            expert_losses[i] += l;
            w[i] *= (1.0 - eps).powf(l);
        }
    }
    let sum_w: f64 = w.iter().sum();
    let probs: Vec<f64> = w.into_iter().map(|wi| wi / sum_w).collect();
    let best_expert = expert_losses.into_iter().fold(f64::INFINITY, f64::min);
    (learner_loss, best_expert, probs)
}

pub fn hedge_loss_bound(best_expert_loss: f64, n: usize, eps: f64) -> f64 {
    (1.0 + eps) * best_expert_loss + (n as f64).ln() / eps
}

/// Problem 33-4: Stochastic / mini-batch Gradient Descent for linear regression `y ≈ X θ` with deterministic Rng shuffling.
pub fn sgd_linear_regression(
    x: &[Vec<f64>],
    y: &[f64],
    eta: f64,
    epochs: usize,
    batch_size: usize,
    seed: u64,
) -> Vec<f64> {
    let m = x.len();
    let d = x[0].len();
    let mut theta = vec![0.0f64; d];
    let mut rng = Rng::new(seed);
    let mut order: Vec<usize> = (0..m).collect();
    let bsz = batch_size.max(1).min(m);

    for ep in 0..epochs {
        rng.shuffle(&mut order);
        let lr = eta / (1.0 + 0.005 * ep as f64);
        for chunk in order.chunks(bsz) {
            let mut grad = vec![0.0f64; d];
            for &i in chunk {
                let pred: f64 = x[i].iter().zip(&theta).map(|(a, b)| a * b).sum();
                let err = pred - y[i];
                for j in 0..d {
                    grad[j] += x[i][j] * err;
                }
            }
            let scale = lr / chunk.len() as f64;
            for j in 0..d {
                theta[j] -= scale * grad[j];
            }
        }
    }
    theta
}

pub fn demo() -> String {
    let mut o = String::new();
    // Minimize f(x) = x^4 - 4x + 5 -> f'(x) = 4x^3 - 4, f''(x) = 12x^2 -> minimum at x = 1.
    let (x_n, it_n) = newton_minimize_1d(|x| 4.0 * x * x * x - 4.0, |x| 12.0 * x * x, 3.0, 1e-12, 50);
    outln!(o, "Newton 1D on x^4 - 4x + 5 from x0=3: x* = {x_n:.10} in {it_n} steps");

    let mut r = Rng::new(3302);
    let losses: Vec<Vec<f64>> = (0..200)
        .map(|_| (0..6).map(|i| if i == 2 { 0.1 + 0.2 * r.f64() } else { 0.3 + 0.6 * r.f64() }).collect())
        .collect();
    let (hl, opt, probs) = hedge(&losses, 0.2);
    outln!(o, "Hedge (200 rounds, 6 experts): loss={hl:.2}, best={opt:.2}, bound={:.2}, top_p={:.3}", hedge_loss_bound(opt, 6, 0.2), probs[2]);

    let xs: Vec<Vec<f64>> = (0..20).map(|i| vec![1.0, i as f64 / 10.0]).collect();
    let ys: Vec<f64> = xs.iter().map(|row| 2.0 + 3.0 * row[1]).collect();
    let th = sgd_linear_regression(&xs, &ys, 0.25, 400, 4, 3304);
    outln!(o, "SGD linear regression y = 2 + 3x -> theta = [{:.4}, {:.4}]", th[0], th[1]);
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newton_converges_quadratically_on_strictly_convex_functions() {
        let (x, it) = newton_minimize_1d(|x| 4.0 * x.powi(3) - 4.0, |x| 12.0 * x * x, 4.0, 1e-12, 30);
        assert!((x - 1.0).abs() < 1e-10);
        assert!(it <= 10);

        // Minimize 0.5*(x-3)^2 + (y+2)^2 + 0.2*(x-3)*(y+2)
        let (pt, it2) = newton_minimize(
            |p| vec![(p[0] - 3.0) + 0.2 * (p[1] + 2.0), 2.0 * (p[1] + 2.0) + 0.2 * (p[0] - 3.0)],
            |_| vec![vec![1.0, 0.2], vec![0.2, 2.0]],
            &[-10.0, 15.0],
            1e-12,
            10,
        );
        assert!((pt[0] - 3.0).abs() < 1e-12 && (pt[1] + 2.0).abs() < 1e-12);
        assert_eq!(it2, 1); // exact in 1 step for quadratic!
    }

    #[test]
    fn hedge_obeys_multiplicative_weights_regret_bound() {
        let mut r = Rng::new(3322);
        for _ in 0..40 {
            let n = r.range(2, 10);
            let rounds = r.range(20, 200);
            let losses: Vec<Vec<f64>> = (0..rounds)
                .map(|_| (0..n).map(|i| if i == 0 { 0.25 * r.f64() } else { r.f64() }).collect())
                .collect();
            for eps in [0.1, 0.25, 0.45] {
                let (hl, opt, probs) = hedge(&losses, eps);
                assert!((probs.iter().sum::<f64>() - 1.0).abs() < 1e-9);
                assert!(hl <= hedge_loss_bound(opt, n, eps) + 1e-9, "hl={hl}, opt={opt}, eps={eps}");
            }
        }
    }

    #[test]
    fn sgd_agrees_with_full_batch_gradient_descent() {
        let mut r = Rng::new(3304);
        let xs: Vec<Vec<f64>> = (0..60).map(|_| vec![1.0, r.f64() * 3.0 - 1.5, r.f64() * 2.0 - 1.0]).collect();
        let ys: Vec<f64> = xs.iter().map(|row| 1.5 - 2.0 * row[1] + 0.75 * row[2]).collect();
        let gd = super::super::gradient_descent::linear_regression(&xs, &ys, 0.15, 4_000);
        let sgd = sgd_linear_regression(&xs, &ys, 0.15, 600, 8, 99);
        for (a, b) in gd.iter().zip(&sgd) {
            assert!((a - b).abs() < 1e-2, "{gd:?} vs {sgd:?}");
        }
    }
}
