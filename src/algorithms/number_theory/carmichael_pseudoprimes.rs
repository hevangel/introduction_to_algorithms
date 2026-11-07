//! @title Pseudoprimes and Carmichael Numbers
//! @chapter 31.8 (1e–4e)
//! @editions 1,2,3,4
//! @summary Fermat's test a^(n−1) ≡ 1 (mod n) never rejects primes but also accepts composites: base-2 pseudoprimes and the Carmichael numbers that fool every coprime base (Korselt's criterion).
//! @order 52
use super::euclid_gcd::gcd;
use super::modular_exponentiation::mod_pow;
use crate::outln;

pub fn is_prime_trial(n: u64) -> bool { n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| n % d != 0) }

/// Fermat test to base a: does a^(n−1) ≡ 1 (mod n)?
pub fn fermat_passes(n: u64, a: u64) -> bool { mod_pow(a, n - 1, n) == 1 }

/// Composite numbers that pass the base-2 test (PSEUDOPRIME), up to `limit`.
pub fn base2_pseudoprimes(limit: u64) -> Vec<u64> { (3..=limit).filter(|&n| !is_prime_trial(n) && fermat_passes(n, 2)).collect() }

/// By definition: composite n for which every a coprime to n passes the Fermat test.
pub fn is_carmichael_by_definition(n: u64) -> bool {
    n > 2 && !is_prime_trial(n) && (1..n).filter(|&a| gcd(a, n) == 1).all(|a| fermat_passes(n, a))
}

/// Korselt's criterion: n is a Carmichael number iff n is composite, squarefree and (p − 1) | (n − 1) for every prime p | n.
pub fn is_carmichael_korselt(n: u64) -> bool {
    if n < 4 || is_prime_trial(n) { return false; }
    let (mut m, mut p) = (n, 2u64);
    let mut factors = 0;
    while p * p <= m {
        if m % p == 0 {
            m /= p;
            if m % p == 0 || (n - 1) % (p - 1) != 0 { return false; } // not squarefree, or p − 1 ∤ n − 1
            factors += 1;
        }
        p += 1;
    }
    if m > 1 { if (n - 1) % (m - 1) != 0 { return false; } factors += 1; }
    factors >= 2
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "base-2 pseudoprimes below 1000: {:?}", base2_pseudoprimes(1000));
    let carm: Vec<u64> = (2..20_000).filter(|&n| is_carmichael_korselt(n)).collect();
    outln!(o, "Carmichael numbers below 20000 (Korselt): {:?}", carm);
    outln!(o, "561 passes the Fermat test for every coprime base: {}", is_carmichael_by_definition(561));
    outln!(o, "341 = 11·31 is a base-2 pseudoprime, but base 3 exposes it: 2→{}, 3→{}", fermat_passes(341, 2), fermat_passes(341, 3));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn definition_equals_korselt_below_3000() {
        for n in 2..3000 { assert_eq!(is_carmichael_by_definition(n), is_carmichael_korselt(n), "n = {n}"); }
    }
    #[test]
    fn known_values() {
        assert_eq!(base2_pseudoprimes(1000), vec![341, 561, 645]);
        let c: Vec<u64> = (2..10_000).filter(|&n| is_carmichael_korselt(n)).collect();
        assert_eq!(c, vec![561, 1105, 1729, 2465, 2821, 6601, 8911]);
        assert!(!fermat_passes(341, 3) && (2..100).filter(|&p| is_prime_trial(p)).all(|p| (1..p).all(|a| fermat_passes(p, a))));
    }
}
