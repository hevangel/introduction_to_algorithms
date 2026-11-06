//! @title Hamiltonian Cycle & Exact TSP
//! @chapter 34.2, 34.5.4 (1e–3e) / 36 (4e)
//! @editions 1,2,3,4
//! @summary Exponential exact algorithms: backtracking search, and the Held-Karp O(n²2ⁿ) dynamic program for tours.
//! @order 20
use crate::outln;

/// Find a Hamiltonian cycle by backtracking (worst case n!). Returns the vertex order starting at 0.
pub fn hamiltonian_cycle_backtracking(adj: &[Vec<bool>]) -> Option<Vec<usize>> {
    let n = adj.len();
    if n == 0 { return None; }
    if n == 1 { return Some(vec![0]); }
    fn go(adj: &[Vec<bool>], path: &mut Vec<usize>, used: &mut Vec<bool>) -> bool {
        let n = adj.len();
        let last = *path.last().unwrap();
        if path.len() == n { return adj[last][path[0]]; }
        for v in 0..n {
            if !used[v] && adj[last][v] {
                used[v] = true;
                path.push(v);
                if go(adj, path, used) { return true; }
                path.pop();
                used[v] = false;
            }
        }
        false
    }
    let (mut path, mut used) = (vec![0], vec![false; n]);
    used[0] = true;
    if go(adj, &mut path, &mut used) { Some(path) } else { None }
}

/// Held–Karp: minimum-cost tour over all n cities in Θ(n² 2ⁿ) time. `dist[i][j]` may be f64::INFINITY for missing edges.
/// Returns (cost, tour starting at 0), or None if no tour exists.
pub fn traveling_salesman_exact(dist: &[Vec<f64>]) -> Option<(f64, Vec<usize>)> {
    let n = dist.len();
    if n == 0 { return None; }
    if n == 1 { return Some((0.0, vec![0])); }
    let full = 1usize << (n - 1); // subsets of cities 1..n−1
    let mut dp = vec![vec![f64::INFINITY; n - 1]; full];
    let mut parent = vec![vec![usize::MAX; n - 1]; full];
    for j in 0..n - 1 { dp[1 << j][j] = dist[0][j + 1]; }
    for mask in 1..full {
        for j in 0..n - 1 {
            if mask >> j & 1 == 0 || dp[mask][j].is_infinite() { continue; }
            for k in 0..n - 1 {
                if mask >> k & 1 == 1 { continue; }
                let nm = mask | 1 << k;
                let c = dp[mask][j] + dist[j + 1][k + 1];
                if c < dp[nm][k] { dp[nm][k] = c; parent[nm][k] = j; }
            }
        }
    }
    let (mut best, mut last) = (f64::INFINITY, usize::MAX);
    for j in 0..n - 1 {
        let c = dp[full - 1][j] + dist[j + 1][0];
        if c < best { best = c; last = j; }
    }
    if best.is_infinite() { return None; }
    let (mut tour, mut mask, mut j) = (vec![], full - 1, last);
    while j != usize::MAX {
        tour.push(j + 1);
        let p = parent[mask][j];
        mask ^= 1 << j;
        j = p;
    }
    tour.push(0);
    tour.reverse();
    Some((best, tour))
}

/// Does a Hamiltonian cycle exist? Same DP with unit costs on edges.
pub fn has_hamiltonian_cycle_dp(adj: &[Vec<bool>]) -> bool {
    let n = adj.len();
    let d: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|j| if i != j && adj[i][j] { 0.0 } else { f64::INFINITY }).collect()).collect();
    traveling_salesman_exact(&d).is_some()
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut adj = vec![vec![false; 6]; 6];
    for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0), (0, 3), (1, 4)] { adj[a][b] = true; adj[b][a] = true; }
    outln!(o, "backtracking finds: {:?}", hamiltonian_cycle_backtracking(&adj));
    outln!(o, "DP agrees (exists): {}", has_hamiltonian_cycle_dp(&adj));
    let pts = [(0.0f64, 0.0f64), (3.0, 0.0), (3.0, 4.0), (0.0, 4.0), (1.5, 2.0)];
    let d: Vec<Vec<f64>> = pts.iter().map(|a| pts.iter().map(|b| (a.0 - b.0).hypot(a.1 - b.1)).collect()).collect();
    outln!(o, "Held-Karp optimal tour of 5 points: {:?}", traveling_salesman_exact(&d).map(|(c, t)| (format!("{c:.3}"), t)));
    let mut star = vec![vec![false; 5]; 5];
    for v in 1..5 { star[0][v] = true; star[v][0] = true; }
    outln!(o, "star graph (no Hamiltonian cycle): {:?}", hamiltonian_cycle_backtracking(&star));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn brute_tsp(d: &[Vec<f64>]) -> f64 {
        let n = d.len();
        fn go(d: &[Vec<f64>], cur: usize, used: u32, cost: f64, best: &mut f64) {
            let n = d.len();
            if used.count_ones() as usize == n { *best = best.min(cost + d[cur][0]); return; }
            for v in 1..n { if used >> v & 1 == 0 { go(d, v, used | 1 << v, cost + d[cur][v], best); } }
        }
        let mut best = f64::INFINITY;
        go(d, 0, 1, 0.0, &mut best);
        let _ = n;
        best
    }
    #[test]
    fn tsp_dp_matches_permutation_search() {
        let mut r = Rng::new(240);
        for n in 2..=8 {
            let pts: Vec<(f64, f64)> = (0..n).map(|_| (r.f64() * 10.0, r.f64() * 10.0)).collect();
            let d: Vec<Vec<f64>> = pts.iter().map(|a| pts.iter().map(|b| (a.0 - b.0).hypot(a.1 - b.1)).collect()).collect();
            let (c, tour) = traveling_salesman_exact(&d).unwrap();
            assert!((c - brute_tsp(&d)).abs() < 1e-9);
            let len: f64 = (0..n).map(|i| d[tour[i]][tour[(i + 1) % n]]).sum();
            assert!((len - c).abs() < 1e-9);
            let mut s = tour.clone();
            s.sort();
            assert_eq!(s, (0..n).collect::<Vec<_>>());
        }
    }
    #[test]
    fn backtracking_agrees_with_dp_on_random_graphs() {
        let mut r = Rng::new(241);
        let (mut yes, mut no) = (0, 0);
        for _ in 0..200 {
            let n = r.range(3, 8);
            let mut adj = vec![vec![false; n]; n];
            for i in 0..n { for j in i + 1..n { if r.f64() < 0.4 { adj[i][j] = true; adj[j][i] = true; } } }
            let bt = hamiltonian_cycle_backtracking(&adj);
            assert_eq!(bt.is_some(), has_hamiltonian_cycle_dp(&adj));
            match bt { Some(c) => { yes += 1; assert!(super::super::np_verification::verify_hamiltonian_cycle(&adj, &c)); } None => no += 1 }
        }
        assert!(yes > 10 && no > 10);
    }
}
