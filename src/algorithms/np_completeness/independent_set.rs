//! @title Independent Set
//! @chapter Problem 34-1 (3e)
//! @editions 3,4
//! @summary An independent set is a vertex set with no edges inside: NP-hard in general (complement of CLIQUE), but linear on trees by dynamic programming and polynomial on bipartite graphs via maximum matching (König).
//! @order 70
use crate::algorithms::graphs::hopcroft_karp::hopcroft_karp;
use crate::outln;

/// Maximum independent set of a graph with at most 64 vertices by branching: take a vertex of maximum degree and try both "in" (delete its
/// neighbours) and "out"; isolated vertices are always taken. Exponential in the worst case.
pub fn max_independent_set(adj: &[Vec<usize>]) -> Vec<usize> {
    assert!(adj.len() <= 64);
    let nbr: Vec<u64> = adj.iter().map(|a| a.iter().fold(0u64, |m, &v| m | 1 << v)).collect();
    fn go(nbr: &[u64], alive: u64) -> u64 {
        if alive == 0 { return 0; }
        // pick the live vertex with most live neighbours
        let (mut best_v, mut best_d) = (alive.trailing_zeros() as usize, 0);
        let mut a = alive;
        while a != 0 { let v = a.trailing_zeros() as usize; a &= a - 1; let d = (nbr[v] & alive).count_ones(); if d >= best_d { best_d = d; best_v = v; } }
        if best_d == 0 { return alive; } // no edges left: all remaining vertices are independent
        let v = best_v;
        let with_v = (1u64 << v) | go(nbr, alive & !(1u64 << v) & !nbr[v]);
        let without_v = go(nbr, alive & !(1u64 << v));
        if with_v.count_ones() >= without_v.count_ones() { with_v } else { without_v }
    }
    let all = if adj.len() == 64 { u64::MAX } else { (1u64 << adj.len()) - 1 };
    let m = go(&nbr, all);
    (0..adj.len()).filter(|&v| m >> v & 1 == 1).collect()
}

/// Maximum independent set size of a forest by DP: in[v] = 1 + Σ out[child], out[v] = Σ max(in[child], out[child]). O(V).
pub fn mis_forest(adj: &[Vec<usize>]) -> usize {
    let n = adj.len();
    let (mut seen, mut total) = (vec![false; n], 0);
    for root in 0..n {
        if seen[root] { continue; }
        // iterative DFS to get an order, then process children before parents
        let (mut order, mut parent, mut st) = (vec![], vec![usize::MAX; n], vec![root]);
        seen[root] = true;
        while let Some(u) = st.pop() { order.push(u); for &v in &adj[u] { if !seen[v] { seen[v] = true; parent[v] = u; st.push(v); } } }
        let (mut inc, mut exc) = (vec![1usize; n], vec![0usize; n]);
        for &u in order.iter().rev() {
            if parent[u] != usize::MAX { let p = parent[u]; inc[p] += exc[u]; exc[p] += inc[u].max(exc[u]); }
        }
        total += inc[root].max(exc[root]);
    }
    total
}

/// Maximum independent set size of a bipartite graph = |V| − (size of a maximum matching) (König's theorem); None if the graph is not bipartite.
pub fn mis_bipartite(adj: &[Vec<usize>]) -> Option<usize> {
    let colors = crate::algorithms::np_completeness::graph_coloring::two_color(adj)?;
    let left: Vec<usize> = (0..adj.len()).filter(|&v| colors[v] == 0).collect();
    let right: Vec<usize> = (0..adj.len()).filter(|&v| colors[v] == 1).collect();
    let (li, ri) = (|v: usize| left.iter().position(|&x| x == v).unwrap(), |v: usize| right.iter().position(|&x| x == v).unwrap());
    let ladj: Vec<Vec<usize>> = left.iter().map(|&u| adj[u].iter().map(|&v| ri(v)).collect()).collect();
    let _ = li;
    let (m, _, _) = hopcroft_karp(left.len(), right.len(), &ladj);
    Some(adj.len() - m)
}

/// CLIQUE ↔ INDEPENDENT-SET: a clique in G is an independent set in the complement graph (Problem 34-1(a)).
pub fn complement(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = adj.len();
    (0..n).map(|u| (0..n).filter(|&v| v != u && !adj[u].contains(&v)).collect()).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    let c5: Vec<Vec<usize>> = (0..5).map(|i| vec![(i + 1) % 5, (i + 4) % 5]).collect();
    outln!(o, "5-cycle: maximum independent set {:?}", max_independent_set(&c5));
    let path: Vec<Vec<usize>> = (0usize..7).map(|i| [i.wrapping_sub(1), i + 1].into_iter().filter(|&x| x < 7).collect()).collect();
    outln!(o, "path on 7 vertices: forest DP {}, bipartite/König {:?}, exact {}", mis_forest(&path), mis_bipartite(&path), max_independent_set(&path).len());
    outln!(o, "5-cycle is not bipartite: {:?}; its complement (also a 5-cycle) has clique number = independence number = {}", mis_bipartite(&c5), max_independent_set(&complement(&c5)).len());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn random_graph(r: &mut Rng, n: usize, p: f64) -> Vec<Vec<usize>> {
        let mut adj = vec![vec![]; n];
        for u in 0..n { for v in u + 1..n { if r.f64() < p { adj[u].push(v); adj[v].push(u); } } }
        adj
    }
    fn brute(adj: &[Vec<usize>]) -> usize {
        let n = adj.len();
        (0u32..1 << n).filter(|m| (0..n).all(|u| m >> u & 1 == 0 || adj[u].iter().all(|&v| m >> v & 1 == 0))).map(|m| m.count_ones() as usize).max().unwrap()
    }
    #[test]
    fn exact_search_matches_enumeration_and_is_independent() {
        let mut r = Rng::new(474);
        for _ in 0..150 {
            let n = r.range(1, 12);
            let p = r.f64() * 0.8;
            let adj = random_graph(&mut r, n, p);
            let s = max_independent_set(&adj);
            assert!(s.iter().all(|&u| s.iter().all(|&v| !adj[u].contains(&v))));
            assert_eq!(s.len(), brute(&adj));
        }
    }
    #[test]
    fn forest_and_bipartite_algorithms_match_the_exact_search() {
        let mut r = Rng::new(475);
        for _ in 0..150 {
            let n = r.range(1, 14);
            let mut forest = vec![vec![]; n];
            for v in 1..n { if r.f64() < 0.85 { let p = r.range(0, v - 1); forest[v].push(p); forest[p].push(v); } }
            assert_eq!(mis_forest(&forest), brute(&forest));
            assert_eq!(mis_bipartite(&forest), Some(brute(&forest)));
            // a random bipartite graph between the first half and the second half
            let h = n / 2;
            let mut bip = vec![vec![]; n];
            for u in 0..h { for v in h..n { if r.f64() < 0.4 { bip[u].push(v); bip[v].push(u); } } }
            assert_eq!(mis_bipartite(&bip), Some(brute(&bip)));
        }
        let c5: Vec<Vec<usize>> = (0..5).map(|i| vec![(i + 1) % 5, (i + 4) % 5]).collect();
        assert_eq!(mis_bipartite(&c5), None);
    }
    #[test]
    fn cliques_are_independent_sets_of_the_complement() {
        let mut r = Rng::new(476);
        for _ in 0..100 {
            let n = r.range(1, 10);
            let adj = random_graph(&mut r, n, 0.5);
            let clique_number = (0u32..1 << n).filter(|m| (0..n).all(|u| m >> u & 1 == 0 || (0..n).all(|v| u == v || m >> v & 1 == 0 || adj[u].contains(&v)))).map(|m| m.count_ones() as usize).max().unwrap();
            assert_eq!(max_independent_set(&complement(&adj)).len(), clique_number);
        }
    }
}
