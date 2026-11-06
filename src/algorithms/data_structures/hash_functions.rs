//! @title Hash Functions & Universal Hashing
//! @chapter 11.3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Division, multiplication and universal hashing methods; string keys as radix-128 integers.
//! @order 70
use crate::common::Rng;
use crate::outln;

/// Division method: h(k) = k mod m (pick m prime, not close to a power of 2).
pub fn division_method(k: u64, m: u64) -> u64 { k % m }

/// Multiplication method with Knuth's constant A ≈ (√5−1)/2: h(k) = ⌊m·(kA mod 1)⌋.
pub fn multiplication_method(k: u64, m: u64) -> u64 {
    const A: f64 = 0.618_033_988_749_894_9;
    let frac = (k as f64 * A).fract();
    (m as f64 * frac).floor() as u64
}

/// Word-level multiplication method for m = 2^p: h(k) = (k·s mod 2^64) >> (64−p), with s = ⌊A·2^64⌋ (odd).
pub fn multiplication_method_word(k: u64, p: u32) -> u64 {
    const S: u64 = 0x9E37_79B9_7F4A_7C15;
    k.wrapping_mul(S) >> (64 - p)
}

/// Interpret a string as an integer in radix 128 (Section 11.3.1), wrapping on overflow.
pub fn string_to_key(s: &str) -> u64 {
    s.bytes().fold(0u64, |acc, b| acc.wrapping_mul(128).wrapping_add(b as u64))
}

/// A member of the universal class H_{pm}: h_ab(k) = ((a·k + b) mod p) mod m.
#[derive(Debug, Clone, Copy)]
pub struct UniversalHash {
    pub a: u64,
    pub b: u64,
    pub p: u64,
    pub m: u64,
}

impl UniversalHash {
    /// Pick a ∈ {1..p−1} and b ∈ {0..p−1} at random; `p` must be a prime larger than any key.
    pub fn random(p: u64, m: u64, rng: &mut Rng) -> Self {
        UniversalHash { a: 1 + rng.next_u64() % (p - 1), b: rng.next_u64() % p, p, m }
    }
    pub fn hash(&self, k: u64) -> u64 {
        (((self.a as u128 * k as u128 + self.b as u128) % self.p as u128) % self.m as u128) as u64
    }
}

/// Number of colliding pairs when hashing `keys` into m slots.
pub fn colliding_pairs(keys: &[u64], m: usize, h: impl Fn(u64) -> u64) -> usize {
    let mut c = vec![0usize; m];
    for &k in keys { c[h(k) as usize] += 1; }
    c.iter().map(|&x| x * x.saturating_sub(1) / 2).sum()
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "division_method(123456, 701)       = {}", division_method(123_456, 701));
    outln!(o, "multiplication_method(123456, 16384) = {}", multiplication_method(123_456, 16384));
    outln!(o, "multiplication_method_word(123456, 14) = {}", multiplication_method_word(123_456, 14));
    outln!(o, "string_to_key(\"pt\") = {} (p=112, t=116 → 112·128+116)", string_to_key("pt"));
    let mut r = Rng::new(17);
    let keys: Vec<u64> = (0..1000).map(|i| i * 1024).collect(); // adversarial for m = 1024 (a power of 2)
    outln!(o, "keys are multiples of 1024; colliding pairs with m=1024:");
    outln!(o, "  division by 1024        : {}", colliding_pairs(&keys, 1024, |k| division_method(k, 1024)));
    outln!(o, "  division by prime 1009  : {}", colliding_pairs(&keys, 1009, |k| division_method(k, 1009)));
    let u = UniversalHash::random(1_000_003, 1024, &mut r);
    outln!(o, "  random universal hash   : {} (expected ≈ n²/2m = {})", colliding_pairs(&keys, 1024, |k| u.hash(k)), 1000 * 999 / 2 / 1024);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ranges_are_respected() {
        let mut r = Rng::new(55);
        for _ in 0..1000 {
            let k = r.next_u64() >> 10;
            assert!(division_method(k, 97) < 97);
            assert!(multiplication_method(k, 1000) < 1000);
            assert!(multiplication_method_word(k, 10) < 1024);
        }
        assert_eq!(string_to_key("pt"), 112 * 128 + 116);
    }
    #[test]
    fn universality_collision_probability_is_at_most_about_one_over_m() {
        let (p, m) = (1_000_003u64, 100u64);
        let mut r = Rng::new(56);
        let trials = 40_000;
        let hits = (0..trials).filter(|_| { let h = UniversalHash::random(p, m, &mut r); h.hash(12) == h.hash(345) }).count();
        // universal: Pr ≤ 1/m = 1%; allow statistical noise
        assert!(hits as f64 / trials as f64 <= 1.5 / m as f64, "{hits}");
    }
    #[test]
    fn division_by_power_of_two_is_bad_for_aligned_keys() {
        let keys: Vec<u64> = (0..200).map(|i| i * 64).collect();
        let bad = colliding_pairs(&keys, 64, |k| division_method(k, 64));
        let good = colliding_pairs(&keys, 61, |k| division_method(k, 61));
        assert!(bad > 10 * good);
    }
}
