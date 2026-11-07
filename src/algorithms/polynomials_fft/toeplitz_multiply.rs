//! @title Toeplitz Matrix-Vector Product
//! @chapter Problem 30-2 (2e–4e)
//! @editions 2,3,4
//! @summary An n×n Toeplitz matrix has constant diagonals; its product with a vector is one polynomial multiplication, so it takes O(n lg n) instead of O(n²).
//! @order 80
use super::number_theoretic_transform::{multiply_mod, P};
use crate::outln;

/// T·x for the Toeplitz matrix T[i][j] = t[i − j + n − 1] (t has 2n − 1 entries). All values are taken modulo P.
pub fn toeplitz_multiply(t: &[u64], x: &[u64]) -> Vec<u64> {
    let n = x.len();
    assert_eq!(t.len(), 2 * n - 1);
    // y_i = Σ_j t[i − j + n − 1]·x_j = coefficient (i + n − 1) of the product t(z)·x(z)
    let c = multiply_mod(t, x);
    (0..n).map(|i| c[i + n - 1] % P).collect()
}

/// The O(n²) definition, for comparison.
pub fn toeplitz_multiply_naive(t: &[u64], x: &[u64]) -> Vec<u64> {
    let n = x.len();
    (0..n).map(|i| (0..n).fold(0, |s, j| (s + t[i + n - 1 - j] * x[j]) % P)).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    // T = [[a, b, c], [d, a, b], [e, d, a]] ↔ t = [e, d, a, b, c]
    let t = [5u64, 4, 1, 2, 3];
    let x = [1u64, 10, 100];
    outln!(o, "Toeplitz T = [[1,2,3],[4,1,2],[5,4,1]], x = {:?}", x);
    outln!(o, "T·x via polynomial multiplication = {:?}", toeplitz_multiply(&t, &x));
    outln!(o, "T·x by the definition             = {:?}", toeplitz_multiply_naive(&t, &x));
    let mut r = crate::common::Rng::new(3);
    let n = 2000;
    let (tt, xx): (Vec<u64>, Vec<u64>) = ((0..2 * n - 1).map(|_| r.range(0, 1000) as u64).collect(), (0..n).map(|_| r.range(0, 1000) as u64).collect());
    outln!(o, "n = {n}: fast product equals O(n²) product: {}", toeplitz_multiply(&tt, &xx) == toeplitz_multiply_naive(&tt, &xx));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn equals_naive_product() {
        let mut r = Rng::new(421);
        for _ in 0..100 {
            let n = r.range(1, 30);
            let t: Vec<u64> = (0..2 * n - 1).map(|_| r.next_u64() % P).collect();
            let x: Vec<u64> = (0..n).map(|_| r.next_u64() % P).collect();
            assert_eq!(toeplitz_multiply(&t, &x), toeplitz_multiply_naive(&t, &x));
        }
    }
    #[test]
    fn identity_like_matrix() {
        // t has a single 1 at the main-diagonal position n−1 ⇒ T is the identity
        let n = 7;
        let mut t = vec![0u64; 2 * n - 1];
        t[n - 1] = 1;
        let x: Vec<u64> = (1..=n as u64).collect();
        assert_eq!(toeplitz_multiply(&t, &x), x);
    }
}
