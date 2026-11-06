//! @title Topological Sort
//! @chapter 22.4 (3e) / 20.4 (4e) / 23.4 (1e–2e)
//! @editions 1,2,3,4
//! @summary Linearly order a DAG's vertices by decreasing DFS finish time, or by repeatedly removing sources (Kahn).
//! @order 40
use super::depth_first_search::{dfs, EdgeKind};
use crate::outln;
use std::collections::VecDeque;

/// TOPOLOGICAL-SORT(G): vertices by decreasing finishing time; None if G has a cycle.
pub fn topological_sort(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let r = dfs(adj);
    if r.edges.iter().any(|e| e.2 == EdgeKind::Back) { return None; }
    let mut order = r.finish_order;
    order.reverse();
    Some(order)
}

/// Kahn's algorithm (Exercise 22.4-5): repeatedly output a vertex of in-degree 0.
pub fn kahn(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut indeg = vec![0; n];
    for vs in adj { for &v in vs { indeg[v] += 1; } }
    let mut q: VecDeque<usize> = (0..n).filter(|&v| indeg[v] == 0).collect();
    let mut order = vec![];
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 { q.push_back(v); }
        }
    }
    if order.len() == n { Some(order) } else { None }
}

/// Exercise 22.4-2: number of distinct paths from s to t in a DAG, in Θ(V+E).
pub fn count_paths(adj: &[Vec<usize>], s: usize, t: usize) -> Option<u64> {
    let order = topological_sort(adj)?;
    let mut ways = vec![0u64; adj.len()];
    ways[s] = 1;
    for u in order {
        for &v in &adj[u] { ways[v] += ways[u]; }
    }
    Some(ways[t])
}

pub fn is_valid_order(adj: &[Vec<usize>], order: &[usize]) -> bool {
    let mut pos = vec![usize::MAX; adj.len()];
    for (i, &v) in order.iter().enumerate() { pos[v] = i; }
    order.len() == adj.len() && adj.iter().enumerate().all(|(u, vs)| vs.iter().all(|&v| pos[u] < pos[v]))
}

pub fn demo() -> String {
    let mut o = String::new();
    // Professor Bumstead's getting-dressed DAG (Figure 22.7)
    let names = ["undershorts", "pants", "belt", "shirt", "tie", "jacket", "socks", "shoes", "watch"];
    let edges = [(0, 1), (0, 7), (1, 2), (1, 7), (2, 5), (3, 2), (3, 4), (4, 5), (6, 7)];
    let mut adj = vec![vec![]; 9];
    for &(a, b) in &edges { adj[a].push(b); }
    let order = topological_sort(&adj).unwrap();
    outln!(o, "DFS-based order : {}", order.iter().map(|&i| names[i]).collect::<Vec<_>>().join(" → "));
    outln!(o, "Kahn's order    : {}", kahn(&adj).unwrap().iter().map(|&i| names[i]).collect::<Vec<_>>().join(" → "));
    outln!(o, "valid? {}", is_valid_order(&adj, &order));
    let diamond = vec![vec![1, 2], vec![3], vec![3], vec![]];
    outln!(o, "paths 0 → 3 in a diamond DAG: {:?}", count_paths(&diamond, 0, 3));
    outln!(o, "cyclic graph: {:?}", topological_sort(&[vec![1], vec![0]]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn random_dag(n: usize, r: &mut Rng) -> Vec<Vec<usize>> {
        let mut perm: Vec<usize> = (0..n).collect();
        r.shuffle(&mut perm);
        let mut adj = vec![vec![]; n];
        for i in 0..n { for j in i + 1..n { if r.f64() < 0.3 { adj[perm[i]].push(perm[j]); } } }
        adj
    }
    #[test]
    fn orders_are_valid() {
        let mut r = Rng::new(124);
        for _ in 0..60 {
            let g = random_dag(r.range(0, 14), &mut r);
            assert!(is_valid_order(&g, &topological_sort(&g).unwrap()));
            assert!(is_valid_order(&g, &kahn(&g).unwrap()));
        }
    }
    #[test]
    fn cycles_are_rejected_and_paths_counted() {
        assert!(topological_sort(&[vec![1], vec![2], vec![0]]).is_none());
        assert!(kahn(&[vec![1], vec![2], vec![0]]).is_none());
        let g = vec![vec![1, 2, 3], vec![2, 3], vec![3], vec![]];
        assert_eq!(count_paths(&g, 0, 3), Some(4));
        assert_eq!(count_paths(&[vec![0]], 0, 0), None);
    }
}
