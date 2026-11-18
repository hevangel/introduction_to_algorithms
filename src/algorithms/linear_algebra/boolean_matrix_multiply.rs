//! @title Boolean Matrix Multiplication
//! @chapter 31.3 (1e)
//! @editions 1
//! @summary Multiply boolean matrices over the OR-AND quasiring by embedding into Z_{n+1} for Strassen's algorithm, plus Four-Russians bit-packing and triangle detection.
//! @order 25
use crate::algorithms::foundations::strassen::{strassen_pow2, Matrix};
use crate::outln;

/// Schoolbook Θ(n³) boolean matrix product `(A · B)[i][j] = OR_k (A[i][k] & B[k][j])`.
pub fn bool_matmul_naive(a: &[Vec<bool>], b: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let n = a.len();
    let mut c = vec![vec![false; n]; n];
    for i in 0..n {
        for k in 0..n {
            if a[i][k] {
                for j in 0..n {
                    c[i][j] |= b[k][j];
                }
            }
        }
    }
    c
}

/// 1e §31.3: Boolean OR-AND lacks additive inverses for Strassen directly; embed {0, 1} into Z,
/// compute integer witness counts via Strassen in O(n^{lg 7}), and threshold (C[i][j] > 0).
pub fn bool_matmul_strassen(a: &[Vec<bool>], b: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let n = a.len();
    if n == 0 {
        return vec![];
    }
    let m = n.next_power_of_two();
    let mut ai: Matrix = vec![vec![0; m]; m];
    let mut bi: Matrix = vec![vec![0; m]; m];
    for i in 0..n {
        for j in 0..n {
            ai[i][j] = a[i][j] as i64;
            bi[i][j] = b[i][j] as i64;
        }
    }
    let ci = strassen_pow2(&ai, &bi, 4);
    (0..n)
        .map(|i| (0..n).map(|j| ci[i][j] > 0).collect())
        .collect()
}

/// Word-packed / Four-Russians lookup-block multiplication over 64-bit chunks.
/// Groups columns of A (and rows of B) into `w`-bit blocks (`w <= 8`), precomputing all 2^w row OR-combinations of B.
pub fn bool_matmul_four_russians(a: &[Vec<bool>], b: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let n = a.len();
    if n == 0 {
        return vec![];
    }
    let words = n.div_ceil(64);
    let mut b_rows = vec![vec![0u64; words]; n];
    for i in 0..n {
        for j in 0..n {
            if b[i][j] {
                b_rows[i][j / 64] |= 1u64 << (j % 64);
            }
        }
    }

    let block_w = ((n as f64).log2().floor() as usize).clamp(1, 8);
    let mut c_words = vec![vec![0u64; words]; n];

    let mut k0 = 0;
    while k0 < n {
        let len = (n - k0).min(block_w);
        let table_size = 1usize << len;
        let mut table = vec![vec![0u64; words]; table_size];
        for mask in 1..table_size {
            let lsb = mask.trailing_zeros() as usize;
            let prev = mask ^ (1 << lsb);
            let row = &b_rows[k0 + lsb];
            for w in 0..words {
                table[mask][w] = table[prev][w] | row[w];
            }
        }
        for i in 0..n {
            let mut mask = 0usize;
            for bit in 0..len {
                if a[i][k0 + bit] {
                    mask |= 1usize << bit;
                }
            }
            if mask != 0 {
                for w in 0..words {
                    c_words[i][w] |= table[mask][w];
                }
            }
        }
        k0 += len;
    }

    (0..n)
        .map(|i| (0..n).map(|j| ((c_words[i][j / 64] >> (j % 64)) & 1) == 1).collect())
        .collect()
}

/// Exercise 31.3-6 (1e): Detect a triangle `(i, j, k)` in an undirected graph via `A^2 & A`.
pub fn has_triangle(adj: &[Vec<bool>]) -> Option<(usize, usize, usize)> {
    let n = adj.len();
    let a2 = bool_matmul_strassen(adj, adj);
    for i in 0..n {
        for j in (i + 1)..n {
            if adj[i][j] && a2[i][j] {
                for k in 0..n {
                    if k != i && k != j && adj[i][k] && adj[k][j] {
                        return Some((i, j, k));
                    }
                }
            }
        }
    }
    None
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = vec![
        vec![true, false, true, false],
        vec![false, true, true, false],
        vec![false, false, true, true],
        vec![true, false, false, false],
    ];
    let b = vec![
        vec![false, true, false, false],
        vec![true, false, false, true],
        vec![false, true, true, false],
        vec![true, false, true, false],
    ];
    let c_naive = bool_matmul_naive(&a, &b);
    let c_str = bool_matmul_strassen(&a, &b);
    let c_4r = bool_matmul_four_russians(&a, &b);
    outln!(o, "4x4 boolean product matches (naive == Strassen == Four Russians): {}", c_naive == c_str && c_naive == c_4r);
    let tri_adj = vec![
        vec![false, true, true, false],
        vec![true, false, true, false],
        vec![true, true, false, true],
        vec![false, false, true, false],
    ];
    outln!(o, "triangle detected via A^2 & A: {:?}", has_triangle(&tri_adj));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn strassen_and_four_russians_match_naive_across_sizes() {
        let mut r = Rng::new(313);
        for n in 1..=24 {
            let a: Vec<Vec<bool>> = (0..n).map(|_| (0..n).map(|_| r.f64() < 0.4).collect()).collect();
            let b: Vec<Vec<bool>> = (0..n).map(|_| (0..n).map(|_| r.f64() < 0.4).collect()).collect();
            let naive = bool_matmul_naive(&a, &b);
            assert_eq!(bool_matmul_strassen(&a, &b), naive);
            assert_eq!(bool_matmul_four_russians(&a, &b), naive);
        }
    }

    #[test]
    fn triangle_detection_matches_triple_enumeration() {
        let mut r = Rng::new(314);
        for _ in 0..80 {
            let n = r.range(3, 12);
            let mut adj = vec![vec![false; n]; n];
            for i in 0..n {
                for j in (i + 1)..n {
                    if r.f64() < 0.3 {
                        adj[i][j] = true;
                        adj[j][i] = true;
                    }
                }
            }
            let brute = (0..n).any(|i| ((i + 1)..n).any(|j| ((j + 1)..n).any(|k| adj[i][j] && adj[j][k] && adj[i][k])));
            let found = has_triangle(&adj);
            assert_eq!(found.is_some(), brute);
            if let Some((i, j, k)) = found {
                assert!(adj[i][j] && adj[j][k] && adj[i][k]);
            }
        }
    }
}
