//! @title Longest Common Subsequence
//! @chapter 15.4 (2e–3e), 14.4 (4e), 16.3 (1e)
//! @editions 1,2,3,4
//! @summary Θ(mn) dynamic program with table reconstruction and Θ(min(m,n))-space length computation.
//! @order 40
use crate::outln;

/// LCS-LENGTH: the c table, c[i][j] = |LCS(x[..i], y[..j])|.
pub fn lcs_table(x: &[u8], y: &[u8]) -> Vec<Vec<usize>> {
    let (m, n) = (x.len(), y.len());
    let mut c = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            c[i][j] = if x[i - 1] == y[j - 1] { c[i - 1][j - 1] + 1 } else { c[i - 1][j].max(c[i][j - 1]) };
        }
    }
    c
}

/// PRINT-LCS: walk the table backwards to build one longest common subsequence.
pub fn lcs(x: &str, y: &str) -> String {
    let (xb, yb) = (x.as_bytes(), y.as_bytes());
    let c = lcs_table(xb, yb);
    let (mut i, mut j) = (xb.len(), yb.len());
    let mut out = vec![];
    while i > 0 && j > 0 {
        if xb[i - 1] == yb[j - 1] { out.push(xb[i - 1]); i -= 1; j -= 1; }
        else if c[i - 1][j] >= c[i][j - 1] { i -= 1; }
        else { j -= 1; }
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

/// Length only, using two rows: O(min(m,n)) space (Exercise 15.4-4).
pub fn lcs_length_two_rows(x: &[u8], y: &[u8]) -> usize {
    let (x, y) = if x.len() < y.len() { (y, x) } else { (x, y) }; // y is the shorter
    let mut prev = vec![0usize; y.len() + 1];
    for &xc in x {
        let mut cur = vec![0usize; y.len() + 1];
        for j in 1..=y.len() {
            cur[j] = if xc == y[j - 1] { prev[j - 1] + 1 } else { prev[j].max(cur[j - 1]) };
        }
        prev = cur;
    }
    prev[y.len()]
}

/// Memoized top-down version.
pub fn lcs_memoized(x: &[u8], y: &[u8]) -> usize {
    fn go(x: &[u8], y: &[u8], i: usize, j: usize, memo: &mut Vec<Vec<Option<usize>>>) -> usize {
        if i == 0 || j == 0 { return 0; }
        if let Some(v) = memo[i][j] { return v; }
        let v = if x[i - 1] == y[j - 1] { go(x, y, i - 1, j - 1, memo) + 1 } else { go(x, y, i - 1, j, memo).max(go(x, y, i, j - 1, memo)) };
        memo[i][j] = Some(v);
        v
    }
    go(x, y, x.len(), y.len(), &mut vec![vec![None; y.len() + 1]; x.len() + 1])
}

pub fn demo() -> String {
    let mut o = String::new();
    let (x, y) = ("ABCBDAB", "BDCABA");
    let c = lcs_table(x.as_bytes(), y.as_bytes());
    outln!(o, "X = {x}, Y = {y}");
    outln!(o, "c table (rows = prefixes of X, columns = prefixes of Y):");
    for row in &c { outln!(o, "  {:?}", row); }
    outln!(o, "LCS = {} (length {})", lcs(x, y), c[x.len()][y.len()]);
    outln!(o, "two-row length = {}, memoized length = {}", lcs_length_two_rows(x.as_bytes(), y.as_bytes()), lcs_memoized(x.as_bytes(), y.as_bytes()));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn is_subsequence(s: &str, t: &str) -> bool {
        let mut it = t.chars();
        s.chars().all(|c| it.any(|d| d == c))
    }
    fn brute_len(x: &[u8], y: &[u8]) -> usize {
        (0..1u32 << x.len()).filter(|m| {
            let sub: Vec<u8> = (0..x.len()).filter(|i| m >> i & 1 == 1).map(|i| x[i]).collect();
            let mut it = y.iter();
            sub.iter().all(|c| it.any(|d| d == c))
        }).map(|m| m.count_ones() as usize).max().unwrap()
    }
    #[test]
    fn book_example() {
        assert_eq!(lcs("ABCBDAB", "BDCABA").len(), 4);
        assert_eq!(lcs_length_two_rows(b"ABCBDAB", b"BDCABA"), 4);
        assert_eq!(lcs("", "abc"), "");
    }
    #[test]
    fn random_strings_agree_with_brute_force() {
        let mut r = Rng::new(102);
        for _ in 0..60 {
            let mk = |r: &mut Rng, n: usize| -> String { (0..n).map(|_| (b'a' + r.range(0, 2) as u8) as char).collect() };
            let (n1, n2) = (r.range(0, 9), r.range(0, 9));
            let (x, y) = (mk(&mut r, n1), mk(&mut r, n2));
            let l = lcs(&x, &y);
            assert!(is_subsequence(&l, &x) && is_subsequence(&l, &y));
            assert_eq!(l.len(), brute_len(x.as_bytes(), y.as_bytes()));
            assert_eq!(lcs_length_two_rows(x.as_bytes(), y.as_bytes()), l.len());
            assert_eq!(lcs_memoized(x.as_bytes(), y.as_bytes()), l.len());
        }
    }
}
