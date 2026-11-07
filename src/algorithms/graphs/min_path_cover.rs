//! @title Minimum Path Cover of a DAG
//! @chapter Problem 26-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary The fewest vertex-disjoint paths covering a DAG equals |V| − (maximum bipartite matching); with shared vertices allowed it equals the largest antichain (Dilworth).
//! @order 215
use super::hopcroft_karp::hopcroft_karp;
use crate::outln;

/// Minimum number of **vertex-disjoint** paths that cover all vertices of a DAG, with the paths themselves.
/// Split every vertex v into v_out (left) and v_in (right); a maximum matching M selects edges to keep: #paths = |V| − |M|.
pub fn min_disjoint_path_cover(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges { adj[u].push(v); }
    let (_, next, _) = hopcroft_karp(n, n, &adj);
    let mut has_pred = vec![false; n];
    for u in 0..n { if next[u] != usize::MAX { has_pred[next[u]] = true; } }
    (0..n).filter(|&v| !has_pred[v]).map(|start| {
        let mut path = vec![start];
        let mut x = start;
        while next[x] != usize::MAX { x = next[x]; path.push(x); }
        path
    }).collect()
}

/// Minimum number of paths covering all vertices when paths **may share vertices**: add all transitive-closure edges first (Exercise 26-2(c)).
pub fn min_path_cover_with_overlap(n: usize, edges: &[(usize, usize)]) -> usize {
    let mut reach = vec![vec![false; n]; n];
    for &(u, v) in edges { reach[u][v] = true; }
    for k in 0..n { for i in 0..n { for j in 0..n { if reach[i][k] && reach[k][j] { reach[i][j] = true; } } } }
    let closure: Vec<(usize, usize)> = (0..n).flat_map(|i| (0..n).map(move |j| (i, j))).filter(|&(i, j)| reach[i][j]).collect();
    min_disjoint_path_cover(n, &closure).len()
}

pub fn demo() -> String {
    let mut o = String::new();
    let e = [(0, 1), (1, 2), (0, 3), (3, 2), (2, 4), (5, 3)];
    let cover = min_disjoint_path_cover(6, &e);
    outln!(o, "DAG edges {:?}", e);
    outln!(o, "minimum vertex-disjoint path cover ({} paths): {:?}", cover.len(), cover);
    outln!(o, "minimum cover when paths may overlap: {} (= size of the largest antichain)", min_path_cover_with_overlap(6, &e));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn random_dag(n: usize, r: &mut Rng) -> Vec<(usize, usize)> {
        let mut perm: Vec<usize> = (0..n).collect();
        r.shuffle(&mut perm);
        let mut e = vec![];
        for i in 0..n { for j in i + 1..n { if r.f64() < 0.35 { e.push((perm[i], perm[j])); } } }
        e
    }
    /// Brute force: vertices in topological order either start a new path or extend a path ending at a predecessor.
    fn brute_disjoint(n: usize, edges: &[(usize, usize)], order: &[usize], pos: usize, ends: &mut Vec<usize>) -> usize {
        if pos == n { return ends.len(); }
        let v = order[pos];
        let mut best = usize::MAX;
        ends.push(v);
        best = best.min(brute_disjoint(n, edges, order, pos + 1, ends));
        ends.pop();
        for i in 0..ends.len() {
            if edges.contains(&(ends[i], v)) { let old = ends[i]; ends[i] = v; best = best.min(brute_disjoint(n, edges, order, pos + 1, ends)); ends[i] = old; }
        }
        best
    }
    #[test]
    fn disjoint_cover_is_valid_and_optimal() {
        let mut r = Rng::new(368);
        for _ in 0..100 {
            let n = r.range(1, 7);
            let e = random_dag(n, &mut r);
            let cover = min_disjoint_path_cover(n, &e);
            let mut seen: Vec<usize> = cover.iter().flatten().copied().collect();
            seen.sort();
            assert_eq!(seen, (0..n).collect::<Vec<_>>());
            assert!(cover.iter().all(|p| p.windows(2).all(|w| e.contains(&(w[0], w[1])))));
            let mut adj = vec![vec![]; n];
            for &(u, v) in &e { adj[u].push(v); }
            let topo = crate::algorithms::graphs::topological_sort::topological_sort(&adj).unwrap();
            assert_eq!(cover.len(), brute_disjoint(n, &e, &topo, 0, &mut vec![]));
        }
    }
    #[test]
    fn overlapping_cover_equals_maximum_antichain() {
        let mut r = Rng::new(369);
        for _ in 0..100 {
            let n = r.range(1, 8);
            let e = random_dag(n, &mut r);
            let mut reach = vec![vec![false; n]; n];
            for &(u, v) in &e { reach[u][v] = true; }
            for k in 0..n { for i in 0..n { for j in 0..n { if reach[i][k] && reach[k][j] { reach[i][j] = true; } } } }
            let antichain = (0u32..1 << n).filter(|m| (0..n).all(|i| (0..n).all(|j| i == j || m >> i & 1 == 0 || m >> j & 1 == 0 || !reach[i][j]))).map(|m| m.count_ones() as usize).max().unwrap();
            assert_eq!(min_path_cover_with_overlap(n, &e), antichain);
        }
    }
}
