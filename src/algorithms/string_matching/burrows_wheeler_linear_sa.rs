//! @title Linear-Time Suffix Array & Burrows-Wheeler Transform
//! @chapter Problems 32-2 & 32-3 (4e)
//! @editions 4
//! @summary O(n) skew (DC3) or SA-IS suffix array construction plus invertible Burrows-Wheeler Transform and O(n) LF-mapping reconstruction.
//! @order 70
use crate::outln;

/// Counting-sort `indices` by `keys[idx + offset]` into `0..=alphabet_max`.
fn radix_pass(indices: &[usize], keys: &[usize], offset: usize, alphabet_max: usize) -> Vec<usize> {
    let mut counts = vec![0usize; alphabet_max + 2];
    for &idx in indices {
        counts[keys[idx + offset] + 1] += 1;
    }
    for i in 1..counts.len() {
        counts[i] += counts[i - 1];
    }
    let mut out = vec![0usize; indices.len()];
    for &idx in indices {
        let key = keys[idx + offset];
        out[counts[key]] = idx;
        counts[key] += 1;
    }
    out
}

/// Core O(n) Kärkkäinen–Sanders DC3 / Skew recursion.
/// Requires `s` to have length >= `n + 3` with `s[0..n]` in `1..=k` and `s[n..n+3] == 0`.
fn dc3_rec(s: &[usize], n: usize, k: usize) -> Vec<usize> {
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![0];
    }
    let n0 = n.div_ceil(3);
    let n1 = (n + 1) / 3;
    let n2 = n / 3;
    let n02 = n0 + n2;

    // Include dummy index n (which is 1 mod 3) whenever n % 3 == 1 so every 0 mod 3 index has an i+1 in s12.
    let mut s12 = Vec::with_capacity(n02);
    for i in 0..(n + n0 - n1) {
        if i % 3 != 0 {
            s12.push(i);
        }
    }

    // Radix sort triples (s[i], s[i+1], s[i+2]).
    let s12 = radix_pass(&s12, s, 2, k);
    let s12 = radix_pass(&s12, s, 1, k);
    let sa12_raw = radix_pass(&s12, s, 0, k);

    // Assign lexicographical names 1..=name to triples.
    let mut name = 0usize;
    let mut prev_triple = (usize::MAX, usize::MAX, usize::MAX);
    let mut s12_rec = vec![0usize; n02 + 3];
    for &idx in &sa12_raw {
        let cur = (s[idx], s[idx + 1], s[idx + 2]);
        if cur != prev_triple {
            name += 1;
            prev_triple = cur;
        }
        if idx % 3 == 1 {
            s12_rec[idx / 3] = name;
        } else {
            s12_rec[idx / 3 + n0] = name;
        }
    }

    let sa12 = if name < n02 {
        let rec_sa = dc3_rec(&s12_rec, n02, name);
        rec_sa
            .into_iter()
            .map(|r| if r < n0 { 3 * r + 1 } else { 3 * (r - n0) + 2 })
            .collect()
    } else {
        sa12_raw
    };

    // Rank array for i in 0..n + 3 (`rank[i] == 0` when i >= n + (n0 - n1)).
    let mut rank = vec![0usize; n + 3];
    for (r, &idx) in sa12.iter().enumerate() {
        rank[idx] = r + 1;
    }

    // Collect i == 0 mod 3 ordered by suffix(i + 1) from sa12, then stable-sort by s[i].
    let s0_pre: Vec<usize> = sa12
        .iter()
        .copied()
        .filter(|&idx| idx % 3 == 1)
        .map(|idx| idx - 1)
        .collect();
    let sa0 = radix_pass(&s0_pre, s, 0, k);

    // Merge sorted sa0 and sorted sa12.
    let mut sa = Vec::with_capacity(n);
    let mut t = 0usize;
    let mut p = n0 - n1; // skips dummy index `n` (which always sorts first at index 0 of sa12)
    while t < n0 && p < n02 {
        let i = sa12[p];
        let j = sa0[t];
        let i_smaller = if i % 3 == 1 {
            (s[i], rank[i + 1]) < (s[j], rank[j + 1])
        } else {
            (s[i], s[i + 1], rank[i + 2]) < (s[j], s[j + 1], rank[j + 2])
        };
        if i_smaller {
            sa.push(i);
            p += 1;
        } else {
            sa.push(j);
            t += 1;
        }
    }
    while p < n02 {
        sa.push(sa12[p]);
        p += 1;
    }
    while t < n0 {
        sa.push(sa0[t]);
        t += 1;
    }
    sa
}

/// Problem 32-2: Linear-time O(n) suffix array construction using Kärkkäinen–Sanders Skew (DC3).
pub fn dc3_suffix_array(text: &[u8]) -> Vec<usize> {
    let n = text.len();
    if n <= 1 {
        return (0..n).collect();
    }
    let mut s = vec![0usize; n + 3];
    for (i, &b) in text.iter().enumerate() {
        s[i] = b as usize + 1;
    }
    dc3_rec(&s, n, 256)
}

/// Problem 32-3: Burrows–Wheeler Transform in O(n) time via DC3 suffix array over `text + sentinel`.
/// Returns `(bwt_bytes, sentinel_row)` where `bwt_bytes` has the same length as `text` and
/// `sentinel_row` is the row where the unique virtual sentinel `$` resides in the BWT last column.
pub fn bwt_encode(text: &[u8]) -> (Vec<u8>, usize) {
    let n = text.len();
    if n == 0 {
        return (vec![], 0);
    }
    let m = n + 1;
    let mut s = vec![0usize; m + 3];
    for (i, &b) in text.iter().enumerate() {
        s[i] = b as usize + 2;
    }
    s[n] = 1; // smallest non-zero symbol acts as unique `$`
    let sa = dc3_rec(&s, m, 257);

    let mut bwt = Vec::with_capacity(n);
    let mut sentinel_row = 0usize;
    for (row, &pos) in sa.iter().enumerate() {
        if pos == 0 {
            sentinel_row = row;
        } else {
            bwt.push(text[pos - 1]);
        }
    }
    (bwt, sentinel_row)
}

/// Problem 32-3 inverse: O(n) Burrows–Wheeler inversion via C-table and LF-mapping rank walk.
pub fn bwt_decode(bwt: &[u8], sentinel_row: usize) -> Vec<u8> {
    let n = bwt.len();
    if n == 0 {
        return vec![];
    }
    let mut counts = [0usize; 256];
    let mut ranks = vec![0usize; n];
    for (i, &b) in bwt.iter().enumerate() {
        ranks[i] = counts[b as usize];
        counts[b as usize] += 1;
    }
    // Offset starts at 1 because the unique `$` is placed at F[0].
    let mut c_start = [0usize; 256];
    let mut acc = 1usize;
    for b in 0..256 {
        c_start[b] = acc;
        acc += counts[b];
    }

    let mut out = Vec::with_capacity(n);
    let mut cur = 0usize; // start at F[0] = '$' (row 0 in sorted suffixes)
    for _ in 0..n {
        if cur == sentinel_row {
            break;
        }
        let adj = if cur > sentinel_row { cur - 1 } else { cur };
        let ch = bwt[adj];
        out.push(ch);
        cur = c_start[ch as usize] + ranks[adj];
    }
    out.reverse();
    out
}

pub fn demo() -> String {
    let mut o = String::new();
    let text = b"mississippi";
    let sa = dc3_suffix_array(text);
    outln!(o, "DC3 suffix array of {:?}: {:?}", String::from_utf8_lossy(text), sa);
    let (bwt, srow) = bwt_encode(text);
    let back = bwt_decode(&bwt, srow);
    outln!(o, "BWT(mississippi) = {:?} (sentinel row {srow}), decoded = {:?}", String::from_utf8_lossy(&bwt), String::from_utf8_lossy(&back));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn dc3_matches_suffix_array_across_random_and_periodic_strings() {
        let mut r = Rng::new(3202);
        // Edge lengths, all-equal, periodic, Fibonacci words
        let mut cases: Vec<Vec<u8>> = vec![
            b"".to_vec(),
            b"a".to_vec(),
            b"aa".to_vec(),
            b"ab".to_vec(),
            b"ba".to_vec(),
            b"aaaaaa".to_vec(),
            b"banana".to_vec(),
            b"mississippi".to_vec(),
            b"abracadabra".to_vec(),
        ];
        // Fibonacci words
        let (mut f0, mut f1) = (b"a".to_vec(), b"b".to_vec());
        for _ in 0..9 {
            let mut next = f1.clone();
            next.extend_from_slice(&f0);
            cases.push(next.clone());
            f0 = f1;
            f1 = next;
        }
        for _ in 0..80 {
            let n = r.range(1, 90);
            let alpha = r.range(2, 6);
            let s: Vec<u8> = (0..n).map(|_| b'a' + r.range(0, alpha - 1) as u8).collect();
            cases.push(s);
        }
        for s in &cases {
            let got = dc3_suffix_array(s);
            let want = super::super::suffix_array::suffix_array(s);
            assert_eq!(got, want, "DC3 mismatch for {:?}", String::from_utf8_lossy(s));
        }
    }

    #[test]
    fn bwt_encode_decode_roundtrips_for_all_lengths_and_arbitrary_bytes() {
        let mut r = Rng::new(3203);
        for len in 0..=120 {
            let s: Vec<u8> = (0..len).map(|_| r.range(0, 255) as u8).collect();
            let (bwt, srow) = bwt_encode(&s);
            assert_eq!(bwt.len(), s.len());
            let decoded = bwt_decode(&bwt, srow);
            assert_eq!(decoded, s, "roundtrip failed for len={len}");
        }
        // Classic textbook check
        let (bwt, srow) = bwt_encode(b"banana");
        assert_eq!(&bwt, b"annbaa");
        assert_eq!(srow, 4);
        assert_eq!(&bwt_decode(&bwt, srow), b"banana");
    }
}
