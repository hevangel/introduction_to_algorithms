//! @title Multithreading a Simple Stencil Calculation
//! @chapter Problem 27-5 (3e)
//! @editions 3,4
//! @summary Each A[i][j] depends on its left, upper and upper-left neighbours: split the array into b×b blocks processed in 2b−1 wavefront stages, recursively — span Θ(n^(log_b(2b−1))), approaching Θ(n^(1+ε)) as b grows.
//! @order 18
use crate::outln;

pub type Grid = Vec<Vec<u64>>;

/// The local rule; any function of the three neighbours works (here a mixing function so that wrong evaluation orders are detected).
pub fn rule(up: u64, left: u64, diag: u64) -> u64 { (up.wrapping_mul(31).wrapping_add(left.wrapping_mul(17)).wrapping_add(diag.wrapping_mul(7)).wrapping_add(1)) % 1_000_003 }

/// Serial reference: boundary row and column given as the first row/column of `a`, the rest computed row by row.
pub fn serial(a: &mut Grid) { let n = a.len(); for i in 1..n { for j in 1..n { a[i][j] = rule(a[i - 1][j], a[i][j - 1], a[i - 1][j - 1]); } } }

/// Recursive blocked evaluation of the rectangular region [r, r+h) × [c, c+w) (the first row/column of the whole array are inputs).
/// The region is cut into at most b×b sub-blocks; the blocks on one anti-diagonal are independent, so they could run in parallel: 2b−1 stages.
/// Returns the span in units of one cell update.
fn blocked(a: &mut Grid, r: usize, c: usize, h: usize, w: usize, b: usize) -> usize {
    if (h <= 1 && w <= 1) || b < 2 {
        let mut span = 0;
        for i in r..r + h { for j in c..c + w { if i > 0 && j > 0 { a[i][j] = rule(a[i - 1][j], a[i][j - 1], a[i - 1][j - 1]); span += 1; } } }
        return span;
    }
    let (sr, sc) = (h.div_ceil(b), w.div_ceil(b));
    let (br, bc) = (h.div_ceil(sr), w.div_ceil(sc));
    let mut span = 0;
    for diag in 0..br + bc - 1 { // wavefront stage: all blocks (bi, bj) with bi + bj = diag are independent of one another
        let mut stage_span = 0;
        for bi in 0..br { for bj in 0..bc {
            if bi + bj != diag { continue; }
            let (rr, cc) = (r + bi * sr, c + bj * sc);
            stage_span = stage_span.max(blocked(a, rr, cc, sr.min(r + h - rr), sc.min(c + w - cc), b));
        } }
        span += stage_span;
    }
    span
}

/// Evaluate an n×n array (n a power of b) with the b×b recursive wavefront scheme. Returns the span.
pub fn p_stencil(a: &mut Grid, b: usize) -> usize { let n = a.len(); blocked(a, 0, 0, n, n, b) }

pub fn demo() -> String {
    let mut o = String::new();
    let init = |n: usize| -> Grid { (0..n).map(|i| (0..n).map(|j| if i == 0 || j == 0 { (i * 131 + j * 17 + 5) as u64 } else { 0 }).collect()).collect() };
    outln!(o, "   n |  b | span of the blocked wavefront | n^(log_b(2b−1)) | serial span n²");
    for (n, b) in [(16usize, 2usize), (64, 2), (64, 4), (81, 3), (256, 2), (256, 4), (256, 16)] {
        let mut a = init(n);
        let span = p_stencil(&mut a, b);
        outln!(o, "{n:>4} | {b:>2} | {span:>29} | {:>15.0} | {}", (n as f64).powf(((2 * b - 1) as f64).ln() / (b as f64).ln()), n * n);
    }
    outln!(o, "as b grows the exponent log_b(2b−1) → 1: the span approaches Θ(n)");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    fn init(n: usize) -> Grid { (0..n).map(|i| (0..n).map(|j| if i == 0 || j == 0 { (i * 131 + j * 17 + 5) as u64 } else { 0 }).collect()).collect() }
    #[test]
    fn blocked_evaluation_equals_the_serial_stencil() {
        for (n, b) in [(1usize, 2usize), (2, 2), (4, 2), (9, 3), (16, 2), (16, 4), (25, 5), (27, 3), (30, 4), (33, 7)] {
            let (mut s, mut p) = (init(n), init(n));
            serial(&mut s);
            p_stencil(&mut p, b);
            assert_eq!(s, p, "n = {n}, b = {b}");
        }
    }
    #[test]
    fn span_is_n_to_the_log_b_of_2b_minus_1() {
        for (b, k) in [(2usize, 6u32), (3, 4), (4, 3), (5, 3)] {
            let n = b.pow(k);
            let mut a = init(n);
            let span = p_stencil(&mut a, b);
            // the interior of the first row/column is input, so the count is slightly below (2b−1)^k; it never exceeds it
            assert!(span <= (2 * b - 1).pow(k), "b = {b}: span {span}");
            assert!(span as f64 >= 0.5 * ((2 * b - 1).pow(k)) as f64, "b = {b}: span {span}");
        }
        let (mut a, mut c) = (init(256), init(256));
        assert!(p_stencil(&mut a, 16) < p_stencil(&mut c, 2), "larger blocks give a shorter span");
    }
}
