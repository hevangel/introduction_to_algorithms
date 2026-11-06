//! @title Subset Sum
//! @chapter 34.5.5 (1e–3e) / 36.5.5 (4e); 35.5 (3e)
//! @editions 1,2,3,4
//! @summary NP-complete in the input length yet solvable in O(nt) pseudo-polynomial time; meet-in-the-middle O(2^{n/2}).
//! @order 50
use crate::outln;

/// Exhaustive search over subsets (Θ(2ⁿ)).
pub fn exhaustive(nums: &[i64], target: i64) -> Option<Vec<usize>> {
    (0u32..1 << nums.len()).find(|m| (0..nums.len()).filter(|i| m >> i & 1 == 1).map(|i| nums[i]).sum::<i64>() == target)
        .map(|m| (0..nums.len()).filter(|i| m >> i & 1 == 1).collect())
}

/// Pseudo-polynomial dynamic program for non-negative numbers: O(n·t) time. Returns indices of a solution.
pub fn dp(nums: &[i64], target: i64) -> Option<Vec<usize>> {
    if target < 0 { return None; }
    let t = target as usize;
    let mut reach = vec![vec![false; t + 1]; nums.len() + 1];
    reach[0][0] = true;
    for (i, &x) in nums.iter().enumerate() {
        for s in 0..=t {
            reach[i + 1][s] = reach[i][s] || (x >= 0 && s >= x as usize && reach[i][s - x as usize]);
        }
    }
    if !reach[nums.len()][t] { return None; }
    let (mut s, mut idx) = (t, vec![]);
    for i in (0..nums.len()).rev() {
        if !reach[i][s] { idx.push(i); s -= nums[i] as usize; }
    }
    idx.reverse();
    Some(idx)
}

/// Meet in the middle: enumerate sums of each half and match them with a sorted two-pointer sweep — O(2^{n/2}·n).
pub fn meet_in_the_middle(nums: &[i64], target: i64) -> Option<Vec<usize>> {
    let (h, n) = (nums.len() / 2, nums.len());
    let sums = |lo: usize, hi: usize| -> Vec<(i64, u32)> {
        let mut v: Vec<(i64, u32)> = (0u32..1 << (hi - lo)).map(|m| ((0..hi - lo).filter(|i| m >> i & 1 == 1).map(|i| nums[lo + i]).sum(), m)).collect();
        v.sort();
        v
    };
    let (a, b) = (sums(0, h), sums(h, n));
    let (mut i, mut j) = (0, b.len() as isize - 1);
    while i < a.len() && j >= 0 {
        let s = a[i].0 + b[j as usize].0;
        if s == target {
            let mut idx: Vec<usize> = (0..h).filter(|k| a[i].1 >> k & 1 == 1).collect();
            idx.extend((0..n - h).filter(|k| b[j as usize].1 >> k & 1 == 1).map(|k| h + k));
            return Some(idx);
        }
        if s < target { i += 1 } else { j -= 1 }
    }
    None
}

pub fn demo() -> String {
    let mut o = String::new();
    let nums = [3, 34, 4, 12, 5, 2];
    for t in [9, 30, 58, 1] {
        outln!(o, "target {:>2}: exhaustive {:?}  dp {:?}  meet-in-the-middle {:?}", t, exhaustive(&nums, t), dp(&nums, t), meet_in_the_middle(&nums, t));
    }
    outln!(o, "dp table size is n·t — exponential in the *bit length* of t, hence pseudo-polynomial");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn check(nums: &[i64], t: i64, idx: &[usize]) { assert!(super::super::np_verification::verify_subset_sum(nums, t, idx)); }
    #[test]
    fn three_solvers_agree() {
        let mut r = Rng::new(247);
        for _ in 0..300 {
            let n = r.range(0, 12);
            let nums = r.vec(n, 1, 30);
            let t = r.range(0, 120) as i64;
            let (e, d, m) = (exhaustive(&nums, t), dp(&nums, t), meet_in_the_middle(&nums, t));
            assert_eq!(e.is_some(), d.is_some());
            assert_eq!(e.is_some(), m.is_some(), "{nums:?} t={t}");
            for sol in [e, d, m].into_iter().flatten() { check(&nums, t, &sol); }
        }
    }
    #[test]
    fn edge_cases() {
        assert_eq!(dp(&[], 0), Some(vec![]));
        assert_eq!(dp(&[5], 0), Some(vec![]));
        assert_eq!(dp(&[5], -1), None);
        assert!(meet_in_the_middle(&[1, 2, 3], 7).is_none());
    }
}
