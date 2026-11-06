//! @title Polynomial Evaluation & Lagrange Interpolation
//! @chapter 30.1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Recover coefficients from n point-value pairs in Θ(n²) using Lagrange's formula; evaluate with Horner.
//! @order 40
use crate::outln;

/// Horner evaluation of Σ cᵢ xⁱ.
pub fn evaluate(c: &[f64], x: f64) -> f64 { c.iter().rev().fold(0.0, |acc, &a| acc * x + a) }

/// Lagrange interpolation: the unique polynomial of degree < n through (xs[i], ys[i]); returns coefficients (lowest first). Θ(n²).
pub fn interpolate(xs: &[f64], ys: &[f64]) -> Vec<f64> {
    let n = xs.len();
    let mut result = vec![0.0; n];
    for i in 0..n {
        // numerator polynomial Π_{j≠i} (x − x_j) built incrementally
        let mut basis = vec![1.0];
        let mut denom = 1.0;
        for j in 0..n {
            if j == i { continue; }
            let mut next = vec![0.0; basis.len() + 1];
            for (k, &b) in basis.iter().enumerate() { next[k + 1] += b; next[k] -= b * xs[j]; }
            basis = next;
            denom *= xs[i] - xs[j];
        }
        for (k, &b) in basis.iter().enumerate() { result[k] += ys[i] * b / denom; }
    }
    result
}

/// Evaluate at many points: Θ(n·m) with Horner (vs Θ(m lg m) using the FFT at roots of unity).
pub fn evaluate_many(c: &[f64], xs: &[f64]) -> Vec<f64> { xs.iter().map(|&x| evaluate(c, x)).collect() }

pub fn demo() -> String {
    let mut o = String::new();
    let c = [2.0, -3.0, 0.0, 1.0]; // x³ − 3x + 2
    let xs = [-2.0, -1.0, 0.0, 1.0, 2.0];
    let ys = evaluate_many(&c, &xs);
    outln!(o, "P(x) = 2 − 3x + x³ sampled at {:?} → {:?}", xs, ys);
    let rec = interpolate(&xs[..4], &ys[..4]);
    outln!(o, "Lagrange from 4 samples: {:?}", rec.iter().map(|v| (v * 1e9).round() / 1e9).collect::<Vec<_>>());
    outln!(o, "uniqueness: a degree-3 polynomial is fixed by any 4 distinct points");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn roundtrip_recovers_coefficients() {
        let mut r = Rng::new(203);
        for n in 1..=8 {
            let c: Vec<f64> = (0..n).map(|_| r.f64() * 10.0 - 5.0).collect();
            let xs: Vec<f64> = (0..n).map(|i| i as f64 - n as f64 / 2.0).collect();
            let back = interpolate(&xs, &evaluate_many(&c, &xs));
            for i in 0..n { assert!((back[i] - c[i]).abs() < 1e-6, "n={n} i={i}"); }
        }
    }
    #[test]
    fn horner_and_line_through_two_points() {
        assert_eq!(evaluate(&[2.0, -3.0, 0.0, 1.0], 2.0), 4.0);
        let line = interpolate(&[1.0, 3.0], &[5.0, 9.0]); // y = 2x + 3
        assert!((line[0] - 3.0).abs() < 1e-12 && (line[1] - 2.0).abs() < 1e-12);
    }
}
