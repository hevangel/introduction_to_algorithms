//! @title Number-Theoretic Transform (FFT over a Finite Field)
//! @chapter Problem 30-6 (1e–4e)
//! @editions 1,2,3,4
//! @summary The DFT works over any ring with a principal n-th root of unity: modulo p = 998244353 the transform and convolution are exact integer arithmetic, no rounding errors.
//! @order 70
use crate::outln;

pub const P: u64 = 998_244_353; // 119·2²³ + 1, primitive root 3
const G: u64 = 3;

fn pow_mod(mut b: u64, mut e: u64) -> u64 { let mut r = 1; b %= P; while e > 0 { if e & 1 == 1 { r = r * b % P; } b = b * b % P; e >>= 1; } r }

/// In-place iterative NTT of a power-of-two length array (Cooley–Tukey with bit reversal), forward or inverse.
pub fn ntt(a: &mut [u64], invert: bool) {
    let n = a.len();
    assert!(n.is_power_of_two() && n <= 1 << 23);
    let mut j = 0;
    for i in 1..n { // bit-reversal permutation
        let mut bit = n >> 1;
        while j & bit != 0 { j ^= bit; bit >>= 1; }
        j ^= bit;
        if i < j { a.swap(i, j); }
    }
    let mut len = 2;
    while len <= n {
        let mut w = pow_mod(G, (P - 1) / len as u64); // principal len-th root of unity modulo P
        if invert { w = pow_mod(w, P - 2); }
        for start in (0..n).step_by(len) {
            let mut wk = 1;
            for k in 0..len / 2 {
                let (u, v) = (a[start + k], a[start + k + len / 2] * wk % P);
                a[start + k] = (u + v) % P;
                a[start + k + len / 2] = (u + P - v) % P;
                wk = wk * w % P;
            }
        }
        len <<= 1;
    }
    if invert { let ninv = pow_mod(n as u64, P - 2); for x in a.iter_mut() { *x = *x * ninv % P; } }
}

/// Product of two polynomials with coefficients in Z_P, exactly, in O(n lg n).
pub fn multiply_mod(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let need = a.len() + b.len() - 1;
    let size = need.next_power_of_two();
    let (mut fa, mut fb) = (a.to_vec(), b.to_vec());
    fa.resize(size, 0);
    fb.resize(size, 0);
    ntt(&mut fa, false);
    ntt(&mut fb, false);
    for i in 0..size { fa[i] = fa[i] * fb[i] % P; }
    ntt(&mut fa, true);
    fa.truncate(need);
    fa
}

pub fn demo() -> String {
    let mut o = String::new();
    let (a, b) = ([1u64, 2, 3], [4u64, 5]);
    outln!(o, "(1 + 2x + 3x²)(4 + 5x) mod {P} = {:?}", multiply_mod(&a, &b));
    let mut r = crate::common::Rng::new(5);
    let n = 1 << 12;
    let x: Vec<u64> = (0..n).map(|_| r.range(0, 999_999) as u64).collect();
    let y: Vec<u64> = (0..n).map(|_| r.range(0, 999_999) as u64).collect();
    let z = multiply_mod(&x, &y);
    let k = 1000;
    let check = (0..=k).filter(|&i| i < n && k - i < n).fold(0u64, |s, i| (s + x[i] * y[k - i]) % P);
    outln!(o, "two degree-{} polynomials with huge coefficients: coefficient {k} of the product = {} (direct sum {})", n - 1, z[k], check);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn convolution_equals_schoolbook_modulo_p() {
        let mut r = Rng::new(419);
        for _ in 0..100 {
            let (la, lb) = (r.range(1, 40), r.range(1, 40));
            let a: Vec<u64> = (0..la).map(|_| r.next_u64() % P).collect();
            let b: Vec<u64> = (0..lb).map(|_| r.next_u64() % P).collect();
            let mut want = vec![0u64; la + lb - 1];
            for i in 0..la { for j in 0..lb { want[i + j] = (want[i + j] + a[i] * b[j]) % P; } }
            assert_eq!(multiply_mod(&a, &b), want);
        }
    }
    #[test]
    fn transform_roundtrip_and_root_of_unity() {
        let mut r = Rng::new(420);
        for lg in 0..10 {
            let a: Vec<u64> = (0..1usize << lg).map(|_| r.next_u64() % P).collect();
            let mut b = a.clone();
            ntt(&mut b, false);
            ntt(&mut b, true);
            assert_eq!(a, b);
        }
        let w = pow_mod(G, (P - 1) / 8);
        assert_eq!((pow_mod(w, 8), pow_mod(w, 4)), (1, P - 1)); // principal 8th root of unity: w⁴ = −1
    }
}
