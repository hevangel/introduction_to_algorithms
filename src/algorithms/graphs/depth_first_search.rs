//! @title Depth-First Search
//! @chapter 22.3 (3e) / 20.3 (4e) / 23.3 (1e–2e)
//! @editions 1,2,3,4
//! @summary Discovery/finish timestamps, parenthesis structure and tree/back/forward/cross edge classification.
//! @order 30
use crate::outln;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind { Tree, Back, Forward, Cross }

/// Result of a depth-first search of the whole graph.
#[derive(Debug, Clone)]
pub struct Dfs {
    /// Discovery times (1-based).
    pub d: Vec<usize>,
    /// Finish times.
    pub f: Vec<usize>,
    pub parent: Vec<Option<usize>>,
    /// Every edge (u, v) examined, with its classification.
    pub edges: Vec<(usize, usize, EdgeKind)>,
    /// Vertices in order of finishing.
    pub finish_order: Vec<usize>,
}

/// DFS(G) with an explicit stack (no recursion, so deep graphs are safe).
pub fn dfs(adj: &[Vec<usize>]) -> Dfs {
    let n = adj.len();
    let mut color = vec![0u8; n]; // 0 white, 1 gray, 2 black
    let mut r = Dfs { d: vec![0; n], f: vec![0; n], parent: vec![None; n], edges: vec![], finish_order: vec![] };
    let mut time = 0;
    for s in 0..n {
        if color[s] != 0 { continue; }
        time += 1;
        r.d[s] = time;
        color[s] = 1;
        let mut stack = vec![(s, 0usize)];
        while let Some(top) = stack.last_mut() {
            let u = top.0;
            if top.1 < adj[u].len() {
                let v = adj[u][top.1];
                top.1 += 1;
                match color[v] {
                    0 => {
                        r.edges.push((u, v, EdgeKind::Tree));
                        r.parent[v] = Some(u);
                        time += 1;
                        r.d[v] = time;
                        color[v] = 1;
                        stack.push((v, 0));
                    }
                    1 => r.edges.push((u, v, EdgeKind::Back)),
                    _ => r.edges.push((u, v, if r.d[u] < r.d[v] { EdgeKind::Forward } else { EdgeKind::Cross })),
                }
            } else {
                color[u] = 2;
                time += 1;
                r.f[u] = time;
                r.finish_order.push(u);
                stack.pop();
            }
        }
    }
    r
}

/// A directed graph is acyclic iff DFS finds no back edge (Lemma 22.11).
pub fn has_cycle(adj: &[Vec<usize>]) -> bool { dfs(adj).edges.iter().any(|e| e.2 == EdgeKind::Back) }

pub fn demo() -> String {
    let mut o = String::new();
    // Figure 22.4-like directed graph: u v w x y z = 0..5
    let adj = vec![vec![1, 3], vec![4], vec![4, 5], vec![1], vec![3], vec![5]];
    let names = ["u", "v", "w", "x", "y", "z"];
    let r = dfs(&adj);
    for v in 0..6 { outln!(o, "{}: discovered {:>2}, finished {:>2}", names[v], r.d[v], r.f[v]); }
    for (u, v, k) in &r.edges { outln!(o, "edge {}→{}: {:?}", names[*u], names[*v], k); }
    outln!(o, "has a cycle? {}", has_cycle(&adj));
    outln!(o, "finish order: {:?}", r.finish_order.iter().map(|&v| names[v]).collect::<Vec<_>>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_figure_22_4() {
        let adj = vec![vec![1, 3], vec![4], vec![4, 5], vec![1], vec![3], vec![5]];
        let r = dfs(&adj);
        assert_eq!((r.d.clone(), r.f.clone()), (vec![1, 2, 9, 4, 3, 10], vec![8, 7, 12, 5, 6, 11]));
        let count = |k| r.edges.iter().filter(|e| e.2 == k).count();
        assert_eq!((count(EdgeKind::Tree), count(EdgeKind::Back), count(EdgeKind::Forward), count(EdgeKind::Cross)), (4, 2, 1, 1));
    }
    #[test]
    fn parenthesis_theorem_and_edge_kinds() {
        let mut rg = Rng::new(123);
        for _ in 0..50 {
            let n = rg.range(1, 12);
            let adj: Vec<Vec<usize>> = (0..n).map(|u| (0..n).filter(|&v| rg.f64() < 0.2 && (u != v || rg.f64() < 0.2)).collect()).collect();
            let r = dfs(&adj);
            for u in 0..n { for v in 0..n {
                if u == v { continue; }
                let disjoint = r.f[u] < r.d[v] || r.f[v] < r.d[u];
                let nested = (r.d[u] < r.d[v] && r.f[v] < r.f[u]) || (r.d[v] < r.d[u] && r.f[u] < r.f[v]);
                assert!(disjoint || nested);
            } }
            for &(u, v, k) in &r.edges {
                let ok = match k {
                    EdgeKind::Tree | EdgeKind::Forward => r.d[u] < r.d[v] && r.f[v] < r.f[u],
                    EdgeKind::Back => r.d[v] <= r.d[u] && r.f[u] <= r.f[v],
                    EdgeKind::Cross => r.f[v] < r.d[u],
                };
                assert!(ok, "{u}->{v} {k:?}");
            }
        }
    }
    #[test]
    fn cycle_detection() {
        assert!(has_cycle(&[vec![1], vec![2], vec![0]]));
        assert!(!has_cycle(&[vec![1, 2], vec![2], vec![]]));
        assert!(has_cycle(&[vec![0]]));
        assert!(!has_cycle(&[]));
    }
}
