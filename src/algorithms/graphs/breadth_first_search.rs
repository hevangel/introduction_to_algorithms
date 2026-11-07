//! @title Breadth-First Search
//! @chapter 22.2 (3e) / 20.2 (4e) / 23.2 (1e–2e)
//! @editions 1,2,3,4
//! @summary Level-by-level traversal: shortest paths (in edges) from a source, bipartiteness test and tree diameter.
//! @order 20
use crate::outln;
use std::collections::VecDeque;

/// BFS(G, s): returns (distance in edges or None if unreachable, predecessor).
pub fn bfs(adj: &[Vec<usize>], s: usize) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let n = adj.len();
    let (mut dist, mut pi) = (vec![None; n], vec![None; n]);
    dist[s] = Some(0);
    let mut q = VecDeque::from([s]);
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if dist[v].is_none() {
                dist[v] = Some(dist[u].unwrap() + 1);
                pi[v] = Some(u);
                q.push_back(v);
            }
        }
    }
    (dist, pi)
}

/// PRINT-PATH(G, s, v): the vertices of a shortest path from s to v.
pub fn path(pi: &[Option<usize>], s: usize, v: usize) -> Option<Vec<usize>> {
    let mut p = vec![v];
    let mut cur = v;
    while cur != s {
        cur = pi[cur]?;
        p.push(cur);
    }
    p.reverse();
    Some(p)
}

/// Exercise 22.2-6: 2-colour an undirected graph with BFS; None if an odd cycle exists.
pub fn is_bipartite(adj: &[Vec<usize>]) -> Option<Vec<u8>> {
    let n = adj.len();
    let mut color = vec![2u8; n];
    for s in 0..n {
        if color[s] != 2 { continue; }
        color[s] = 0;
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                if color[v] == 2 { color[v] = 1 - color[u]; q.push_back(v); }
                else if color[v] == color[u] { return None; }
            }
        }
    }
    Some(color)
}

/// Exercise 22.2-7 (a tree's diameter): BFS from any vertex to the farthest vertex, then BFS again.
pub fn tree_diameter(adj: &[Vec<usize>]) -> usize {
    let farthest = |s: usize| {
        let (d, _) = bfs(adj, s);
        let v = (0..adj.len()).filter(|&v| d[v].is_some()).max_by_key(|&v| d[v]).unwrap();
        (v, d[v].unwrap())
    };
    let (a, _) = farthest(0);
    farthest(a).1
}

pub fn demo() -> String {
    let mut o = String::new();
    // Figure 22.3-style graph (undirected)
    let edges = [(0, 1), (0, 4), (1, 5), (5, 2), (5, 6), (2, 6), (2, 3), (3, 6), (3, 7), (6, 7)];
    let mut adj = vec![vec![]; 8];
    for &(a, b) in &edges { adj[a].push(b); adj[b].push(a); }
    let (d, pi) = bfs(&adj, 1);
    outln!(o, "BFS from vertex 1: distances {:?}", d);
    outln!(o, "predecessors: {:?}", pi);
    outln!(o, "shortest path 1 → 7: {:?}", path(&pi, 1, 7));
    outln!(o, "bipartite? {:?}", is_bipartite(&adj).is_some());
    let square = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
    outln!(o, "4-cycle bipartite colouring: {:?}", is_bipartite(&square));
    let tree = vec![vec![1], vec![0, 2, 3], vec![1, 4], vec![1], vec![2]];
    outln!(o, "tree diameter = {}", tree_diameter(&tree));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn floyd(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
        let n = adj.len();
        let mut d = vec![vec![usize::MAX / 2; n]; n];
        for u in 0..n { d[u][u] = 0; for &v in &adj[u] { d[u][v] = 1; } }
        for k in 0..n { for i in 0..n { for j in 0..n { d[i][j] = d[i][j].min(d[i][k] + d[k][j]); } } }
        d
    }
    #[test]
    fn distances_match_floyd_warshall() {
        let mut r = Rng::new(122);
        for _ in 0..30 {
            let n = r.range(1, 12);
            let adj: Vec<Vec<usize>> = (0..n).map(|u| (0..n).filter(|&v| u != v && r.f64() < 0.2).collect()).collect();
            let fw = floyd(&adj);
            for s in 0..n {
                let (d, pi) = bfs(&adj, s);
                for v in 0..n {
                    assert_eq!(d[v], if fw[s][v] >= usize::MAX / 2 { None } else { Some(fw[s][v]) });
                    if let Some(p) = path(&pi, s, v) { assert_eq!(p.len() - 1, d[v].unwrap()); }
                }
            }
        }
    }
    #[test]
    fn bipartite_and_diameter() {
        let c5 = (0..5).map(|i| vec![(i + 1) % 5, (i + 4) % 5]).collect::<Vec<_>>();
        assert!(is_bipartite(&c5).is_none());
        let c6 = (0..6).map(|i| vec![(i + 1) % 6, (i + 5) % 6]).collect::<Vec<_>>();
        let col = is_bipartite(&c6).unwrap();
        for u in 0..6 { for &v in &c6[u] { assert_ne!(col[u], col[v]); } }
        let path_graph: Vec<Vec<usize>> = (0..7usize).map(|i| [i.wrapping_sub(1), i + 1].into_iter().filter(|&x| x < 7).collect()).collect();
        assert_eq!(tree_diameter(&path_graph), 6);
    }
}
