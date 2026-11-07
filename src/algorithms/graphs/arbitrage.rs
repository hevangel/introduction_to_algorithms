//! @title Currency Arbitrage
//! @chapter Problem 24-3 (1e–4e) / 22-3
//! @editions 1,2,3,4
//! @summary An arbitrage cycle is a cycle with product of exchange rates > 1 — a negative cycle for weights −ln R. Detect and print one with Bellman-Ford.
//! @order 125
use crate::outln;

/// Find a cycle of currencies whose exchange-rate product exceeds 1 (arbitrage), if any.
/// `r[i][j]` = units of currency j obtained for one unit of currency i. Returns (cycle, profit factor).
pub fn find_arbitrage(r: &[Vec<f64>]) -> Option<(Vec<usize>, f64)> {
    let n = r.len();
    let w = |i: usize, j: usize| -r[i][j].ln(); // product of rates > 1  ⇔  sum of −ln rates < 0
    let (mut d, mut pi) = (vec![0.0f64; n], vec![usize::MAX; n]); // virtual source: every currency starts at distance 0
    let mut last = None;
    for _ in 0..n {
        last = None;
        for i in 0..n { for j in 0..n { if i != j && d[i] + w(i, j) < d[j] - 1e-12 { d[j] = d[i] + w(i, j); pi[j] = i; last = Some(j); } } }
        last?;
    }
    let mut x = last?;
    for _ in 0..n { x = pi[x]; } // walk back n steps to land on the cycle
    let mut cycle = vec![x];
    let mut y = pi[x];
    while y != x { cycle.push(y); y = pi[y]; }
    cycle.reverse();
    let profit = (0..cycle.len()).map(|k| r[cycle[k]][cycle[(k + 1) % cycle.len()]]).product();
    Some((cycle, profit))
}

pub fn demo() -> String {
    let mut o = String::new();
    // 0 = USD, 1 = EUR, 2 = GBP, 3 = JPY
    let r = vec![
        vec![1.0, 0.90, 0.78, 110.0],
        vec![1.12, 1.0, 0.87, 123.0],
        vec![1.30, 1.15, 1.0, 142.0],
        vec![0.0090, 0.0081, 0.0070, 1.0],
    ];
    match find_arbitrage(&r) {
        Some((c, p)) => outln!(o, "arbitrage cycle {:?} multiplies money by {:.4}", c, p),
        None => outln!(o, "no arbitrage"),
    }
    let consistent: Vec<Vec<f64>> = (0..4).map(|i| (0..4).map(|j| [1.0, 0.9, 0.8, 110.0][j] / [1.0, 0.9, 0.8, 110.0][i]).collect()).collect();
    outln!(o, "rates derived from a single price vector: {:?}", find_arbitrage(&consistent).map(|x| x.0));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn detects_planted_arbitrage_and_returns_a_profitable_cycle() {
        let mut rng = Rng::new(358);
        for _ in 0..100 {
            let n = rng.range(2, 7);
            let price: Vec<f64> = (0..n).map(|_| 0.5 + rng.f64() * 100.0).collect();
            let mut r: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|j| price[j] / price[i]).collect()).collect();
            assert!(find_arbitrage(&r).is_none(), "consistent rates have no arbitrage");
            let (a, b) = (rng.range(0, n - 1), rng.range(0, n - 1));
            if a == b { continue; }
            r[a][b] *= 1.2; // a 20 % better direct rate creates a profitable 2-cycle a → b → a
            let (cycle, profit) = find_arbitrage(&r).expect("arbitrage must be found");
            assert!(profit > 1.0 + 1e-9, "profit {profit}");
            assert!(cycle.len() >= 2 && (0..cycle.len()).all(|k| cycle[k] != cycle[(k + 1) % cycle.len()]));
        }
    }
    #[test]
    fn matches_brute_force_on_random_tables() {
        let mut rng = Rng::new(359);
        for _ in 0..200 {
            let n = rng.range(2, 5);
            let r: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.5 + rng.f64() * 1.2 }).collect()).collect();
            // exhaustive: any simple cycle with product > 1?
            fn go(r: &[Vec<f64>], start: usize, u: usize, prod: f64, vis: &mut Vec<bool>) -> bool {
                for v in 0..r.len() {
                    if v == u { continue; }
                    if v == start { if prod * r[u][v] > 1.0 + 1e-9 { return true; } }
                    else if v > start && !vis[v] { vis[v] = true; let f = go(r, start, v, prod * r[u][v], vis); vis[v] = false; if f { return true; } }
                }
                false
            }
            let exists = (0..n).any(|s| { let mut vis = vec![false; n]; vis[s] = true; go(&r, s, s, 1.0, &mut vis) });
            assert_eq!(find_arbitrage(&r).is_some(), exists);
        }
    }
}
