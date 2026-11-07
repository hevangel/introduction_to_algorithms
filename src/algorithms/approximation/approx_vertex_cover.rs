//! @title Approximate Vertex Cover
//! @chapter 35.1 (2e–4e)
//! @editions 2,3,4
//! @summary Take both endpoints of a maximal matching: a polynomial-time 2-approximation.
//! @order 10
use crate::outln;

/// APPROX-VERTEX-COVER(G): repeatedly pick an uncovered edge, add both endpoints, delete all edges they touch.
pub fn approx_vertex_cover(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut in_cover = vec![false; n];
    for &(u, v) in edges {
        if !in_cover[u] && !in_cover[v] { // edge still uncovered → it joins the maximal matching
            in_cover[u] = true;
            in_cover[v] = true;
        }
    }
    (0..n).filter(|&v| in_cover[v]).collect()
}

/// The matching A chosen by the algorithm (|cover| = 2|A|, and any cover needs ≥ |A| vertices).
pub fn matching_chosen(n: usize, edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut used = vec![false; n];
    let mut m = vec![];
    for &(u, v) in edges {
        if !used[u] && !used[v] { used[u] = true; used[v] = true; m.push((u, v)); }
    }
    m
}

pub fn demo() -> String {
    let mut o = String::new();
    let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (1, 5)];
    let c = approx_vertex_cover(7, &edges);
    let opt = super::super::np_completeness::np_reductions::min_vertex_cover(7, &edges);
    outln!(o, "edges: {:?}", edges);
    outln!(o, "matching: {:?}", matching_chosen(7, &edges));
    outln!(o, "approximate cover {:?} (size {}), optimal size {} → ratio {:.2} ≤ 2", c, c.len(), opt, c.len() as f64 / opt as f64);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::np_completeness::np_reductions::min_vertex_cover;
    use crate::common::Rng;
    #[test]
    fn valid_cover_within_factor_two() {
        let mut r = Rng::new(250);
        for _ in 0..300 {
            let n = r.range(2, 11);
            let edges: Vec<(usize, usize)> = (0..r.range(1, 20)).map(|_| (r.range(0, n - 1), r.range(0, n - 1))).filter(|e| e.0 != e.1).collect();
            if edges.is_empty() { continue; }
            let c = approx_vertex_cover(n, &edges);
            assert!(edges.iter().all(|&(u, v)| c.contains(&u) || c.contains(&v)));
            let opt = min_vertex_cover(n, &edges);
            assert!(c.len() <= 2 * opt, "{} vs {}", c.len(), opt);
            assert_eq!(c.len(), 2 * matching_chosen(n, &edges).len());
        }
    }
    #[test]
    fn worst_case_is_tight_on_complete_bipartite_graphs() {
        // K_{n,n}: optimum n, the matching-based cover takes all 2n vertices
        let n = 5;
        let edges: Vec<(usize, usize)> = (0..n).flat_map(|i| (0..n).map(move |j| (i, n + j))).collect();
        assert_eq!(approx_vertex_cover(2 * n, &edges).len(), 2 * n);
    }
}
