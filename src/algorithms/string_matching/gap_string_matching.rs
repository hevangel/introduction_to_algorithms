//! @title String Matching with Gap Characters
//! @chapter Exercise 32.1-4 (1e–4e)
//! @editions 1,2,3,4
//! @summary A pattern like ab*ba*c where * matches any string (including the empty one): greedily match the gap-free pieces left to right in O(n + m) with KMP.
//! @order 96
use super::kmp::kmp_matcher;
use crate::outln;

/// Does the text contain a substring matching `pattern`, where the byte `gap` stands for an arbitrary (possibly empty) string?
/// Greedy: find each non-empty piece at its leftmost possible position after the previous one.
pub fn gap_match(text: &[u8], pattern: &[u8], gap: u8) -> bool {
    let mut from = 0;
    for piece in pattern.split(|&c| c == gap).filter(|s| !s.is_empty()) {
        match kmp_matcher(&text[from..], piece).first() { Some(&pos) => from += pos + piece.len(), None => return false }
    }
    true
}

pub fn demo() -> String {
    let mut o = String::new();
    let t = b"cabccbacbacab";
    for p in ["ab*ba*c", "ba*ab", "c*c*c*c", "a*b*c*d", "cab", "*"] {
        outln!(o, "text {} contains {p:<8}: {}", String::from_utf8_lossy(t), gap_match(t, p.as_bytes(), b'*'));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    /// Exhaustive check: does some substring match the pattern (glob semantics)?
    fn glob(p: &[u8], s: &[u8]) -> bool {
        match p.split_first() {
            None => s.is_empty(),
            Some((b'*', rest)) => (0..=s.len()).any(|i| glob(rest, &s[i..])),
            Some((c, rest)) => s.first() == Some(c) && glob(rest, &s[1..]),
        }
    }
    fn brute(t: &[u8], p: &[u8]) -> bool { (0..=t.len()).any(|i| (i..=t.len()).any(|j| glob(p, &t[i..j]))) }
    #[test]
    fn agrees_with_exhaustive_glob_matching() {
        let mut r = Rng::new(402);
        for _ in 0..500 {
            let t: Vec<u8> = (0..r.range(0, 14)).map(|_| b'a' + r.range(0, 2) as u8).collect();
            let p: Vec<u8> = (0..r.range(1, 7)).map(|_| if r.range(0, 3) == 0 { b'*' } else { b'a' + r.range(0, 2) as u8 }).collect();
            assert_eq!(gap_match(&t, &p, b'*'), brute(&t, &p), "t={} p={}", String::from_utf8_lossy(&t), String::from_utf8_lossy(&p));
        }
    }
    #[test]
    fn edge_cases() {
        assert!(gap_match(b"", b"***", b'*'));
        assert!(!gap_match(b"", b"a", b'*'));
        assert!(gap_match(b"xaybz", b"a*b", b'*'));
        assert!(!gap_match(b"bxa", b"a*b", b'*'));
    }
}
