//! @title Repetition Factor and the Repetition Matcher
//! @chapter Problem 32-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary ρ(y) is the largest r with y = xʳ; computed from the prefix function. The repetition matcher shifts by ⌈q/(ρ*+1)⌉ after a mismatch at q.
//! @order 95
use super::kmp::prefix_function;
use crate::outln;

/// ρ(y): the largest r such that y = x^r for some string x (ρ of the empty string is 0). From the prefix function: O(|y|).
pub fn repetition_factor(y: &[u8]) -> usize {
    if y.is_empty() { return 0; }
    let n = y.len();
    let p = n - prefix_function(y)[n - 1]; // smallest period
    if n % p == 0 { n / p } else { 1 }
}

/// ρ*(P) = max over prefixes P_k of ρ(P_k), all computed in one O(m) pass.
pub fn max_prefix_repetition(p: &[u8]) -> usize {
    let pi = prefix_function(p);
    (1..=p.len()).map(|k| { let per = k - pi[k - 1]; if k % per == 0 { k / per } else { 1 } }).max().unwrap_or(0)
}

/// REPETITION-MATCHER(P, T): report all shifts s with T[s..s+m] = P, advancing by max(1, ⌈q/(ρ*+1)⌉) after a mismatch at position q.
pub fn repetition_matcher(p: &[u8], t: &[u8]) -> Vec<usize> {
    let (m, n) = (p.len(), t.len());
    let mut out = vec![];
    if m == 0 || m > n { return out; }
    let k = 1 + max_prefix_repetition(p);
    let (mut q, mut s) = (0, 0);
    while s <= n - m {
        if t[s + q] == p[q] { q += 1; if q == m { out.push(s); } }
        if q == m || t[s + q] != p[q] {
            s += 1usize.max(q.div_ceil(k));
            q = 0;
        }
    }
    out
}

pub fn demo() -> String {
    let mut o = String::new();
    for y in ["abababab", "abcabcabc", "aaaa", "abaab", "a"] { outln!(o, "ρ({y}) = {}", repetition_factor(y.as_bytes())); }
    let p = b"abab";
    outln!(o, "ρ*(abab) = {}", max_prefix_repetition(p));
    let t = b"abababababcabab";
    outln!(o, "matches of abab in {}: {:?}", String::from_utf8_lossy(t), repetition_matcher(p, t));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn naive(p: &[u8], t: &[u8]) -> Vec<usize> { if p.len() > t.len() { vec![] } else { (0..=t.len() - p.len()).filter(|&s| &t[s..s + p.len()] == p).collect() } }
    #[test]
    fn repetition_factor_matches_definition() {
        let mut r = Rng::new(400);
        for _ in 0..300 {
            let y: Vec<u8> = (0..r.range(1, 12)).map(|_| b'a' + r.range(0, 1) as u8).collect();
            let brute = (1..=y.len()).filter(|d| y.len() % d == 0 && (0..y.len()).all(|i| y[i] == y[i % d])).map(|d| y.len() / d).max().unwrap();
            assert_eq!(repetition_factor(&y), brute, "{}", String::from_utf8_lossy(&y));
        }
        assert_eq!(repetition_factor(b""), 0);
    }
    #[test]
    fn matcher_agrees_with_naive_search() {
        let mut r = Rng::new(401);
        for _ in 0..500 {
            let alphabet = r.range(1, 3);
            let p: Vec<u8> = (0..r.range(1, 8)).map(|_| b'a' + r.range(0, alphabet) as u8).collect();
            let t: Vec<u8> = (0..r.range(0, 40)).map(|_| b'a' + r.range(0, alphabet) as u8).collect();
            assert_eq!(repetition_matcher(&p, &t), naive(&p, &t), "p={} t={}", String::from_utf8_lossy(&p), String::from_utf8_lossy(&t));
        }
    }
}
