//! @title Searching an Unsorted Array (Randomized)
//! @chapter Problem 5-2 (2e–4e)
//! @editions 2,3,4
//! @summary Random probing with replacement vs. a random permutation vs. a deterministic scan: expected probes n, (n+1)/2, n/(k+1).
//! @order 185
use crate::common::Rng;
use crate::outln;

/// RANDOM-SEARCH: probe uniformly random indices (with replacement) until `x` is found or every index has been probed.
/// Returns (found, probes).
pub fn random_probe(a: &[i64], x: i64, rng: &mut Rng) -> (bool, usize) {
    let n = a.len();
    let (mut seen, mut left, mut probes) = (vec![false; n], n, 0);
    while left > 0 {
        let i = rng.range(0, n - 1);
        probes += 1;
        if a[i] == x { return (true, probes); }
        if !seen[i] { seen[i] = true; left -= 1; }
    }
    (false, probes)
}

/// Probe indices in a random permutation order (without replacement).
pub fn permutation_probe(a: &[i64], x: i64, rng: &mut Rng) -> (bool, usize) {
    let mut order: Vec<usize> = (0..a.len()).collect();
    rng.shuffle(&mut order);
    for (k, &i) in order.iter().enumerate() { if a[i] == x { return (true, k + 1); } }
    (false, a.len())
}

/// DETERMINISTIC-SEARCH: left-to-right scan.
pub fn deterministic_scan(a: &[i64], x: i64) -> (bool, usize) {
    for (i, &v) in a.iter().enumerate() { if v == x { return (true, i + 1); } }
    (false, a.len())
}

fn mean_probes(f: impl Fn(&mut Rng) -> usize, trials: usize, rng: &mut Rng) -> f64 { (0..trials).map(|_| f(rng)).sum::<usize>() as f64 / trials as f64 }

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(9);
    let n = 100;
    for k in [1usize, 4, 10] {
        let a: Vec<i64> = (0..n).map(|i| if i < k { 7 } else { 0 }).collect();
        let trials = 4000;
        let (m1, m2) = (mean_probes(|r| random_probe(&a, 7, r).1, trials, &mut r), mean_probes(|r| permutation_probe(&a, 7, r).1, trials, &mut r));
        outln!(o, "k = {:>2} matches: with replacement {:>6.1} probes (theory n/k = {:.1}); permutation {:>5.1} (theory (n+1)/(k+1) = {:.1})", k, m1, n as f64 / k as f64, m2, (n as f64 + 1.0) / (k as f64 + 1.0));
    }
    let none = vec![0i64; n];
    outln!(o, "no match: with replacement {:.0} probes (≈ n·H_n = {:.0}); permutation {}", mean_probes(|r| random_probe(&none, 7, r).1, 2000, &mut r), n as f64 * (1..=n).map(|i| 1.0 / i as f64).sum::<f64>(), permutation_probe(&none, 7, &mut r).1);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn expected_probe_counts() {
        let mut r = Rng::new(304);
        let n = 50;
        let mut a = vec![0i64; n];
        a[17] = 7;
        let with_repl = mean_probes(|r| random_probe(&a, 7, r).1, 6000, &mut r);
        let without = mean_probes(|r| permutation_probe(&a, 7, r).1, 6000, &mut r);
        assert!((with_repl - 50.0).abs() < 3.0, "{with_repl}");
        assert!((without - 25.5).abs() < 1.0, "{without}");
        assert_eq!(deterministic_scan(&a, 7), (true, 18));
    }
    #[test]
    fn absent_element_costs() {
        let mut r = Rng::new(305);
        let a = vec![0i64; 30];
        assert_eq!(permutation_probe(&a, 1, &mut r), (false, 30));
        let h30: f64 = (1..=30).map(|i| 1.0 / i as f64).sum();
        let m = mean_probes(|r| random_probe(&a, 1, r).1, 4000, &mut r);
        assert!((m - 30.0 * h30).abs() < 5.0, "{m}");
    }
}
