//! @title Acyclic Subgraphs
//! @chapter Greedy-algorithm problems (1e–2e): acyclic subgraphs
//! @editions 1,2
//! @summary Undirected: a maximum acyclic subgraph is a spanning forest (|V| − #components edges). Directed: choose forward or backward edges of any vertex order to keep ≥ |E|/2 edges acyclic.
//! @order 280
use crate::outln;

/// Undirected graphs: the edge set of a spanning forest (maximum acyclic subgraph), via union-find. Returns the edge indices.
pub fn undirected_max_acyclic(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    fn find(p: &mut Vec<usize>, x: usize) -> usize { let mut r = x; while p[r] != r { r = p[r]; } let mut c = x; while p[c] != r { let nx = p[c]; p[c] = r; c = nx; } r }
    let mut p: Vec<usize> = (0..n).collect();
    let mut keep = vec![];
    for (i, &(u, v)) in edges.iter().enumerate() {
        let (a, b) = (find(&mut p, u), find(&mut p, v));
        if a != b { p[a] = b; keep.push(i); }
    }
    keep
}

/// Directed graphs: with vertices in index order, either all "forward" edges (u < v) or all "backward" edges (u > v) form an acyclic
/// subgraph; return the larger one (at least half of the non-loop edges). Returns edge indices.
pub fn directed_half_acyclic(edges: &[(usize, usize)]) -> Vec<usize> {
    let fwd: Vec<usize> = (0..edges.len()).filter(|&i| edges[i].0 < edges[i].1).collect();
    let bwd: Vec<usize> = (0..edges.len()).filter(|&i| edges[i].0 > edges[i].1).collect();
    if fwd.len() >= bwd.len() { fwd } else { bwd }
}

/// Exact maximum acyclic subgraph of a small digraph: best over all vertex orders of the edges that go forward in the order.
pub fn directed_optimum(n: usize, edges: &[(usize, usize)]) -> usize {
    fn go(order: &mut Vec<usize>, used: u32, n: usize, edges: &[(usize, usize)], best: &mut usize) {
        if order.len() == n {
            let mut pos = vec![0; n];
            for (i, &v) in order.iter().enumerate() { pos[v] = i; }
            *best = (*best).max(edges.iter().filter(|e| pos[e.0] < pos[e.1]).count());
            return;
        }
        for v in 0..n { if used >> v & 1 == 0 { order.push(v); go(order, used | 1 << v, n, edges, best); order.pop(); } }
    }
    let mut best = 0;
    go(&mut vec![], 0, n, edges, &mut best);
    best
}

pub fn is_acyclic(n: usize, edges: &[(usize, usize)], keep: &[usize]) -> bool {
    let mut adj = vec![vec![]; n];
    for &i in keep { adj[edges[i].0].push(edges[i].1); }
    crate::algorithms::graphs::topological_sort::topological_sort(&adj).is_some()
}

pub fn demo() -> String {
    let mut o = String::new();
    let edges = [(0, 1), (1, 2), (2, 0), (2, 3), (3, 1), (3, 4), (4, 0)];
    let keep = directed_half_acyclic(&edges);
    outln!(o, "digraph {:?}", edges);
    outln!(o, "half-acyclic subgraph keeps {} of {} edges (≥ |E|/2), acyclic = {}", keep.len(), edges.len(), is_acyclic(5, &edges, &keep));
    outln!(o, "true maximum acyclic subgraph has {} edges", directed_optimum(5, &edges));
    let und = [(0, 1), (1, 2), (2, 0), (3, 4)];
    outln!(o, "undirected {:?}: spanning forest keeps edges {:?} (n − components = {})", und, undirected_max_acyclic(5, &und), 5 - 2);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn directed_guarantee_and_acyclicity() {
        let mut r = Rng::new(347);
        for _ in 0..150 {
            let n = r.range(2, 6);
            let edges: Vec<(usize, usize)> = (0..r.range(1, 14)).map(|_| (r.range(0, n - 1), r.range(0, n - 1))).filter(|e| e.0 != e.1).collect();
            if edges.is_empty() { continue; }
            let keep = directed_half_acyclic(&edges);
            assert!(is_acyclic(n, &edges, &keep));
            assert!(2 * keep.len() >= edges.len());
            let opt = directed_optimum(n, &edges);
            assert!(keep.len() <= opt && 2 * keep.len() >= opt, "{} vs optimum {opt}", keep.len());
        }
    }
    #[test]
    fn undirected_forest_has_n_minus_components_edges() {
        let mut r = Rng::new(348);
        for _ in 0..100 {
            let n = r.range(1, 9);
            let edges: Vec<(usize, usize)> = (0..r.range(0, 15)).map(|_| (r.range(0, n - 1), r.range(0, n - 1))).filter(|e| e.0 != e.1).collect();
            let keep = undirected_max_acyclic(n, &edges);
            let comps = crate::algorithms::data_structures::disjoint_set::connected_components(n, &edges).1;
            assert_eq!(keep.len(), n - comps);
            // maximality: adding any other edge would close a cycle
            for i in 0..edges.len() { if !keep.contains(&i) { let mut k2 = keep.clone(); k2.push(i); assert!(undirected_max_acyclic(n, &k2.iter().map(|&j| edges[j]).collect::<Vec<_>>()).len() < k2.len()); } }
        }
    }
}
