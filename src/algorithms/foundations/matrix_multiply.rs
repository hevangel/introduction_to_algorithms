//! @title Matrix Multiplication (Naive & Recursive)
//! @chapter 4.2 (1e–4e, as 28.1 in 1e–3e)
//! @editions 1,2,3,4
//! @summary Θ(n³) triple loop and the Θ(n³) divide-and-conquer version on n/2 × n/2 blocks.
//! @order 100
use crate::outln;

pub type Matrix = Vec<Vec<i64>>;

pub fn identity(n: usize) -> Matrix {
    let mut m = vec![vec![0; n]; n];
    for i in 0..n {
        m[i][i] = 1;
    }
    m
}

/// SQUARE-MATRIX-MULTIPLY: C = A·B by the definition, Θ(n³).
pub fn square_matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let mut c = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

fn add_into(c: &mut Matrix, r0: usize, c0: usize, x: &Matrix) {
    for i in 0..x.len() {
        for j in 0..x.len() {
            c[r0 + i][c0 + j] += x[i][j];
        }
    }
}

fn block(a: &Matrix, r0: usize, c0: usize, h: usize) -> Matrix {
    (0..h).map(|i| a[r0 + i][c0..c0 + h].to_vec()).collect()
}

/// SQUARE-MATRIX-MULTIPLY-RECURSIVE (n must be a power of two): eight half-size products.
pub fn square_matrix_multiply_recursive(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    if n == 1 {
        return vec![vec![a[0][0] * b[0][0]]];
    }
    let h = n / 2;
    let mut c = vec![vec![0; n]; n];
    for (ci, cj) in [(0, 0), (0, h), (h, 0), (h, h)] {
        // C_ij = A_i1·B_1j + A_i2·B_2j
        for k in [0, h] {
            let p = square_matrix_multiply_recursive(&block(a, ci, k, h), &block(b, k, cj, h));
            add_into(&mut c, ci, cj, &p);
        }
    }
    c
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]];
    let b = identity(4);
    outln!(o, "A·I (naive)     = {:?}", square_matrix_multiply(&a, &b));
    let c = square_matrix_multiply(&a, &a);
    outln!(o, "A·A (naive)     = {:?}", c);
    let r = square_matrix_multiply_recursive(&a, &a);
    outln!(o, "A·A (recursive) = {:?}", r);
    outln!(o, "results equal: {}", c == r);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn rand_matrix(n: usize, r: &mut Rng) -> Matrix {
        (0..n).map(|_| r.vec(n, -5, 5)).collect()
    }
    #[test]
    fn recursive_matches_naive() {
        let mut r = Rng::new(7);
        for n in [1, 2, 4, 8, 16] {
            let (a, b) = (rand_matrix(n, &mut r), rand_matrix(n, &mut r));
            assert_eq!(square_matrix_multiply_recursive(&a, &b), square_matrix_multiply(&a, &b));
        }
    }
    #[test]
    fn identity_is_neutral() {
        let mut r = Rng::new(8);
        let a = rand_matrix(5, &mut r);
        assert_eq!(square_matrix_multiply(&a, &identity(5)), a);
    }
}
