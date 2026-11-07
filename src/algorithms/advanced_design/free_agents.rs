//! @title Signing Free-Agent Players
//! @chapter Problem 15-12 (3e) / 14-12 (4e)
//! @editions 3,4
//! @summary Fill every position with exactly one free agent, maximising total VORP under a salary budget: multiple-choice knapsack in O(N·X·P).
//! @order 270
use crate::outln;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player { pub salary: usize, pub vorp: i64 }

/// For each position a list of candidates; choose exactly one per position with total salary ≤ budget, maximising total VORP.
/// Returns (best VORP, chosen candidate index per position) or None if no assignment fits the budget.
pub fn sign_players(positions: &[Vec<Player>], budget: usize) -> Option<(i64, Vec<usize>)> {
    let n = positions.len();
    let neg = i64::MIN / 4;
    // v[i][x]: best VORP for positions 0..i with total salary exactly ≤ x (neg if impossible)
    let mut v = vec![vec![neg; budget + 1]; n + 1];
    let mut pick = vec![vec![usize::MAX; budget + 1]; n + 1];
    v[0] = vec![0; budget + 1];
    for i in 1..=n {
        for x in 0..=budget {
            for (c, p) in positions[i - 1].iter().enumerate() {
                if p.salary <= x && v[i - 1][x - p.salary] > neg && v[i - 1][x - p.salary] + p.vorp > v[i][x] { v[i][x] = v[i - 1][x - p.salary] + p.vorp; pick[i][x] = c; }
            }
        }
    }
    if v[n][budget] <= neg { return None; }
    let (mut x, mut chosen) = (budget, vec![0; n]);
    for i in (1..=n).rev() { let c = pick[i][x]; chosen[i - 1] = c; x -= positions[i - 1][c].salary; }
    Some((v[n][budget], chosen))
}

pub fn demo() -> String {
    let mut o = String::new();
    let pos = vec![
        vec![Player { salary: 5, vorp: 6 }, Player { salary: 2, vorp: 2 }, Player { salary: 8, vorp: 9 }],   // pitcher
        vec![Player { salary: 4, vorp: 5 }, Player { salary: 1, vorp: 1 }],                                   // catcher
        vec![Player { salary: 6, vorp: 7 }, Player { salary: 3, vorp: 3 }, Player { salary: 2, vorp: 1 }],   // shortstop
    ];
    for budget in [6usize, 9, 12, 20] {
        outln!(o, "budget {:>2}: {:?}", budget, sign_players(&pos, budget));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn brute(pos: &[Vec<Player>], i: usize, left: usize) -> Option<i64> {
        if i == pos.len() { return Some(0); }
        pos[i].iter().filter(|p| p.salary <= left).filter_map(|p| brute(pos, i + 1, left - p.salary).map(|r| r + p.vorp)).max()
    }
    #[test]
    fn matches_enumeration() {
        let mut r = Rng::new(344);
        for _ in 0..200 {
            let positions: Vec<Vec<Player>> = (0..r.range(1, 5)).map(|_| (0..r.range(1, 4)).map(|_| Player { salary: r.range(0, 8), vorp: r.range(0, 20) as i64 }).collect()).collect();
            let budget = r.range(0, 25);
            let got = sign_players(&positions, budget);
            assert_eq!(got.as_ref().map(|x| x.0), brute(&positions, 0, budget));
            if let Some((v, ch)) = got {
                assert_eq!(ch.iter().enumerate().map(|(i, &c)| positions[i][c].vorp).sum::<i64>(), v);
                assert!(ch.iter().enumerate().map(|(i, &c)| positions[i][c].salary).sum::<usize>() <= budget);
            }
        }
    }
    #[test]
    fn infeasible_budget() {
        assert_eq!(sign_players(&[vec![Player { salary: 5, vorp: 1 }]], 4), None);
    }
}
