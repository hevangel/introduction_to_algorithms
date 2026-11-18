//! @title Dynamic Planar Graph Low-Outdegree Orientation
//! @chapter Problem 20-5 (4e)
//! @editions 4
//! @summary Orient edges dynamically during incremental planar vertex insertion so out-degree stays ≤ 5 and adjacency queries run in O(1) time.
//! @order 55
use crate::outln;
use std::collections::VecDeque;

/// Maintains a dynamic simple planar graph under vertex/edge insertions with invariant `out_degree(v) <= 5`
/// for every vertex, enabling O(1) worst-case `has_edge(u, v)` queries by scanning at most 5 out-neighbors on each endpoint.
#[derive(Debug, Clone)]
pub struct DynamicPlanarGraph {
    out_adj: Vec<Vec<usize>>,
    reorientations: usize,
}

impl DynamicPlanarGraph {
    pub fn new(n: usize) -> Self {
        Self { out_adj: vec![Vec::new(); n], reorientations: 0 }
    }

    pub fn num_vertices(&self) -> usize {
        self.out_adj.len()
    }

    pub fn out_degree(&self, u: usize) -> usize {
        self.out_adj[u].len()
    }

    pub fn max_out_degree(&self) -> usize {
        self.out_adj.iter().map(|a| a.len()).max().unwrap_or(0)
    }

    pub fn reorientation_count(&self) -> usize {
        self.reorientations
    }

    /// O(1) adjacency test: checks at most 5 outgoing arcs from `u` and at most 5 from `v`.
    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        self.out_adj[u].contains(&v) || self.out_adj[v].contains(&u)
    }

    /// Insert an undirected edge `{u, v}` oriented outward from the lower-outdegree endpoint,
    /// rebalancing via directed augmenting path reversal whenever out-degree exceeds 5.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        if u == v || self.has_edge(u, v) {
            return;
        }
        let (src, dst) = if self.out_adj[u].len() <= self.out_adj[v].len() { (u, v) } else { (v, u) };
        self.add_directed_edge(src, dst);
    }

    /// Add edge `src -> dst` and trigger path-reversal rebalancing if `out_degree(src) > 5`.
    pub fn add_directed_edge(&mut self, src: usize, dst: usize) {
        if src == dst || self.has_edge(src, dst) {
            return;
        }
        self.out_adj[src].push(dst);
        if self.out_adj[src].len() > 5 {
            self.rebalance(src);
        }
    }

    /// Insert a new vertex connected to `neighbors` in the current planar graph, returning its vertex id.
    pub fn add_vertex_with_edges(&mut self, neighbors: &[usize]) -> usize {
        let v = self.out_adj.len();
        self.out_adj.push(Vec::new());
        for &u in neighbors {
            self.add_edge(v, u);
        }
        v
    }

    fn rebalance(&mut self, overloaded: usize) {
        let n = self.out_adj.len();
        let mut parent = vec![usize::MAX; n];
        let mut q = VecDeque::new();
        parent[overloaded] = overloaded;
        q.push_back(overloaded);
        let mut target = None;

        while let Some(x) = q.pop_front() {
            if x != overloaded && self.out_adj[x].len() < 5 {
                target = Some(x);
                break;
            }
            for &y in &self.out_adj[x] {
                if parent[y] == usize::MAX {
                    parent[y] = x;
                    q.push_back(y);
                }
            }
        }
        let sink = target.expect("planar graph must admit out-degree <= 5 orientation");
        let mut cur = sink;
        while cur != overloaded {
            let prev = parent[cur];
            let pos = self.out_adj[prev].iter().position(|&w| w == cur).unwrap();
            self.out_adj[prev].swap_remove(pos);
            self.out_adj[cur].push(prev);
            self.reorientations += 1;
            cur = prev;
        }
    }
}

/// Static 5-degeneracy linear-time orientation for a planar graph given all at once:
/// repeatedly peel a vertex of degree <= 5 and orient its remaining edges outward.
pub fn orient_planar_static(n: usize, edges: &[(usize, usize)]) -> DynamicPlanarGraph {
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in edges {
        if u != v && !adj[u].contains(&v) {
            adj[u].push(v);
            adj[v].push(u);
        }
    }
    let mut deg: Vec<usize> = adj.iter().map(|a| a.len()).collect();
    let mut removed = vec![false; n];
    let mut stack: Vec<usize> = (0..n).filter(|&u| deg[u] <= 5).collect();
    let mut out_adj = vec![Vec::new(); n];
    let mut peeled = 0usize;

    while let Some(u) = stack.pop() {
        if removed[u] {
            continue;
        }
        removed[u] = true;
        peeled += 1;
        for &v in &adj[u] {
            if !removed[v] {
                out_adj[u].push(v);
                deg[v] -= 1;
                if deg[v] <= 5 {
                    stack.push(v);
                }
            }
        }
    }
    assert_eq!(peeled, n, "input graph is not 5-degenerate (non-planar)");
    DynamicPlanarGraph { out_adj, reorientations: 0 }
}

pub fn demo() -> String {
    let mut o = String::new();
    // Maximal planar octahedron (6 vertices, 12 edges) + subdivision hub insertions.
    let mut g = DynamicPlanarGraph::new(6);
    let oct_edges = [
        (0, 1), (1, 2), (2, 0),
        (3, 4), (4, 5), (5, 3),
        (0, 3), (0, 5), (1, 3), (1, 4), (2, 4), (2, 5),
    ];
    for &(u, v) in &oct_edges {
        g.add_edge(u, v);
    }
    let hub = g.add_vertex_with_edges(&[0, 1, 2, 3, 4, 5]);
    outln!(
        o,
        "planar graph vertices = {}, max out-degree = {}, edge flips = {}",
        g.num_vertices(),
        g.max_out_degree(),
        g.reorientation_count()
    );
    outln!(o, "O(1) query has_edge({hub}, 4) = {}, has_edge(0, 4) = {}", g.has_edge(hub, 4), g.has_edge(0, 4));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn incremental_triangulation_preserves_out_deg_le_5() {
        let mut r = Rng::new(2005);
        let mut g = DynamicPlanarGraph::new(4);
        let mut faces: Vec<[usize; 3]> = vec![[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]];
        let mut all_edges: Vec<(usize, usize)> = Vec::new();
        for u in 0..4 {
            for v in (u + 1)..4 {
                g.add_edge(u, v);
                all_edges.push((u, v));
            }
        }
        for _ in 0..80 {
            let fi = r.range(0, faces.len() - 1);
            let [a, b, c] = faces.swap_remove(fi);
            let nv = g.add_vertex_with_edges(&[a, b, c]);
            all_edges.push((nv, a));
            all_edges.push((nv, b));
            all_edges.push((nv, c));
            faces.push([a, b, nv]);
            faces.push([b, c, nv]);
            faces.push([c, a, nv]);
            assert!(g.max_out_degree() <= 5);
        }
        for &(u, v) in &all_edges {
            assert!(g.has_edge(u, v));
            assert!(g.has_edge(v, u));
        }
        let stat = orient_planar_static(g.num_vertices(), &all_edges);
        assert!(stat.max_out_degree() <= 5);
        for &(u, v) in &all_edges {
            assert!(stat.has_edge(u, v));
        }
    }

    #[test]
    fn forced_reorientation_restores_bound() {
        // Planar wheel hub 0 receiving 10 outward-directed arcs triggers path-reversal rebalancing.
        let mut g = DynamicPlanarGraph::new(11);
        for rim in 1..=10 {
            g.add_directed_edge(0, rim);
            assert!(g.max_out_degree() <= 5);
        }
        assert!(g.reorientation_count() >= 5);
        for rim in 1..=10 {
            assert!(g.has_edge(0, rim));
            assert!(g.has_edge(rim, 0));
        }
    }
}
