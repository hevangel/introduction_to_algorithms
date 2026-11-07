//! @title Quadratic Residues and Square Roots Modulo a Prime
//! @chapter Problem 31-4 (3e)
//! @editions 3,4
//! @summary Exactly half of Z_p* are squares; Euler's criterion decides which in O(lg p), and Tonelli–Shanks finds a square root in O(lg² p) (closed form a^((p+1)/4) when p ≡ 3 mod 4).
//! @order 56
use super::modular_exponentiation::mod_pow;
use crate::outln;

pub fn is_prime(n: u64) -> bool { n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| n % d != 0) }

/// Legendre symbol (a / p) for an odd prime p by Euler's criterion: a^((p−1)/2) mod p is 1, p − 1 or 0.
pub fn legendre(a: u64, p: u64) -> i32 {
    match mod_pow(a % p, (p - 1) / 2, p) { 0 => 0, 1 => 1, _ => -1 }
}

/// A square root of `a` modulo the odd prime p (Tonelli–Shanks), or None if a is a non-residue. The other root is p − x.
pub fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 { return Some(0); }
    if legendre(a, p) != 1 { return None; }
    if p % 4 == 3 { return Some(mod_pow(a, (p + 1) / 4, p)); } // closed form
    let (mut q, mut s) = (p - 1, 0);
    while q % 2 == 0 { q /= 2; s += 1; }                      // p − 1 = q·2^s with q odd
    let z = (2..p).find(|&z| legendre(z, p) == -1).unwrap();   // any non-residue
    let (mut m, mut c, mut t, mut r) = (s, mod_pow(z, q, p), mod_pow(a, q, p), mod_pow(a, (q + 1) / 2, p));
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 { tt = (tt as u128 * tt as u128 % p as u128) as u64; i += 1; }
        let b = mod_pow(c, 1 << (m - i - 1), p);
        m = i;
        c = (b as u128 * b as u128 % p as u128) as u64;
        t = (t as u128 * c as u128 % p as u128) as u64;
        r = (r as u128 * b as u128 % p as u128) as u64;
    }
    Some(r)
}

pub fn demo() -> String {
    let mut o = String::new();
    for p in [7u64, 13, 17, 41, 97, 1_000_000_007] {
        let residues = (1..p.min(200)).filter(|&a| legendre(a, p) == 1).count();
        outln!(o, "p = {p}: p mod 4 = {}, residues among 1..{}: {residues}", p % 4, p.min(200) - 1);
    }
    for (a, p) in [(2u64, 7u64), (10, 13), (5, 41), (3, 97), (2, 1_000_000_007)] {
        outln!(o, "sqrt({a}) mod {p} = {:?}", sqrt_mod(a, p));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exactly_half_are_residues_and_criterion_matches_squares() {
        for p in (3..300u64).filter(|&p| is_prime(p)) {
            let squares: std::collections::BTreeSet<u64> = (1..p).map(|x| x * x % p).collect();
            assert_eq!(squares.len() as u64, (p - 1) / 2, "p = {p}");
            for a in 1..p { assert_eq!(legendre(a, p) == 1, squares.contains(&a)); }
        }
    }
    #[test]
    fn square_roots_square_back_for_both_kinds_of_prime() {
        for p in (3..400u64).filter(|&p| is_prime(p)) {
            for a in 0..p {
                match sqrt_mod(a, p) {
                    Some(x) => { assert_eq!(x * x % p, a, "p = {p}, a = {a}"); assert!(legendre(a, p) >= 0); }
                    None => assert_eq!(legendre(a, p), -1),
                }
            }
        }
        let p = 1_000_000_007u64; // p ≡ 3 (mod 4)
        let q = 998_244_353u64;   // p ≡ 1 (mod 4): the general Tonelli–Shanks branch
        for (a, m) in [(2u64, p), (5, p), (3, q), (10, q), (123_456_789, q)] { if let Some(x) = sqrt_mod(a, m) { assert_eq!((x as u128 * x as u128 % m as u128) as u64, a); } }
        assert!(sqrt_mod(3, q).is_some() || legendre(3, q) == -1);
    }
}
