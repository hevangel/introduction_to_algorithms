//! @title Online Hiring (Secretary Problem)
//! @chapter 5.4.4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Observe the first k=n/e candidates, then hire the first one who beats them: success ≈ 1/e.
//! @order 160
use crate::common::Rng;
use crate::outln;

/// ON-LINE-MAXIMUM(k, n): reject the first k, then accept the first score above all of those.
/// Returns the index hired (the last candidate if none beats the sample).
pub fn online_maximum(scores: &[usize], k: usize) -> usize {
    let n = scores.len();
    let mut best_score = 0;
    for i in 0..k.min(n) {
        best_score = best_score.max(scores[i]);
    }
    for i in k..n {
        if scores[i] > best_score {
            return i;
        }
    }
    n - 1
}

/// Estimated probability of hiring the overall best candidate.
pub fn success_rate(n: usize, k: usize, trials: usize, rng: &mut Rng) -> f64 {
    let mut ok = 0;
    for _ in 0..trials {
        let mut v: Vec<usize> = (1..=n).collect();
        rng.shuffle(&mut v);
        if v[online_maximum(&v, k)] == n {
            ok += 1;
        }
    }
    ok as f64 / trials as f64
}

pub fn demo() -> String {
    let mut o = String::new();
    let n = 100;
    let mut r = Rng::new(99);
    outln!(o, "n = {n}; theoretical optimum k = n/e ≈ {:.1}, success ≈ 1/e ≈ {:.3}", n as f64 / std::f64::consts::E, 1.0 / std::f64::consts::E);
    for k in [0, 5, 20, 37, 60, 90] {
        outln!(o, "k = {:>2}: success rate = {:.3}", k, success_rate(n, k, 20000, &mut r));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic_cases() {
        assert_eq!(online_maximum(&[3, 1, 4, 1, 5], 2), 2);
        assert_eq!(online_maximum(&[9, 1, 2, 3], 1), 3); // nobody beats 9 → last
        assert_eq!(online_maximum(&[1, 2, 3], 0), 0);
    }
    #[test]
    fn near_one_over_e_at_optimal_k() {
        let mut r = Rng::new(21);
        let p = success_rate(100, 37, 30000, &mut r);
        assert!((0.33..0.42).contains(&p), "{p}");
        let bad = success_rate(100, 0, 30000, &mut r);
        assert!(bad < 0.03);
    }
}
