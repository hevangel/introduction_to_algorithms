//! @title Quadratic Residues & Tonelli-Shanks
//! @chapter Problem 31-4 (2e–4e); Problem 33-4 (1e)
//! @editions 1,2,3,4
//! @summary Test quadratic residuosity modulo odd prime p via Euler's criterion and extract modular square roots in O(lg² p) via Tonelli-Shanks.
//! @order 75
use super::modular_exponentiation::mod_pow;
use crate::outln;

/// Legendre symbol (a / p) for odd prime p via Euler's criterion: a^((p-1)/2) mod p.
/// Returns 0 if p | a, 1 if a is a quadratic residue mod p, or -1 if non-residue.
pub fn legendre_symbol(a: u64, p: u64) -> i8 {
    assert!(p >= 3 && p % 2 == 1);
    let a = a % p;
    if a == 0 {
        return 0;
    }
    let e = mod_pow(a, (p - 1) / 2, p);
    if e == 1 { 1 } else { -1 }
}

/// Tonelli-Shanks modular square root: find r in [0, p) with r^2 === a (mod p) for prime p.
/// Returns None if a is a quadratic non-residue mod p.
pub fn mod_sqrt(a: u64, p: u64) -> Option<u64> {
    assert!(p >= 2);
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if p == 2 {
        return Some(a);
    }
    if legendre_symbol(a, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(mod_pow(a, (p + 1) / 4, p));
    }

    // Write p - 1 = q * 2^s with q odd.
    let mut q = p - 1;
    let mut s = 0u32;
    while q.is_multiple_of(2) {
        q /= 2;
        s += 1;
    }
    let mut z = 2u64;
    while legendre_symbol(z, p) != -1 {
        z += 1;
    }
    let mut m = s;
    let mut c = mod_pow(z, q, p);
    let mut t = mod_pow(a, q, p);
    let mut r = mod_pow(a, q.div_ceil(2), p);

    while t != 1 {
        let mut i = 1u32;
        let mut tt = (t as u128 * t as u128 % p as u128) as u64;
        while tt != 1 {
            tt = (tt as u128 * tt as u128 % p as u128) as u64;
            i += 1;
        }
        let b = mod_pow(c, 1u64 << (m - i - 1), p);
        m = i;
        c = (b as u128 * b as u128 % p as u128) as u64;
        t = (t as u128 * c as u128 % p as u128) as u64;
        r = (r as u128 * b as u128 % p as u128) as u64;
    }
    Some(r)
}

pub fn demo() -> String {
    let mut o = String::new();
    let p = 41u64; // p = 1 (mod 8) exercises full Tonelli-Shanks
    let qrs: Vec<u64> = (1..p).filter(|&a| legendre_symbol(a, p) == 1).collect();
    outln!(o, "quadratic residues mod {p} (count {} = (p-1)/2): {:?}", qrs.len(), qrs);
    for a in [2u64, 5, 10, 13] {
        match mod_sqrt(a, p) {
            Some(r) => outln!(o, "  sqrt({a}) mod {p} = {r} (and {}), check: {r}^2 mod {p} = {}", p - r, (r * r) % p),
            None => outln!(o, "  sqrt({a}) mod {p} = None (non-residue)"),
        }
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    fn primes_up_to(limit: u64) -> Vec<u64> {
        let mut is_p = vec![true; limit as usize + 1];
        is_p[0] = false;
        is_p[1] = false;
        for i in 2..=limit as usize {
            if is_p[i] {
                let mut m = i * i;
                while m <= limit as usize {
                    is_p[m] = false;
                    m += i;
                }
            }
        }
        (2..=limit).filter(|&p| is_p[p as usize]).collect()
    }

    #[test]
    fn legendre_and_tonelli_shanks_exact_for_primes_up_to_1000() {
        for p in primes_up_to(1000) {
            if p == 2 {
                assert_eq!(mod_sqrt(1, 2), Some(1));
                continue;
            }
            if p <= 200 {
                let mut residues = vec![false; p as usize];
                for x in 1..p {
                    residues[((x * x) % p) as usize] = true;
                }
                for a in 1..p {
                    let sym = legendre_symbol(a, p);
                    assert_eq!(sym == 1, residues[a as usize]);
                    match mod_sqrt(a, p) {
                        Some(r) => {
                            assert_eq!(sym, 1);
                            assert_eq!((r * r) % p, a);
                            assert_eq!(((p - r) * (p - r)) % p, a);
                        }
                        None => assert_eq!(sym, -1),
                    }
                }
            } else {
                for a in 1..40 {
                    let a_mod = a % p;
                    if a_mod == 0 {
                        continue;
                    }
                    let sym = legendre_symbol(a_mod, p);
                    match mod_sqrt(a_mod, p) {
                        Some(r) => {
                            assert_eq!(sym, 1);
                            assert_eq!((r * r) % p, a_mod);
                        }
                        None => assert_eq!(sym, -1),
                    }
                }
            }
        }
    }
}
