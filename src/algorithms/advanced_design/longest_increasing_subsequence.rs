//! @title Longest Monotonically Increasing Subsequence
//! @chapter Problem 15-4 / Exercises 15.4-5, 15.4-6 (2e–3e), 14.4 (4e)
//! @editions 2,3,4
//! @summary O(n²) dynamic program and O(n lg n) patience-sorting variant using binary search.
//! @order 80
use crate::outln;

/// Θ(n²): `len[i]` = length of the longest increasing subsequence ending at i.
pub fn lis_quadratic(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    if n == 0 { return vec![]; }
    let (mut len, mut prev) = (vec![1usize; n], vec![usize::MAX; n]);
    for i in 0..n {
        for j in 0..i {
            if a[j] < a[i] && len[j] + 1 > len[i] { len[i] = len[j] + 1; prev[i] = j; }
        }
    }
    let mut end = (0..n).max_by_key(|&i| (len[i], std::cmp::Reverse(i))).unwrap();
    let mut out = vec![];
    while end != usize::MAX { out.push(a[end]); end = prev[end]; }
    out.reverse();
    out
}

/// Θ(n lg n): `tails[k]` is the smallest possible last element of an increasing subsequence of length k+1.
pub fn lis_nlogn(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    let (mut tails, mut tails_idx, mut prev) = (Vec::<i64>::new(), Vec::<usize>::new(), vec![usize::MAX; n]);
    for i in 0..n {
        let pos = tails.partition_point(|&t| t < a[i]); // binary search for the first tail ≥ a[i]
        if pos == tails.len() { tails.push(a[i]); tails_idx.push(i); } else { tails[pos] = a[i]; tails_idx[pos] = i; }
        if pos > 0 { prev[i] = tails_idx[pos - 1]; }
    }
    let mut out = vec![];
    let mut k = tails_idx.last().copied().unwrap_or(usize::MAX);
    while k != usize::MAX { out.push(a[k]); k = prev[k]; }
    out.reverse();
    out
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = [5, 2, 8, 6, 3, 6, 9, 7];
    outln!(o, "A = {:?}", a);
    outln!(o, "O(n²)      LIS = {:?}", lis_quadratic(&a));
    outln!(o, "O(n lg n)  LIS = {:?}", lis_nlogn(&a));
    let b = [10, 22, 9, 33, 21, 50, 41, 60, 80];
    outln!(o, "B = {:?}\n  LIS length (n lg n) = {}", b, lis_nlogn(&b).len());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn is_inc_subseq(s: &[i64], a: &[i64]) -> bool {
        s.windows(2).all(|w| w[0] < w[1]) && { let mut it = a.iter(); s.iter().all(|x| it.any(|y| y == x)) }
    }
    fn brute(a: &[i64]) -> usize {
        (0..1u32 << a.len()).filter(|m| { let v: Vec<i64> = (0..a.len()).filter(|i| m >> i & 1 == 1).map(|i| a[i]).collect(); v.windows(2).all(|w| w[0] < w[1]) }).map(|m| m.count_ones() as usize).max().unwrap()
    }
    #[test]
    fn examples() {
        assert_eq!(lis_nlogn(&[10, 22, 9, 33, 21, 50, 41, 60, 80]).len(), 6);
        assert_eq!(lis_quadratic(&[]), Vec::<i64>::new());
        assert_eq!(lis_nlogn(&[3, 3, 3]).len(), 1);
        assert_eq!(lis_nlogn(&[1, 2, 3, 4]), vec![1, 2, 3, 4]);
    }
    #[test]
    fn both_agree_with_brute_force() {
        let mut r = Rng::new(106);
        for _ in 0..100 {
            let n = r.range(1, 12);
            let a = r.vec(n, 0, 10);
            let (q, f) = (lis_quadratic(&a), lis_nlogn(&a));
            assert!(is_inc_subseq(&q, &a) && is_inc_subseq(&f, &a));
            assert_eq!(q.len(), brute(&a));
            assert_eq!(f.len(), q.len());
        }
    }
}
