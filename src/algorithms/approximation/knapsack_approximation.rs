//! @title Approximating the 0-1 Knapsack Problem
//! @chapter Problem 35-7 (3e)
//! @editions 3,4
//! @summary Greedy by value density (best of the greedy prefix and the most valuable single item) is a 2-approximation; scaling values and running the DP over values gives a fully polynomial-time approximation scheme.
//! @order 65
use crate::outln;

#[derive(Debug, Clone, Copy)]
pub struct Item { pub weight: u64, pub value: u64 }

/// Exact optimum by DP over capacities: O(n·W).
pub fn knapsack_exact(items: &[Item], cap: u64) -> u64 {
    let mut dp = vec![0u64; cap as usize + 1];
    for it in items { for c in (it.weight as usize..=cap as usize).rev() { dp[c] = dp[c].max(dp[c - it.weight as usize] + it.value); } }
    dp[cap as usize]
}

/// 2-approximation: sort by value/weight; take the longest feasible prefix; return the better of that and the best single item that fits.
pub fn knapsack_half(items: &[Item], cap: u64) -> u64 {
    let mut order: Vec<usize> = (0..items.len()).filter(|&i| items[i].weight <= cap).collect();
    order.sort_by(|&a, &b| (items[b].value as f64 / items[b].weight.max(1) as f64).partial_cmp(&(items[a].value as f64 / items[a].weight.max(1) as f64)).unwrap());
    let (mut w, mut v) = (0, 0);
    for &i in &order { if w + items[i].weight <= cap { w += items[i].weight; v += items[i].value; } else { break; } }
    v.max(order.iter().map(|&i| items[i].value).max().unwrap_or(0))
}

/// FPTAS: scale each value down to ⌊v·n/(ε·v_max)⌋, then DP over *scaled values* storing the minimum weight that achieves each value.
/// The returned total (in true values) is at least (1 − ε)·OPT, in time O(n³/ε).
pub fn knapsack_fptas(items: &[Item], cap: u64, eps: f64) -> u64 {
    let fit: Vec<&Item> = items.iter().filter(|it| it.weight <= cap).collect();
    let Some(vmax) = fit.iter().map(|it| it.value).max() else { return 0 };
    if vmax == 0 { return 0; }
    let k = eps * vmax as f64 / fit.len() as f64; // each scaled item loses less than k, the whole solution less than ε·vmax ≤ ε·OPT
    let scaled: Vec<u64> = fit.iter().map(|it| (it.value as f64 / k).floor() as u64).collect();
    let total: usize = scaled.iter().sum::<u64>() as usize;
    let inf = u64::MAX / 2;
    let mut minw = vec![vec![inf; total + 1]; fit.len() + 1];
    minw[0][0] = 0;
    for (i, it) in fit.iter().enumerate() {
        for v in 0..=total {
            minw[i + 1][v] = minw[i][v];
            if v >= scaled[i] as usize && minw[i][v - scaled[i] as usize] + it.weight < minw[i + 1][v] { minw[i + 1][v] = minw[i][v - scaled[i] as usize] + it.weight; }
        }
    }
    // best scaled value that fits, then recover the chosen set and report its true value
    let mut v = (0..=total).rev().find(|&v| minw[fit.len()][v] <= cap).unwrap();
    let mut value = 0;
    for i in (0..fit.len()).rev() {
        if minw[i + 1][v] != minw[i][v] { value += fit[i].value; v -= scaled[i] as usize; }
    }
    value
}

pub fn demo() -> String {
    let mut o = String::new();
    let items = [Item { weight: 10, value: 60 }, Item { weight: 20, value: 100 }, Item { weight: 30, value: 120 }, Item { weight: 25, value: 90 }, Item { weight: 5, value: 25 }];
    outln!(o, "capacity 50: exact {}, greedy/2-approximation {}", knapsack_exact(&items, 50), knapsack_half(&items, 50));
    for eps in [0.5, 0.2, 0.05] { outln!(o, "FPTAS with ε = {eps}: {}", knapsack_fptas(&items, 50, eps)); }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn approximation_guarantees() {
        let mut r = Rng::new(483);
        for _ in 0..300 {
            let n = r.range(1, 10);
            let items: Vec<Item> = (0..n).map(|_| Item { weight: r.range(1, 30) as u64, value: r.range(0, 100) as u64 }).collect();
            let cap = r.range(1, 80) as u64;
            let opt = knapsack_exact(&items, cap);
            assert!(2 * knapsack_half(&items, cap) >= opt, "half: {} vs {opt}", knapsack_half(&items, cap));
            for eps in [0.5f64, 0.2, 0.1] {
                let a = knapsack_fptas(&items, cap, eps);
                assert!(a <= opt && a as f64 >= (1.0 - eps) * opt as f64 - 1e-9, "eps {eps}: {a} vs {opt}");
            }
        }
    }
    #[test]
    fn small_eps_is_exact_on_the_textbook_style_instance() {
        let items = [Item { weight: 10, value: 60 }, Item { weight: 20, value: 100 }, Item { weight: 30, value: 120 }];
        assert_eq!(knapsack_exact(&items, 50), 220);
        assert_eq!(knapsack_fptas(&items, 50, 0.01), 220);
    }
}
