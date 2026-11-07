//! @title Parallel Matrix-Vector Product (P-MAT-VEC)
//! @chapter 27.1 (3e) / 26.1 (4e)
//! @editions 3,4
//! @summary A parallel for over rows gives work Θ(n²) and span Θ(lg n + n); making the inner sum a parallel reduction brings span to Θ(lg n). Includes a determinacy-race demonstration.
//! @order 12
use crate::outln;
use std::thread;

/// P-MAT-VEC: every row is an independent strand (`parallel for`), executed here by `workers` threads over contiguous row blocks.
pub fn p_mat_vec(a: &[Vec<f64>], x: &[f64], workers: usize) -> Vec<f64> {
    let n = a.len();
    let mut y = vec![0.0; n];
    let chunk = n.div_ceil(workers.max(1)).max(1);
    thread::scope(|s| {
        for (ci, ys) in y.chunks_mut(chunk).enumerate() {
            s.spawn(move || { for (k, out) in ys.iter_mut().enumerate() { *out = a[ci * chunk + k].iter().zip(x).map(|(p, q)| p * q).sum(); } });
        }
    });
    y
}

/// Work and span (in unit operations) of P-MAT-VEC for an n×n matrix:
/// recursive halving of the parallel for costs lg n, each row is a serial dot product of n steps. Returns (work, span).
pub fn work_span_serial_rows(n: usize) -> (usize, usize) { (n * n, n + n.next_power_of_two().ilog2() as usize) }
/// Same with each dot product computed as a parallel reduction tree: span lg n + lg n.
pub fn work_span_parallel_rows(n: usize) -> (usize, usize) { (n * n, 2 * n.next_power_of_two().ilog2() as usize + 1) }

/// The racy "P-MAT-VEC-RACE" of Exercise 27.1: the inner loop is also parallel and every strand updates y[i] — a determinacy race.
/// We replay one possible interleaving of the two-step read/modify/write to show a lost update.
pub fn racy_accumulate(contrib: &[f64]) -> f64 {
    // each strand reads y, adds its contribution, writes back; strands run "simultaneously": all read 0 first
    let reads: Vec<f64> = contrib.iter().map(|_| 0.0).collect();
    let writes: Vec<f64> = reads.iter().zip(contrib).map(|(r, c)| r + c).collect();
    *writes.last().unwrap_or(&0.0) // the last write wins; the other updates are lost
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 10.0]];
    outln!(o, "A·(1,1,1) with 2 workers = {:?}", p_mat_vec(&a, &[1.0, 1.0, 1.0], 2));
    for n in [8usize, 64, 1024] {
        let ((w, s1), (_, s2)) = (work_span_serial_rows(n), work_span_parallel_rows(n));
        outln!(o, "n = {n:>5}: work {w:>8}; span {s1:>5} (serial dot products, parallelism {:.1}) / {s2} (tree reduction, parallelism {:.1})", w as f64 / s1 as f64, w as f64 / s2 as f64);
    }
    outln!(o, "racy version: contributions [1,2,3,4] accumulate to {} instead of 10", racy_accumulate(&[1.0, 2.0, 3.0, 4.0]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_serial_product_for_any_worker_count() {
        let mut r = Rng::new(430);
        for _ in 0..50 {
            let n = r.range(1, 30);
            let a: Vec<Vec<f64>> = (0..n).map(|_| (0..n).map(|_| r.f64()).collect()).collect();
            let x: Vec<f64> = (0..n).map(|_| r.f64()).collect();
            let want: Vec<f64> = a.iter().map(|row| row.iter().zip(&x).map(|(p, q)| p * q).sum()).collect();
            for w in [1, 2, 3, 8, 100] { assert_eq!(p_mat_vec(&a, &x, w), want); }
        }
    }
    #[test]
    fn span_formulas_and_race() {
        assert_eq!(work_span_serial_rows(8), (64, 11));
        assert!(work_span_parallel_rows(1024).1 < work_span_serial_rows(1024).1);
        assert_eq!(racy_accumulate(&[1.0, 2.0, 3.0, 4.0]), 4.0);
    }
}
