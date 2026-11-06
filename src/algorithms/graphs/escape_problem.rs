//! @title The Escape Problem
//! @chapter Problem 26-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Can m starting points on an n×n grid escape to the boundary along vertex-disjoint paths? Maximum flow with unit vertex capacities.
//! @order 195
use super::ford_fulkerson::FlowNetwork;
use crate::outln;

/// The maximum number of starting points that can escape to the boundary along pairwise **vertex-disjoint** paths in an n×n grid.
/// Every grid vertex is split into (in, out) joined by a capacity-1 edge; boundary vertices connect to the sink.
pub fn max_escapees(n: usize, starts: &[(usize, usize)]) -> usize {
    let id = |r: usize, c: usize| r * n + c;
    let (s, t) = (2 * n * n, 2 * n * n + 1);
    let mut g = FlowNetwork::new(2 * n * n + 2);
    for r in 0..n { for c in 0..n {
        let v = id(r, c);
        g.add_edge(2 * v, 2 * v + 1, 1);                                        // vertex capacity 1
        if r == 0 || c == 0 || r == n - 1 || c == n - 1 { g.add_edge(2 * v + 1, t, 1); } // boundary vertices lead out of the grid
        for (dr, dc) in [(0i64, 1i64), (1, 0), (0, -1), (-1, 0)] {
            let (nr, nc) = (r as i64 + dr, c as i64 + dc);
            if nr >= 0 && nc >= 0 && (nr as usize) < n && (nc as usize) < n { g.add_edge(2 * v + 1, 2 * id(nr as usize, nc as usize), 1); }
        }
    } }
    for &(r, c) in starts { g.add_edge(s, 2 * id(r, c), 1); }
    g.edmonds_karp(s, t) as usize
}

/// Is escape possible for all starting points?
pub fn can_escape(n: usize, starts: &[(usize, usize)]) -> bool { max_escapees(n, starts) == starts.len() }

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "5×5 grid, 4 starting points in the middle: all escape? {} (flow {})", can_escape(5, &[(1, 1), (1, 3), (3, 1), (3, 3)]), max_escapees(5, &[(1, 1), (1, 3), (3, 1), (3, 3)]));
    // the 3×3 interior ring: 5 starts around a 5×5 grid centre cannot all get out through the 4 sides of the inner square... try the crowded centre
    let crowd: Vec<(usize, usize)> = (1..4).flat_map(|r| (1..4).map(move |c| (r, c))).collect();
    outln!(o, "5×5 grid, all 9 interior vertices occupied: escapees {} of {} (the 3×3 block has only 12 neighbours but every path must cross its own ring)", max_escapees(5, &crowd), crowd.len());
    outln!(o, "single start in the centre of a 3×3 grid: {}", can_escape(3, &[(1, 1)]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    /// Exhaustive search for vertex-disjoint escape routes (each path stops at its first boundary vertex).
    fn brute(n: usize, starts: &[(usize, usize)], i: usize, used: &mut Vec<bool>) -> usize {
        if i == starts.len() { return 0; }
        let mut best = brute(n, starts, i + 1, used); // this start does not escape
        fn dfs(n: usize, (r, c): (usize, usize), starts: &[(usize, usize)], i: usize, used: &mut Vec<bool>, best: &mut usize) {
            used[r * n + c] = true;
            if r == 0 || c == 0 || r == n - 1 || c == n - 1 { *best = (*best).max(1 + brute(n, starts, i + 1, used)); }
            else {
                for (dr, dc) in [(0i64, 1i64), (1, 0), (0, -1), (-1, 0)] {
                    let (nr, nc) = ((r as i64 + dr) as usize, (c as i64 + dc) as usize);
                    if !used[nr * n + nc] { dfs(n, (nr, nc), starts, i, used, best); }
                }
            }
            used[r * n + c] = false;
        }
        let s = starts[i];
        if !used[s.0 * n + s.1] { dfs(n, s, starts, i, used, &mut best); }
        best
    }
    #[test]
    fn matches_exhaustive_path_search() {
        let mut r = Rng::new(370);
        for _ in 0..60 {
            let n = r.range(2, 4);
            let mut cells: Vec<(usize, usize)> = (0..n).flat_map(|a| (0..n).map(move |b| (a, b))).collect();
            r.shuffle(&mut cells);
            cells.truncate(r.range(0, 4.min(n * n)));
            assert_eq!(max_escapees(n, &cells), brute(n, &cells, 0, &mut vec![false; n * n]), "n={n} {cells:?}");
        }
    }
    #[test]
    fn known_cases() {
        assert!(can_escape(3, &[(1, 1)]));
        assert!(can_escape(5, &[(1, 1), (1, 3), (3, 1), (3, 3)]));
        assert_eq!(max_escapees(3, &[(1, 1), (0, 0)]), 2);
        let crowd: Vec<(usize, usize)> = (1..4).flat_map(|r| (1..4).map(move |c| (r, c))).collect();
        assert!(max_escapees(5, &crowd) < 9);
    }
}
