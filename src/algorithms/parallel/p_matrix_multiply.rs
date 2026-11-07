//! @title Parallel Matrix Multiplication
//! @chapter 27.2 (3e) / 26.2 (4e)
//! @editions 3,4
//! @summary P-MAT-VEC and P-SQUARE-MATRIX-MULTIPLY with parallel loops; recursive divide-and-conquer with spawn.
//! @order 20
use crate::outln;
use std::thread;

pub type Matrix = Vec<Vec<i64>>;

fn workers(n: usize) -> usize { thread::available_parallelism().map_or(2, |x| x.get()).clamp(2, 8).min(n.max(1)) }

fn serial(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let mut c = vec![vec![0; n]; n];
    for i in 0..n { for k in 0..n { for j in 0..n { c[i][j] += a[i][k] * b[k][j]; } } }
    c
}

/// P-MAT-VEC: y = A·x with the outer loop run in parallel (Θ(n²) work, Θ(lg n) span with ideal parallel-for).
pub fn p_mat_vec(a: &Matrix, x: &[i64]) -> Vec<i64> {
    let mut y = vec![0i64; a.len()];
    let chunk = y.len().div_ceil(workers(y.len()));
    thread::scope(|s| {
        for (ci, ys) in y.chunks_mut(chunk.max(1)).enumerate() {
            s.spawn(move || {
                for (k, yi) in ys.iter_mut().enumerate() { *yi = a[ci * chunk + k].iter().zip(x).map(|(p, q)| p * q).sum(); }
            });
        }
    });
    y
}

/// P-SQUARE-MATRIX-MULTIPLY: `parallel for` over rows (each row of C is independent, so no races).
pub fn p_matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let mut c = vec![vec![0i64; n]; n];
    let chunk = n.div_ceil(workers(n)).max(1);
    thread::scope(|s| {
        for (ci, rows) in c.chunks_mut(chunk).enumerate() {
            s.spawn(move || {
                for (k, row) in rows.iter_mut().enumerate() {
                    let i = ci * chunk + k;
                    for kk in 0..n { for j in 0..n { row[j] += a[i][kk] * b[kk][j]; } }
                }
            });
        }
    });
    c
}

fn block(m: &Matrix, r: usize, c: usize, h: usize) -> Matrix { (0..h).map(|i| m[r + i][c..c + h].to_vec()).collect() }

/// P-MATRIX-MULTIPLY-RECURSIVE for n a power of two: the 8 half-size products run as parallel tasks (span Θ(lg² n)).
pub fn p_matrix_multiply_recursive(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    if n <= 32 { return serial(a, b); }
    let h = n / 2;
    let q = |m: &Matrix| [block(m, 0, 0, h), block(m, 0, h, h), block(m, h, 0, h), block(m, h, h, h)];
    let (aq, bq) = (q(a), q(b));
    // C_ij = A_i1·B_1j + A_i2·B_2j
    let prods: Vec<Matrix> = thread::scope(|s| {
        let handles: Vec<_> = (0..8).map(|t| {
            let (i, j, k) = (t / 4, (t / 2) % 2, t % 2);
            let (aa, bb) = (&aq[i * 2 + k], &bq[k * 2 + j]);
            s.spawn(move || p_matrix_multiply_recursive(aa, bb))
        }).collect();
        handles.into_iter().map(|x| x.join().unwrap()).collect()
    });
    let mut c = vec![vec![0i64; n]; n];
    for t in 0..8 {
        let (i, j) = (t / 4, (t / 2) % 2);
        for r in 0..h { for col in 0..h { c[i * h + r][j * h + col] += prods[t][r][col]; } }
    }
    c
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut rng = crate::common::Rng::new(5);
    let n = 64;
    let a: Matrix = (0..n).map(|_| rng.vec(n, -9, 9)).collect();
    let b: Matrix = (0..n).map(|_| rng.vec(n, -9, 9)).collect();
    let want = serial(&a, &b);
    outln!(o, "n = {n}: parallel-for result equals serial: {}", p_matrix_multiply(&a, &b) == want);
    outln!(o, "n = {n}: recursive spawn result equals serial: {}", p_matrix_multiply_recursive(&a, &b) == want);
    let x = rng.vec(n, -5, 5);
    outln!(o, "P-MAT-VEC first entries: {:?}", &p_mat_vec(&a, &x)[..4]);
    outln!(o, "work Θ(n³); span Θ(n) for the parallel-for version (inner loops serial), Θ(lg n) with parallel inner loops");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn all_versions_agree() {
        let mut r = Rng::new(150);
        for n in [1usize, 3, 8, 33, 64] {
            let a: Matrix = (0..n).map(|_| r.vec(n, -9, 9)).collect();
            let b: Matrix = (0..n).map(|_| r.vec(n, -9, 9)).collect();
            assert_eq!(p_matrix_multiply(&a, &b), serial(&a, &b), "n={n}");
            if n.is_power_of_two() { assert_eq!(p_matrix_multiply_recursive(&a, &b), serial(&a, &b), "n={n}"); }
        }
    }
    #[test]
    fn mat_vec() {
        let mut r = Rng::new(151);
        let n = 37;
        let a: Matrix = (0..n).map(|_| r.vec(n, -9, 9)).collect();
        let x = r.vec(n, -9, 9);
        let want: Vec<i64> = a.iter().map(|row| row.iter().zip(&x).map(|(p, q)| p * q).sum()).collect();
        assert_eq!(p_mat_vec(&a, &x), want);
    }
}
