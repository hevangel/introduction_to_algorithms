//! @title Hungarian Algorithm (Assignment Problem)
//! @chapter 25.3 (4e)
//! @editions 4
//! @summary Minimum-cost perfect matching in a complete bipartite graph in O(n³) using vertex potentials.
//! @order 210
use crate::outln;

/// Minimum-cost assignment for an n×n cost matrix (a[i][j] = cost of giving job j to worker i).
/// Returns (total cost, assignment[i] = job of worker i).
pub fn hungarian(cost: &[Vec<i64>]) -> (i64, Vec<usize>) {
    let n = cost.len();
    if n == 0 { return (0, vec![]); }
    const INF: i64 = i64::MAX / 4;
    // 1-based arrays; u, v are potentials, p[j] = worker matched to job j, way[j] = previous job on the alternating path
    let (mut u, mut v) = (vec![0i64; n + 1], vec![0i64; n + 1]);
    let (mut p, mut way) = (vec![0usize; n + 1], vec![0usize; n + 1]);
    for i in 1..=n {
        p[0] = i;
        let mut j0 = 0;
        let mut minv = vec![INF; n + 1];
        let mut used = vec![false; n + 1];
        loop {
            used[j0] = true;
            let (i0, mut delta, mut j1) = (p[j0], INF, 0);
            for j in 1..=n {
                if !used[j] {
                    let cur = cost[i0 - 1][j - 1] - u[i0] - v[j];
                    if cur < minv[j] { minv[j] = cur; way[j] = j0; }
                    if minv[j] < delta { delta = minv[j]; j1 = j; }
                }
            }
            for j in 0..=n {
                if used[j] { u[p[j]] += delta; v[j] -= delta; } else { minv[j] -= delta; }
            }
            j0 = j1;
            if p[j0] == 0 { break; }
        }
        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == 0 { break; }
        }
    }
    let mut assign = vec![0; n];
    for j in 1..=n { assign[p[j] - 1] = j - 1; }
    let total = (0..n).map(|i| cost[i][assign[i]]).sum();
    (total, assign)
}

/// Maximum-weight assignment (the 4th edition's formulation) by negating weights.
pub fn max_weight_assignment(w: &[Vec<i64>]) -> (i64, Vec<usize>) {
    let neg: Vec<Vec<i64>> = w.iter().map(|r| r.iter().map(|x| -x).collect()).collect();
    let (c, a) = hungarian(&neg);
    (-c, a)
}

pub fn demo() -> String {
    let mut o = String::new();
    let cost = vec![vec![9, 2, 7, 8], vec![6, 4, 3, 7], vec![5, 8, 1, 8], vec![7, 6, 9, 4]];
    let (c, a) = hungarian(&cost);
    outln!(o, "cost matrix:");
    for r in &cost { outln!(o, "  {:?}", r); }
    outln!(o, "minimum total cost = {c}; worker → job = {:?}", a);
    let (w, a) = max_weight_assignment(&cost);
    outln!(o, "maximum-weight assignment = {w}; worker → job = {:?}", a);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn brute(cost: &[Vec<i64>], row: usize, used: &mut Vec<bool>) -> i64 {
        if row == cost.len() { return 0; }
        let mut best = i64::MAX;
        for j in 0..cost.len() {
            if !used[j] { used[j] = true; best = best.min(cost[row][j] + brute(cost, row + 1, used)); used[j] = false; }
        }
        best
    }
    #[test]
    fn book_style_example() {
        let cost = vec![vec![9, 2, 7, 8], vec![6, 4, 3, 7], vec![5, 8, 1, 8], vec![7, 6, 9, 4]];
        assert_eq!(hungarian(&cost).0, 13);
    }
    #[test]
    fn matches_brute_force_including_negative_costs() {
        let mut r = Rng::new(142);
        for _ in 0..80 {
            let n = r.range(1, 7);
            let cost: Vec<Vec<i64>> = (0..n).map(|_| r.vec(n, -10, 30)).collect();
            let (c, a) = hungarian(&cost);
            assert_eq!(c, brute(&cost, 0, &mut vec![false; n]));
            let mut seen = a.clone();
            seen.sort();
            assert_eq!(seen, (0..n).collect::<Vec<_>>());
        }
        assert_eq!(hungarian(&[]).0, 0);
    }
}
