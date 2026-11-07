//! @title Sorting Variable-Length Items
//! @chapter Problem 8-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Sort integers of different digit counts in time linear in the total number of digits (group by length, then radix sort), and strings in time linear in their total length (MSD radix sort).
//! @order 217
use crate::outln;

fn digit_count(mut x: u64) -> usize { let mut d = 1; while x >= 10 { x /= 10; d += 1; } d }

/// Problem 8-3(a): sort non-negative integers with varying numbers of digits in O(total digits).
/// Counting sort by digit count first (shorter numbers are smaller), then LSD radix sort inside each group of equal digit count.
pub fn sort_integers_by_digits(a: &[u64]) -> Vec<u64> {
    let maxd = a.iter().map(|&x| digit_count(x)).max().unwrap_or(0);
    let mut groups: Vec<Vec<u64>> = vec![vec![]; maxd + 1];
    for &x in a { groups[digit_count(x)].push(x); }          // counting sort on the digit count
    let mut out = Vec::with_capacity(a.len());
    for (d, mut g) in groups.into_iter().enumerate() {
        let mut p = 1u64;
        for _ in 0..d {                                       // d passes of a stable bucket sort on one decimal digit
            let mut buckets: Vec<Vec<u64>> = vec![vec![]; 10];
            for &x in &g { buckets[(x / p % 10) as usize].push(x); }
            g = buckets.into_iter().flatten().collect();
            p = p.saturating_mul(10);
        }
        out.extend(g);
    }
    out
}

/// Problem 8-3(b): sort strings (byte arrays) lexicographically — a proper prefix sorts before its extensions — with MSD radix sort.
/// Each string character is examined once for each level it takes to separate the string from its neighbours: O(total length + number of buckets visited).
pub fn sort_strings(a: &[Vec<u8>]) -> Vec<Vec<u8>> {
    fn rec<'a>(items: Vec<&'a [u8]>, depth: usize, out: &mut Vec<&'a [u8]>) {
        if items.len() <= 1 { out.extend(items); return; }
        let mut ended = vec![];
        let mut buckets: Vec<Vec<&[u8]>> = vec![vec![]; 256];
        for s in items { if s.len() == depth { ended.push(s); } else { buckets[s[depth] as usize].push(s); } }
        out.extend(ended);                                   // strings that end here come first
        for b in buckets { if !b.is_empty() { rec(b, depth + 1, out); } }
    }
    let mut out = Vec::with_capacity(a.len());
    rec(a.iter().map(|s| s.as_slice()).collect(), 0, &mut out);
    out.into_iter().map(|s| s.to_vec()).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    let nums = [1000u64, 7, 42, 123456, 9, 100, 99999, 12, 0, 5000];
    outln!(o, "integers {:?}", nums);
    outln!(o, "sorted    {:?}", sort_integers_by_digits(&nums));
    let words: Vec<Vec<u8>> = ["banana", "app", "apple", "", "band", "ban", "apply", "a"].iter().map(|s| s.as_bytes().to_vec()).collect();
    let sorted = sort_strings(&words);
    outln!(o, "strings   {:?}", sorted.iter().map(|s| String::from_utf8_lossy(s).to_string()).collect::<Vec<_>>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn integers_match_std_sort() {
        let mut r = Rng::new(441);
        for _ in 0..200 {
            let n = r.range(0, 60);
            let a: Vec<u64> = (0..n).map(|_| { let digits = r.range(0, 12) as u32; r.next_u64() % 10u64.pow(digits + 1) }).collect();
            let mut e = a.clone();
            e.sort();
            assert_eq!(sort_integers_by_digits(&a), e);
        }
    }
    #[test]
    fn strings_match_std_sort() {
        let mut r = Rng::new(442);
        for _ in 0..200 {
            let a: Vec<Vec<u8>> = (0..r.range(0, 40)).map(|_| (0..r.range(0, 8)).map(|_| b'a' + r.range(0, 2) as u8).collect()).collect();
            let mut e = a.clone();
            e.sort();
            assert_eq!(sort_strings(&a), e);
        }
        assert_eq!(sort_strings(&[b"b".to_vec(), b"".to_vec(), b"ab".to_vec(), b"a".to_vec()]), vec![b"".to_vec(), b"a".to_vec(), b"ab".to_vec(), b"b".to_vec()]);
    }
}
