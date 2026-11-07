//! @title Birthday Paradox, Balls & Bins, Streaks
//! @chapter 5.4 (1e–4e)
//! @editions 1,2,3,4
//! @summary The probabilistic-analysis examples: birthday collisions, coupon collector, longest streak of heads.
//! @order 170
use crate::common::Rng;
use crate::outln;

/// Exact probability that among k people in a year of n days at least two share a birthday.
pub fn birthday_collision_probability(k: usize, n: usize) -> f64 {
    let mut no_collision = 1.0;
    for i in 0..k {
        no_collision *= (n - i.min(n)) as f64 / n as f64;
    }
    1.0 - no_collision
}

/// Smallest k for which the collision probability is at least 1/2.
pub fn smallest_group_for_even_odds(n: usize) -> usize {
    (1..=n + 1).find(|&k| birthday_collision_probability(k, n) >= 0.5).unwrap()
}

/// Expected number of colliding pairs, C(k,2)/n (Section 5.4.1, indicator variables).
pub fn expected_colliding_pairs(k: usize, n: usize) -> f64 {
    (k * (k - 1)) as f64 / (2.0 * n as f64)
}

/// Balls and bins: throw balls into n bins until every bin is non-empty. Returns the number of tosses.
pub fn balls_until_all_bins_nonempty(n: usize, rng: &mut Rng) -> usize {
    let mut seen = vec![false; n];
    let (mut left, mut tosses) = (n, 0);
    while left > 0 {
        let b = rng.range(0, n - 1);
        tosses += 1;
        if !seen[b] {
            seen[b] = true;
            left -= 1;
        }
    }
    tosses
}

/// Length of the longest run of heads in n fair coin flips (Section 5.4.3).
pub fn longest_streak(n: usize, rng: &mut Rng) -> usize {
    let (mut best, mut cur) = (0, 0);
    for _ in 0..n {
        if rng.next_u64() & 1 == 1 {
            cur += 1;
            best = best.max(cur);
        } else {
            cur = 0;
        }
    }
    best
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "smallest group with ≥ 50% shared-birthday chance (365 days): {}", smallest_group_for_even_odds(365));
    for k in [10, 23, 40, 57] {
        outln!(o, "  k={:>2}: P(collision) = {:.4}", k, birthday_collision_probability(k, 365));
    }
    let mut r = Rng::new(1);
    let n = 100;
    let avg: f64 = (0..2000).map(|_| balls_until_all_bins_nonempty(n, &mut r) as f64).sum::<f64>() / 2000.0;
    let h: f64 = (1..=n).map(|i| 1.0 / i as f64).sum();
    outln!(o, "coupon collector n={n}: simulated {:.1} tosses, theory n·H_n = {:.1}", avg, n as f64 * h);
    let s: f64 = (0..500).map(|_| longest_streak(1000, &mut r) as f64).sum::<f64>() / 500.0;
    outln!(o, "longest head streak in 1000 flips: simulated {:.2}, lg 1000 = {:.2}", s, 1000f64.log2());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn birthday_threshold_is_23() {
        assert_eq!(smallest_group_for_even_odds(365), 23);
        assert!(birthday_collision_probability(23, 365) > 0.507);
        assert!((birthday_collision_probability(366, 365) - 1.0).abs() < 1e-12);
        assert_eq!(birthday_collision_probability(1, 365), 0.0);
    }
    #[test]
    fn coupon_collector_expectation() {
        let mut r = Rng::new(31);
        let n = 50;
        let avg: f64 = (0..4000).map(|_| balls_until_all_bins_nonempty(n, &mut r) as f64).sum::<f64>() / 4000.0;
        let h: f64 = (1..=n).map(|i| 1.0 / i as f64).sum();
        assert!((avg - n as f64 * h).abs() < 6.0, "{avg}");
    }
    #[test]
    fn streak_is_logarithmic() {
        let mut r = Rng::new(32);
        let avg: f64 = (0..300).map(|_| longest_streak(1024, &mut r) as f64).sum::<f64>() / 300.0;
        assert!((7.0..11.5).contains(&avg), "{avg}");
    }
}
