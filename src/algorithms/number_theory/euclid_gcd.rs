//! @title Euclid's GCD Algorithm
//! @chapter 31.2 (1e–4e); Problem 31-1 binary gcd
//! @editions 1,2,3,4
//! @summary gcd(a, b) = gcd(b, a mod b): O(lg b) steps (Lamé's theorem); plus binary GCD and lcm.
//! @order 10
use crate::outln;

/// EUCLID(a, b), iterative.
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { (a, b) = (b, a % b); }
    a
}

/// Number of division steps EUCLID performs (Lamé: ≤ ⌈log_φ b⌉ + 1... worst case on consecutive Fibonacci numbers).
pub fn gcd_steps(mut a: u64, mut b: u64) -> usize {
    let mut s = 0;
    while b != 0 { (a, b) = (b, a % b); s += 1; }
    s
}

/// Problem 31-1: binary gcd using only subtraction, parity tests and shifts.
pub fn binary_gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b { std::mem::swap(&mut a, &mut b); }
        b -= a;
        if b == 0 { return a << shift; }
    }
}

pub fn lcm(a: u64, b: u64) -> u64 { if a == 0 || b == 0 { 0 } else { a / gcd(a, b) * b } }

/// gcd of a list (Exercise 31.2-8).
pub fn gcd_many(xs: &[u64]) -> u64 { xs.iter().fold(0, |g, &x| gcd(g, x)) }

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "EUCLID(30, 21) = {}   binary_gcd(30, 21) = {}", gcd(30, 21), binary_gcd(30, 21));
    outln!(o, "gcd(99, 78) = {}, lcm(4, 6) = {}, gcd of [12, 18, 30] = {}", gcd(99, 78), lcm(4, 6), gcd_many(&[12, 18, 30]));
    outln!(o, "worst case: consecutive Fibonacci numbers");
    let (mut a, mut b) = (1u64, 1u64);
    for k in 2..=40 { (a, b) = (b, a + b); if k % 8 == 0 { outln!(o, "  gcd(F{}, F{}) takes {} steps", k + 1, k, gcd_steps(b, a)); } }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn known_values() {
        assert_eq!((gcd(30, 21), gcd(99, 78), gcd(0, 5), gcd(7, 0), gcd(0, 0)), (3, 3, 5, 7, 0));
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(gcd_many(&[12, 18, 30]), 6);
        assert_eq!(gcd_many(&[]), 0);
    }
    #[test]
    fn binary_equals_euclid() {
        let mut r = Rng::new(210);
        for _ in 0..2000 {
            let (a, b) = (r.next_u64() >> r.range(0, 60), r.next_u64() >> r.range(0, 60));
            assert_eq!(binary_gcd(a, b), gcd(a, b));
        }
    }
    #[test]
    fn lame_bound_on_fibonacci() {
        let (mut a, mut b) = (1u64, 2u64);
        for k in 1..60usize {
            (a, b) = (b, a + b);
            // gcd(F_{k+2}, F_{k+1}) needs about k steps, which is ≤ log_φ(b) + 1
            assert!(gcd_steps(b, a) as f64 <= (b as f64).ln() / 1.618033988749895f64.ln() + 2.0, "k={k}");
        }
    }
}
