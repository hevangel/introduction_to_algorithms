//! @title 0-1 Knapsack
//! @chapter Exercise 16.2-2 (2e–3e), 15.2-2 (4e)
//! @editions 1,2,3,4
//! @summary Pseudo-polynomial dynamic program O(nW) for items that cannot be split.
//! @order 90
use crate::outln;

/// Best total value and the chosen item indices for capacity `cap`; items are `(weight, value)`.
pub fn knapsack_01(items: &[(usize, u64)], cap: usize) -> (u64, Vec<usize>) {
    let n = items.len();
    let mut dp = vec![vec![0u64; cap + 1]; n + 1];
    for i in 1..=n {
        let (w, v) = items[i - 1];
        for c in 0..=cap {
            dp[i][c] = dp[i - 1][c];
            if c >= w && dp[i - 1][c - w] + v > dp[i][c] { dp[i][c] = dp[i - 1][c - w] + v; }
        }
    }
    let (mut c, mut chosen) = (cap, vec![]);
    for i in (1..=n).rev() {
        if dp[i][c] != dp[i - 1][c] { chosen.push(i - 1); c -= items[i - 1].0; }
    }
    chosen.reverse();
    (dp[n][cap], chosen)
}

/// Unbounded variant: unlimited copies of each item.
pub fn knapsack_unbounded(items: &[(usize, u64)], cap: usize) -> u64 {
    let mut dp = vec![0u64; cap + 1];
    for c in 1..=cap {
        for &(w, v) in items { if c >= w { dp[c] = dp[c].max(dp[c - w] + v); } }
    }
    dp[cap]
}

pub fn demo() -> String {
    let mut o = String::new();
    // the thief's example in Section 16.2: items worth $60/$100/$120 weighing 10/20/30 lb, capacity 50
    let items = [(10, 60), (20, 100), (30, 120)];
    let (best, chosen) = knapsack_01(&items, 50);
    outln!(o, "items (weight, value) = {:?}, capacity 50", items);
    outln!(o, "0-1 knapsack optimum = {} using items {:?}", best, chosen);
    outln!(o, "unbounded optimum    = {}", knapsack_unbounded(&items, 50));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn thief_example() {
        let (v, c) = knapsack_01(&[(10, 60), (20, 100), (30, 120)], 50);
        assert_eq!(v, 220);
        assert_eq!(c, vec![1, 2]);
    }
    #[test]
    fn matches_brute_force_and_respects_capacity() {
        let mut r = Rng::new(107);
        for _ in 0..80 {
            let n = r.range(0, 10);
            let items: Vec<(usize, u64)> = (0..n).map(|_| (r.range(1, 9), r.range(1, 30) as u64)).collect();
            let cap = r.range(0, 25);
            let (best, chosen) = knapsack_01(&items, cap);
            assert!(chosen.iter().map(|&i| items[i].0).sum::<usize>() <= cap);
            assert_eq!(chosen.iter().map(|&i| items[i].1).sum::<u64>(), best);
            let brute = (0..1u32 << n).filter(|m| (0..n).filter(|i| m >> i & 1 == 1).map(|i| items[i].0).sum::<usize>() <= cap)
                .map(|m| (0..n).filter(|i| m >> i & 1 == 1).map(|i| items[i].1).sum::<u64>()).max().unwrap();
            assert_eq!(best, brute);
            assert!(knapsack_unbounded(&items, cap) >= best);
        }
    }
}
