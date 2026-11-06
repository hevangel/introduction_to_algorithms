//! @title RSA Public-Key Cryptosystem
//! @chapter 31.7 (1e–4e)
//! @editions 1,2,3,4
//! @summary Key generation from two primes, encryption/decryption by modular exponentiation, and signatures.
//! @order 80
use super::extended_euclid::mod_inverse;
use super::miller_rabin::is_prime;
use super::modular_exponentiation::mod_pow;
use crate::common::Rng;
use crate::outln;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PublicKey { pub n: u64, pub e: u64 }
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SecretKey { pub n: u64, pub d: u64 }

/// Random prime with exactly `bits` bits (bits ≤ 31 so that n = p·q fits in u64).
pub fn random_prime(bits: u32, rng: &mut Rng) -> u64 {
    loop {
        let c = (rng.next_u64() >> (64 - bits)) | (1 << (bits - 1)) | 1;
        if is_prime(c) { return c; }
    }
}

/// Key generation from primes p ≠ q: n = pq, pick e coprime to φ(n) = (p−1)(q−1), d = e⁻¹ mod φ(n).
pub fn keygen_from_primes(p: u64, q: u64) -> Option<(PublicKey, SecretKey)> {
    let phi = (p - 1) * (q - 1);
    let e = [65537u64, 17, 5, 3].into_iter().find(|&e| e < phi && super::euclid_gcd::gcd(e, phi) == 1)?;
    keygen_with_e(p, q, e)
}

/// Same, with a caller-chosen public exponent e (must be coprime to φ(n)).
pub fn keygen_with_e(p: u64, q: u64, e: u64) -> Option<(PublicKey, SecretKey)> {
    let (n, phi) = (p * q, (p - 1) * (q - 1));
    if super::euclid_gcd::gcd(e, phi) != 1 { return None; }
    let d = mod_inverse(e as i64, phi as i64)? as u64;
    Some((PublicKey { n, e }, SecretKey { n, d }))
}

pub fn keygen(bits: u32, rng: &mut Rng) -> (PublicKey, SecretKey) {
    loop {
        let (p, q) = (random_prime(bits, rng), random_prime(bits, rng));
        if p != q { if let Some(k) = keygen_from_primes(p, q) { return k; } }
    }
}

/// P(M) = M^e mod n
pub fn encrypt(pk: &PublicKey, m: u64) -> u64 { mod_pow(m, pk.e, pk.n) }
/// S(C) = C^d mod n
pub fn decrypt(sk: &SecretKey, c: u64) -> u64 { mod_pow(c, sk.d, sk.n) }
/// Signature of a message digest m: S(m).
pub fn sign(sk: &SecretKey, m: u64) -> u64 { mod_pow(m, sk.d, sk.n) }
pub fn verify(pk: &PublicKey, m: u64, sig: u64) -> bool { mod_pow(sig, pk.e, pk.n) == m % pk.n }

pub fn demo() -> String {
    let mut o = String::new();
    // the book's example: p = 11, q = 29, n = 319, e = 3, d = 187
    let (pk, sk) = keygen_with_e(11, 29, 3).unwrap();
    outln!(o, "p=11, q=29: public key (n={}, e={}), secret key d={}", pk.n, pk.e, sk.d);
    let c = encrypt(&pk, 100);
    outln!(o, "encrypt 100 → {c}; decrypt → {}", decrypt(&sk, c));
    let mut r = Rng::new(99);
    let (pk2, sk2) = keygen(30, &mut r);
    let m = 123_456_789_012u64 % pk2.n;
    let c2 = encrypt(&pk2, m);
    outln!(o, "60-bit modulus n = {}: message {m} → ciphertext {c2} → {}", pk2.n, decrypt(&sk2, c2));
    let sig = sign(&sk2, m);
    outln!(o, "signature verifies: {}   tampered message verifies: {}", verify(&pk2, m, sig), verify(&pk2, m + 1, sig));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn book_numbers() {
        let (pk, sk) = keygen_with_e(11, 29, 3).unwrap();
        assert_eq!((pk.n, pk.e, sk.d), (319, 3, 187));
        assert!(keygen_with_e(11, 29, 2).is_none());
        let (pk2, sk2) = keygen_from_primes(11, 29).unwrap();
        assert_eq!((pk2.e, sk2.d), (17, 33));
        assert_eq!(encrypt(&pk, 100), 254);
        assert_eq!(decrypt(&sk, 254), 100);
    }
    #[test]
    fn roundtrip_and_signatures_with_random_keys() {
        let mut r = Rng::new(214);
        for bits in [8u32, 12, 20, 31] {
            let (pk, sk) = keygen(bits, &mut r);
            for _ in 0..30 {
                let m = r.next_u64() % pk.n;
                assert_eq!(decrypt(&sk, encrypt(&pk, m)), m);
                assert_eq!(encrypt(&pk, decrypt(&sk, m)), m);
                let s = sign(&sk, m);
                assert!(verify(&pk, m, s));
                if pk.n > 2 { assert!(!verify(&pk, (m + 1) % pk.n, s)); }
            }
        }
    }
    #[test]
    fn random_primes_are_prime_with_requested_size() {
        let mut r = Rng::new(215);
        for _ in 0..20 { let p = random_prime(31, &mut r); assert!(is_prime(p)); assert_eq!(64 - p.leading_zeros(), 31); }
    }
}
