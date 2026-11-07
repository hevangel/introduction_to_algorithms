//! @title Chirp Transform: Evaluating a Polynomial at Many Points
//! @chapter Problem 30-5 (3e)
//! @editions 3,4
//! @summary Evaluate A at the n points ω⁰, ω¹, …, ω^{n−1} for any ω (not necessarily a root of unity) in O(n lg n) by the identity jk = C(j+k, 2) − C(j, 2) − C(k, 2), which turns the sum into a convolution.
//! @order 110
use super::number_theoretic_transform::{multiply_mod, P};
use crate::outln;

fn pow_mod(mut b: u64, mut e: u64) -> u64 { let mut r = 1; b %= P; while e > 0 { if e & 1 == 1 { r = r * b % P; } b = b * b % P; e >>= 1; } r }

/// y_k = A(ω^k) = Σ_j a_j ω^{jk} (mod P) for k = 0 … n−1, ω a non-zero residue.
pub fn chirp_transform(a: &[u64], omega: u64) -> Vec<u64> {
    let n = a.len();
    assert!(omega % P != 0);
    let winv = pow_mod(omega, P - 2);
    let tri = |r: u64| r * r.saturating_sub(1) / 2 % (P - 1); // C(r, 2) reduced modulo the group order
    // jk = C(j+k,2) − C(j,2) − C(k,2)   ⇒   y_k = ω^{−C(k,2)} · Σ_j (a_j ω^{−C(j,2)}) · ω^{C(j+k,2)}
    let f_rev: Vec<u64> = (0..n).rev().map(|j| a[j] % P * pow_mod(winv, tri(j as u64)) % P).collect();
    let g: Vec<u64> = (0..2 * n - 1).map(|r| pow_mod(omega, tri(r as u64))).collect();
    let c = multiply_mod(&f_rev, &g); // c[n−1+k] = Σ_j f_j g_{j+k}
    (0..n).map(|k| c[n - 1 + k] * pow_mod(winv, tri(k as u64)) % P).collect()
}

/// Direct evaluation with Horner's rule at each ω^k: O(n²).
pub fn evaluate_naive(a: &[u64], omega: u64) -> Vec<u64> {
    let mut x = 1u64;
    (0..a.len()).map(|_| { let y = a.iter().rev().fold(0, |acc, &c| (acc * x + c) % P); x = x * (omega % P) % P; y }).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = [3u64, 1, 4, 1, 5, 9, 2, 6];
    outln!(o, "A(x) = 3 + x + 4x² + x³ + 5x⁴ + 9x⁵ + 2x⁶ + 6x⁷ at ω^k for ω = 12345:");
    outln!(o, "  chirp : {:?}", chirp_transform(&a, 12345));
    outln!(o, "  direct: {:?}", evaluate_naive(&a, 12345));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_direct_evaluation_for_arbitrary_omega() {
        let mut r = Rng::new(468);
        for _ in 0..150 {
            let n = r.range(1, 40);
            let a: Vec<u64> = (0..n).map(|_| r.next_u64() % P).collect();
            let omega = 1 + r.next_u64() % (P - 1);
            assert_eq!(chirp_transform(&a, omega), evaluate_naive(&a, omega));
        }
    }
    #[test]
    fn omega_one_sums_coefficients_and_single_point() {
        assert_eq!(chirp_transform(&[1, 2, 3], 1), vec![6, 6, 6]);
        assert_eq!(chirp_transform(&[5], 77), vec![5]);
    }
}
