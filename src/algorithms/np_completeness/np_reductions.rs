//! @title Polynomial-Time Reductions
//! @chapter 34.3, 34.5 (1e–3e) / 36.3, 36.5 (4e)
//! @editions 1,2,3,4
//! @summary 3-CNF-SAT ≤ₚ CLIQUE ≤ₚ VERTEX-COVER and HAM-CYCLE ≤ₚ TSP, implemented as instance transformations and tested for equivalence.
//! @order 40
use super::np_verification::Cnf;
use crate::outln;

pub type Graph = Vec<Vec<bool>>;

/// 3-CNF-SAT ≤ₚ CLIQUE (Theorem 34.11): one vertex per literal occurrence; join vertices of *different* clauses whose literals are consistent.
/// Returns (graph, vertex → (clause, literal), k = number of clauses). The formula is satisfiable iff the graph has a k-clique.
pub fn sat_to_clique(f: &Cnf) -> (Graph, Vec<(usize, i32)>, usize) {
    let verts: Vec<(usize, i32)> = f.iter().enumerate().flat_map(|(i, c)| c.iter().map(move |&l| (i, l))).collect();
    let n = verts.len();
    let mut g = vec![vec![false; n]; n];
    for a in 0..n { for b in 0..n {
        if verts[a].0 != verts[b].0 && verts[a].1 != -verts[b].1 { g[a][b] = true; }
    } }
    (g, verts, f.len())
}

/// CLIQUE ≤ₚ VERTEX-COVER (Theorem 34.12): G has a k-clique iff its complement has a vertex cover of size |V| − k.
pub fn clique_to_vertex_cover(g: &Graph, k: usize) -> (Vec<(usize, usize)>, usize) {
    let n = g.len();
    let mut edges = vec![];
    for u in 0..n { for v in u + 1..n { if !g[u][v] { edges.push((u, v)); } } }
    (edges, n - k)
}

/// HAM-CYCLE ≤ₚ TSP (Section 34.5.4): distance 0 for edges, 1 for non-edges; G is Hamiltonian iff a tour of cost ≤ 0 exists.
pub fn hamiltonian_to_tsp(g: &Graph) -> Vec<Vec<f64>> {
    let n = g.len();
    (0..n).map(|i| (0..n).map(|j| if i == j { 0.0 } else if g[i][j] { 0.0 } else { 1.0 }).collect()).collect()
}

/// Maximum clique size by branch-and-bound over vertex subsets (exponential).
pub fn max_clique(g: &Graph) -> usize {
    fn go(g: &Graph, cand: Vec<usize>, size: usize, best: &mut usize) {
        *best = (*best).max(size);
        if size + cand.len() <= *best { return; }
        for (i, &v) in cand.iter().enumerate() {
            let next: Vec<usize> = cand[i + 1..].iter().copied().filter(|&u| g[v][u]).collect();
            go(g, next, size + 1, best);
        }
    }
    let mut best = 0;
    go(g, (0..g.len()).collect(), 0, &mut best);
    best
}

/// Minimum vertex cover size (exponential subset search).
pub fn min_vertex_cover(n: usize, edges: &[(usize, usize)]) -> usize {
    (0u32..1 << n).filter(|m| edges.iter().all(|&(u, v)| m >> u & 1 == 1 || m >> v & 1 == 1)).map(|m| m.count_ones() as usize).min().unwrap()
}

pub fn demo() -> String {
    let mut o = String::new();
    // the book's example: φ = (x1 ∨ ¬x2 ∨ ¬x3) ∧ (¬x1 ∨ x2 ∨ x3) ∧ (x1 ∨ x2 ∨ x3)
    let f: Cnf = vec![vec![1, -2, -3], vec![-1, 2, 3], vec![1, 2, 3]];
    let (g, verts, k) = sat_to_clique(&f);
    outln!(o, "φ has {} clauses → graph with {} vertices", k, verts.len());
    outln!(o, "max clique size = {} (satisfiable iff it equals k = {k})", max_clique(&g));
    let (edges, kc) = clique_to_vertex_cover(&g, k);
    outln!(o, "complement graph: {} edges; vertex-cover size needed = {}; true minimum = {}", edges.len(), kc, min_vertex_cover(verts.len(), &edges));
    let mut ring = vec![vec![false; 5]; 5];
    for i in 0..5 { ring[i][(i + 1) % 5] = true; ring[(i + 1) % 5][i] = true; }
    let d = hamiltonian_to_tsp(&ring);
    outln!(o, "5-cycle → TSP instance, optimal tour cost = {:?} (0 ⇒ Hamiltonian)", super::hamiltonian_cycle::traveling_salesman_exact(&d).map(|x| x.0));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::np_completeness::cnf_sat::brute_force_sat;
    use crate::common::Rng;
    #[test]
    fn sat_iff_clique_of_size_k() {
        let mut r = Rng::new(244);
        let (mut sat, mut unsat) = (0, 0);
        for _ in 0..300 {
            let (n, m) = (r.range(1, 3), r.range(2, 8));
            let f: Cnf = (0..m).map(|_| (0..3).map(|_| { let v = r.range(1, n) as i32; if r.range(0, 1) == 0 { v } else { -v } }).collect()).collect();
            let (g, _, k) = sat_to_clique(&f);
            let has_clique = max_clique(&g) >= k;
            assert_eq!(has_clique, brute_force_sat(n, &f).is_some(), "{f:?}");
            if has_clique { sat += 1 } else { unsat += 1 }
        }
        assert!(sat > 20 && unsat > 20, "{sat}/{unsat}");
    }
    #[test]
    fn clique_iff_complement_vertex_cover() {
        let mut r = Rng::new(245);
        for _ in 0..100 {
            let n = r.range(2, 9);
            let mut g = vec![vec![false; n]; n];
            for i in 0..n { for j in i + 1..n { if r.f64() < 0.5 { g[i][j] = true; g[j][i] = true; } } }
            for k in 0..=n {
                let (edges, kc) = clique_to_vertex_cover(&g, k);
                assert_eq!(max_clique(&g) >= k, min_vertex_cover(n, &edges) <= kc, "n={n} k={k}");
            }
        }
    }
    #[test]
    fn hamiltonian_iff_zero_cost_tour() {
        let mut r = Rng::new(246);
        for _ in 0..100 {
            let n = r.range(3, 7);
            let mut g = vec![vec![false; n]; n];
            for i in 0..n { for j in i + 1..n { if r.f64() < 0.5 { g[i][j] = true; g[j][i] = true; } } }
            let cost = super::super::hamiltonian_cycle::traveling_salesman_exact(&hamiltonian_to_tsp(&g)).unwrap().0;
            assert_eq!(cost == 0.0, super::super::hamiltonian_cycle::hamiltonian_cycle_backtracking(&g).is_some());
        }
    }
}
