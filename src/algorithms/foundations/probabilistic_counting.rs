//! @title Probabilistic Counting
//! @chapter Problem 5-1 (2e–4e)
//! @editions 2,3,4
//! @summary Count to n in only lg n bits: increment the stored value i with probability 1/(nᵢ₊₁ − nᵢ); unbiased, Morris-counter variance.
//! @order 180
use crate::common::Rng;
use crate::outln;

/// A probabilistic counter storing a small index i that represents the count n_i.
pub struct Counter { i: usize, values: Box<dyn Fn(usize) -> u64> }

impl Counter {
    /// n_i = 100·i (Problem 5-1(b)): variance of the estimate after n increments is 99n.
    pub fn linear() -> Self { Counter { i: 0, values: Box::new(|i| 100 * i as u64) } }
    /// n_i = 2^i − 1 (Morris's counter): count up to 2^b with about lg lg n bits; variance ≈ n²/2.
    pub fn morris() -> Self { Counter { i: 0, values: Box::new(|i| (1u64 << i) - 1) } }
    pub fn stored(&self) -> usize { self.i }
    /// INCREMENT: advance i with probability 1/(n_{i+1} − n_i).
    pub fn increment(&mut self, rng: &mut Rng) {
        let gap = (self.values)(self.i + 1) - (self.values)(self.i);
        if rng.next_u64() % gap == 0 { self.i += 1; }
    }
    /// The represented count n_i, an unbiased estimator of the number of increments.
    pub fn estimate(&self) -> u64 { (self.values)(self.i) }
}

/// Mean and variance of the estimate after `n` increments over `trials` runs.
pub fn simulate(mk: fn() -> Counter, n: usize, trials: usize, rng: &mut Rng) -> (f64, f64) {
    let est: Vec<f64> = (0..trials).map(|_| { let mut c = mk(); for _ in 0..n { c.increment(rng); } c.estimate() as f64 }).collect();
    let mean = est.iter().sum::<f64>() / trials as f64;
    (mean, est.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / trials as f64)
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(3);
    let n = 1000;
    let (m, v) = simulate(Counter::linear, n, 4000, &mut r);
    outln!(o, "n_i = 100i  : mean estimate {:.1} (true {n}), variance {:.0} (theory 99n = {})", m, v, 99 * n);
    let (m, v) = simulate(Counter::morris, n, 4000, &mut r);
    outln!(o, "n_i = 2^i−1 : mean estimate {:.1} (true {n}), variance {:.0} (theory ≈ n²/2 = {})", m, v, n * n / 2);
    let mut c = Counter::morris();
    for _ in 0..1_000_000 { c.increment(&mut r); }
    outln!(o, "Morris counter after 10⁶ increments stores i = {} (a {}-bit register) and estimates {}", c.stored(), 64 - (c.stored() as u64).leading_zeros(), c.estimate());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn estimators_are_unbiased() {
        let mut r = Rng::new(302);
        let (m, v) = simulate(Counter::linear, 1000, 4000, &mut r);
        assert!((m - 1000.0).abs() < 30.0, "{m}");
        assert!((v / 99_000.0 - 1.0).abs() < 0.2, "{v}");
        let (m, _) = simulate(Counter::morris, 1000, 6000, &mut r);
        assert!((m - 1000.0).abs() < 70.0, "{m}");
    }
    #[test]
    fn stored_index_is_tiny() {
        let mut r = Rng::new(303);
        let mut c = Counter::morris();
        for _ in 0..100_000 { c.increment(&mut r); }
        assert!(c.stored() < 25);
        assert_eq!(Counter::morris().estimate(), 0);
    }
}
