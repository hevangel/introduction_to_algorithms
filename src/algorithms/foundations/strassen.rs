//! @title Strassen's Algorithm
//! @chapter 4.2 (4e) / 28.2 (1e–3e)
//! @editions 1,2,3,4
//! @summary Seven recursive half-size products instead of eight: Θ(n^lg 7) ≈ Θ(n^2.81).
//! @order 110
use crate::outln;

pub type Matrix = Vec<Vec<i64>>;

fn naive(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let mut c = vec![vec![0; n]; n];
    for i in 0..n {
        for k in 0..n {
            for j in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

fn add(a: &Matrix, b: &Matrix) -> Matrix {
    a.iter().zip(b).map(|(x, y)| x.iter().zip(y).map(|(p, q)| p + q).collect()).collect()
}
fn sub(a: &Matrix, b: &Matrix) -> Matrix {
    a.iter().zip(b).map(|(x, y)| x.iter().zip(y).map(|(p, q)| p - q).collect()).collect()
}
fn block(a: &Matrix, r: usize, c: usize, h: usize) -> Matrix {
    (0..h).map(|i| a[r + i][c..c + h].to_vec()).collect()
}

/// STRASSEN for n a power of two; falls back to the triple loop below `cutoff`.
pub fn strassen_pow2(a: &Matrix, b: &Matrix, cutoff: usize) -> Matrix {
    let n = a.len();
    if n <= cutoff.max(1) {
        return naive(a, b);
    }
    let h = n / 2;
    let (a11, a12, a21, a22) = (block(a, 0, 0, h), block(a, 0, h, h), block(a, h, 0, h), block(a, h, h, h));
    let (b11, b12, b21, b22) = (block(b, 0, 0, h), block(b, 0, h, h), block(b, h, 0, h), block(b, h, h, h));
    // the seven products of Section 4.2
    let p1 = strassen_pow2(&a11, &sub(&b12, &b22), cutoff);
    let p2 = strassen_pow2(&add(&a11, &a12), &b22, cutoff);
    let p3 = strassen_pow2(&add(&a21, &a22), &b11, cutoff);
    let p4 = strassen_pow2(&a22, &sub(&b21, &b11), cutoff);
    let p5 = strassen_pow2(&add(&a11, &a22), &add(&b11, &b22), cutoff);
    let p6 = strassen_pow2(&sub(&a12, &a22), &add(&b21, &b22), cutoff);
    let p7 = strassen_pow2(&sub(&a11, &a21), &add(&b11, &b12), cutoff);
    let c11 = add(&sub(&add(&p5, &p4), &p2), &p6);
    let c12 = add(&p1, &p2);
    let c21 = add(&p3, &p4);
    let c22 = sub(&sub(&add(&p5, &p1), &p3), &p7);
    let mut c = vec![vec![0; n]; n];
    for i in 0..h {
        for j in 0..h {
            c[i][j] = c11[i][j];
            c[i][j + h] = c12[i][j];
            c[i + h][j] = c21[i][j];
            c[i + h][j + h] = c22[i][j];
        }
    }
    c
}

/// Strassen for any square size: pads with zeros up to the next power of two.
pub fn strassen(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let m = n.next_power_of_two();
    let pad = |x: &Matrix| -> Matrix {
        let mut p = vec![vec![0; m]; m];
        for i in 0..n {
            p[i][..n].copy_from_slice(&x[i]);
        }
        p
    };
    let c = strassen_pow2(&pad(a), &pad(b), 2);
    (0..n).map(|i| c[i][..n].to_vec()).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = vec![vec![1, 3], vec![7, 5]];
    let b = vec![vec![6, 8], vec![4, 2]];
    outln!(o, "A·B (Strassen) = {:?}", strassen(&a, &b));
    outln!(o, "A·B (naive)    = {:?}", naive(&a, &b));
    let mut r = crate::common::Rng::new(42);
    let big: Matrix = (0..6).map(|_| r.vec(6, -9, 9)).collect();
    outln!(o, "6×6 padded to 8×8 matches naive: {}", strassen(&big, &big) == naive(&big, &big));
    outln!(o, "multiplications: 8^k (definition) vs 7^k (Strassen); lg 7 ≈ {:.4}", 7f64.log2());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_naive_for_many_sizes() {
        let mut r = Rng::new(9);
        for n in 1..=12 {
            let a: Matrix = (0..n).map(|_| r.vec(n, -9, 9)).collect();
            let b: Matrix = (0..n).map(|_| r.vec(n, -9, 9)).collect();
            assert_eq!(strassen(&a, &b), naive(&a, &b), "n={n}");
        }
    }
    #[test]
    fn pure_strassen_no_cutoff() {
        let mut r = Rng::new(10);
        let a: Matrix = (0..16).map(|_| r.vec(16, -3, 3)).collect();
        assert_eq!(strassen_pow2(&a, &a, 1), naive(&a, &a));
    }
}
