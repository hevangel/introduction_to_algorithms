//! @title Image Compression by Seam Carving
//! @chapter Problem 15-8 (3e), 14-8 (4e)
//! @editions 3,4
//! @summary Find the vertical seam of minimum total disruption in an m×n grid: O(mn) DP.
//! @order 130
use crate::outln;

/// Minimum-cost vertical seam through the disruption grid `d[row][col]` (≥ 0).
/// Returns (cost, column chosen in each row, top to bottom). Adjacent rows' columns differ by at most 1.
pub fn min_seam(d: &[Vec<i64>]) -> (i64, Vec<usize>) {
    let (m, n) = (d.len(), d[0].len());
    let mut best = vec![vec![i64::MAX; n]; m];
    best[0] = d[0].clone();
    for i in 1..m {
        for j in 0..n {
            let lo = j.saturating_sub(1);
            let hi = (j + 1).min(n - 1);
            let prev = (lo..=hi).map(|k| best[i - 1][k]).min().unwrap();
            best[i][j] = prev + d[i][j];
        }
    }
    let mut j = (0..n).min_by_key(|&j| best[m - 1][j]).unwrap();
    let cost = best[m - 1][j];
    let mut seam = vec![j; m];
    for i in (1..m).rev() {
        let lo = j.saturating_sub(1);
        let hi = (j + 1).min(n - 1);
        j = (lo..=hi).min_by_key(|&k| best[i - 1][k]).unwrap();
        seam[i - 1] = j;
    }
    (cost, seam)
}

/// Remove a seam from the grid, producing an m×(n−1) grid.
pub fn remove_seam(d: &[Vec<i64>], seam: &[usize]) -> Vec<Vec<i64>> {
    d.iter().zip(seam).map(|(row, &c)| row.iter().enumerate().filter(|&(j, _)| j != c).map(|(_, &v)| v).collect()).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    let d = vec![
        vec![5, 9, 2, 8, 6],
        vec![3, 1, 7, 4, 9],
        vec![8, 6, 5, 1, 3],
        vec![4, 7, 2, 9, 5],
    ];
    let (cost, seam) = min_seam(&d);
    outln!(o, "disruption grid:");
    for row in &d { outln!(o, "  {:?}", row); }
    outln!(o, "minimum seam cost = {}, columns by row = {:?}", cost, seam);
    let d2 = remove_seam(&d, &seam);
    outln!(o, "after carving the seam the grid is {}×{}", d2.len(), d2[0].len());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn brute(d: &[Vec<i64>], row: usize, col: usize) -> i64 {
        let n = d[0].len();
        let rest = if row + 1 == d.len() { 0 } else {
            (col.saturating_sub(1)..=(col + 1).min(n - 1)).map(|c| brute(d, row + 1, c)).min().unwrap()
        };
        d[row][col] + rest
    }
    #[test]
    fn matches_brute_force_and_seam_is_connected() {
        let mut r = Rng::new(110);
        for _ in 0..60 {
            let (m, n) = (r.range(1, 6), r.range(1, 6));
            let d: Vec<Vec<i64>> = (0..m).map(|_| r.vec(n, 0, 9)).collect();
            let (cost, seam) = min_seam(&d);
            assert_eq!(cost, (0..n).map(|c| brute(&d, 0, c)).min().unwrap());
            assert_eq!(seam.iter().enumerate().map(|(i, &c)| d[i][c]).sum::<i64>(), cost);
            assert!(seam.windows(2).all(|w| (w[0] as i64 - w[1] as i64).abs() <= 1));
            if n > 1 { assert_eq!(remove_seam(&d, &seam)[0].len(), n - 1); }
        }
    }
}
