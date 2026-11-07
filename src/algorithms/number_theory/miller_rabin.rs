//! @title Primality Testing: Fermat & Miller-Rabin
//! @chapter 31.8 (1e–4e)
//! @editions 1,2,3,4
//! @summary The pseudoprimality test (fooled by Carmichael numbers) and the randomized Miller-Rabin test with error ≤ 2^−s.
//! @order 70
use super::modular_exponentiation::mod_pow;
use crate::common::Rng;
use crate::outln;

/// PSEUDOPRIME(n): is 2^{n−1} ≡ 1 (mod n)? Always true for primes; also true for rare composites.
pub fn pseudoprime(n: u64) -> bool { n >= 2 && mod_pow(2, n - 1, n) == 1 }

/// WITNESS(a, n): true if `a` proves that n is composite (n odd, > 2).
pub fn witness(a: u64, n: u64) -> bool {
    let (mut t, mut u) = (0, n - 1);
    while u % 2 == 0 { u /= 2; t += 1; }                 // n − 1 = 2^t · u with u odd
    let mut x = mod_pow(a, u, n);
    for _ in 0..t {
        let y = (x as u128 * x as u128 % n as u128) as u64;
        if y == 1 && x != 1 && x != n - 1 { return true; } // nontrivial square root of 1
        x = y;
    }
    x != 1                                                // Fermat test fails
}

/// MILLER-RABIN(n, s): s random bases; "composite" answers are always correct, "prime" is wrong with probability ≤ 2^−s.
pub fn miller_rabin(n: u64, s: usize, rng: &mut Rng) -> bool {
    if n < 2 { return false; }
    if n < 4 { return true; }
    if n % 2 == 0 { return false; }
    (0..s).all(|_| !witness(2 + rng.next_u64() % (n - 3), n))
}

/// Deterministic for all 64-bit n: the first 12 primes as bases suffice.
pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 { return n == p; }
    }
    ![2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37].iter().any(|&a| witness(a, n))
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(1);
    outln!(o, "n    | pseudoprime(2) | Miller-Rabin (s=10) | deterministic is_prime");
    for n in [97u64, 561, 1105, 1729, 2047, 7919, 1_000_000_007, 1_000_000_007 * 3] {
        outln!(o, "{:<12} | {:<5} | {:<5} | {}", n, pseudoprime(n), miller_rabin(n, 10, &mut r), is_prime(n));
    }
    outln!(o, "561 = 3·11·17 is a Carmichael number: it passes Fermat tests for every base coprime to it, yet witness(2, 561) = {}", witness(2, 561));
    let primes = (2..100u64).filter(|&n| is_prime(n)).count();
    outln!(o, "primes below 100: {primes}");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sieve(n: usize) -> Vec<bool> {
        let mut s = vec![true; n + 1];
        s[0] = false;
        if n >= 1 { s[1] = false; }
        for i in 2..=n { if s[i] { let mut j = i * i; while j <= n { s[j] = false; j += i; } } }
        s
    }
    #[test]
    fn deterministic_test_matches_sieve() {
        let s = sieve(30_000);
        for n in 0..=30_000u64 { assert_eq!(is_prime(n), s[n as usize], "n={n}"); }
    }
    #[test]
    fn carmichael_numbers_fool_fermat_but_not_miller_rabin() {
        for n in [561u64, 1105, 1729, 2465, 2821, 6601] {
            assert!(pseudoprime(n), "{n}");
            assert!(witness(2, n) || (2..n).any(|a| witness(a, n)));
            assert!(!is_prime(n));
        }
        assert!(pseudoprime(341)); // the smallest base-2 pseudoprime that isn't Carmichael
        assert!(witness(2, 561));
    }
    #[test]
    fn randomized_variant_and_large_primes() {
        let mut r = Rng::new(213);
        let s = sieve(5000);
        for n in 0..=5000u64 { assert_eq!(miller_rabin(n, 20, &mut r), s[n as usize], "n={n}"); }
        assert!(is_prime(18_446_744_073_709_551_557)); // largest prime below 2^64
        assert!(!is_prime(18_446_744_073_709_551_556));
        assert!(is_prime(1_000_000_007) && !is_prime(1_000_000_007 * 998_244_353 % u64::MAX | 1 | 4));
    }
}
