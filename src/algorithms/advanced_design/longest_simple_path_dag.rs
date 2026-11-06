//! @title Longest Simple Path in a DAG
//! @chapter Problem 15-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Dynamic programming over a topological order finds the longest path between two vertices of a DAG in Θ(V+E); in general graphs the problem is NP-hard.
//! @order 250
use crate::outln;

/// Longest (maximum-weight) path from `s` to `t` in a DAG given as weighted edges. None if the graph has a cycle or t is unreachable.
pub fn longest_path(n: usize, edges: &[(usize, usize, i64)], s: usize, t: usize) -> Option<(i64, Vec<usize>)> {
    let mut indeg = vec![0; n];
    let mut adj = vec![vec![]; n];
    for &(u, v, w) in edges { adj[u].push((v, w)); indeg[v] += 1; }
    let mut order = vec![];
    let mut stack: Vec<usize> = (0..n).filter(|&v| indeg[v] == 0).collect();
    while let Some(u) = stack.pop() { order.push(u); for &(v, _) in &adj[u] { indeg[v] -= 1; if indeg[v] == 0 { stack.push(v); } } }
    if order.len() != n { return None; }
    // best[v] = weight of the longest path s ⇝ v; optimal substructure: a longest path to v ends with a longest path to its predecessor
    let (mut best, mut pred) = (vec![None; n], vec![usize::MAX; n]);
    best[s] = Some(0i64);
    for &u in &order {
        let Some(bu) = best[u] else { continue };
        for &(v, w) in &adj[u] { if best[v].is_none_or(|b| bu + w > b) { best[v] = Some(bu + w); pred[v] = u; } }
    }
    let total = best[t]?;
    let (mut path, mut v) = (vec![t], t);
    while v != s { v = pred[v]; path.push(v); }
    path.reverse();
    Some((total, path))
}

pub fn demo() -> String {
    let mut o = String::new();
    let e = [(0, 1, 3), (0, 2, 2), (1, 3, 4), (2, 3, 7), (1, 2, -1), (3, 4, 2), (2, 4, 6)];
    outln!(o, "edges {:?}", e);
    outln!(o, "longest path 0 → 4: {:?}", longest_path(5, &e, 0, 4));
    outln!(o, "cyclic graph: {:?}", longest_path(2, &[(0, 1, 1), (1, 0, 1)], 0, 1));
    outln!(o, "in a general graph the same question decides Hamiltonian path (NP-hard); the DAG order removes the cycles");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn brute(adj: &[Vec<(usize, i64)>], u: usize, t: usize) -> Option<i64> {
        if u == t { return Some(0); }
        adj[u].iter().filter_map(|&(v, w)| brute(adj, v, t).map(|x| x + w)).max()
    }
    #[test]
    fn matches_exhaustive_path_enumeration() {
        let mut r = Rng::new(340);
        for _ in 0..200 {
            let n = r.range(2, 9);
            let mut perm: Vec<usize> = (0..n).collect();
            r.shuffle(&mut perm);
            let mut edges = vec![];
            for i in 0..n { for j in i + 1..n { if r.f64() < 0.4 { edges.push((perm[i], perm[j], r.range(0, 12) as i64 - 4)); } } }
            let mut adj = vec![vec![]; n];
            for &(u, v, w) in &edges { adj[u].push((v, w)); }
            let (s, t) = (r.range(0, n - 1), r.range(0, n - 1));
            let got = longest_path(n, &edges, s, t);
            assert_eq!(got.as_ref().map(|x| x.0), brute(&adj, s, t));
            if let Some((w, p)) = got {
                let walked: i64 = p.windows(2).map(|x| edges.iter().filter(|e| e.0 == x[0] && e.1 == x[1]).map(|e| e.2).max().unwrap()).sum();
                assert_eq!(walked, w);
            }
        }
    }
    #[test]
    fn cycle_detected() { assert!(longest_path(3, &[(0, 1, 1), (1, 2, 1), (2, 0, 1)], 0, 2).is_none()); }
}
