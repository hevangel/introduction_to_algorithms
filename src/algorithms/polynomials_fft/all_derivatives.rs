//! @title Evaluating All Derivatives of a Polynomial at a Point
//! @chapter Problem 30-4 (3e)
//! @editions 3,4
//! @summary All n derivatives A^(t)(x₀) (a Taylor shift) from one polynomial multiplication in O(n lg n), instead of O(n²) with repeated Horner evaluations.
//! @order 100
use super::number_theoretic_transform::{multiply_mod, P};
use crate::outln;

fn pow_mod(mut b: u64, mut e: u64) -> u64 { let mut r = 1; b %= P; while e > 0 { if e & 1 == 1 { r = r * b % P; } b = b * b % P; e >>= 1; } r }

/// A^(t)(x₀) mod P for t = 0 … n−1, where A(x) = Σ a_j x^j (coefficients mod P, n < P).
/// A^(t)(x₀) = Σ_{j≥t} a_j·j!·x₀^{j−t}/(j−t)!, a correlation of u_j = a_j·j! with v_k = x₀^k/k!, computed with one convolution.
pub fn all_derivatives(a: &[u64], x0: u64) -> Vec<u64> {
    let n = a.len();
    let mut fact = vec![1u64; n + 1];
    for i in 1..=n { fact[i] = fact[i - 1] * i as u64 % P; }
    let inv_fact: Vec<u64> = fact.iter().map(|&f| pow_mod(f, P - 2)).collect();
    let u_rev: Vec<u64> = (0..n).rev().map(|j| a[j] % P * fact[j] % P).collect();     // reversed so that convolution = correlation
    let mut xp = 1u64;
    let v: Vec<u64> = (0..n).map(|k| { let t = xp * inv_fact[k] % P; xp = xp * (x0 % P) % P; t }).collect();
    let c = multiply_mod(&u_rev, &v);
    (0..n).map(|t| c[n - 1 - t]).collect()
}

/// Reference O(n²): repeatedly evaluate with Horner's rule and differentiate.
pub fn all_derivatives_naive(a: &[u64], x0: u64) -> Vec<u64> {
    let mut cur: Vec<u64> = a.iter().map(|&c| c % P).collect();
    let mut out = vec![];
    for _ in 0..a.len() {
        out.push(cur.iter().rev().fold(0, |acc, &c| (acc * (x0 % P) + c) % P));
        cur = (1..cur.len()).map(|j| cur[j] * j as u64 % P).collect();
        if cur.is_empty() { cur.push(0); }
    }
    out
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = [1u64, 2, 3, 4]; // 1 + 2x + 3x² + 4x³
    outln!(o, "A(x) = 1 + 2x + 3x² + 4x³; derivatives at x₀ = 2:");
    outln!(o, "  fast : {:?}", all_derivatives(&a, 2));
    outln!(o, "  naive: {:?}   (A(2) = 49, A′(2) = 62, A″(2) = 54, A‴(2) = 24)", all_derivatives_naive(&a, 2));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_repeated_horner_modulo_p() {
        let mut r = Rng::new(467);
        for _ in 0..150 {
            let n = r.range(1, 40);
            let a: Vec<u64> = (0..n).map(|_| r.next_u64() % P).collect();
            let x0 = r.next_u64() % P;
            assert_eq!(all_derivatives(&a, x0), all_derivatives_naive(&a, x0));
        }
    }
    #[test]
    fn small_integer_example() {
        assert_eq!(all_derivatives(&[1, 2, 3, 4], 2), vec![49, 62, 54, 24]);
        assert_eq!(all_derivatives(&[7], 5), vec![7]);
    }
}
