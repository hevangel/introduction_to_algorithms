//! @title Fast Fourier Transform (Recursive)
//! @chapter 30.2–30.3 (1e–4e)
//! @editions 1,2,3,4
//! @summary RECURSIVE-FFT evaluates a polynomial at the n complex n-th roots of unity in Θ(n lg n); the inverse DFT interpolates.
//! @order 10
use crate::outln;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex { pub re: f64, pub im: f64 }

impl Complex {
    pub const ZERO: Complex = Complex { re: 0.0, im: 0.0 };
    pub fn new(re: f64, im: f64) -> Self { Complex { re, im } }
    /// e^{iθ}
    pub fn cis(theta: f64) -> Self { Complex { re: theta.cos(), im: theta.sin() } }
    pub fn scale(self, k: f64) -> Self { Complex { re: self.re * k, im: self.im * k } }
    pub fn abs(self) -> f64 { self.re.hypot(self.im) }
}
impl Add for Complex { type Output = Complex; fn add(self, o: Complex) -> Complex { Complex::new(self.re + o.re, self.im + o.im) } }
impl Sub for Complex { type Output = Complex; fn sub(self, o: Complex) -> Complex { Complex::new(self.re - o.re, self.im - o.im) } }
impl Mul for Complex { type Output = Complex; fn mul(self, o: Complex) -> Complex { Complex::new(self.re * o.re - self.im * o.im, self.re * o.im + self.im * o.re) } }

/// RECURSIVE-FFT(a): n must be a power of 2. Set `invert` for the inverse DFT (scaled by 1/n).
pub fn fft(a: &[Complex], invert: bool) -> Vec<Complex> {
    let mut y = fft_rec(a, if invert { -1.0 } else { 1.0 });
    if invert { let n = y.len() as f64; for v in y.iter_mut() { *v = v.scale(1.0 / n); } }
    y
}

fn fft_rec(a: &[Complex], sign: f64) -> Vec<Complex> {
    let n = a.len();
    assert!(n.is_power_of_two(), "length must be a power of two");
    if n == 1 { return a.to_vec(); }
    let wn = Complex::cis(sign * 2.0 * std::f64::consts::PI / n as f64);
    let even: Vec<Complex> = a.iter().step_by(2).copied().collect();
    let odd: Vec<Complex> = a.iter().skip(1).step_by(2).copied().collect();
    let (y0, y1) = (fft_rec(&even, sign), fft_rec(&odd, sign));
    let mut y = vec![Complex::ZERO; n];
    let mut w = Complex::new(1.0, 0.0);
    for k in 0..n / 2 {
        let t = w * y1[k];
        y[k] = y0[k] + t;            // butterfly
        y[k + n / 2] = y0[k] - t;
        w = w * wn;
    }
    y
}

/// The O(n²) definition of the DFT, for comparison.
pub fn dft_naive(a: &[Complex]) -> Vec<Complex> {
    let n = a.len();
    (0..n).map(|k| (0..n).fold(Complex::ZERO, |acc, j| acc + a[j] * Complex::cis(2.0 * std::f64::consts::PI * (j * k) as f64 / n as f64))).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    // polynomial 1 + 2x + 3x² + 4x³ evaluated at the 4th roots of unity {1, i, −1, −i}
    let a: Vec<Complex> = [1.0, 2.0, 3.0, 4.0].iter().map(|&x| Complex::new(x, 0.0)).collect();
    let y = fft(&a, false);
    for (k, v) in y.iter().enumerate() { outln!(o, "A(ω⁴^{k}) = {:>7.3} {:+.3}i", v.re, v.im); }
    let back = fft(&y, true);
    outln!(o, "inverse FFT recovers coefficients: {:?}", back.iter().map(|c| (c.re * 1e9).round() / 1e9).collect::<Vec<_>>());
    outln!(o, "Θ(n lg n) vs Θ(n²): for n = 2^20 that is ≈ 2·10⁷ vs 10¹² butterfly operations");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_definition_and_roundtrips() {
        let mut r = Rng::new(200);
        for k in 0..=7 {
            let n = 1usize << k;
            let a: Vec<Complex> = (0..n).map(|_| Complex::new(r.f64() - 0.5, r.f64() - 0.5)).collect();
            let (f, d) = (fft(&a, false), dft_naive(&a));
            for i in 0..n { assert!((f[i] - d[i]).abs() < 1e-9, "n={n} i={i}"); }
            let back = fft(&f, true);
            for i in 0..n { assert!((back[i] - a[i]).abs() < 1e-9); }
        }
    }
    #[test]
    fn evaluates_at_roots_of_unity() {
        let y = fft(&[1.0, 2.0, 3.0, 4.0].map(|x| Complex::new(x, 0.0)), false);
        let want = [(10.0, 0.0), (-2.0, -2.0), (-2.0, 0.0), (-2.0, 2.0)];
        for (g, w) in y.iter().zip(want) { assert!((g.re - w.0).abs() < 1e-12 && (g.im - w.1).abs() < 1e-12); }
    }
}
