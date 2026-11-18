//! @title The Cow-Path Problem (Online Search)
//! @chapter Problem 27-1 (4e)
//! @editions 4
//! @summary Doubling spiral search along a 1D line finds a target at unknown signed distance d in ≤ 9|d| total steps, optimal among deterministic algorithms.
//! @order 40
use crate::outln;

/// Classic discrete doubling cow-path search (factor r = 2) starting at 0,
/// alternating +2^0, -2^1, +2^2, -2^3, ... until reaching integer coordinate `target`.
/// Returns `(total_distance_walked, turn_count)`. Always satisfies total <= 9 * |target|.
pub fn cow_path_search(target: i64) -> (i64, usize) {
    if target == 0 {
        return (0, 0);
    }
    let mut pos = 0i64;
    let mut dist = 0i64;
    let mut reach = 1i64;
    let mut dir = 1i64;
    let mut turns = 0usize;

    loop {
        let endpoint = dir * reach;
        // Check if target lies on segment [pos -> endpoint].
        if (pos <= target && target <= endpoint) || (endpoint <= target && target <= pos) {
            dist += (target - pos).abs();
            return (dist, turns);
        }
        dist += (endpoint - pos).abs();
        pos = endpoint;
        dir = -dir;
        reach = reach.saturating_mul(2);
        turns += 1;
    }
}

/// Generalized geometric cow-path search with multiplier `r > 1.0` on continuous coordinate `target`.
/// Achieves asymptotic competitive ratio <= 1 + 2 * r^2 / (r - 1) (minimized at r = 2 giving 9).
pub fn cow_path_search_factor(target: f64, r: f64) -> (f64, usize) {
    assert!(r > 1.0);
    if target == 0.0 {
        return (0.0, 0);
    }
    let mut pos = 0.0f64;
    let mut dist = 0.0f64;
    let mut reach = 1.0f64;
    let mut dir = 1.0f64;
    let mut turns = 0usize;

    loop {
        let endpoint = dir * reach;
        if (pos <= target && target <= endpoint) || (endpoint <= target && target <= pos) {
            dist += (target - pos).abs();
            return (dist, turns);
        }
        dist += (endpoint - pos).abs();
        pos = endpoint;
        dir = -dir;
        reach *= r;
        turns += 1;
    }
}

pub fn theoretical_ratio(r: f64) -> f64 {
    1.0 + 2.0 * r * r / (r - 1.0)
}

pub fn demo() -> String {
    let mut o = String::new();
    for d in [1i64, -1, 2, -2, 3, -5, 17, -65, 1025] {
        let (walked, turns) = cow_path_search(d);
        let ratio = walked as f64 / d.abs() as f64;
        outln!(o, "target = {:>5}: walked = {:>5} in {:>2} turns (ratio = {:.3} ≤ 9)", d, walked, turns, ratio);
    }
    for r in [1.5, 2.0, 3.0, 4.0] {
        outln!(o, "factor r = {r:.1}: worst-case bound = {:.3}", theoretical_ratio(r));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubling_never_exceeds_nine_times_distance() {
        assert_eq!(cow_path_search(0), (0, 0));
        let mut worst_ratio = 0.0f64;
        for d in -10_000i64..=10_000 {
            if d == 0 {
                continue;
            }
            let (dist, _) = cow_path_search(d);
            assert!(dist <= 9 * d.abs(), "d={d}, dist={dist} > {}", 9 * d.abs());
            worst_ratio = worst_ratio.max(dist as f64 / d.abs() as f64);
        }
        assert!(worst_ratio > 7.5 && worst_ratio <= 9.0);
    }

    #[test]
    fn general_factor_obeys_theoretical_ratio_bound() {
        for &r in &[1.5f64, 2.0, 2.5, 3.0, 4.0] {
            let bound = theoretical_ratio(r);
            for k in 0..10 {
                let d = r.powi(k) + 0.001;
                for &sign in &[1.0, -1.0] {
                    let (dist, _) = cow_path_search_factor(sign * d, r);
                    assert!(dist <= bound * d + 1e-6, "r={r}, d={d}: {dist} > {}", bound * d);
                }
            }
        }
    }
}
