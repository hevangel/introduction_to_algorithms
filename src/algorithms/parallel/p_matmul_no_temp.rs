//! @title Saving Temporary Space in Matrix Multiplication
//! @chapter Problem 27-2 (3e)
//! @editions 3,4
//! @summary Multiply without the temporary matrix of P-MATRIX-MULTIPLY-RECURSIVE by adding the eight sub-products straight into C in two rounds of four: the span grows from Θ(lg² n) to Θ(n) but no extra storage is needed.
//! @order 25
use crate::outln;

pub type Matrix = Vec<Vec<i64>>;

/// C += A·B for n×n blocks (n a power of two) given by their top-left corners. Four independent products update the four quadrants of C, then the
/// other four products add their halves: since two products never write the same quadrant at the same time, no temporary is needed.
/// Returns the span in units of scalar multiply-adds.
fn mm(c: &mut Matrix, a: &Matrix, b: &Matrix, (cr, cc): (usize, usize), (ar, ac): (usize, usize), (br, bc): (usize, usize), n: usize) -> usize {
    if n == 1 { c[cr][cc] += a[ar][ac] * b[br][bc]; return 1; }
    let h = n / 2;
    let mut span = 0;
    for round in 0..2 { // round 0: A_i1·B_1j into C_ij; round 1: A_i2·B_2j into C_ij
        let mut round_span = 0;
        for i in 0..2 { for j in 0..2 { // the four products of a round touch four different quadrants of C: they could run in parallel
            let k = round;
            round_span = round_span.max(mm(c, a, b, (cr + i * h, cc + j * h), (ar + i * h, ac + k * h), (br + k * h, bc + j * h), h));
        } }
        span += round_span;
    }
    span
}

/// C = A·B without temporary storage. Returns (C, span). Work is Θ(n³); span T(n) = 2T(n/2) + Θ(1) = Θ(n).
pub fn multiply_no_temp(a: &Matrix, b: &Matrix) -> (Matrix, usize) {
    let n = a.len();
    let mut c = vec![vec![0i64; n]; n];
    let span = mm(&mut c, a, b, (0, 0), (0, 0), (0, 0), n);
    (c, span)
}

/// Span of the version that uses a temporary matrix (all eight products in parallel, then a parallel matrix addition): Θ(lg² n).
pub fn span_with_temp(n: usize) -> usize { let l = n.ilog2() as usize; l * (l + 1) / 2 + 1 }

pub fn naive(a: &Matrix, b: &Matrix) -> Matrix { let n = a.len(); (0..n).map(|i| (0..n).map(|j| (0..n).map(|k| a[i][k] * b[k][j]).sum()).collect()).collect() }

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = crate::common::Rng::new(27);
    outln!(o, "   n | span without temporary | span with temporary (lg² n) | extra space with temporary");
    for n in [2usize, 8, 32, 128] {
        let a: Matrix = (0..n).map(|_| r.vec(n, -5, 5)).collect();
        let b: Matrix = (0..n).map(|_| r.vec(n, -5, 5)).collect();
        let (c, span) = multiply_no_temp(&a, &b);
        assert_eq!(c, naive(&a, &b));
        outln!(o, "{n:>4} | {span:>22} | {:>27} | Θ(n²) vs 0", span_with_temp(n));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn equals_the_naive_product() {
        let mut r = Rng::new(502);
        for k in 0..6u32 {
            let n = 1usize << k;
            let a: Matrix = (0..n).map(|_| r.vec(n, -9, 9)).collect();
            let b: Matrix = (0..n).map(|_| r.vec(n, -9, 9)).collect();
            assert_eq!(multiply_no_temp(&a, &b).0, naive(&a, &b), "n = {n}");
        }
    }
    #[test]
    fn span_is_linear_without_the_temporary() {
        for k in 1..8u32 {
            let n = 1usize << k;
            let id: Matrix = (0..n).map(|i| (0..n).map(|j| (i == j) as i64).collect()).collect();
            assert_eq!(multiply_no_temp(&id, &id).1, n, "T(n) = 2T(n/2) + 1 gives exactly n for this accounting");
        }
        assert!(span_with_temp(1 << 7) < 1 << 7);
    }
}
