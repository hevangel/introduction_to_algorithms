//! @title Bit-Reversed Binary Counter
//! @chapter Problem 17-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary BIT-REVERSED-INCREMENT enumerates 0…2^k−1 in bit-reversed order with O(1) amortised flips, giving the FFT permutation in Θ(n).
//! @order 240
use crate::outln;

/// A k-bit counter A[0..k) (bit i has weight 2^i) that counts in bit-reversed order: it carries *downwards* from the top bit.
#[derive(Debug, Clone)]
pub struct BitReversedCounter { bits: Vec<bool>, pub flips: usize }

impl BitReversedCounter {
    pub fn new(k: usize) -> Self { BitReversedCounter { bits: vec![false; k], flips: 0 } }
    pub fn value(&self) -> usize { self.bits.iter().enumerate().map(|(i, &b)| (b as usize) << i).sum() }
    /// BIT-REVERSED-INCREMENT: add 1 to the number whose binary representation is the reverse of A. Starting from the most
    /// significant position of A, clear the run of 1s and set the first 0 — the mirror image of the ordinary INCREMENT.
    pub fn increment(&mut self) {
        let mut i = self.bits.len();
        while i > 0 && self.bits[i - 1] { self.bits[i - 1] = false; self.flips += 1; i -= 1; }
        if i > 0 { self.bits[i - 1] = true; self.flips += 1; }
    }
}

/// The bit-reversal permutation rev_k(0), rev_k(1), …, rev_k(2^k − 1) in Θ(2^k) total time (used by the iterative FFT).
pub fn bit_reversal_permutation(k: usize) -> Vec<usize> {
    let mut c = BitReversedCounter::new(k);
    (0..1usize << k).map(|_| { let v = c.value(); c.increment(); v }).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "bit-reversal permutation for k = 3: {:?}", bit_reversal_permutation(3));
    outln!(o, "k = 4: {:?}", bit_reversal_permutation(4));
    for k in [8usize, 12, 16, 20] {
        let mut c = BitReversedCounter::new(k);
        for _ in 0..(1usize << k) - 1 { c.increment(); }
        outln!(o, "k = {:>2}: {} increments cost {} bit flips ({:.2} per increment, amortised O(1))", k, (1usize << k) - 1, c.flips, c.flips as f64 / ((1usize << k) - 1) as f64);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::polynomials_fft::fft_iterative::rev;
    #[test]
    fn enumerates_the_bit_reversal_permutation() {
        for k in 1..=10 {
            let p = bit_reversal_permutation(k);
            assert_eq!(p, (0..1usize << k).map(|i| rev(i, k as u32)).collect::<Vec<_>>(), "k={k}");
        }
        assert_eq!(bit_reversal_permutation(3), vec![0, 4, 2, 6, 1, 5, 3, 7]);
    }
    #[test]
    fn amortised_constant_cost() {
        for k in [6usize, 10, 14] {
            let mut c = BitReversedCounter::new(k);
            let n = 1usize << k;
            for _ in 0..n - 1 { c.increment(); }
            assert!(c.flips < 2 * n, "k={k} flips={}", c.flips);
        }
    }
}
