//! @title APSP by Matrix Multiplication
//! @chapter 25.1 (3e) / 23.1 (4e) / 26.1 (1e)
//! @editions 1,2,3,4
//! @summary Shortest paths via (min,+) matrix products: SLOW-ALL-PAIRS Θ(V⁴), FASTER-ALL-PAIRS Θ(V³ lg V).
//! @order 160
use crate::outln;

pub const INF: i64 = i64::MAX / 4;
pub type Matrix = Vec<Vec<i64>>;

/// EXTEND-SHORTEST-PATHS(L, W): the (min, +) product L ⊗ W — Θ(n³).
pub fn extend_shortest_paths(l: &Matrix, w: &Matrix) -> Matrix {
    let n = l.len();
    let mut out = vec![vec![INF; n]; n];
    for i in 0..n { for j in 0..n { for k in 0..n {
        if l[i][k] < INF && w[k][j] < INF { out[i][j] = out[i][j].min(l[i][k] + w[k][j]); }
    } } }
    out
}

/// SLOW-ALL-PAIRS-SHORTEST-PATHS: L^(m) for m = 1 … n−1, one extension at a time — Θ(n⁴).
pub fn slow_all_pairs(w: &Matrix) -> Matrix {
    let n = w.len();
    let mut l = w.clone();
    for _ in 2..n { l = extend_shortest_paths(&l, w); }
    l
}

/// FASTER-ALL-PAIRS-SHORTEST-PATHS: repeated squaring, ⌈lg(n−1)⌉ products — Θ(n³ lg n).
pub fn faster_all_pairs(w: &Matrix) -> Matrix {
    let n = w.len();
    let mut l = w.clone();
    let mut m = 1;
    while m < n.saturating_sub(1) {
        l = extend_shortest_paths(&l, &l);
        m *= 2;
    }
    l
}

pub fn demo() -> String {
    let mut o = String::new();
    let e = [(0, 1, 3), (0, 2, 8), (0, 4, -4), (1, 3, 1), (1, 4, 7), (2, 1, 4), (3, 0, 2), (3, 2, -5), (4, 3, 6)];
    let w = super::floyd_warshall::matrix(5, &e);
    let slow = slow_all_pairs(&w);
    let fast = faster_all_pairs(&w);
    outln!(o, "SLOW (4 extensions):");
    for r in &slow { outln!(o, "  {:?}", r); }
    outln!(o, "FASTER (2 squarings) gives the same matrix: {}", slow == fast);
    outln!(o, "min-plus product used {} times vs {}", 2, 3);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_floyd_warshall() {
        let mut r = Rng::new(137);
        for _ in 0..30 {
            let n = r.range(1, 9);
            let edges: Vec<(usize, usize, i64)> = (0..r.range(0, 25)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(0, 20) as i64)).collect();
            let w = super::super::floyd_warshall::matrix(n, &edges);
            let (fw, _) = super::super::floyd_warshall::floyd_warshall(&w);
            assert_eq!(slow_all_pairs(&w), fw);
            assert_eq!(faster_all_pairs(&w), fw);
        }
    }
    #[test]
    fn identity_for_min_plus() {
        let mut id = vec![vec![INF; 3]; 3];
        for i in 0..3 { id[i][i] = 0; }
        let a = vec![vec![0, 2, INF], vec![INF, 0, 5], vec![1, INF, 0]];
        assert_eq!(extend_shortest_paths(&a, &id), a);
        assert_eq!(extend_shortest_paths(&id, &a), a);
    }
}
