//! @title Euler Tours
//! @chapter Problem 22-3 (3e) / 20-3 (4e) / 23-3 (1e–2e)
//! @editions 1,2,3,4
//! @summary Closed walks using every edge exactly once (Hierholzer), for directed and undirected graphs.
//! @order 70
use crate::outln;

/// Euler circuit of an undirected multigraph: a vertex sequence (first == last) using each edge once.
/// Returns None if one doesn't exist (an odd degree, or edges in several components).
pub fn euler_circuit_undirected(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    if edges.is_empty() { return Some(vec![]); }
    let mut deg = vec![0; n];
    let mut adj = vec![vec![]; n];
    for (id, &(a, b)) in edges.iter().enumerate() {
        adj[a].push((b, id));
        adj[b].push((a, id));
        deg[a] += 1;
        deg[b] += 1;
    }
    if deg.iter().any(|d| d % 2 == 1) { return None; }
    let start = edges[0].0;
    let (mut used, mut ptr) = (vec![false; edges.len()], vec![0usize; n]);
    let (mut stack, mut circuit) = (vec![start], vec![]);
    while let Some(&u) = stack.last() {
        while ptr[u] < adj[u].len() && used[adj[u][ptr[u]].1] { ptr[u] += 1; }
        if ptr[u] < adj[u].len() {
            let (v, id) = adj[u][ptr[u]];
            used[id] = true;
            stack.push(v);
        } else {
            circuit.push(u);
            stack.pop();
        }
    }
    if circuit.len() == edges.len() + 1 { circuit.reverse(); Some(circuit) } else { None }
}

/// Euler circuit of a directed multigraph (in-degree = out-degree everywhere, all edges connected).
pub fn euler_circuit_directed(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    if edges.is_empty() { return Some(vec![]); }
    let (mut indeg, mut outdeg, mut adj) = (vec![0; n], vec![0; n], vec![vec![]; n]);
    for &(a, b) in edges { adj[a].push(b); outdeg[a] += 1; indeg[b] += 1; }
    if indeg != outdeg { return None; }
    let (mut ptr, mut stack, mut circuit) = (vec![0usize; n], vec![edges[0].0], vec![]);
    while let Some(&u) = stack.last() {
        if ptr[u] < adj[u].len() { ptr[u] += 1; stack.push(adj[u][ptr[u] - 1]); }
        else { circuit.push(u); stack.pop(); }
    }
    if circuit.len() == edges.len() + 1 { circuit.reverse(); Some(circuit) } else { None }
}

pub fn demo() -> String {
    let mut o = String::new();
    let bowtie = [(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 2)];
    outln!(o, "undirected bowtie : {:?}", euler_circuit_undirected(5, &bowtie));
    outln!(o, "path graph (odd degrees): {:?}", euler_circuit_undirected(3, &[(0, 1), (1, 2)]));
    outln!(o, "two disjoint triangles  : {:?}", euler_circuit_undirected(6, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]));
    let digraph = [(0, 1), (1, 2), (2, 0), (0, 3), (3, 0)];
    outln!(o, "directed circuit  : {:?}", euler_circuit_directed(4, &digraph));
    outln!(o, "unbalanced digraph: {:?}", euler_circuit_directed(3, &[(0, 1), (1, 2)]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn check_undirected(n: usize, edges: &[(usize, usize)], c: &[usize]) {
        assert_eq!(c.len(), edges.len() + 1);
        assert_eq!(c.first(), c.last());
        let mut want: Vec<(usize, usize)> = edges.iter().map(|&(a, b)| (a.min(b), a.max(b))).collect();
        let mut got: Vec<(usize, usize)> = c.windows(2).map(|w| (w[0].min(w[1]), w[0].max(w[1]))).collect();
        want.sort();
        got.sort();
        assert_eq!(got, want);
        let _ = n;
    }
    #[test]
    fn undirected_cases() {
        let bow = [(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 2)];
        check_undirected(5, &bow, &euler_circuit_undirected(5, &bow).unwrap());
        assert!(euler_circuit_undirected(3, &[(0, 1), (1, 2)]).is_none());
        assert!(euler_circuit_undirected(6, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]).is_none());
        assert_eq!(euler_circuit_undirected(3, &[]), Some(vec![]));
    }
    #[test]
    fn random_eulerian_graphs() {
        let mut r = Rng::new(127);
        for _ in 0..40 {
            // union of random cycles through vertex 0 is always Eulerian and connected
            let n = r.range(3, 9);
            let mut edges = vec![];
            for _ in 0..r.range(1, 4) {
                let mut cyc: Vec<usize> = vec![0];
                for _ in 0..r.range(2, 5) { cyc.push(r.range(1, n - 1)); }
                for w in 0..cyc.len() { edges.push((cyc[w], cyc[(w + 1) % cyc.len()])); }
            }
            edges.retain(|e| e.0 != e.1);
            if edges.is_empty() { continue; }
            check_undirected(n, &edges, &euler_circuit_undirected(n, &edges).unwrap());
            let dc = euler_circuit_directed(n, &edges).unwrap();
            assert_eq!(dc.len(), edges.len() + 1);
            let mut want = edges.clone();
            let mut got: Vec<(usize, usize)> = dc.windows(2).map(|w| (w[0], w[1])).collect();
            want.sort();
            got.sort();
            assert_eq!(got, want);
        }
    }
    #[test]
    fn directed_requires_balance() {
        assert!(euler_circuit_directed(3, &[(0, 1), (1, 2)]).is_none());
        assert!(euler_circuit_directed(2, &[(0, 1), (1, 0), (0, 1), (1, 0)]).is_some());
    }
}
