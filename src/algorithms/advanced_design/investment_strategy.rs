//! @title Planning an Investment Strategy
//! @chapter Problem 15-10 (3e) / 14-10 (4e)
//! @editions 3,4
//! @summary Maximise the money after y years when each year all money sits in one investment: stay (fee f₁) or switch (fee f₂). O(n·y) DP with the two best predecessors.
//! @order 265
use crate::outln;

/// `rates[j][i]` is the return factor of investment i in year j. Staying in the same investment costs fee `f1`, switching costs `f2` dollars.
/// The initial money counts as a "switch" into the first investment. Returns (final money, chosen investment per year).
pub fn best_strategy(start: f64, rates: &[Vec<f64>], f1: f64, f2: f64) -> (f64, Vec<usize>) {
    let (years, n) = (rates.len(), rates[0].len());
    // best[j][i]: most money at the end of year j when year j's money was in investment i
    let mut best = vec![vec![f64::NEG_INFINITY; n]; years];
    let mut from = vec![vec![usize::MAX; n]; years];
    for i in 0..n { best[0][i] = (start - f2) * rates[0][i]; }
    for j in 1..years {
        // top two previous values give max_{k ≠ i} in O(1) per investment
        let (mut m1, mut m2) = ((f64::NEG_INFINITY, usize::MAX), (f64::NEG_INFINITY, usize::MAX));
        for k in 0..n { let v = (best[j - 1][k], k); if v.0 > m1.0 { m2 = m1; m1 = v; } else if v.0 > m2.0 { m2 = v; } }
        for i in 0..n {
            let stay = (best[j - 1][i] - f1) * rates[j][i];
            let other = if m1.1 != i { m1 } else { m2 };
            let switch = if other.1 == usize::MAX { f64::NEG_INFINITY } else { (other.0 - f2) * rates[j][i] };
            if stay >= switch { best[j][i] = stay; from[j][i] = i; } else { best[j][i] = switch; from[j][i] = other.1; }
        }
    }
    let (mut i, &final_money) = best[years - 1].iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap();
    let mut plan = vec![0; years];
    for j in (0..years).rev() { plan[j] = i; if j > 0 { i = from[j][i]; } }
    (final_money, plan)
}

/// Money at the end of a given plan (one investment per year).
pub fn simulate(start: f64, rates: &[Vec<f64>], plan: &[usize], f1: f64, f2: f64) -> f64 {
    let mut money = start;
    for (j, &i) in plan.iter().enumerate() {
        let fee = if j > 0 && plan[j - 1] == i { f1 } else { f2 };
        money = (money - fee) * rates[j][i];
    }
    money
}

pub fn demo() -> String {
    let mut o = String::new();
    let rates = vec![vec![1.10, 1.02, 1.05], vec![0.95, 1.20, 1.01], vec![1.15, 0.98, 1.04], vec![1.00, 1.12, 1.30], vec![1.08, 1.01, 0.90]];
    let (money, plan) = best_strategy(10_000.0, &rates, 20.0, 150.0);
    outln!(o, "5 years, 3 investments, fees: stay $20 / switch $150");
    outln!(o, "optimal plan (investment per year) = {:?}, final money ${:.2}", plan, money);
    outln!(o, "always investment 1: ${:.2}", simulate(10_000.0, &rates, &[0; 5], 20.0, 150.0));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn brute(start: f64, rates: &[Vec<f64>], f1: f64, f2: f64) -> f64 {
        let (years, n) = (rates.len(), rates[0].len());
        (0..n.pow(years as u32)).map(|code| {
            let plan: Vec<usize> = (0..years).map(|j| code / n.pow(j as u32) % n).collect();
            simulate(start, rates, &plan, f1, f2)
        }).fold(f64::NEG_INFINITY, f64::max)
    }
    #[test]
    fn matches_exhaustive_search() {
        let mut r = Rng::new(343);
        for _ in 0..100 {
            let (years, n) = (r.range(1, 5), r.range(1, 4));
            let rates: Vec<Vec<f64>> = (0..years).map(|_| (0..n).map(|_| 0.9 + r.f64() * 0.4).collect()).collect();
            let (f1, f2) = (r.f64() * 50.0, 50.0 + r.f64() * 200.0);
            let (m, plan) = best_strategy(10_000.0, &rates, f1, f2);
            assert!((m - brute(10_000.0, &rates, f1, f2)).abs() < 1e-6);
            assert!((simulate(10_000.0, &rates, &plan, f1, f2) - m).abs() < 1e-6);
        }
    }
    #[test]
    fn single_investment_and_fee_effects() {
        let rates = vec![vec![1.1], vec![1.1], vec![1.1]];
        let (m, plan) = best_strategy(1000.0, &rates, 10.0, 50.0);
        assert_eq!(plan, vec![0, 0, 0]);
        assert!((m - (((1000.0 - 50.0) * 1.1 - 10.0) * 1.1 - 10.0) * 1.1).abs() < 1e-9);
    }
}
