//! @title Coin Changing
//! @chapter Problem 16-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Greedy change-making is optimal for powers of c, but not for arbitrary denominations; DP always works.
//! @order 190
use crate::outln;

/// Greedy: repeatedly take the largest coin that fits. Returns the coins used (None if exact change is impossible).
pub fn greedy_change(coins: &[u64], amount: u64) -> Option<Vec<u64>> {
    let mut cs = coins.to_vec();
    cs.sort_by(|a, b| b.cmp(a));
    let (mut rest, mut used) = (amount, vec![]);
    for c in cs {
        while rest >= c { rest -= c; used.push(c); }
    }
    if rest == 0 { Some(used) } else { None }
}

/// Dynamic programming: minimum number of coins and one optimal multiset (O(n·amount)).
pub fn dp_change(coins: &[u64], amount: u64) -> Option<Vec<u64>> {
    let a = amount as usize;
    let mut best = vec![usize::MAX; a + 1];
    let mut pick = vec![0u64; a + 1];
    best[0] = 0;
    for v in 1..=a {
        for &c in coins {
            if c as usize <= v && best[v - c as usize] != usize::MAX && best[v - c as usize] + 1 < best[v] {
                best[v] = best[v - c as usize] + 1;
                pick[v] = c;
            }
        }
    }
    if best[a] == usize::MAX { return None; }
    let (mut v, mut used) = (a, vec![]);
    while v > 0 { used.push(pick[v]); v -= pick[v] as usize; }
    Some(used)
}

/// Problem 16-1(a): for coins c⁰, c¹, …, cᵏ the greedy algorithm is optimal.
pub fn power_coins(c: u64, k: u32) -> Vec<u64> { (0..=k).map(|i| c.pow(i)).collect() }

pub fn demo() -> String {
    let mut o = String::new();
    let us = [25, 10, 5, 1];
    outln!(o, "US coins, 67¢: greedy {:?}", greedy_change(&us, 67).unwrap());
    let pw = power_coins(3, 4);
    outln!(o, "coins {:?}: 100 → greedy {:?}, dp {:?}", pw, greedy_change(&pw, 100).unwrap(), dp_change(&pw, 100).unwrap());
    let odd = [1, 3, 4];
    outln!(o, "coins {:?}: 6 → greedy {:?} (3 coins) but dp {:?} (2 coins)", odd, greedy_change(&odd, 6).unwrap(), dp_change(&odd, 6).unwrap());
    outln!(o, "coins [5,10]: 3 → greedy {:?}", greedy_change(&[5, 10], 3));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greedy_is_optimal_for_powers_of_c() {
        for c in 2..=5u64 {
            let coins = power_coins(c, 4);
            for amount in 0..400 {
                assert_eq!(greedy_change(&coins, amount).unwrap().len(), dp_change(&coins, amount).unwrap().len(), "c={c} n={amount}");
            }
        }
    }
    #[test]
    fn greedy_fails_in_general() {
        let coins = [1, 3, 4];
        assert_eq!(greedy_change(&coins, 6).unwrap().len(), 3);
        assert_eq!(dp_change(&coins, 6).unwrap().len(), 2);
    }
    #[test]
    fn us_coins_are_canonical_and_impossible_amounts() {
        let us = [25, 10, 5, 1];
        for n in 0..300 { assert_eq!(greedy_change(&us, n).unwrap().len(), dp_change(&us, n).unwrap().len()); }
        assert_eq!(dp_change(&[5, 10], 3), None);
        assert_eq!(greedy_change(&[2], 3), None);
        assert_eq!(dp_change(&[2], 0), Some(vec![]));
    }
}
