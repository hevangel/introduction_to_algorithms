//! @title Rabin-Karp Algorithm
//! @chapter 32.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Compare rolling hashes of the pattern and each text window; verify hash hits. Expected Θ(n + m).
//! @order 20
use crate::outln;

/// RABIN-KARP-MATCHER(T, P, d, q): returns (valid shifts, number of spurious hits).
/// Windows are hashed as base-`d` numbers modulo the prime `q`, updated in O(1) per shift.
pub fn rabin_karp(t: &[u8], p: &[u8], d: u64, q: u64) -> (Vec<usize>, usize) {
    let (n, m) = (t.len(), p.len());
    if m > n { return (vec![], 0); }
    // h = d^{m−1} mod q — the weight of the leading digit
    let mut h = 1u64;
    for _ in 1..m { h = h * d % q; }
    let (mut p0, mut t0) = (0u64, 0u64);
    for i in 0..m {
        p0 = (d * p0 + p[i] as u64) % q;
        t0 = (d * t0 + t[i] as u64) % q;
    }
    let (mut shifts, mut spurious) = (vec![], 0);
    for s in 0..=n - m {
        if p0 == t0 {
            if &t[s..s + m] == p { shifts.push(s); } else { spurious += 1; } // verify to rule out spurious hits
        }
        if s < n - m {
            // remove the leading digit, shift, append the new trailing digit
            t0 = (d * ((t0 + q - t[s] as u64 * h % q) % q) + t[s + m] as u64) % q;
        }
    }
    (shifts, spurious)
}

pub fn demo() -> String {
    let mut o = String::new();
    // Figure 32.5: T = 2359023141526739921, P = 31415, q = 13, d = 10
    let (t, p) = ("2359023141526739921", "31415");
    let (shifts, spurious) = rabin_karp(t.as_bytes(), p.as_bytes(), 10, 13);
    outln!(o, "T = {t}, P = {p}, q = 13: valid shifts {:?}, spurious hits {}", shifts, spurious);
    let text = "abracadabra".repeat(2000);
    for q in [101u64, 1_000_000_007] {
        let (s, sp) = rabin_karp(text.as_bytes(), b"cadabra", 256, q);
        outln!(o, "q = {:>10}: {} matches in {} chars, {} spurious hits", q, s.len(), text.len(), sp);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_figure() {
        let (s, sp) = rabin_karp(b"2359023141526739921", b"31415", 10, 13);
        assert_eq!(s, vec![6]);
        assert_eq!(sp, 1); // Figure 32.5: exactly one spurious hit with q = 13
    }
    #[test]
    fn agrees_with_naive_on_random_binary_strings() {
        let mut r = Rng::new(220);
        for _ in 0..200 {
            let t: Vec<u8> = (0..r.range(0, 60)).map(|_| b'a' + r.range(0, 1) as u8).collect();
            let p: Vec<u8> = (0..r.range(1, 5)).map(|_| b'a' + r.range(0, 1) as u8).collect();
            for q in [7u64, 101, 1_000_003] {
                assert_eq!(rabin_karp(&t, &p, 256, q).0, super::super::naive_string_matcher::naive_string_matcher(&t, &p));
            }
        }
    }
}
