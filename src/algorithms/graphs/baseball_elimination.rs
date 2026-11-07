//! @title Baseball Elimination
//! @chapter Chapter 26 application (baseball elimination)
//! @editions 2,3,4
//! @summary Has a team been mathematically eliminated from first place? Test it with one maximum-flow computation: games → teams → sink, with team capacities w_k + r_k − w_i.
//! @order 196
use super::ford_fulkerson::FlowNetwork;
use crate::outln;

/// Teams 0..n have `wins[i]` wins so far; `games[i][j]` games remain between i and j (symmetric, zero diagonal).
/// Team `k` is eliminated if it cannot finish with at least as many wins as every other team, even if it wins all of its own remaining games.
pub fn eliminated(wins: &[i64], games: &[Vec<i64>], k: usize) -> bool {
    let n = wins.len();
    let best = wins[k] + games[k].iter().sum::<i64>(); // most wins team k can possibly reach
    if (0..n).any(|i| i != k && wins[i] > best) { return true; } // trivially eliminated
    // flow network: source → game nodes (i<j, both ≠ k) → team nodes → sink
    let others: Vec<usize> = (0..n).filter(|&i| i != k).collect();
    let pairs: Vec<(usize, usize)> = others.iter().flat_map(|&i| others.iter().filter(move |&&j| j > i).map(move |&j| (i, j))).filter(|&(i, j)| games[i][j] > 0).collect();
    let (s, t) = (0, 1);
    let team_node = |i: usize| 2 + i;
    let mut g = FlowNetwork::new(2 + n + pairs.len());
    let mut total = 0;
    for (idx, &(i, j)) in pairs.iter().enumerate() {
        let node = 2 + n + idx;
        g.add_edge(s, node, games[i][j]);
        g.add_edge(node, team_node(i), i64::MAX / 4);
        g.add_edge(node, team_node(j), i64::MAX / 4);
        total += games[i][j];
    }
    for &i in &others { g.add_edge(team_node(i), t, best - wins[i]); } // team i may win at most best − w_i of the remaining games
    g.edmonds_karp(s, t) < total
}

pub fn demo() -> String {
    let mut o = String::new();
    // an illustrative four-team season (team names are just labels)
    let wins = [83, 80, 78, 77];
    let g = vec![vec![0, 1, 6, 1], vec![1, 0, 0, 2], vec![6, 0, 0, 0], vec![1, 2, 0, 0]];
    for (k, name) in ["Atlanta", "Philadelphia", "New York", "Montreal"].iter().enumerate() {
        outln!(o, "{name:<13} wins {} remaining {}: eliminated = {}", wins[k], g[k].iter().sum::<i64>(), eliminated(&wins, &g, k));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    /// Exhaustive: can the other teams' remaining games be decided so that nobody exceeds `best`?
    fn feasible(wins: &mut Vec<i64>, games: &[(usize, usize)], idx: usize, best: i64) -> bool {
        if wins.iter().any(|&w| w > best) { return false; }
        if idx == games.len() { return true; }
        let (i, j) = games[idx];
        for w in [i, j] { wins[w] += 1; let ok = feasible(wins, games, idx + 1, best); wins[w] -= 1; if ok { return true; } }
        false
    }
    fn brute(wins: &[i64], games: &[Vec<i64>], k: usize) -> bool {
        let n = wins.len();
        let best = wins[k] + games[k].iter().sum::<i64>();
        let mut list = vec![];
        for i in 0..n { for j in i + 1..n { if i != k && j != k { for _ in 0..games[i][j] { list.push((i, j)); } } } }
        let mut w = wins.to_vec();
        w[k] = best; // k wins all of its own remaining games
        !feasible(&mut w, &list, 0, best)
    }
    #[test]
    fn matches_exhaustive_enumeration_of_outcomes() {
        let mut r = Rng::new(454);
        let mut count = 0;
        while count < 300 {
            let n = r.range(2, 5);
            let wins: Vec<i64> = (0..n).map(|_| r.range(0, 8) as i64).collect();
            let mut g = vec![vec![0i64; n]; n];
            for i in 0..n { for j in i + 1..n { let x = r.range(0, 2) as i64; g[i][j] = x; g[j][i] = x; } }
            if (0..n).map(|i| (i + 1..n).map(|j| g[i][j]).sum::<i64>()).sum::<i64>() > 12 { continue; }
            for k in 0..n { assert_eq!(eliminated(&wins, &g, k), brute(&wins, &g, k), "wins {wins:?} games {g:?} k {k}"); }
            count += 1;
        }
    }
    #[test]
    fn a_team_with_the_most_wins_is_never_eliminated() {
        let wins = [10, 3, 3];
        let g = vec![vec![0, 0, 0], vec![0, 0, 4], vec![0, 4, 0]];
        assert!(!eliminated(&wins, &g, 0));
        assert!(eliminated(&wins, &g, 1)); // 3 + 0 remaining wins < 10
    }
}
