//! @title Naive String Matching
//! @chapter 32.1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Slide the pattern over the text one position at a time: Θ((n−m+1)m).
//! @order 10
use crate::outln;

/// NAIVE-STRING-MATCHER(T, P): all valid shifts s with T[s..s+m] = P.
pub fn naive_string_matcher(t: &[u8], p: &[u8]) -> Vec<usize> {
    let (n, m) = (t.len(), p.len());
    if m > n { return vec![]; }
    (0..=n - m).filter(|&s| &t[s..s + m] == p).collect()
}

/// Count of character comparisons performed (to show worst-case behaviour).
pub fn comparisons(t: &[u8], p: &[u8]) -> usize {
    let (n, m) = (t.len(), p.len());
    let mut c = 0;
    if m > n { return 0; }
    for s in 0..=n - m {
        for j in 0..m {
            c += 1;
            if t[s + j] != p[j] { break; }
        }
    }
    c
}

pub fn demo() -> String {
    let mut o = String::new();
    let (t, p) = ("acaabc", "aab");
    outln!(o, "T = {t}, P = {p}: valid shifts {:?}", naive_string_matcher(t.as_bytes(), p.as_bytes()));
    let t = "a".repeat(1000);
    let p = format!("{}b", "a".repeat(99));
    outln!(o, "worst case T = a^1000, P = a^99 b: {} comparisons ≈ (n−m+1)·m = {}", comparisons(t.as_bytes(), p.as_bytes()), (1000 - 100 + 1) * 100);
    outln!(o, "all occurrences of 'aa' in 'aaaa': {:?}", naive_string_matcher(b"aaaa", b"aa"));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_cases() {
        assert_eq!(naive_string_matcher(b"acaabc", b"aab"), vec![2]);
        assert_eq!(naive_string_matcher(b"aaaa", b"aa"), vec![0, 1, 2]);
        assert_eq!(naive_string_matcher(b"abc", b""), vec![0, 1, 2, 3]);
        assert!(naive_string_matcher(b"ab", b"abc").is_empty());
    }
    #[test]
    fn quadratic_worst_case() {
        let t = vec![b'a'; 200];
        let mut p = vec![b'a'; 49];
        p.push(b'b');
        assert_eq!(comparisons(&t, &p), (200 - 50 + 1) * 50);
    }
}
