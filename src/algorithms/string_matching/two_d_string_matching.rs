//! @title Two-Dimensional Pattern Matching (Rabin-Karp)
//! @chapter Exercise 32.2-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Find an m×m pattern in an n×n text array with a rolling hash along rows and then along columns: O(n² + m²) expected plus verification.
//! @order 97
use crate::outln;

const MOD: u64 = 1_000_000_007;
const B: u64 = 257;

fn pow_mod(mut b: u64, mut e: usize) -> u64 { let mut r = 1; while e > 0 { if e & 1 == 1 { r = r * b % MOD; } b = b * b % MOD; e >>= 1; } r }

/// All (row, col) positions where the m×m `pattern` occurs in `text`.
pub fn match_2d(text: &[Vec<u8>], pattern: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let (n_rows, m) = (text.len(), pattern.len());
    if m == 0 || n_rows < m || text[0].len() < m { return vec![]; }
    let n_cols = text[0].len();
    let (bm1, bm) = (pow_mod(B, m - 1), pow_mod(B, m));
    // step 1: hash of every length-m window of every row
    let row_hash = |row: &[u8]| -> Vec<u64> {
        let mut h = row[..m].iter().fold(0, |h, &c| (h * B + c as u64) % MOD);
        let mut out = vec![h];
        for c in 1..=row.len() - m { h = ((h * B + MOD - row[c - 1] as u64 * bm % MOD) + row[c + m - 1] as u64) % MOD; out.push(h); }
        out
    };
    let th: Vec<Vec<u64>> = text.iter().map(|r| row_hash(r)).collect();
    // the pattern's m row-hashes combine into one number
    let ph = pattern.iter().map(|r| row_hash(r)[0]).fold(0, |h, x| (h * B + x) % MOD);
    let _ = bm1;
    // step 2: slide down each column of window hashes
    let mut out = vec![];
    for c in 0..=n_cols - m {
        let mut h = (0..m).fold(0, |h, r| (h * B + th[r][c]) % MOD);
        for r in 0..=n_rows - m {
            if r > 0 { h = ((h * B + MOD - th[r - 1][c] * bm % MOD) + th[r + m - 1][c]) % MOD; }
            // verify to rule out hash collisions
            if h == ph && (0..m).all(|i| text[r + i][c..c + m] == pattern[i][..]) { out.push((r, c)); }
        }
    }
    out.sort();
    out
}

pub fn demo() -> String {
    let mut o = String::new();
    let text: Vec<Vec<u8>> = ["abcabc", "bcabca", "abcabc", "cabcab", "abcabc", "bcabca"].iter().map(|s| s.bytes().collect()).collect();
    let pat: Vec<Vec<u8>> = ["ca", "ab"].iter().map(|s| s.bytes().collect()).collect();
    outln!(o, "text:");
    for r in &text { outln!(o, "  {}", String::from_utf8_lossy(r)); }
    outln!(o, "2×2 pattern ca/ab occurs at (row, col): {:?}", match_2d(&text, &pat));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn naive(t: &[Vec<u8>], p: &[Vec<u8>]) -> Vec<(usize, usize)> {
        let (n, m) = (t.len(), p.len());
        let mut out = vec![];
        if n >= m && !t.is_empty() && t[0].len() >= m { for r in 0..=n - m { for c in 0..=t[0].len() - m { if (0..m).all(|i| t[r + i][c..c + m] == p[i][..]) { out.push((r, c)); } } } }
        out
    }
    #[test]
    fn agrees_with_naive_on_small_alphabets() {
        let mut r = Rng::new(403);
        for _ in 0..300 {
            let (rows, cols, m) = (r.range(1, 9), r.range(1, 9), r.range(1, 3));
            let alpha = r.range(1, 2);
            let text: Vec<Vec<u8>> = (0..rows).map(|_| (0..cols).map(|_| b'a' + r.range(0, alpha) as u8).collect()).collect();
            let pat: Vec<Vec<u8>> = (0..m).map(|_| (0..m).map(|_| b'a' + r.range(0, alpha) as u8).collect()).collect();
            assert_eq!(match_2d(&text, &pat), naive(&text, &pat));
        }
    }
    #[test]
    fn planted_pattern_is_found() {
        let mut r = Rng::new(404);
        let mut text: Vec<Vec<u8>> = (0..40).map(|_| (0..40).map(|_| b'a' + r.range(0, 3) as u8).collect()).collect();
        let pat: Vec<Vec<u8>> = (0..4).map(|_| (0..4).map(|_| b'a' + r.range(0, 25) as u8).collect()).collect();
        for i in 0..4 { text[17 + i][9..13].copy_from_slice(&pat[i]); }
        assert!(match_2d(&text, &pat).contains(&(17, 9)));
    }
}
