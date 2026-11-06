//! @title Waiting for an Elevator (Competitive Analysis)
//! @chapter 27.1 (4e)
//! @editions 4
//! @summary Wait up to the cost of the stairs, then walk: a deterministic 2-competitive online strategy (ski rental).
//! @order 10
use crate::outln;

/// Cost of the strategy "wait up to `threshold`, then take the stairs (cost `k` extra)" when the elevator arrives at time `a`.
/// If a ≤ threshold we wait and ride (cost a); otherwise we give up at `threshold` and pay `threshold + k`.
pub fn online_cost(a: f64, threshold: f64, k: f64) -> f64 { if a <= threshold { a } else { threshold + k } }

/// Cost of the optimal offline strategy that knows `a` in advance: min(a, k).
pub fn offline_cost(a: f64, k: f64) -> f64 { a.min(k) }

/// Worst-case competitive ratio of the threshold strategy over arrival times in a fine grid.
pub fn worst_ratio(threshold: f64, k: f64) -> f64 {
    let mut worst = 1.0f64;
    let mut a = 0.0;
    while a <= 20.0 * k {
        let opt = offline_cost(a, k);
        if opt > 0.0 { worst = worst.max(online_cost(a, threshold, k) / opt); }
        a += k / 200.0;
    }
    // just after the threshold the online algorithm pays threshold + k while OPT pays min(threshold, k)
    worst.max((threshold + k) / threshold.min(k).max(1e-12))
}

pub fn demo() -> String {
    let mut o = String::new();
    let k = 1.0;
    outln!(o, "stairs cost k = {k}; ratio of 'wait θ, then walk' vs the offline optimum:");
    for theta in [0.1, 0.25, 0.5, 1.0, 2.0, 4.0, 10.0] {
        outln!(o, "  θ = {:>4}: worst-case ratio {:.3}", theta, worst_ratio(theta, k));
    }
    outln!(o, "best threshold θ = k gives ratio 2: the strategy is 2-competitive, and no deterministic strategy does better");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn costs() {
        assert_eq!(online_cost(0.5, 1.0, 1.0), 0.5);
        assert_eq!(online_cost(3.0, 1.0, 1.0), 2.0);
        assert_eq!(offline_cost(3.0, 1.0), 1.0);
        assert_eq!(offline_cost(0.4, 1.0), 0.4);
    }
    #[test]
    fn threshold_k_is_two_competitive_and_optimal() {
        let k = 2.5;
        let r = worst_ratio(k, k);
        assert!((r - 2.0).abs() < 0.02, "{r}");
        for theta in [0.2, 0.5, 1.0, 2.0, 5.0, 12.0] {
            if (theta - k).abs() > 0.2 { assert!(worst_ratio(theta, k) > r + 0.1, "θ={theta}"); }
        }
    }
    #[test]
    fn never_exceeds_two_for_any_arrival_time() {
        let k = 3.0;
        for i in 0..2000 {
            let a = i as f64 * 0.01;
            let opt = offline_cost(a, k);
            if opt > 0.0 { assert!(online_cost(a, k, k) <= 2.0 * opt + 1e-9); }
        }
    }
}
