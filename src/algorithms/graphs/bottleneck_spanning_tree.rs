//! @title Bottleneck Spanning Trees
//! @chapter Problem 23-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary A spanning tree minimising its heaviest edge: every MST is one; test "bottleneck ≤ b" by connectivity of the light edges and search over weights.
//! @order 83
use crate::outln;

pub type Edge = (usize, usize, i64);

fn find(p: &mut Vec<usize>, x: usize) -> usize { let mut r = x; while p[r] != r { r = p[r]; } let mut c = x; while p[c] != r { let nx = p[c]; p[c] = r; c = nx; } r }

/// Does the subgraph of edges with weight ≤ b connect all n vertices? (A spanning tree with bottleneck ≤ b exists iff yes.)
pub fn connected_with_limit(n: usize, edges: &[Edge], b: i64) -> bool {
    let mut p: Vec<usize> = (0..n).collect();
    let mut comps = n;
    for &(u, v, w) in edges { if w <= b { let (x, y) = (find(&mut p, u), find(&mut p, v)); if x != y { p[x] = y; comps -= 1; } } }
    comps <= 1
}

/// Smallest possible bottleneck weight over all spanning trees, found by binary search over the sorted distinct weights: O(E lg E).
pub fn min_bottleneck(n: usize, edges: &[Edge]) -> Option<i64> {
    if n <= 1 { return Some(i64::MIN); }
    let mut ws: Vec<i64> = edges.iter().map(|e| e.2).collect();
    ws.sort();
    ws.dedup();
    let idx = ws.partition_point(|&b| !connected_with_limit(n, edges, b));
    ws.get(idx).copied()
}

/// The bottleneck (maximum edge weight) of a set of tree edges.
pub fn bottleneck(edges: &[Edge], tree: &[usize]) -> Option<i64> { tree.iter().map(|&i| edges[i].2).max() }

pub fn demo() -> String {
    let mut o = String::new();
    let e: Vec<Edge> = vec![(0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2), (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7)];
    let (_, mst) = super::mst_kruskal::kruskal(9, &e);
    outln!(o, "minimum bottleneck by search: {:?}", min_bottleneck(9, &e));
    outln!(o, "bottleneck of the MST (Kruskal): {:?}  — an MST is always a minimum bottleneck spanning tree", bottleneck(&e, &mst));
    outln!(o, "connected using edges ≤ 8? {}; ≤ 9? {}", connected_with_limit(9, &e, 8), connected_with_limit(9, &e, 9));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn spanning_tree_bottlenecks(n: usize, edges: &[Edge]) -> Vec<i64> {
        let m = edges.len();
        (0u32..1 << m).filter(|mask| mask.count_ones() as usize == n - 1).filter_map(|mask| {
            let ids: Vec<usize> = (0..m).filter(|i| mask >> i & 1 == 1).collect();
            let mut p: Vec<usize> = (0..n).collect();
            ids.iter().all(|&i| { let (a, b) = (find(&mut p, edges[i].0), find(&mut p, edges[i].1)); if a == b { false } else { p[a] = b; true } }).then(|| ids.iter().map(|&i| edges[i].2).max().unwrap())
        }).collect()
    }
    #[test]
    fn matches_brute_force_and_the_mst_is_a_bottleneck_tree() {
        let mut r = Rng::new(352);
        for _ in 0..100 {
            let n = r.range(2, 6);
            let mut edges: Vec<Edge> = (1..n).map(|v| (r.range(0, v - 1), v, r.range(1, 15) as i64)).collect();
            for _ in 0..r.range(0, 5) { let (a, b) = (r.range(0, n - 1), r.range(0, n - 1)); if a != b { edges.push((a, b, r.range(1, 15) as i64)); } }
            let want = *spanning_tree_bottlenecks(n, &edges).iter().min().unwrap();
            assert_eq!(min_bottleneck(n, &edges), Some(want));
            let (_, mst) = super::super::mst_kruskal::kruskal(n, &edges);
            assert_eq!(bottleneck(&edges, &mst), Some(want));
        }
    }
    #[test]
    fn disconnected_graph_has_no_spanning_tree() {
        assert_eq!(min_bottleneck(4, &[(0, 1, 3), (2, 3, 5)]), None);
        assert!(connected_with_limit(2, &[(0, 1, 3)], 3) && !connected_with_limit(2, &[(0, 1, 3)], 2));
    }
}
