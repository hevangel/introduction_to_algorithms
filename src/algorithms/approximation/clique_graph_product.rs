//! @title Approximating the Size of a Maximum Clique (Graph Powers)
//! @chapter Problem 35-2 (3e)
//! @editions 3,4
//! @summary The k-th power G^(k) has ω(G^(k)) = ω(G)^k, so a constant-factor approximation for CLIQUE could be amplified into a (1+ε)-approximation — and then, by the problem's argument, into a polynomial-time approximation scheme.
//! @order 45
use crate::outln;

/// Adjacency bitmasks of G (n ≤ 8 vertices so that G² has ≤ 64).
pub type Adj = Vec<u64>;

pub fn from_edges(n: usize, edges: &[(usize, usize)]) -> Adj { let mut a = vec![0u64; n]; for &(u, v) in edges { a[u] |= 1 << v; a[v] |= 1 << u; } a }

/// Maximum clique by branching on bitmasks (≤ 64 vertices). Returns a vertex set as a bitmask.
pub fn max_clique(adj: &[u64]) -> u64 {
    fn go(adj: &[u64], cand: u64, cur: u64, best: &mut u64) {
        if cur.count_ones() > best.count_ones() { *best = cur; }
        if cur.count_ones() + cand.count_ones() <= best.count_ones() { return; }
        let mut c = cand;
        while c != 0 {
            let v = c.trailing_zeros() as usize;
            c &= c - 1;
            if cur.count_ones() + (cand & adj[v]).count_ones() + 1 > best.count_ones() { go(adj, cand & adj[v] & !((1u64 << v) - 1) & !(1u64 << v), cur | 1 << v, best); }
        }
    }
    let mut best = 0;
    let all = if adj.len() == 64 { u64::MAX } else { (1u64 << adj.len()) - 1 };
    go(adj, all, 0, &mut best);
    best
}

/// The k = 2 power G²: vertices are pairs (u₁, u₂) numbered u₁·n + u₂; (u₁,u₂) ~ (v₁,v₂) iff the pairs differ and, in each coordinate, the
/// entries are equal or adjacent in G.
pub fn square(adj: &[u64]) -> Adj {
    let n = adj.len();
    let near = |a: usize, b: usize| a == b || adj[a] >> b & 1 == 1;
    let mut g = vec![0u64; n * n];
    for u in 0..n * n { for v in 0..n * n { if u != v && near(u / n, v / n) && near(u % n, v % n) { g[u] |= 1 << v; } } }
    g
}

/// From a clique C of G² recover a clique of G of size ≥ √|C|: the larger of its two coordinate projections is a clique in G.
pub fn recover_clique(c: u64, n: usize) -> u64 {
    let (mut p1, mut p2) = (0u64, 0u64);
    for v in 0..n * n { if c >> v & 1 == 1 { p1 |= 1 << (v / n); p2 |= 1 << (v % n); } }
    if p1.count_ones() >= p2.count_ones() { p1 } else { p2 }
}

pub fn demo() -> String {
    let mut o = String::new();
    // 5-cycle plus a chord: ω = 3 (triangle)
    let g = from_edges(6, &[(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 5), (5, 0)]);
    let w = max_clique(&g).count_ones();
    let g2 = square(&g);
    let w2 = max_clique(&g2).count_ones();
    outln!(o, "ω(G) = {w}, ω(G²) = {w2} = ω(G)²: {}", w2 == w * w);
    // a clique of G² of size 5 (not maximum) still yields a clique of G of size ≥ ⌈√5⌉ = 3
    let some_clique = max_clique(&g2);
    outln!(o, "recovered a clique of G of size {} from a clique of G² of size {}", recover_clique(some_clique, 6).count_ones(), some_clique.count_ones());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn is_clique(adj: &[u64], set: u64) -> bool { (0..adj.len()).all(|u| set >> u & 1 == 0 || (0..adj.len()).all(|v| u == v || set >> v & 1 == 0 || adj[u] >> v & 1 == 1)) }
    #[test]
    fn clique_number_of_the_square_is_the_square_of_the_clique_number() {
        let mut r = Rng::new(479);
        for _ in 0..80 {
            let n = r.range(1, 7);
            let edges: Vec<(usize, usize)> = (0..n).flat_map(|u| (u + 1..n).map(move |v| (u, v))).filter(|_| r.f64() < 0.5).collect();
            let g = from_edges(n, &edges);
            let w = max_clique(&g).count_ones();
            assert!(is_clique(&g, max_clique(&g)));
            assert_eq!(max_clique(&square(&g)).count_ones(), w * w);
        }
    }
    #[test]
    fn recovered_cliques_are_cliques_of_size_at_least_the_square_root() {
        let mut r = Rng::new(480);
        for _ in 0..80 {
            let n = r.range(2, 7);
            let edges: Vec<(usize, usize)> = (0..n).flat_map(|u| (u + 1..n).map(move |v| (u, v))).filter(|_| r.f64() < 0.5).collect();
            let g = from_edges(n, &edges);
            let g2 = square(&g);
            let c = max_clique(&g2);
            let rec = recover_clique(c, n);
            assert!(is_clique(&g, rec));
            assert!((rec.count_ones() as f64) >= (c.count_ones() as f64).sqrt() - 1e-9);
        }
    }
}
