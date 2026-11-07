//! @title Polynomial Multiplication via FFT
//! @chapter 30.1–30.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Multiply two degree-n polynomials in Θ(n lg n): evaluate with FFT, multiply pointwise, interpolate with the inverse FFT.
//! @order 30
use super::fft_recursive::{fft, Complex};
use crate::outln;

/// Schoolbook Θ(n²) product of coefficient vectors (lowest degree first).
pub fn multiply_naive(a: &[i64], b: &[i64]) -> Vec<i64> {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let mut c = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() { for (j, &y) in b.iter().enumerate() { c[i + j] += x * y; } }
    c
}

/// Θ(n lg n) product using the FFT: pad to a power of two, transform, multiply pointwise, inverse-transform, round.
pub fn multiply_fft(a: &[i64], b: &[i64]) -> Vec<i64> {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let len = a.len() + b.len() - 1;
    let n = len.next_power_of_two();
    let pad = |v: &[i64]| -> Vec<Complex> { (0..n).map(|i| Complex::new(*v.get(i).unwrap_or(&0) as f64, 0.0)).collect() };
    let (fa, fb) = (fft(&pad(a), false), fft(&pad(b), false));
    let prod: Vec<Complex> = fa.iter().zip(&fb).map(|(&x, &y)| x * y).collect();
    fft(&prod, true).iter().take(len).map(|c| c.re.round() as i64).collect()
}

pub fn format_poly(c: &[i64]) -> String {
    let terms: Vec<String> = c.iter().enumerate().filter(|(_, &v)| v != 0).map(|(i, &v)| match i { 0 => format!("{v}"), 1 => format!("{v}x"), _ => format!("{v}x^{i}") }).collect();
    if terms.is_empty() { "0".into() } else { terms.join(" + ") }
}

pub fn demo() -> String {
    let mut o = String::new();
    let (a, b) = ([1, 2, 3], [4, 5, 6, 7]); // (1 + 2x + 3x²)(4 + 5x + 6x² + 7x³)
    outln!(o, "A(x) = {}", format_poly(&a));
    outln!(o, "B(x) = {}", format_poly(&b));
    outln!(o, "A·B (FFT)   = {}", format_poly(&multiply_fft(&a, &b)));
    outln!(o, "A·B (naive) = {}", format_poly(&multiply_naive(&a, &b)));
    // big integers are polynomials in 10: 12345 × 6789
    let digits = |mut n: u64| -> Vec<i64> { let mut v = vec![]; while n > 0 { v.push((n % 10) as i64); n /= 10; } v };
    let mut c = multiply_fft(&digits(12345), &digits(6789));
    let mut carry = 0;
    for x in c.iter_mut() { *x += carry; carry = *x / 10; *x %= 10; }
    while carry > 0 { c.push(carry % 10); carry /= 10; }
    outln!(o, "12345 × 6789 via digit convolution = {}", c.iter().rev().map(|d| d.to_string()).collect::<String>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_schoolbook() {
        let mut r = Rng::new(202);
        for _ in 0..40 {
            let (na, nb) = (r.range(1, 40), r.range(1, 40));
            let (a, b) = (r.vec(na, -30, 30), r.vec(nb, -30, 30));
            assert_eq!(multiply_fft(&a, &b), multiply_naive(&a, &b));
        }
        assert!(multiply_fft(&[], &[1]).is_empty());
    }
    #[test]
    fn book_style_product() {
        assert_eq!(multiply_fft(&[1, 2, 3], &[4, 5, 6, 7]), vec![4, 13, 28, 34, 32, 21]);
        assert_eq!(multiply_fft(&[1, 1], &[1, 1]), vec![1, 2, 1]);
    }
}
