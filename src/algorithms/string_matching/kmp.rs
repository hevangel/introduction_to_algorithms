//! @title Knuth-Morris-Pratt Algorithm
//! @chapter 32.4 (1e–4e)
//! @editions 1,2,3,4
//! @summary The prefix function π lets the matcher never move backwards in the text: Θ(n + m) with O(m) space.
//! @order 40
use crate::outln;

/// COMPUTE-PREFIX-FUNCTION(P): π[q] = length of the longest proper prefix of P[..=q] that is also a suffix of it (0-based).
pub fn prefix_function(p: &[u8]) -> Vec<usize> {
    let mut pi = vec![0; p.len()];
    let mut k = 0;
    for q in 1..p.len() {
        while k > 0 && p[k] != p[q] { k = pi[k - 1]; } // fall back along the failure links
        if p[k] == p[q] { k += 1; }
        pi[q] = k;
    }
    pi
}

/// KMP-MATCHER(T, P): all valid shifts.
pub fn kmp_matcher(t: &[u8], p: &[u8]) -> Vec<usize> {
    let m = p.len();
    if m == 0 { return (0..=t.len()).collect(); }
    let pi = prefix_function(p);
    let (mut q, mut shifts) = (0, vec![]);
    for (i, &c) in t.iter().enumerate() {
        while q > 0 && p[q] != c { q = pi[q - 1]; }
        if p[q] == c { q += 1; }
        if q == m {
            shifts.push(i + 1 - m);
            q = pi[q - 1]; // look for the next (possibly overlapping) match
        }
    }
    shifts
}

/// Smallest period of P (Exercise 32.4-?): m − π[m−1].
pub fn smallest_period(p: &[u8]) -> usize { if p.is_empty() { 0 } else { p.len() - prefix_function(p)[p.len() - 1] } }

pub fn demo() -> String {
    let mut o = String::new();
    let p = b"ababaca";
    outln!(o, "π for ababaca = {:?}", prefix_function(p));
    outln!(o, "KMP on T = abababacaba: shifts {:?}", kmp_matcher(b"abababacaba", p));
    outln!(o, "KMP on T = aaaaaaa, P = aa: shifts {:?}", kmp_matcher(b"aaaaaaa", b"aa"));
    outln!(o, "smallest period of 'abcabcabc' = {}", smallest_period(b"abcabcabc"));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn prefix_function_of_figure() {
        assert_eq!(prefix_function(b"ababaca"), vec![0, 0, 1, 2, 3, 0, 1]);
        assert_eq!(prefix_function(b"aaaa"), vec![0, 1, 2, 3]);
        assert_eq!(prefix_function(b""), Vec::<usize>::new());
    }
    #[test]
    fn agrees_with_naive() {
        let mut r = Rng::new(222);
        for _ in 0..300 {
            let t: Vec<u8> = (0..r.range(0, 60)).map(|_| b'a' + r.range(0, 1) as u8).collect();
            let p: Vec<u8> = (0..r.range(1, 7)).map(|_| b'a' + r.range(0, 1) as u8).collect();
            assert_eq!(kmp_matcher(&t, &p), super::super::naive_string_matcher::naive_string_matcher(&t, &p));
        }
        assert_eq!(smallest_period(b"abcabcabc"), 3);
        assert_eq!(smallest_period(b"abcd"), 4);
    }
}
