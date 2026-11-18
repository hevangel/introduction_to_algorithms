//! @title Vandermonde Matrices & GF(2) Linear Permutations
//! @chapter Problems D-1 & D-2 (3e–4e)
//! @editions 3,4
//! @summary Exact rational determinant matching Π(x_k - x_j) for Vandermonde matrices, plus invertible GF(2) linear matrix bijections on b-bit words.
//! @order 50
use crate::outln;

/// Minimal exact rational arithmetic (num / den) to compute exact integer determinants cleanly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rat {
    num: i128,
    den: i128,
}

fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.max(1)
}

impl Rat {
    fn new(num: i128, den: i128) -> Self {
        assert!(den != 0);
        let sign = if den < 0 { -1 } else { 1 };
        let g = gcd_i128(num, den);
        Self { num: sign * num / g, den: sign * den / g }
    }
    fn from_int(v: i128) -> Self {
        Self { num: v, den: 1 }
    }
    fn sub(self, o: Self) -> Self {
        Self::new(self.num * o.den - o.num * self.den, self.den * o.den)
    }
    fn mul(self, o: Self) -> Self {
        Self::new(self.num * o.num, self.den * o.den)
    }
    fn div(self, o: Self) -> Self {
        Self::new(self.num * o.den, self.den * o.num)
    }
}

/// Problem D-1: Build the n x n Vandermonde matrix V_{ij} = x_i^j (0 <= i, j < n).
pub fn vandermonde_matrix(xs: &[i128]) -> Vec<Vec<i128>> {
    let n = xs.len();
    let mut v = vec![vec![0i128; n]; n];
    for (i, &x) in xs.iter().enumerate() {
        let mut p = 1i128;
        for j in 0..n {
            v[i][j] = p;
            p *= x;
        }
    }
    v
}

/// Exact integer determinant via Gaussian elimination in Q.
pub fn exact_det(mat: &[Vec<i128>]) -> i128 {
    let n = mat.len();
    let mut a: Vec<Vec<Rat>> = mat
        .iter()
        .map(|row| row.iter().map(|&v| Rat::from_int(v)).collect())
        .collect();
    let mut det = Rat::from_int(1);
    for k in 0..n {
        let Some(piv) = (k..n).find(|&i| a[i][k].num != 0) else {
            return 0;
        };
        if piv != k {
            a.swap(k, piv);
            det = Rat::from_int(-det.num).mul(Rat::new(1, det.den));
        }
        let pivot = a[k][k];
        det = det.mul(pivot);
        for i in (k + 1)..n {
            if a[i][k].num == 0 {
                continue;
            }
            let factor = a[i][k].div(pivot);
            for j in k..n {
                let diff = factor.mul(a[k][j]);
                a[i][j] = a[i][j].sub(diff);
            }
        }
    }
    assert_eq!(det.den, 1);
    det.num
}

/// Closed-form Vandermonde product: prod_{0 <= j < k < n} (x_k - x_j).
pub fn vandermonde_det_formula(xs: &[i128]) -> i128 {
    let n = xs.len();
    let mut prod = 1i128;
    for j in 0..n {
        for k in (j + 1)..n {
            prod *= xs[k] - xs[j];
        }
    }
    prod
}

/// Problem D-2: b x b binary matrix over GF(2) acting on b-bit integers as linear permutations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gf2Matrix {
    pub b: usize,
    pub rows: Vec<u64>,
}

impl Gf2Matrix {
    pub fn new(b: usize, rows: Vec<u64>) -> Self {
        assert!(b <= 32 && rows.len() == b);
        let mask = if b == 64 { u64::MAX } else { (1u64 << b) - 1 };
        Self { b, rows: rows.into_iter().map(|r| r & mask).collect() }
    }

    /// Apply M * x mod 2 where bits of x form a column vector (bit 0 at LSB).
    pub fn apply(&self, x: u64) -> u64 {
        let mut out = 0u64;
        for (i, &row) in self.rows.iter().enumerate() {
            if (row & x).count_ones() % 2 == 1 {
                out |= 1u64 << i;
            }
        }
        out
    }

    /// Compute M^{-1} in GL_b(F_2) by Gauss-Jordan elimination over GF(2), or None if singular.
    pub fn inverse(&self) -> Option<Self> {
        let b = self.b;
        let mut left = self.rows.clone();
        let mut right: Vec<u64> = (0..b).map(|i| 1u64 << i).collect();
        for col in 0..b {
            let piv = (col..b).find(|&r| (left[r] >> col) & 1 == 1)?;
            left.swap(col, piv);
            right.swap(col, piv);
            for r in 0..b {
                if r != col && ((left[r] >> col) & 1 == 1) {
                    left[r] ^= left[col];
                    right[r] ^= right[col];
                }
            }
        }
        Some(Self::new(b, right))
    }

    pub fn bit_reversal(b: usize) -> Self {
        let rows = (0..b).map(|i| 1u64 << (b - 1 - i)).collect();
        Self::new(b, rows)
    }

    /// Matrix for Gray encoding G(x) = x ^ (x >> 1) (upper triangular with 1s on diagonal and superdiagonal).
    pub fn gray_encode_matrix(b: usize) -> Self {
        let rows = (0..b)
            .map(|i| {
                let mut r = 1u64 << i;
                if i + 1 < b {
                    r |= 1u64 << (i + 1);
                }
                r
            })
            .collect();
        Self::new(b, rows)
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let xs = [1, 2, 4, 7];
    let v = vandermonde_matrix(&xs);
    let det_ge = exact_det(&v);
    let det_f = vandermonde_det_formula(&xs);
    outln!(o, "Vandermonde xs={:?}: exact_det = {det_ge}, product formula = {det_f}", xs);

    let g = Gf2Matrix::gray_encode_matrix(4);
    let g_inv = g.inverse().unwrap();
    let mapped: Vec<u64> = (0..16).map(|x| g.apply(x)).collect();
    let back: Vec<u64> = mapped.iter().map(|&y| g_inv.apply(y)).collect();
    outln!(o, "GF(2) 4-bit Gray matrix output: {:?}", mapped);
    outln!(o, "GF(2) inverse recovers identity? {}", back == (0..16).collect::<Vec<_>>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn vandermonde_determinant_matches_product_formula_exactly() {
        let mut r = Rng::new(3001);
        for n in 1..=6 {
            for _ in 0..10 {
                let xs: Vec<i128> = (0..n).map(|_| r.range(0, 12) as i128 - 6).collect();
                let v = vandermonde_matrix(&xs);
                assert_eq!(exact_det(&v), vandermonde_det_formula(&xs));
            }
        }
    }

    #[test]
    fn gf2_invertible_matrices_induce_bijections_and_singular_ones_collide() {
        let mut r = Rng::new(3002);
        for b in 1..=7 {
            let rev = Gf2Matrix::bit_reversal(b);
            assert_eq!(rev.inverse(), Some(rev.clone()));
            let gray = Gf2Matrix::gray_encode_matrix(b);
            let gray_inv = gray.inverse().unwrap();

            let dom = 1usize << b;
            let mut seen = vec![false; dom];
            for x in 0..dom as u64 {
                let y = gray.apply(x);
                assert_eq!(y, x ^ (x >> 1));
                assert_eq!(gray_inv.apply(y), x);
                seen[y as usize] = true;
            }
            assert!(seen.into_iter().all(|s| s));
        }

        // Random 5-bit matrices: verify invertible <-> bijection on 0..32.
        let b = 5usize;
        let dom = 1usize << b;
        for _ in 0..40 {
            let rows: Vec<u64> = (0..b).map(|_| r.next_u64() & 31).collect();
            let m = Gf2Matrix::new(b, rows);
            let imgs: Vec<u64> = (0..dom as u64).map(|x| m.apply(x)).collect();
            let mut sorted = imgs.clone();
            sorted.sort();
            sorted.dedup();
            match m.inverse() {
                Some(inv) => {
                    assert_eq!(sorted.len(), dom);
                    for x in 0..dom as u64 {
                        assert_eq!(inv.apply(m.apply(x)), x);
                    }
                }
                None => assert!(sorted.len() < dom),
            }
        }
    }
}
