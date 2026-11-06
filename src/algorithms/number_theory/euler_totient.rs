//! @title Euler's Totient Function
//! @chapter 31.3 (1e–4e)
//! @editions 1,2,3,4
//! @summary φ(n) counts residues coprime to n: φ(n) = n·∏(1 − 1/p) over primes p | n; sieve all values up to N in O(N lg lg N); Euler's theorem aᵠ⁽ⁿ⁾ ≡ 1.
//! @order 32
use super::euclid_gcd::gcd;
use super::modular_exponentiation::mod_pow;
use crate::outln;

/// φ(n) by trial-division factorisation, O(√n).
pub fn totient(mut n: u64) -> u64 {
    let mut result = n;
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 { while n % p == 0 { n /= p; } result -= result / p; } // multiply by (1 − 1/p)
        p += 1;
    }
    if n > 1 { result -= result / n; }
    result
}

/// φ(1..=limit) with a sieve: for every prime p, subtract φ-share from multiples. O(N lg lg N).
pub fn totient_sieve(limit: usize) -> Vec<u64> {
    let mut phi: Vec<u64> = (0..=limit as u64).collect();
    for p in 2..=limit {
        if phi[p] == p as u64 { // p is prime (untouched)
            for m in (p..=limit).step_by(p) { phi[m] -= phi[m] / p as u64; }
        }
    }
    phi
}

pub fn demo() -> String {
    let mut o = String::new();
    let phi = totient_sieve(30);
    outln!(o, "φ(1..30) = {:?}", &phi[1..]);
    for n in [36u64, 97, 1000, 123_456_789] { outln!(o, "φ({n}) = {}", totient(n)); }
    let n = 1000;
    let s: u64 = (1..=n).filter(|d| n % d == 0).map(totient).sum();
    outln!(o, "Σ_{{d | {n}}} φ(d) = {s} = n");
    outln!(o, "Euler's theorem: 7^φ(100) mod 100 = {}", mod_pow(7, totient(100), 100));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_counting_coprime_residues() {
        let phi = totient_sieve(500);
        for n in 1..=500u64 {
            let brute = (1..=n).filter(|&k| gcd(k, n) == 1).count() as u64;
            assert_eq!(totient(n), brute, "n = {n}");
            assert_eq!(phi[n as usize], brute);
        }
    }
    #[test]
    fn identities() {
        for n in 1..300u64 {
            assert_eq!((1..=n).filter(|d| n % d == 0).map(totient).sum::<u64>(), n); // Σ φ(d) = n
            for a in (1..n.max(2)).filter(|&a| gcd(a, n) == 1) { assert_eq!(mod_pow(a, totient(n), n), 1 % n); } // Euler's theorem
        }
        assert_eq!(totient(97), 96);
    }
}
