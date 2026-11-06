//! @title Maximum Subarray
//! @chapter 4.1 (3e); Exercise 4.1-5 (4e)
//! @editions 3,4
//! @summary Divide-and-conquer (and Kadane's linear scan) for the contiguous subarray of maximum sum.
//! @order 90
use crate::outln;

/// FIND-MAX-CROSSING-SUBARRAY: best subarray containing the midpoint. Returns (low, high, sum), inclusive.
pub fn find_max_crossing_subarray(a: &[i64], low: usize, mid: usize, high: usize) -> (usize, usize, i64) {
    let (mut left_sum, mut sum, mut max_left) = (i64::MIN, 0, mid);
    for i in (low..=mid).rev() {
        sum += a[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }
    let (mut right_sum, mut max_right) = (i64::MIN, mid + 1);
    sum = 0;
    for j in mid + 1..=high {
        sum += a[j];
        if sum > right_sum {
            right_sum = sum;
            max_right = j;
        }
    }
    (max_left, max_right, left_sum + right_sum)
}

fn rec(a: &[i64], low: usize, high: usize) -> (usize, usize, i64) {
    if low == high {
        return (low, high, a[low]);
    }
    let mid = (low + high) / 2;
    let l = rec(a, low, mid);
    let r = rec(a, mid + 1, high);
    let c = find_max_crossing_subarray(a, low, mid, high);
    if l.2 >= r.2 && l.2 >= c.2 { l } else if r.2 >= l.2 && r.2 >= c.2 { r } else { c }
}

/// FIND-MAXIMUM-SUBARRAY: Θ(n lg n) divide and conquer. Panics on empty input.
pub fn find_maximum_subarray(a: &[i64]) -> (usize, usize, i64) {
    rec(a, 0, a.len() - 1)
}

/// Θ(n²) brute force.
pub fn brute_force(a: &[i64]) -> (usize, usize, i64) {
    let mut best = (0, 0, a[0]);
    for i in 0..a.len() {
        let mut s = 0;
        for j in i..a.len() {
            s += a[j];
            if s > best.2 {
                best = (i, j, s);
            }
        }
    }
    best
}

/// Θ(n) scan (Exercise 4.1-5): extend the best subarray ending at each index.
pub fn kadane(a: &[i64]) -> (usize, usize, i64) {
    let (mut best, mut cur_start, mut cur) = ((0, 0, a[0]), 0, a[0]);
    for j in 1..a.len() {
        if cur < 0 {
            cur = a[j];
            cur_start = j;
        } else {
            cur += a[j];
        }
        if cur > best.2 {
            best = (cur_start, j, cur);
        }
    }
    best
}

pub fn demo() -> String {
    let mut o = String::new();
    // daily price changes of the stock example in the book
    let a = [13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];
    outln!(o, "A = {:?}", a);
    outln!(o, "divide & conquer : {:?}", find_maximum_subarray(&a));
    outln!(o, "kadane           : {:?}", kadane(&a));
    outln!(o, "brute force      : {:?}", brute_force(&a));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_example() {
        let a = [13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];
        assert_eq!(find_maximum_subarray(&a), (7, 10, 43));
    }
    #[test]
    fn all_methods_agree_on_sum() {
        let mut r = Rng::new(6);
        for n in 1..60 {
            let v = r.vec(n, -20, 20);
            let s = brute_force(&v).2;
            assert_eq!(find_maximum_subarray(&v).2, s);
            assert_eq!(kadane(&v).2, s);
        }
    }
    #[test]
    fn all_negative_picks_largest() {
        assert_eq!(kadane(&[-5, -2, -9]), (1, 1, -2));
        assert_eq!(find_maximum_subarray(&[-5, -2, -9]), (1, 1, -2));
    }
}
