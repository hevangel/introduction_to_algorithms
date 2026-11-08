//! @title Multithreaded Matrix Algorithms: Transpose and LU Decomposition
//! @chapter Problem 27-3 (3e)
//! @editions 3,4
//! @summary P-TRANSPOSE swaps blocks recursively (span Θ(lg n)); recursive block LU, LUP (with pivoting) and Cholesky factorisations use parallel triangular solves and Schur complements — the structure behind LAPACK-style blocked factorisations.
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

/// Recursive LUP decomposition with partial pivoting (Problem 27-3, LUP part) of an m×n panel (m ≥ n): returns (L m×n unit lower-trapezoidal,
/// U n×n upper triangular, p) with row i of P·A equal to row p[i] of A and P·A = L·U. The left half of the columns is factored recursively,
/// the permutation is applied to the right half, U12 = L11⁻¹A12 (parallel over columns), the Schur complement A22 − L21U12 is formed
/// (parallel matrix product) and factored recursively. None if a pivot column is zero (singular).
pub fn lup_panel(a: &Matrix) -> Option<(Matrix, Matrix, Vec<usize>)> {
    let (m, n) = (a.len(), a[0].len());
    if n == 1 {
        let piv = (0..m).max_by(|&i, &j| a[i][0].abs().partial_cmp(&a[j][0].abs()).unwrap()).unwrap();
        if a[piv][0].abs() < 1e-12 { return None; }
        let mut p: Vec<usize> = (0..m).collect();
        p.swap(0, piv);
        let rows: Vec<f64> = p.iter().map(|&i| a[i][0]).collect();
        let l: Matrix = (0..m).map(|i| vec![if i == 0 { 1.0 } else { rows[i] / rows[0] }]).collect();
        return Some((l, vec![vec![rows[0]]], p));
    }
    let h = n / 2;
    let left: Matrix = a.iter().map(|r| r[..h].to_vec()).collect();
    let (l1, u11, p1) = lup_panel(&left)?;                                            // factor the left panel (m × h)
    let right: Matrix = p1.iter().map(|&i| a[i][h..].to_vec()).collect();               // apply its row permutation to the right half
    let mut u12 = vec![vec![0.0; n - h]; h];                                           // U12 = L11⁻¹ · A12 (forward substitution per column)
    for j in 0..n - h { for i in 0..h { u12[i][j] = right[i][j] - (0..i).map(|k| l1[i][k] * u12[k][j]).sum::<f64>(); } }
    let s: Matrix = (0..m - h).map(|i| (0..n - h).map(|j| right[h + i][j] - (0..h).map(|k| l1[h + i][k] * u12[k][j]).sum::<f64>()).collect()).collect();
    let (l2, u22, p2) = lup_panel(&s)?;                                                // factor the Schur complement
    let mut l = vec![vec![0.0; n]; m];
    for i in 0..h { for j in 0..h { l[i][j] = l1[i][j]; } }
    for i in 0..m - h { for j in 0..h { l[h + i][j] = l1[h + p2[i]][j]; } for j in 0..n - h { l[h + i][h + j] = l2[i][j]; } }
    let mut u = vec![vec![0.0; n]; n];
    for i in 0..h { for j in 0..h { u[i][j] = u11[i][j]; } for j in 0..n - h { u[i][h + j] = u12[i][j]; } }
    for i in 0..n - h { for j in 0..n - h { u[h + i][h + j] = u22[i][j]; } }
    let mut p = p1.clone();
    for i in 0..m - h { p[h + i] = p1[h + p2[i]]; }
    Some((l, u, p))
}

/// LUP decomposition of a square matrix.
pub fn lup_recursive(a: &Matrix) -> Option<(Matrix, Matrix, Vec<usize>)> { lup_panel(a) }

/// Recursive Cholesky factorisation A = L·Lᵀ (Problem 27-3, Cholesky part): factor A11, get L21 = A21·L11⁻ᵀ by independent row solves,
/// and recurse on the Schur complement A22 − L21·L21ᵀ. None if A is not symmetric positive-definite.
pub fn cholesky_recursive(a: &Matrix) -> Option<Matrix> {
    let n = a.len();
    if n == 1 { return (a[0][0] > 1e-12).then(|| vec![vec![a[0][0].sqrt()]]); }
    let h = n / 2;
    let a11: Matrix = (0..h).map(|i| a[i][..h].to_vec()).collect();
    let l11 = cholesky_recursive(&a11)?;
    let mut l21 = vec![vec![0.0; h]; n - h]; // each row solves  L11 · x = (row of A21)ᵀ  by forward substitution
    for i in 0..n - h { for j in 0..h { l21[i][j] = (a[h + i][j] - (0..j).map(|k| l21[i][k] * l11[j][k]).sum::<f64>()) / l11[j][j]; } }
    let s: Matrix = (0..n - h).map(|i| (0..n - h).map(|j| a[h + i][h + j] - (0..h).map(|k| l21[i][k] * l21[j][k]).sum::<f64>()).collect()).collect();
    let l22 = cholesky_recursive(&s)?;
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..h { for j in 0..h { l[i][j] = l11[i][j]; } }
    for i in 0..n - h { for j in 0..h { l[h + i][j] = l21[i][j]; } for j in 0..n - h { l[h + i][h + j] = l22[i][j]; } }
    Some(l)
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
    let (lp, up, perm) = lup_recursive(&a).unwrap();
    let pa: Matrix = perm.iter().map(|&i| a[i].clone()).collect();
    let lu = mul(&lp, &up);
    outln!(o, "LUP: permutation {:?}, max |PA − LU| = {:.1e}", perm, (0..4).flat_map(|i| (0..4).map(move |j| (i, j))).map(|(i, j)| (pa[i][j] - lu[i][j]).abs()).fold(0.0, f64::max));
    let spd: Matrix = vec![vec![4.0, 12.0, -16.0], vec![12.0, 37.0, -43.0], vec![-16.0, -43.0, 98.0]];
    outln!(o, "recursive Cholesky of the classic 3×3 matrix: {:?}", cholesky_recursive(&spd).unwrap().iter().map(|r| r.iter().map(|v| (v * 1000.0).round() / 1000.0).collect::<Vec<_>>()).collect::<Vec<_>>());
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
    fn lup_with_pivoting_handles_matrices_that_plain_lu_cannot() {
        let mut r = Rng::new(503);
        for _ in 0..150 {
            let n = r.range(1, 12);
            let a: Matrix = (0..n).map(|_| (0..n).map(|_| r.f64() * 2.0 - 1.0).collect()).collect(); // no diagonal dominance: pivoting is essential
            let Some((l, u, p)) = lup_recursive(&a) else { continue };
            let pa: Matrix = p.iter().map(|&i| a[i].clone()).collect();
            let lu = mul(&l, &u);
            let mut seen = p.clone();
            seen.sort();
            assert_eq!(seen, (0..n).collect::<Vec<_>>(), "p is a permutation");
            for i in 0..n { for j in 0..n {
                assert!((lu[i][j] - pa[i][j]).abs() < 1e-8, "n = {n}: PA ≠ LU at ({i},{j})");
                if j > i { assert_eq!(l[i][j], 0.0); } else if j < i { assert_eq!(u[i][j], 0.0); assert!(l[i][j].abs() <= 1.0 + 1e-9, "partial pivoting keeps |l| ≤ 1 within a panel"); }
            } }
        }
        // a zero in the (0,0) position: plain LU fails, LUP swaps rows
        let a: Matrix = vec![vec![0.0, 1.0], vec![2.0, 3.0]];
        let (l, u, p) = lup_recursive(&a).unwrap();
        assert_eq!(p, vec![1, 0]);
        assert!((mul(&l, &u)[0][1] - 3.0).abs() < 1e-12);
        assert!(lup_recursive(&vec![vec![1.0, 2.0], vec![2.0, 4.0]]).is_none(), "singular matrices are reported");
    }
    #[test]
    fn recursive_cholesky_matches_the_sequential_one() {
        let mut r = Rng::new(504);
        for _ in 0..100 {
            let n = r.range(1, 10);
            let b: Matrix = (0..n).map(|_| (0..n).map(|_| r.f64() * 2.0 - 1.0).collect()).collect();
            let mut a: Matrix = (0..n).map(|i| (0..n).map(|j| (0..n).map(|k| b[k][i] * b[k][j]).sum()).collect()).collect(); // BᵀB + I is SPD
            for i in 0..n { a[i][i] += 1.0; }
            let l = cholesky_recursive(&a).expect("SPD");
            let want = crate::algorithms::linear_algebra::cholesky::cholesky(&a).unwrap();
            for i in 0..n { for j in 0..n { assert!((l[i][j] - want[i][j]).abs() < 1e-8); } }
        }
        assert!(cholesky_recursive(&vec![vec![1.0, 2.0], vec![2.0, 1.0]]).is_none());
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
