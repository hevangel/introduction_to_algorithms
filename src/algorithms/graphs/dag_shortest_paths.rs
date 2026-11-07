//! @title Shortest Paths in a DAG
//! @chapter 24.2 (3e) / 22.2 (4e) / 25.4 (1e)
//! @editions 1,2,3,4
//! @summary Relax edges in topological order: Θ(V+E) even with negative weights; longest paths and critical paths.
//! @order 110
use crate::outln;

pub const INF: i64 = i64::MAX / 4;
pub type Edge = (usize, usize, i64);

fn topo(n: usize, edges: &[Edge]) -> Option<Vec<usize>> {
    let mut indeg = vec![0; n];
    let mut adj = vec![vec![]; n];
    for &(u, v, _) in edges { adj[u].push(v); indeg[v] += 1; }
    let mut stack: Vec<usize> = (0..n).filter(|&v| indeg[v] == 0).collect();
    let mut order = vec![];
    while let Some(u) = stack.pop() {
        order.push(u);
        for &v in &adj[u] { indeg[v] -= 1; if indeg[v] == 0 { stack.push(v); } }
    }
    if order.len() == n { Some(order) } else { None }
}

/// DAG-SHORTEST-PATHS(G, w, s): None if the graph has a cycle.
pub fn dag_shortest_paths(n: usize, edges: &[Edge], s: usize) -> Option<(Vec<i64>, Vec<Option<usize>>)> {
    let order = topo(n, edges)?;
    let mut adj = vec![vec![]; n];
    for &(u, v, w) in edges { adj[u].push((v, w)); }
    let (mut d, mut pi) = (vec![INF; n], vec![None; n]);
    d[s] = 0;
    for u in order {
        if d[u] == INF { continue; }
        for &(v, w) in &adj[u] {
            if d[u] + w < d[v] { d[v] = d[u] + w; pi[v] = Some(u); }
        }
    }
    Some((d, pi))
}

/// Longest paths from s (negate the weights): the critical-path method used in PERT charts. Unreachable = None.
pub fn dag_longest_paths(n: usize, edges: &[Edge], s: usize) -> Option<Vec<Option<i64>>> {
    let neg: Vec<Edge> = edges.iter().map(|&(u, v, w)| (u, v, -w)).collect();
    let (d, _) = dag_shortest_paths(n, &neg, s)?;
    Some(d.into_iter().map(|x| if x == INF { None } else { Some(-x) }).collect())
}

pub fn demo() -> String {
    let mut o = String::new();
    // Figure 24.5: r=0 s=1 t=2 x=3 y=4 z=5, source s
    let e: Vec<Edge> = vec![(0, 1, 5), (0, 2, 3), (1, 2, 2), (1, 3, 6), (2, 3, 7), (2, 4, 4), (2, 5, 2), (3, 4, -1), (3, 5, 1), (4, 5, -2)];
    let (d, _) = dag_shortest_paths(6, &e, 1).unwrap();
    outln!(o, "shortest distances from s: {:?}  (INF = unreachable)", d.iter().map(|&x| if x == INF { "∞".to_string() } else { x.to_string() }).collect::<Vec<_>>());
    outln!(o, "longest (critical) path lengths from s: {:?}", dag_longest_paths(6, &e, 1).unwrap());
    outln!(o, "cyclic graph → {:?}", dag_shortest_paths(2, &[(0, 1, 1), (1, 0, 1)], 0).map(|x| x.0));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_figure_24_5() {
        let e: Vec<Edge> = vec![(0, 1, 5), (0, 2, 3), (1, 2, 2), (1, 3, 6), (2, 3, 7), (2, 4, 4), (2, 5, 2), (3, 4, -1), (3, 5, 1), (4, 5, -2)];
        let (d, _) = dag_shortest_paths(6, &e, 1).unwrap();
        assert_eq!(d, vec![INF, 0, 2, 6, 5, 3]);
    }
    #[test]
    fn matches_bellman_ford_on_random_dags() {
        let mut r = Rng::new(131);
        for _ in 0..60 {
            let n = r.range(1, 12);
            let mut perm: Vec<usize> = (0..n).collect();
            r.shuffle(&mut perm);
            let mut edges = vec![];
            for i in 0..n { for j in i + 1..n { if r.f64() < 0.35 { edges.push((perm[i], perm[j], r.range(0, 20) as i64 - 8)); } } }
            let s = r.range(0, n - 1);
            let (d, _) = dag_shortest_paths(n, &edges, s).unwrap();
            let (bf, _) = super::super::bellman_ford::bellman_ford(n, &edges, s).unwrap();
            assert_eq!(d, bf);
            let lp = dag_longest_paths(n, &edges, s).unwrap();
            for v in 0..n { assert_eq!(lp[v].is_some(), d[v] != INF); }
        }
    }
    #[test]
    fn cycle_returns_none() {
        assert!(dag_shortest_paths(3, &[(0, 1, 1), (1, 2, 1), (2, 0, 1)], 0).is_none());
    }
}
