//! @title The Generic MST Method (Safe Edges)
//! @chapter 23.1 (1e–3e) / 21.1 (4e)
//! @editions 1,2,3,4
//! @summary GENERIC-MST: repeatedly add a light edge crossing a cut that respects the current forest A — Kruskal and Prim are special cases.
//! @order 78
use crate::outln;

pub type Edge = (usize, usize, i64);

/// GENERIC-MST on a connected graph. At each step the cut is (S, V−S) where S is the connected component of A containing vertex 0
/// (so no edge of A crosses it); the lightest crossing edge is a *safe edge* (Theorem 23.1) and joins A.
/// Returns (total weight, chosen edge indices, the sequence of cuts' |S| sizes).
pub fn generic_mst(n: usize, edges: &[Edge]) -> (i64, Vec<usize>, Vec<usize>) {
    let mut a: Vec<usize> = vec![];
    let mut cut_sizes = vec![];
    while a.len() + 1 < n {
        let mut in_s = vec![false; n];
        let mut stack = vec![0];
        in_s[0] = true;
        while let Some(u) = stack.pop() {
            for &i in &a { let (x, y, _) = edges[i]; let other = if x == u { y } else if y == u { x } else { continue }; if !in_s[other] { in_s[other] = true; stack.push(other); } }
        }
        cut_sizes.push(in_s.iter().filter(|&&b| b).count());
        // light edge crossing the cut
        let light = (0..edges.len()).filter(|&i| in_s[edges[i].0] != in_s[edges[i].1]).min_by_key(|&i| edges[i].2);
        match light { Some(i) => a.push(i), None => break } // disconnected graph: stop
    }
    (a.iter().map(|&i| edges[i].2).sum(), a, cut_sizes)
}

pub fn demo() -> String {
    let mut o = String::new();
    let e: Vec<Edge> = vec![(0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2), (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7)];
    let (w, a, cuts) = generic_mst(9, &e);
    outln!(o, "GENERIC-MST total weight {w} (Kruskal: {})", super::mst_kruskal::kruskal(9, &e).0);
    outln!(o, "edges added in order: {:?}", a.iter().map(|&i| (e[i].0, e[i].1, e[i].2)).collect::<Vec<_>>());
    outln!(o, "size of the vertex set S on the source side of each cut: {:?}", cuts);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn equals_kruskal_weight_on_random_connected_graphs() {
        let mut r = Rng::new(350);
        for _ in 0..100 {
            let n = r.range(1, 12);
            let mut edges: Vec<Edge> = (1..n).map(|v| (r.range(0, v - 1), v, r.range(1, 30) as i64)).collect();
            for _ in 0..r.range(0, 20) { let (a, b) = (r.range(0, n - 1), r.range(0, n - 1)); if a != b { edges.push((a, b, r.range(1, 30) as i64)); } }
            let (w, a, _) = generic_mst(n, &edges);
            assert_eq!(w, super::super::mst_kruskal::kruskal(n, &edges).0);
            assert_eq!(a.len(), n.saturating_sub(1));
        }
    }
    #[test]
    fn disconnected_graph_stops_early() {
        let (_, a, _) = generic_mst(4, &[(0, 1, 1), (2, 3, 1)]);
        assert_eq!(a.len(), 1);
    }
}
