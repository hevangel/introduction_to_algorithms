//! @title Weighted-Majority Algorithm
//! @chapter 33.2 (4e)
//! @editions 4
//! @summary Learning from expert advice: shrink the weight of every wrong expert by (1−ε); mistakes ≤ 2(1+ε)m + (2 ln n)/ε.
//! @order 20
use crate::outln;

/// WEIGHTED-MAJORITY(predictions, outcomes, ε): `predictions[t][i]` is expert i's prediction in round t.
/// Returns (learner's mistakes, mistakes of the best expert, final weights).
pub fn weighted_majority(predictions: &[Vec<bool>], outcomes: &[bool], eps: f64) -> (usize, usize, Vec<f64>) {
    let n = predictions[0].len();
    let mut w = vec![1.0f64; n];
    let mut mistakes = 0;
    let mut expert_mistakes = vec![0usize; n];
    for (t, preds) in predictions.iter().enumerate() {
        // predict with the weighted vote
        let (yes, no): (f64, f64) = preds.iter().zip(&w).fold((0.0, 0.0), |(y, n), (&p, &wi)| if p { (y + wi, n) } else { (y, n + wi) });
        let guess = yes >= no;
        if guess != outcomes[t] { mistakes += 1; }
        // penalise the experts that were wrong
        for i in 0..n {
            if preds[i] != outcomes[t] { w[i] *= 1.0 - eps; expert_mistakes[i] += 1; }
        }
    }
    (mistakes, *expert_mistakes.iter().min().unwrap(), w)
}

/// Theorem 33.? bound on the learner's mistakes: 2(1+ε)·m* + 2·ln(n)/ε.
pub fn mistake_bound(best_expert_mistakes: usize, n: usize, eps: f64) -> f64 { 2.0 * (1.0 + eps) * best_expert_mistakes as f64 + 2.0 * (n as f64).ln() / eps }

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = crate::common::Rng::new(5);
    let (n, rounds) = (10, 400);
    let outcomes: Vec<bool> = (0..rounds).map(|_| r.range(0, 1) == 1).collect();
    // expert 0 is right 90% of the time, the others are coin flips
    let preds: Vec<Vec<bool>> = (0..rounds).map(|t| (0..n).map(|i| if i == 0 { if r.f64() < 0.9 { outcomes[t] } else { !outcomes[t] } } else { r.range(0, 1) == 1 }).collect()).collect();
    for eps in [0.5, 0.2, 0.1] {
        let (m, best, w) = weighted_majority(&preds, &outcomes, eps);
        outln!(o, "ε = {:<3}: learner mistakes {:>3}, best expert {:>3}, bound {:>6.1}, heaviest expert = {}", eps, m, best, mistake_bound(best, n, eps), w.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0);
    }
    outln!(o, "the learner quickly learns to trust expert 0; the regret against it stays within the bound");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn mistake_bound_holds_on_random_instances() {
        let mut r = Rng::new(262);
        for _ in 0..100 {
            let (n, rounds) = (r.range(2, 12), r.range(10, 300));
            let outcomes: Vec<bool> = (0..rounds).map(|_| r.range(0, 1) == 1).collect();
            let good = r.f64() * 0.5;
            let preds: Vec<Vec<bool>> = (0..rounds).map(|t| (0..n).map(|i| if i == 0 { if r.f64() < good { !outcomes[t] } else { outcomes[t] } } else { r.range(0, 1) == 1 }).collect()).collect();
            for eps in [0.5, 0.25, 0.1] {
                let (m, best, _) = weighted_majority(&preds, &outcomes, eps);
                assert!(m as f64 <= mistake_bound(best, n, eps) + 1e-9, "m={m} best={best} n={n} eps={eps}");
            }
        }
    }
    #[test]
    fn perfect_expert_makes_learner_converge() {
        let rounds = 200;
        let outcomes: Vec<bool> = (0..rounds).map(|t| t % 3 == 0).collect();
        let preds: Vec<Vec<bool>> = (0..rounds).map(|t| vec![outcomes[t], !outcomes[t], !outcomes[t], !outcomes[t], !outcomes[t]]).collect();
        let (m, best, w) = weighted_majority(&preds, &outcomes, 0.5);
        assert_eq!(best, 0);
        assert!(m <= 3, "{m}"); // 2 ln 5 / 0.5 ≈ 6.4 bound; in fact it learns almost at once
        assert!(w[0] == 1.0 && w[1] < 1e-30);
    }
}
