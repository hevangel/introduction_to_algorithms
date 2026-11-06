//! @title Hiring Problem & Indicator Variables
//! @chapter 5.1–5.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary HIRE-ASSISTANT and its expected number of hires (the harmonic number H_n ≈ ln n).
//! @order 140
use crate::common::Rng;
use crate::outln;

/// HIRE-ASSISTANT: returns how many times a better candidate appeared (hires). `ranks[i]` is the quality.
pub fn hire_assistant(ranks: &[usize]) -> usize {
    let mut best = 0usize;
    let mut hires = 0;
    for &r in ranks {
        if r > best {
            best = r;
            hires += 1;
        }
    }
    hires
}

/// H_n = 1 + 1/2 + … + 1/n.
pub fn harmonic(n: usize) -> f64 {
    (1..=n).map(|i| 1.0 / i as f64).sum()
}

/// Average number of hires over `trials` uniformly random permutations of 1..=n.
pub fn average_hires(n: usize, trials: usize, rng: &mut Rng) -> f64 {
    let mut total = 0;
    for _ in 0..trials {
        let mut v: Vec<usize> = (1..=n).collect();
        rng.shuffle(&mut v);
        total += hire_assistant(&v);
    }
    total as f64 / trials as f64
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "best-to-worst order hires once   : {}", hire_assistant(&[5, 4, 3, 2, 1]));
    outln!(o, "worst-to-best order hires n times: {}", hire_assistant(&[1, 2, 3, 4, 5]));
    let mut r = Rng::new(2024);
    for n in [10, 100, 1000] {
        outln!(o, "n={:>4}: simulated avg hires = {:.3}   H_n = {:.3}   ln n = {:.3}", n, average_hires(n, 2000, &mut r), harmonic(n), (n as f64).ln());
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extremes() {
        assert_eq!(hire_assistant(&[1, 2, 3, 4, 5, 6]), 6);
        assert_eq!(hire_assistant(&[6, 5, 4, 3, 2, 1]), 1);
        assert_eq!(hire_assistant(&[]), 0);
    }
    #[test]
    fn average_matches_harmonic_number() {
        let mut r = Rng::new(11);
        let avg = average_hires(50, 20000, &mut r);
        assert!((avg - harmonic(50)).abs() < 0.1, "{avg} vs {}", harmonic(50));
    }
}
