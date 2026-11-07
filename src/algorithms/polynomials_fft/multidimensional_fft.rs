//! @title Multidimensional FFT
//! @chapter Problem 30-3 (3e)
//! @editions 3,4
//! @summary A d-dimensional DFT is a one-dimensional FFT along each axis in turn: O(N lg N) for N = n₁⋯n_d points; used for 2-D convolution and bivariate polynomial products.
//! @order 90
use super::fft_recursive::{fft, Complex};
use crate::outln;

/// In-place d-dimensional DFT of a row-major array with shape `dims` (every dimension a power of two).
pub fn fft_nd(data: &mut [Complex], dims: &[usize], invert: bool) {
    assert_eq!(data.len(), dims.iter().product::<usize>());
    let mut stride = 1usize;
    for &len in dims.iter().rev() { // axis with the given stride: elements stride apart form one line
        let outer = data.len() / (len * stride);
        for o in 0..outer {
            for inner in 0..stride {
                let base = o * len * stride + inner;
                let line: Vec<Complex> = (0..len).map(|k| data[base + k * stride]).collect();
                for (k, v) in fft(&line, invert).into_iter().enumerate() { data[base + k * stride] = v; }
            }
        }
        stride *= len;
    }
}

/// Product of two bivariate polynomials A(x, y) = Σ a[i][j] xⁱ yʲ given as coefficient matrices, via the 2-D FFT.
pub fn multiply_bivariate(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let (ra, ca, rb, cb) = (a.len(), a[0].len(), b.len(), b[0].len());
    let (rows, cols) = ((ra + rb - 1).next_power_of_two(), (ca + cb - 1).next_power_of_two());
    let embed = |m: &[Vec<i64>]| { let mut d = vec![Complex::ZERO; rows * cols]; for (i, row) in m.iter().enumerate() { for (j, &v) in row.iter().enumerate() { d[i * cols + j] = Complex::new(v as f64, 0.0); } } d };
    let (mut fa, mut fb) = (embed(a), embed(b));
    fft_nd(&mut fa, &[rows, cols], false);
    fft_nd(&mut fb, &[rows, cols], false);
    for i in 0..fa.len() { fa[i] = fa[i] * fb[i]; }
    fft_nd(&mut fa, &[rows, cols], true);
    (0..ra + rb - 1).map(|i| (0..ca + cb - 1).map(|j| fa[i * cols + j].re.round() as i64).collect()).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = vec![vec![1, 2], vec![3, 4]]; // 1 + 2y + 3x + 4xy
    let b = vec![vec![5, 6], vec![7, 8]];
    outln!(o, "(1 + 2y + 3x + 4xy)(5 + 6y + 7x + 8xy) coefficient matrix = {:?}", multiply_bivariate(&a, &b));
    let mut d: Vec<Complex> = (0..8).map(|i| Complex::new(i as f64, 0.0)).collect();
    fft_nd(&mut d, &[2, 2, 2], false);
    outln!(o, "3-D DFT of 0..7 (a 2×2×2 cube): DC term {:.1}, others {:?}", d[0].re, d[1..].iter().map(|c| (c.re, c.im)).collect::<Vec<_>>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn two_d_transform_matches_the_double_sum() {
        let mut r = Rng::new(464);
        let (m, n) = (4usize, 8usize);
        let data: Vec<Complex> = (0..m * n).map(|_| Complex::new(r.f64(), r.f64())).collect();
        let mut got = data.clone();
        fft_nd(&mut got, &[m, n], false);
        for u in 0..m { for v in 0..n {
            let mut s = Complex::ZERO;
            for i in 0..m { for j in 0..n { s = s + data[i * n + j] * Complex::cis(2.0 * std::f64::consts::PI * (u * i) as f64 / m as f64 + 2.0 * std::f64::consts::PI * (v * j) as f64 / n as f64); } }
            assert!((got[u * n + v] - s).abs() < 1e-9, "({u},{v})");
        } }
    }
    #[test]
    fn roundtrip_in_three_dimensions() {
        let mut r = Rng::new(465);
        let data: Vec<Complex> = (0..4 * 2 * 8).map(|_| Complex::new(r.f64(), r.f64())).collect();
        let mut x = data.clone();
        fft_nd(&mut x, &[4, 2, 8], false);
        fft_nd(&mut x, &[4, 2, 8], true);
        for (p, q) in x.iter().zip(&data) { assert!((*p - *q).abs() < 1e-9); }
    }
    #[test]
    fn bivariate_product_matches_schoolbook() {
        let mut r = Rng::new(466);
        for _ in 0..50 {
            let (ra, ca, rb, cb) = (r.range(1, 5), r.range(1, 5), r.range(1, 5), r.range(1, 5));
            let a: Vec<Vec<i64>> = (0..ra).map(|_| r.vec(ca, -9, 9)).collect();
            let b: Vec<Vec<i64>> = (0..rb).map(|_| r.vec(cb, -9, 9)).collect();
            let mut want = vec![vec![0i64; ca + cb - 1]; ra + rb - 1];
            for i in 0..ra { for j in 0..ca { for k in 0..rb { for l in 0..cb { want[i + k][j + l] += a[i][j] * b[k][l]; } } } }
            assert_eq!(multiply_bivariate(&a, &b), want);
        }
    }
}
