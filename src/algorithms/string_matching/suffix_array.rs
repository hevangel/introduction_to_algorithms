//! @title Suffix Arrays & LCP Arrays
//! @chapter 32.5 (4e)
//! @editions 4
//! @summary Sorted array of all suffixes (prefix doubling) with Kasai's LCP array: substring search in O(m lg n).
//! @order 60
use crate::outln;

/// SUFFIX-ARRAY(T): indices of all suffixes of T in lexicographic order (prefix doubling, O(n lg² n)).
pub fn suffix_array(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rank: Vec<usize> = s.iter().map(|&c| c as usize).collect();
    let mut k = 1;
    while k < n.max(1) {
        // sort by the pair (rank of first k chars, rank of next k chars)
        let key = |i: usize| (rank[i], if i + k < n { rank[i + k] + 1 } else { 0 });
        sa.sort_by_key(|&i| key(i));
        let mut new_rank = vec![0; n];
        for w in 1..n { new_rank[sa[w]] = new_rank[sa[w - 1]] + (key(sa[w]) != key(sa[w - 1])) as usize; }
        rank = new_rank;
        if rank[sa[n - 1]] == n - 1 { break; } // all ranks distinct
        k *= 2;
    }
    sa
}

/// LCP array by Kasai et al.: lcp[i] = length of the longest common prefix of suffixes sa[i-1] and sa[i] (lcp[0] = 0). O(n).
pub fn lcp_array(s: &[u8], sa: &[usize]) -> Vec<usize> {
    let n = s.len();
    let mut rank = vec![0; n];
    for (i, &p) in sa.iter().enumerate() { rank[p] = i; }
    let (mut lcp, mut h) = (vec![0; n], 0);
    for i in 0..n {
        if rank[i] > 0 {
            let j = sa[rank[i] - 1];
            while i + h < n && j + h < n && s[i + h] == s[j + h] { h += 1; }
            lcp[rank[i]] = h;
            h = h.saturating_sub(1);
        } else {
            h = 0;
        }
    }
    lcp
}

/// All occurrences of `p` in `s` by binary searching the suffix array: O(m lg n + occurrences).
pub fn search(s: &[u8], sa: &[usize], p: &[u8]) -> Vec<usize> {
    let lo = sa.partition_point(|&i| &s[i..(i + p.len()).min(s.len())] < p);
    let hi = sa.partition_point(|&i| &s[i..(i + p.len()).min(s.len())] <= p);
    let mut r: Vec<usize> = sa[lo..hi].to_vec();
    r.sort();
    r
}

/// Longest repeated substring: the maximum LCP value.
pub fn longest_repeated_substring(s: &[u8]) -> &[u8] {
    let sa = suffix_array(s);
    let lcp = lcp_array(s, &sa);
    match (0..s.len()).max_by_key(|&i| lcp[i]) {
        Some(i) if lcp[i] > 0 => &s[sa[i]..sa[i] + lcp[i]],
        _ => &s[0..0],
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let t = b"banana";
    let sa = suffix_array(t);
    let lcp = lcp_array(t, &sa);
    outln!(o, "T = banana");
    outln!(o, " i | SA[i] | LCP[i] | suffix");
    for i in 0..t.len() { outln!(o, "{:>2} | {:>5} | {:>6} | {}", i, sa[i], lcp[i], String::from_utf8_lossy(&t[sa[i]..])); }
    outln!(o, "occurrences of 'ana': {:?}", search(t, &sa, b"ana"));
    outln!(o, "longest repeated substring: {:?}", String::from_utf8_lossy(longest_repeated_substring(b"abracadabra")));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn banana() {
        let sa = suffix_array(b"banana");
        assert_eq!(sa, vec![5, 3, 1, 0, 4, 2]);
        assert_eq!(lcp_array(b"banana", &sa), vec![0, 1, 3, 0, 0, 2]);
        assert_eq!(search(b"banana", &sa, b"ana"), vec![1, 3]);
        assert_eq!(longest_repeated_substring(b"abracadabra"), b"abra");
    }
    #[test]
    fn matches_sorting_all_suffixes_and_naive_search() {
        let mut r = Rng::new(224);
        for _ in 0..200 {
            let s: Vec<u8> = (0..r.range(0, 40)).map(|_| b'a' + r.range(0, 2) as u8).collect();
            let sa = suffix_array(&s);
            let mut want: Vec<usize> = (0..s.len()).collect();
            want.sort_by(|&a, &b| s[a..].cmp(&s[b..]));
            assert_eq!(sa, want);
            let lcp = lcp_array(&s, &sa);
            for i in 1..s.len() {
                let (a, b) = (&s[sa[i - 1]..], &s[sa[i]..]);
                assert_eq!(lcp[i], a.iter().zip(b).take_while(|(x, y)| x == y).count());
            }
            let p: Vec<u8> = (0..r.range(1, 4)).map(|_| b'a' + r.range(0, 2) as u8).collect();
            assert_eq!(search(&s, &sa, &p), super::super::naive_string_matcher::naive_string_matcher(&s, &p));
        }
    }
}
