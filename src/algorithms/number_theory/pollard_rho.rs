//! @title Pollard's Rho Integer Factorization
//! @chapter 31.9 (1e–3e)
//! @editions 1,2,3
//! @summary Find a factor of n in expected O(n^¼) steps by detecting a cycle of x ↦ x²+c mod n.
//! @order 90
use super::euclid_gcd::gcd;
use super::miller_rabin::is_prime;
use crate::common::Rng;
use crate::outln;

fn mulmod(a: u64, b: u64, n: u64) -> u64 { (a as u128 * b as u128 % n as u128) as u64 }

/// POLLARD-RHO(n): returns a nontrivial factor of the composite n (Floyd cycle detection, retried with new constants).
pub fn pollard_rho(n: u64, rng: &mut Rng) -> u64 {
    if n % 2 == 0 { return 2; }
    loop {
        let c = 1 + rng.next_u64() % (n - 1);
        let f = |x: u64| (mulmod(x, x, n) + c) % n;
        let (mut x, mut y, mut d) = (2 + rng.next_u64() % (n - 2), 0u64, 1u64);
        y = x.max(y);
        while d == 1 {
            x = f(x);            // tortoise: one step
            y = f(f(y));         // hare: two steps
            d = gcd(x.abs_diff(y), n);
        }
        if d != n { return d; }  // d == n means the cycle closed modulo n itself: retry with another c
    }
}

/// Complete prime factorisation (sorted with multiplicity): trial division for tiny primes, then Pollard's rho + Miller–Rabin.
pub fn factorize(mut n: u64, rng: &mut Rng) -> Vec<u64> {
    let mut f = vec![];
    for p in 2..100u64 { while n % p == 0 { f.push(p); n /= p; } }
    fn rec(n: u64, f: &mut Vec<u64>, rng: &mut Rng) {
        if n == 1 { return; }
        if is_prime(n) { f.push(n); return; }
        let d = pollard_rho(n, rng);
        rec(d, f, rng);
        rec(n / d, f, rng);
    }
    rec(n, &mut f, rng);
    f.sort();
    f
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(5);
    outln!(o, "POLLARD-RHO(1387) = {}  (1387 = 19·73)", pollard_rho(1387, &mut r));
    for n in [8051u64, 10_403, 600_851_475_143, 1_000_000_016_000_000_063, 999_999_999_989 * 3] {
        outln!(o, "{} = {:?}", n, factorize(n, &mut r));
    }
    let n = 1_000_003u64 * 999_983;
    outln!(o, "semiprime {n}: rho finds {} (≈ n^¼ = {:.0} iterations expected)", pollard_rho(n, &mut r), (n as f64).powf(0.25));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_nontrivial_factors() {
        let mut r = Rng::new(216);
        for n in [1387u64, 8051, 10_403, 455_459, 1_000_003 * 999_983] {
            let d = pollard_rho(n, &mut r);
            assert!(d > 1 && d < n && n % d == 0, "n={n} d={d}");
        }
    }
    #[test]
    fn full_factorisation_multiplies_back() {
        let mut r = Rng::new(217);
        for _ in 0..100 {
            let n = 2 + r.next_u64() % 1_000_000_000_000;
            let f = factorize(n, &mut r);
            assert_eq!(f.iter().product::<u64>(), n);
            assert!(f.iter().all(|&p| is_prime(p)));
        }
        assert_eq!(factorize(1, &mut r), Vec::<u64>::new());
        assert_eq!(factorize(600_851_475_143, &mut r), vec![71, 839, 1471, 6857]);
        assert_eq!(factorize(1 << 40, &mut r), vec![2; 40]);
    }
}
