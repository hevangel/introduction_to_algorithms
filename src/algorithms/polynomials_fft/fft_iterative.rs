//! @title Iterative FFT (Bit-Reversal Butterflies)
//! @chapter 30.3 (1e–3e)
//! @editions 1,2,3
//! @summary BIT-REVERSE-COPY then lg n in-place butterfly passes: the loop structure used by hardware FFT circuits.
//! @order 20
use super::fft_recursive::Complex;
use crate::outln;

/// rev(k): reverse the lowest `bits` bits of k.
pub fn rev(k: usize, bits: u32) -> usize { k.reverse_bits() >> (usize::BITS - bits) }

/// BIT-REVERSE-COPY(a, A)
pub fn bit_reverse_copy(a: &[Complex]) -> Vec<Complex> {
    let n = a.len();
    let bits = n.trailing_zeros();
    if bits == 0 { return a.to_vec(); }
    (0..n).map(|k| a[rev(k, bits)]).collect()
}

/// ITERATIVE-FFT(a): in-place, Θ(n lg n); `invert` gives the inverse transform.
pub fn iterative_fft(a: &[Complex], invert: bool) -> Vec<Complex> {
    let n = a.len();
    assert!(n.is_power_of_two());
    let mut x = bit_reverse_copy(a);
    let sign = if invert { -1.0 } else { 1.0 };
    let mut m = 2;
    while m <= n {
        // pass s: combine adjacent FFTs of size m/2 into FFTs of size m
        let wm = Complex::cis(sign * 2.0 * std::f64::consts::PI / m as f64);
        for k in (0..n).step_by(m) {
            let mut w = Complex::new(1.0, 0.0);
            for j in 0..m / 2 {
                let t = w * x[k + j + m / 2];
                let u = x[k + j];
                x[k + j] = u + t;           // butterfly operation
                x[k + j + m / 2] = u - t;
                w = w * wm;
            }
        }
        m *= 2;
    }
    if invert { let s = 1.0 / n as f64; for v in x.iter_mut() { *v = v.scale(s); } }
    x
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "bit-reversal permutation for n = 8: {:?}", (0..8).map(|k| rev(k, 3)).collect::<Vec<_>>());
    let a: Vec<Complex> = (1..=8).map(|x| Complex::new(x as f64, 0.0)).collect();
    let y = iterative_fft(&a, false);
    outln!(o, "iterative FFT of 1..8 (re parts): {:?}", y.iter().map(|c| (c.re * 1e6).round() / 1e6).collect::<Vec<_>>());
    let same = super::fft_recursive::fft(&a, false);
    outln!(o, "equals recursive FFT: {}", y.iter().zip(&same).all(|(p, q)| (*p - *q).abs() < 1e-9));
    outln!(o, "passes: lg n = 3, each with n/2 = 4 butterflies → 12 butterflies total");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn bit_reversal() {
        assert_eq!((0..8).map(|k| rev(k, 3)).collect::<Vec<_>>(), vec![0, 4, 2, 6, 1, 5, 3, 7]);
    }
    #[test]
    fn equals_recursive_fft_and_inverts() {
        let mut r = Rng::new(201);
        for k in 0..=9 {
            let n = 1usize << k;
            let a: Vec<Complex> = (0..n).map(|_| Complex::new(r.f64(), r.f64())).collect();
            let (i, rc) = (iterative_fft(&a, false), super::super::fft_recursive::fft(&a, false));
            for j in 0..n { assert!((i[j] - rc[j]).abs() < 1e-8); }
            let back = iterative_fft(&i, true);
            for j in 0..n { assert!((back[j] - a[j]).abs() < 1e-8); }
        }
    }
}
