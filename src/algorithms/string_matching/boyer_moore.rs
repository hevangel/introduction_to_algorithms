//! @title Boyer-Moore Algorithm
//! @chapter 34.5 (1e)
//! @editions 1
//! @summary Scan the pattern right to left; bad-character and good-suffix heuristics skip text positions: sublinear in practice.
//! @order 50
use crate::outln;

/// Bad-character table: last position of each byte in P (−1 if absent).
pub fn last_occurrence(p: &[u8]) -> [isize; 256] {
    let mut l = [-1isize; 256];
    for (i, &c) in p.iter().enumerate() { l[c as usize] = i as isize; }
    l
}

/// Strong good-suffix shifts: `shift[j]` is how far to slide after a mismatch at pattern index j−1 (shift[0] after a full match).
pub fn good_suffix_shift(p: &[u8]) -> Vec<usize> {
    let m = p.len();
    let (mut shift, mut border) = (vec![0usize; m + 1], vec![0usize; m + 1]);
    // case 1: the matched suffix occurs elsewhere in P, preceded by a different character
    let (mut i, mut j) = (m, m + 1);
    border[i] = j;
    while i > 0 {
        while j <= m && p[i - 1] != p[j - 1] {
            if shift[j] == 0 { shift[j] = j - i; }
            j = border[j];
        }
        i -= 1;
        j -= 1;
        border[i] = j;
    }
    // case 2: only a prefix of P matches a part of the suffix
    j = border[0];
    for i in 0..=m {
        if shift[i] == 0 { shift[i] = j; }
        if i == j { j = border[j]; }
    }
    shift
}

/// BOYER-MOORE-MATCHER: all valid shifts, using both heuristics. Returns (shifts, character comparisons).
pub fn boyer_moore(t: &[u8], p: &[u8]) -> (Vec<usize>, usize) {
    let (n, m) = (t.len(), p.len());
    if m == 0 { return ((0..=n).collect(), 0); }
    if m > n { return (vec![], 0); }
    let (last, gs) = (last_occurrence(p), good_suffix_shift(p));
    let (mut s, mut found, mut cmps) = (0usize, vec![], 0usize);
    while s <= n - m {
        let mut j = m;
        while j > 0 && { cmps += 1; p[j - 1] == t[s + j - 1] } { j -= 1; }
        if j == 0 {
            found.push(s);
            s += gs[0];
        } else {
            let bad = (j as isize - 1 - last[t[s + j - 1] as usize]).max(1) as usize;
            s += bad.max(gs[j]);
        }
    }
    (found, cmps)
}

pub fn demo() -> String {
    let mut o = String::new();
    let (t, p) = (b"here is a simple example, a simple one".as_slice(), b"simple".as_slice());
    let (shifts, cmps) = boyer_moore(t, p);
    outln!(o, "matches at {:?} using {} comparisons (text length {})", shifts, cmps, t.len());
    outln!(o, "good-suffix shifts for 'ABYXCDBYXCD'-like pattern 'anpanman': {:?}", good_suffix_shift(b"anpanman"));
    let big = "abcdefghij".repeat(5000);
    let (s, c) = boyer_moore(big.as_bytes(), b"jabcdefghx");
    outln!(o, "sublinear behaviour: {} matches, {} comparisons for a text of {} chars", s.len(), c, big.len());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn agrees_with_naive_including_periodic_patterns() {
        let mut r = Rng::new(223);
        for _ in 0..500 {
            let alpha = r.range(1, 3) as u8;
            let t: Vec<u8> = (0..r.range(0, 80)).map(|_| b'a' + r.range(0, alpha as usize - 1) as u8).collect();
            let p: Vec<u8> = (0..r.range(1, 8)).map(|_| b'a' + r.range(0, alpha as usize - 1) as u8).collect();
            assert_eq!(boyer_moore(&t, &p).0, super::super::naive_string_matcher::naive_string_matcher(&t, &p), "t={:?} p={:?}", String::from_utf8_lossy(&t), String::from_utf8_lossy(&p));
        }
        assert_eq!(boyer_moore(b"aaaaaa", b"aa").0, vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn skips_text_when_pattern_chars_are_absent() {
        let t = vec![b'x'; 10_000];
        let (s, cmps) = boyer_moore(&t, b"abcdefgh");
        assert!(s.is_empty());
        assert!(cmps < t.len() / 4, "{cmps}");
    }
}
