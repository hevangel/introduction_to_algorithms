//! @title Modular Exponentiation
//! @chapter 31.6 (1e–4e)
//! @editions 1,2,3,4
//! @summary a^b mod n by repeated squaring in O(β) multiplications; powers, orders and primitive roots.
//! @order 50
use crate::outln;

/// MODULAR-EXPONENTIATION(a, b, n): scan the bits of b from high to low, squaring each step.
pub fn mod_pow(a: u64, b: u64, n: u64) -> u64 {
    if n == 1 { return 0; }
    let mut d: u128 = 1;
    for i in (0..64 - b.leading_zeros()).rev() {
        d = d * d % n as u128;                         // invariant: d = a^c mod n, c = prefix of b
        if b >> i & 1 == 1 { d = d * a as u128 % n as u128; }
    }
    d as u64
}

/// Number of modular multiplications used: 2·(bit length of b) at most.
pub fn multiplications(b: u64) -> u32 { (64 - b.leading_zeros()) + b.count_ones() }

/// Multiplicative order of a modulo n (smallest t > 0 with aᵗ ≡ 1), if gcd(a, n) = 1.
pub fn order(a: u64, n: u64) -> Option<u64> {
    if super::euclid_gcd::gcd(a, n) != 1 { return None; }
    let (mut x, mut t) = (a % n, 1);
    while x != 1 % n { x = (x as u128 * a as u128 % n as u128) as u64; t += 1; }
    Some(t)
}

/// Smallest primitive root modulo a prime p (generator of Z_p*).
pub fn primitive_root(p: u64) -> Option<u64> {
    if p == 2 { return Some(1); }
    (2..p).find(|&g| order(g, p) == Some(p - 1))
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "7^560 mod 561 = {}  (561 is a Carmichael number)", mod_pow(7, 560, 561));
    outln!(o, "2^1000000007 mod 998244353 = {}", mod_pow(2, 1_000_000_007, 998_244_353));
    outln!(o, "multiplications for an exponent with 30 bits: ≤ {}", multiplications((1 << 30) - 1));
    outln!(o, "order of 2 mod 11 = {:?}; order of 3 mod 11 = {:?}", order(2, 11), order(3, 11));
    outln!(o, "smallest primitive root mod 23 = {:?}", primitive_root(23));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_naive_power() {
        for a in 0..12u64 { for b in 0..12u64 { for n in 1..40u64 {
            let want = (0..b).fold(1u64 % n, |acc, _| acc * a % n);
            assert_eq!(mod_pow(a, b, n), want, "{a}^{b} mod {n}");
        } } }
    }
    #[test]
    fn fermat_and_large_moduli() {
        let mut r = Rng::new(212);
        let p = 1_000_000_007u64;
        for _ in 0..50 { let a = r.range(1, 1_000_000) as u64; assert_eq!(mod_pow(a, p - 1, p), 1); }
        assert_eq!(mod_pow(2, 10, 1_000_000_007), 1024);
        assert_eq!(mod_pow(u64::MAX, u64::MAX, u64::MAX - 58) < u64::MAX - 58, true);
    }
    #[test]
    fn orders_and_roots() {
        assert_eq!(order(2, 11), Some(10));
        assert_eq!(order(3, 11), Some(5));
        assert_eq!(order(4, 8), None);
        assert_eq!(primitive_root(23), Some(5));
        assert_eq!(primitive_root(7), Some(3));
        assert_eq!(multiplications(0b1011), 4 + 3);
    }
}
