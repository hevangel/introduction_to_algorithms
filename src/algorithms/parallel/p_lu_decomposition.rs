//! @title Multithreaded Matrix Algorithms: Transpose and LU Decomposition
//! @chapter Problem 27-3 (3e)
//! @editions 3,4
//! @summary P-TRANSPOSE swaps blocks recursively (span Θ(lg n)); a recursive block LU decomposition works on quadrants with parallel triangular solves — the parallel structure behind LAPACK-style blocked factorisations.
//! @order 22
use crate::outln;

pub type Matrix = Vec<Vec<f64>>;

/// In-place transpose of a square matrix whose side is a power of two, by recursion: transpose A11 and A22, and exchange A12 with Aᵀ21.
/// The four sub-exchanges (and the two sub-transposes) are independent, so with parallel strands the span is Θ(lg n). Returns the recursion depth.
pub fn p_transpose(a: &mut Matrix) -> usize { let n = a.len(); transpose_block(a, 0, 0, n) }

fn transpose_block(a: &mut Matrix, r: usize, c: usize, n: usize) -> usize {
    if n <= 1 { return 0; }
    let h = n / 2;
    let d1 = transpose_block(a, r, c, h);          // A11 ┐ these two are independent (spawn)
    let d2 = transpose_block(a, r + h, c + h, h);  // A22 ┘
    let d3 = swap_transpose(a, (r, c + h), (r + h, c), h); // A12 ↔ A21ᵀ
    1 + d1.max(d2).max(d3)
}

/// Exchange block p with the transpose of block q (both h×h): four independent sub-exchanges.
fn swap_transpose(a: &mut Matrix, p: (usize, usize), q: (usize, usize), h: usize) -> usize {
    if h == 1 { let t = a[p.0][p.1]; a[p.0][p.1] = a[q.0][q.1]; a[q.0][q.1] = t; return 0; }
    let k = h / 2;
    let ds = [
        swap_transpose(a, (p.0, p.1), (q.0, q.1), k),
        swap_transpose(a, (p.0, p.1 + k), (q.0 + k, q.1), k),
        swap_transpose(a, (p.0 + k, p.1), (q.0, q.1 + k), k),
        swap_transpose(a, (p.0 + k, p.1 + k), (q.0 + k, q.1 + k), k),
    ];
    1 + ds.into_iter().max().unwrap()
}

/// Recursive LU decomposition without pivoting (A = L·U, L unit lower triangular), by quadrants:
/// A11 = L11U11 (recurse); U12 = L11⁻¹A12 and L21 = A21U11⁻¹ (independent triangular solves, parallel over columns/rows);
/// then recurse on the Schur complement S = A22 − L21U12. Requires non-singular leading minors (e.g. diagonally dominant A).
pub fn lu_recursive(a: &Matrix) -> (Matrix, Matrix) {
    let n = a.len();
    if n == 1 { return (vec![vec![1.0]], a.clone()); }
    let h = n / 2;
    let sub = |r0: usize, r1: usize, c0: usize, c1: usize| -> Matrix { (r0..r1).map(|i| a[i][c0..c1].to_vec()).collect() };
    let (a11, a12, a21, a22) = (sub(0, h, 0, h), sub(0, h, h, n), sub(h, n, 0, h), sub(h, n, h, n));
    let (l11, u11) = lu_recursive(&a11);
    // U12: solve L11·X = A12 column by column (forward substitution, unit diagonal)
    let mut u12 = vec![vec![0.0; n - h]; h];
    for j in 0..n - h { for i in 0..h { u12[i][j] = a12[i][j] - (0..i).map(|k| l11[i][k] * u12[k][j]).sum::<f64>(); } }
    // L21: solve Y·U11 = A21 row by row (back substitution on the right)
    let mut l21 = vec![vec![0.0; h]; n - h];
    for i in 0..n - h { for j in 0..h { l21[i][j] = (a21[i][j] - (0..j).map(|k| l21[i][k] * u11[k][j]).sum::<f64>()) / u11[j][j]; } }
    let s: Matrix = (0..n - h).map(|i| (0..n - h).map(|j| a22[i][j] - (0..h).map(|k| l21[i][k] * u12[k][j]).sum::<f64>()).collect()).collect();
    let (l22, u22) = lu_recursive(&s);
    let (mut l, mut u) = (vec![vec![0.0; n]; n], vec![vec![0.0; n]; n]);
    for i in 0..h { for j in 0..h { l[i][j] = l11[i][j]; u[i][j] = u11[i][j]; } for j in 0..n - h { u[i][h + j] = u12[i][j]; } }
    for i in 0..n - h { for j in 0..h { l[h + i][j] = l21[i][j]; } for j in 0..n - h { l[h + i][h + j] = l22[i][j]; u[h + i][h + j] = u22[i][j]; } }
    (l, u)
}

pub fn mul(a: &Matrix, b: &Matrix) -> Matrix { let n = a.len(); (0..n).map(|i| (0..n).map(|j| (0..n).map(|k| a[i][k] * b[k][j]).sum()).collect()).collect() }

pub fn demo() -> String {
    let mut o = String::new();
    let mut m: Matrix = (0..4).map(|i| (0..4).map(|j| (4 * i + j) as f64).collect()).collect();
    let depth = p_transpose(&mut m);
    outln!(o, "transpose of 0..15 as 4×4 (recursion depth {depth}): {:?}", m);
    let a: Matrix = vec![vec![4.0, 3.0, 2.0, 1.0], vec![3.0, 5.0, 1.0, 2.0], vec![2.0, 1.0, 6.0, 3.0], vec![1.0, 2.0, 3.0, 7.0]];
    let (l, u) = lu_recursive(&a);
    outln!(o, "L = {:?}", l.iter().map(|r| r.iter().map(|v| (v * 1000.0).round() / 1000.0).collect::<Vec<_>>()).collect::<Vec<_>>());
    outln!(o, "U = {:?}", u.iter().map(|r| r.iter().map(|v| (v * 1000.0).round() / 1000.0).collect::<Vec<_>>()).collect::<Vec<_>>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn transposes_every_power_of_two_size_with_logarithmic_depth() {
        for k in 0..6u32 {
            let n = 1usize << k;
            let orig: Matrix = (0..n).map(|i| (0..n).map(|j| (i * 100 + j) as f64).collect()).collect();
            let mut m = orig.clone();
            let depth = p_transpose(&mut m);
            for i in 0..n { for j in 0..n { assert_eq!(m[i][j], orig[j][i]); } }
            assert!(depth <= 2 * k as usize + 2, "n = {n}: depth {depth}");
        }
    }
    #[test]
    fn recursive_lu_reconstructs_diagonally_dominant_matrices_of_any_size() {
        let mut r = Rng::new(501);
        for _ in 0..100 {
            let n = r.range(1, 12);
            let mut a: Matrix = (0..n).map(|_| (0..n).map(|_| r.f64() * 2.0 - 1.0).collect()).collect();
            for i in 0..n { a[i][i] += n as f64 + 1.0; } // strict diagonal dominance: no pivoting needed
            let (l, u) = lu_recursive(&a);
            let lu = mul(&l, &u);
            for i in 0..n { for j in 0..n {
                assert!((lu[i][j] - a[i][j]).abs() < 1e-9, "n = {n}");
                if j > i { assert_eq!(l[i][j], 0.0); } else if j < i { assert_eq!(u[i][j], 0.0); }
            } assert!((l[i][i] - 1.0).abs() < 1e-12); }
        }
    }
}
