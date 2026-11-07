//! @title Longest Palindromic Subsequence
//! @chapter Problem 15-2 (2e–3e), 14-2 (4e)
//! @editions 2,3,4
//! @summary The longest subsequence that reads the same forwards and backwards: Θ(n²).
//! @order 60
use crate::outln;

/// Returns one longest palindromic subsequence of `s` (by bytes).
pub fn longest_palindromic_subsequence(s: &str) -> String {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 { return String::new(); }
    // l[i][j] = length of the LPS of b[i..=j]
    let mut l = vec![vec![0usize; n]; n];
    for i in (0..n).rev() {
        l[i][i] = 1;
        for j in i + 1..n {
            l[i][j] = if b[i] == b[j] { l[i + 1][j - 1] + 2 } else { l[i + 1][j].max(l[i][j - 1]) };
        }
    }
    // reconstruct
    let (mut i, mut j) = (0usize, n - 1);
    let (mut front, mut back) = (vec![], vec![]);
    while i <= j {
        if i == j { front.push(b[i]); break; }
        if b[i] == b[j] {
            front.push(b[i]);
            back.push(b[j]);
            if j == i + 1 { break; }
            i += 1;
            j -= 1;
        } else if l[i + 1][j] >= l[i][j - 1] { i += 1; } else { j -= 1; }
    }
    back.reverse();
    front.extend(back);
    String::from_utf8(front).unwrap()
}

/// Minimum number of characters to insert to make `s` a palindrome = n − |LPS|.
pub fn min_insertions_to_palindrome(s: &str) -> usize { s.len() - longest_palindromic_subsequence(s).len() }

pub fn demo() -> String {
    let mut o = String::new();
    for s in ["character", "underqualified", "bananas", "racecar", "abcd"] {
        let l = longest_palindromic_subsequence(s);
        outln!(o, "{:<15} -> {:<10} (length {}, insertions needed {})", s, l, l.len(), min_insertions_to_palindrome(s));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn is_pal(s: &str) -> bool { s.bytes().eq(s.bytes().rev()) }
    fn is_subseq(s: &str, t: &str) -> bool { let mut it = t.chars(); s.chars().all(|c| it.any(|d| d == c)) }
    fn brute(s: &str) -> usize {
        let b = s.as_bytes();
        (0..1u32 << b.len()).filter(|m| { let v: Vec<u8> = (0..b.len()).filter(|i| m >> i & 1 == 1).map(|i| b[i]).collect(); v.iter().eq(v.iter().rev()) }).map(|m| m.count_ones() as usize).max().unwrap()
    }
    #[test]
    fn book_examples() {
        assert_eq!(longest_palindromic_subsequence("character").len(), 5); // carac
        assert_eq!(longest_palindromic_subsequence("underqualified").len(), brute("underqualified"));
        assert_eq!(longest_palindromic_subsequence("racecar"), "racecar");
        assert_eq!(longest_palindromic_subsequence(""), "");
        assert_eq!(longest_palindromic_subsequence("z"), "z");
    }
    #[test]
    fn random_strings() {
        let mut r = Rng::new(104);
        for _ in 0..80 {
            let n = r.range(0, 12);
            let s: String = (0..n).map(|_| (b'a' + r.range(0, 2) as u8) as char).collect();
            let l = longest_palindromic_subsequence(&s);
            assert!(is_pal(&l) && is_subseq(&l, &s), "{s} -> {l}");
            assert_eq!(l.len(), if n == 0 { 0 } else { brute(&s) }, "{s}");
        }
    }
}
