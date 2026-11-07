//! @title Approximating a Maximum Spanning Tree
//! @chapter Problem 35-6 (3e)
//! @editions 3,4
//! @summary Let S_G be the heaviest edge at each vertex: S_G is a forest contained in the maximum spanning tree T_G and w(T_G) ≤ 2·w(S_G), so completing S_G to a spanning tree in O(V+E) time gives a 2-approximation.
//! @order 70
use crate::outln;

pub type Edge = (usize, usize, i64);

fn find(p: &mut Vec<usize>, x: usize) -> usize { let mut r = x; while p[r] != r { r = p[r]; } let mut c = x; while p[c] != r { let nx = p[c]; p[c] = r; c = nx; } r }

/// S_G: indices of max(v), the maximum-weight edge incident on v, for every vertex v with an incident edge (deduplicated).
pub fn max_edges(n: usize, edges: &[Edge]) -> Vec<usize> {
    let mut best: Vec<Option<usize>> = vec![None; n];
    for (i, &(u, v, w)) in edges.iter().enumerate() { for x in [u, v] { if best[x].is_none_or(|b| edges[b].2 < w) { best[x] = Some(i); } } }
    let mut s: Vec<usize> = best.into_iter().flatten().collect();
    s.sort();
    s.dedup();
    s
}

/// T_G: the maximum-weight spanning tree (Kruskal with edges in decreasing order). Returns edge indices.
pub fn maximum_spanning_tree(n: usize, edges: &[Edge]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..edges.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(edges[i].2));
    let mut p: Vec<usize> = (0..n).collect();
    order.into_iter().filter(|&i| { let (a, b) = (find(&mut p, edges[i].0), find(&mut p, edges[i].1)); if a != b { p[a] = b; true } else { false } }).collect()
}

/// A 2-approximation in linear time (apart from union-find): take S_G (a forest, since weights are distinct), then add arbitrary edges that
/// connect different components until a spanning tree results. Its weight is ≥ w(S_G) ≥ w(T_G)/2.
pub fn two_approximation(n: usize, edges: &[Edge]) -> Vec<usize> {
    let mut p: Vec<usize> = (0..n).collect();
    let mut tree = vec![];
    for i in max_edges(n, edges) { let (a, b) = (find(&mut p, edges[i].0), find(&mut p, edges[i].1)); if a != b { p[a] = b; tree.push(i); } }
    for (i, &(u, v, _)) in edges.iter().enumerate() { let (a, b) = (find(&mut p, u), find(&mut p, v)); if a != b { p[a] = b; tree.push(i); } }
    tree
}

pub fn weight(edges: &[Edge], set: &[usize]) -> i64 { set.iter().map(|&i| edges[i].2).sum() }

pub fn demo() -> String {
    let mut o = String::new();
    let path: Vec<Edge> = vec![(0, 1, 1), (1, 2, 3), (2, 3, 2)];
    outln!(o, "path with weights 1,3,2: S_G = {:?}, T_G = {:?} (equal)", max_edges(4, &path), maximum_spanning_tree(4, &path));
    let two: Vec<Edge> = vec![(0, 1, 5), (2, 3, 6), (1, 2, 1)];
    outln!(o, "two heavy edges joined by a light one: S_G = {:?} ≠ T_G = {:?}; 2-approximation weight {} vs optimum {}", max_edges(4, &two), maximum_spanning_tree(4, &two), weight(&two, &two_approximation(4, &two)), weight(&two, &maximum_spanning_tree(4, &two)));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn random_connected(r: &mut Rng) -> (usize, Vec<Edge>) {
        let n = r.range(2, 12);
        let mut weights: Vec<i64> = (1..=200).collect();
        r.shuffle(&mut weights);
        let mut edges: Vec<Edge> = (1..n).map(|v| (r.range(0, v - 1), v, 0)).collect();
        for _ in 0..r.range(0, 15) { let (a, b) = (r.range(0, n - 1), r.range(0, n - 1)); if a != b { edges.push((a, b, 0)); } }
        for (e, w) in edges.iter_mut().zip(weights) { e.2 = w; }
        (n, edges)
    }
    #[test]
    fn s_is_inside_t_and_the_weight_bounds_hold() {
        let mut r = Rng::new(513);
        for _ in 0..300 {
            let (n, edges) = random_connected(&mut r);
            let (s, t) = (max_edges(n, &edges), maximum_spanning_tree(n, &edges));
            assert!(s.iter().all(|i| t.contains(i)), "S_G ⊆ T_G");
            assert!(weight(&edges, &t) <= 2 * weight(&edges, &s), "w(T) ≤ 2 w(S)");
            let a = two_approximation(n, &edges);
            assert_eq!(a.len(), n - 1);
            let mut p: Vec<usize> = (0..n).collect();
            assert!(a.iter().all(|&i| { let (x, y) = (find(&mut p, edges[i].0), find(&mut p, edges[i].1)); if x != y { p[x] = y; true } else { false } }), "a tree");
            assert!(2 * weight(&edges, &a) >= weight(&edges, &t), "2-approximation");
        }
    }
    #[test]
    fn examples_where_s_equals_and_differs_from_t() {
        let path: Vec<Edge> = vec![(0, 1, 1), (1, 2, 3), (2, 3, 2)];
        assert_eq!(max_edges(4, &path).len(), 3);
        let two: Vec<Edge> = vec![(0, 1, 5), (2, 3, 6), (1, 2, 1)];
        assert_eq!(max_edges(4, &two), vec![0, 1]);
        assert_eq!(maximum_spanning_tree(4, &two).len(), 3);
    }
}
