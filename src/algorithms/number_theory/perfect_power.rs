//! @title Perfect-Power Test
//! @chapter Chapter 31 exercises (testing for perfect powers)
//! @editions 1,2,3,4
//! @summary Decide whether n = bᵏ for integers b > 1, k > 1 by trying every exponent k ≤ lg n with an integer k-th root by binary search.
//! @order 19
use crate::outln;
use std::cmp::Ordering;

/// Compare bᵏ with n without overflow.
fn cmp_pow(b: u64, k: u32, n: u64) -> Ordering {
    let mut acc: u64 = 1;
    for _ in 0..k {
        match acc.checked_mul(b) { Some(x) if x <= n => acc = x, _ => return Ordering::Greater }
    }
    acc.cmp(&n)
}

/// ⌊n^(1/k)⌋ by binary search on the answer.
pub fn kth_root(n: u64, k: u32) -> u64 {
    let (mut lo, mut hi) = (1u64, n.max(1));
    while lo < hi {
        let mid = lo + (hi - lo).div_ceil(2);
        if cmp_pow(mid, k, n) == Ordering::Greater { hi = mid - 1; } else { lo = mid; }
    }
    lo
}

/// If n = bᵏ with b ≥ 2 and k ≥ 2, return (b, k) with the **largest** such k; otherwise None. O(lg² n · lg n) bit operations.
pub fn perfect_power(n: u64) -> Option<(u64, u32)> {
    if n < 4 { return None; }
    (2..=63 - n.leading_zeros()).rev().find_map(|k| {
        let b = kth_root(n, k);
        (b >= 2 && cmp_pow(b, k, n) == Ordering::Equal).then_some((b, k))
    })
}

pub fn demo() -> String {
    let mut o = String::new();
    for n in [1u64, 2, 4, 36, 64, 100, 243, 1000, 1_000_003, 3_486_784_401, 1u64 << 62] {
        match perfect_power(n) {
            Some((b, k)) => outln!(o, "{n:>20} = {b}^{k}"),
            None => outln!(o, "{n:>20} is not a perfect power"),
        }
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_exhaustive_search() {
        for n in 1u64..6000 {
            let brute = (2..=n.max(2)).take_while(|b| b * b <= n).any(|b| { let mut x = b * b; while x < n { x *= b; } x == n });
            assert_eq!(perfect_power(n).is_some(), brute, "n = {n}");
            if let Some((b, k)) = perfect_power(n) { assert_eq!(b.pow(k), n); }
        }
    }
    #[test]
    fn roots_and_extremes() {
        assert_eq!(kth_root(1000, 3), 10);
        assert_eq!(kth_root(999, 3), 9);
        assert_eq!(perfect_power(3_486_784_401), Some((3, 20)));
        assert_eq!(perfect_power(1u64 << 62), Some((2, 62)));
        assert_eq!(perfect_power(u64::MAX), None);
    }
}
