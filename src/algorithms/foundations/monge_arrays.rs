//! @title Monge Arrays
//! @chapter Problem 4-6 (2e–4e)
//! @editions 2,3,4
//! @summary Row-minimum positions of a Monge array are monotone: find all of them by divide and conquer in O(m + n lg m).
//! @order 190
use crate::outln;

/// Is `a` Monge: a[i][j] + a[k][l] ≤ a[i][l] + a[k][j] for all i<k, j<l? (Checking adjacent entries suffices.)
pub fn is_monge(a: &[Vec<i64>]) -> bool {
    (0..a.len().saturating_sub(1)).all(|i| (0..a[0].len().saturating_sub(1)).all(|j| a[i][j] + a[i + 1][j + 1] <= a[i][j + 1] + a[i + 1][j]))
}

/// Leftmost minimum column of every row of an m×n Monge array, given as an oracle `f(i, j)`.
/// The leftmost-minimum position never decreases with the row index, so solve the middle row and recurse on both halves.
pub fn row_minima(m: usize, n: usize, f: &dyn Fn(usize, usize) -> i64) -> Vec<usize> {
    let mut res = vec![0; m];
    fn go(rows: std::ops::Range<usize>, cols: std::ops::RangeInclusive<usize>, f: &dyn Fn(usize, usize) -> i64, res: &mut Vec<usize>) {
        if rows.is_empty() { return; }
        let mid = (rows.start + rows.end) / 2;
        let mut best = *cols.start();
        for j in cols.clone() { if f(mid, j) < f(mid, best) { best = j; } }
        res[mid] = best;
        go(rows.start..mid, *cols.start()..=best, f, res);
        go(mid + 1..rows.end, best..=*cols.end(), f, res);
    }
    go(0..m, 0..=n - 1, f, &mut res);
    res
}

/// Brute-force Θ(mn) leftmost row minima.
pub fn row_minima_brute(a: &[Vec<i64>]) -> Vec<usize> {
    a.iter().map(|r| (0..r.len()).fold(0, |b, j| if r[j] < r[b] { j } else { b })).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    // A[i][j] = (i − 2j)²  is Monge (a convex function of a linear form)
    let a: Vec<Vec<i64>> = (0..8).map(|i| (0..8).map(|j| (i as i64 - 2 * j as i64).pow(2)).collect()).collect();
    outln!(o, "is Monge: {}", is_monge(&a));
    outln!(o, "row minima (divide & conquer): {:?}", row_minima(8, 8, &|i, j| a[i][j]));
    outln!(o, "row minima (brute force)     : {:?}", row_minima_brute(&a));
    let calls = std::cell::Cell::new(0usize);
    let big = |i: usize, j: usize| { calls.set(calls.get() + 1); (i as i64 - j as i64 / 2).pow(2) };
    row_minima(1000, 1000, &big);
    outln!(o, "1000×1000 array: {} entry evaluations (≈ m + n lg m = {}) instead of 10⁶", calls.get(), 1000 + 1000 * 10);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn random_monge(m: usize, n: usize, r: &mut Rng) -> Vec<Vec<i64>> {
        // A[i][j] = r_i + c_j − Σ_{a≤i,b≤j} P[a][b] with P ≥ 0 has non-positive second differences
        let (ri, cj) = (r.vec(m, 0, 50), r.vec(n, 0, 50));
        let p: Vec<Vec<i64>> = (0..m).map(|_| r.vec(n, 0, 6)).collect();
        (0..m).map(|i| (0..n).map(|j| ri[i] + cj[j] - (0..=i).map(|a| (0..=j).map(|b| p[a][b]).sum::<i64>()).sum::<i64>()).collect()).collect()
    }
    #[test]
    fn matches_brute_force_on_random_monge_arrays() {
        let mut r = Rng::new(306);
        for _ in 0..200 {
            let (m, n) = (r.range(1, 14), r.range(1, 14));
            let a = random_monge(m, n, &mut r);
            assert!(is_monge(&a));
            assert_eq!(row_minima(m, n, &|i, j| a[i][j]), row_minima_brute(&a));
        }
    }
    #[test]
    fn monotone_minima_and_non_monge_detection() {
        let mut r = Rng::new(307);
        let a = random_monge(30, 25, &mut r);
        let mins = row_minima_brute(&a);
        assert!(mins.windows(2).all(|w| w[0] <= w[1]));
        assert!(!is_monge(&[vec![5, 0], vec![0, 5]]));
    }
}
