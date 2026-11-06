//! @title String Matching with Finite Automata
//! @chapter 32.3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Build the string-matching automaton δ(q, a) in O(m|Σ|) and scan the text in Θ(n).
//! @order 30
use crate::outln;

/// COMPUTE-TRANSITION-FUNCTION(P, Σ): δ[q][a] = length of the longest prefix of P that is a suffix of P[..q]·a.
/// Alphabet = all 256 byte values. Built in O(m·|Σ|) using the "restart state" trick.
pub fn compute_transition_function(p: &[u8]) -> Vec<[usize; 256]> {
    let m = p.len();
    let mut delta = vec![[0usize; 256]; m + 1];
    if m == 0 { return delta; }
    delta[0][p[0] as usize] = 1;
    let mut x = 0; // state reached after reading P[1..q−1] from the start: the "restart" state
    for q in 1..=m {
        delta[q] = delta[x];                    // on a mismatch behave like the restart state
        if q < m {
            delta[q][p[q] as usize] = q + 1;    // on the matching character advance
            x = delta[x][p[q] as usize];
        }
    }
    delta
}

/// FINITE-AUTOMATON-MATCHER(T, δ, m): scan T once; each arrival at state m reports a match ending there.
pub fn finite_automaton_matcher(t: &[u8], p: &[u8]) -> Vec<usize> {
    let m = p.len();
    let delta = compute_transition_function(p);
    let (mut q, mut shifts) = (0, vec![]);
    if m == 0 { return (0..=t.len()).collect(); }
    for (i, &c) in t.iter().enumerate() {
        q = delta[q][c as usize];
        if q == m { shifts.push(i + 1 - m); }
    }
    shifts
}

pub fn demo() -> String {
    let mut o = String::new();
    let p = b"ababaca";
    let delta = compute_transition_function(p);
    outln!(o, "automaton for P = ababaca (states 0..7); transitions on a, b, c:");
    for q in 0..=p.len() { outln!(o, "  state {q}: a→{} b→{} c→{}", delta[q][b'a' as usize], delta[q][b'b' as usize], delta[q][b'c' as usize]); }
    outln!(o, "T = abababacaba: matches at shifts {:?}", finite_automaton_matcher(b"abababacaba", p));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn figure_32_7_automaton() {
        let d = compute_transition_function(b"ababaca");
        let row = |q: usize| (d[q][b'a' as usize], d[q][b'b' as usize], d[q][b'c' as usize]);
        let want = [(1, 0, 0), (1, 2, 0), (3, 0, 0), (1, 4, 0), (5, 0, 0), (1, 4, 6), (7, 0, 0), (1, 2, 0)];
        for (q, w) in want.iter().enumerate() { assert_eq!(row(q), *w, "state {q}"); }
    }
    #[test]
    fn agrees_with_naive() {
        let mut r = Rng::new(221);
        for _ in 0..200 {
            let t: Vec<u8> = (0..r.range(0, 50)).map(|_| b'a' + r.range(0, 2) as u8).collect();
            let p: Vec<u8> = (0..r.range(1, 6)).map(|_| b'a' + r.range(0, 2) as u8).collect();
            assert_eq!(finite_automaton_matcher(&t, &p), super::super::naive_string_matcher::naive_string_matcher(&t, &p));
        }
    }
}
