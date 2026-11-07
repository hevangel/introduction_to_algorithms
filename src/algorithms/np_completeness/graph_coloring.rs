//! @title Graph Coloring
//! @chapter Problem 34-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary 2-colorability is decidable in O(V+E) (bipartite test), 3-coloring and chromatic number are NP-complete; greedy uses at most Δ+1 colors; exact search by backtracking.
//! @order 60
use crate::outln;
use std::collections::VecDeque;

/// Problem 34-3(b): is the graph 2-colorable (bipartite)? Returns a coloring if so. O(V + E).
pub fn two_color(adj: &[Vec<usize>]) -> Option<Vec<u8>> {
    let n = adj.len();
    let mut color = vec![u8::MAX; n];
    for s in 0..n {
        if color[s] != u8::MAX { continue; }
        color[s] = 0;
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                if color[v] == u8::MAX { color[v] = 1 - color[u]; q.push_back(v); }
                else if color[v] == color[u] { return None; }
            }
        }
    }
    Some(color)
}

/// Greedy coloring in the given vertex order: each vertex gets the smallest color unused by its neighbours. Uses at most Δ + 1 colors.
pub fn greedy_coloring(adj: &[Vec<usize>], order: &[usize]) -> Vec<usize> {
    let mut color = vec![usize::MAX; adj.len()];
    for &u in order {
        let used: Vec<usize> = adj[u].iter().filter_map(|&v| (color[v] != usize::MAX).then_some(color[v])).collect();
        color[u] = (0..).find(|c| !used.contains(c)).unwrap();
    }
    color
}

/// Exact k-coloring by backtracking (exponential worst case — the problem is NP-complete for k ≥ 3).
pub fn k_coloring(adj: &[Vec<usize>], k: usize) -> Option<Vec<usize>> {
    fn go(adj: &[Vec<usize>], k: usize, u: usize, color: &mut Vec<usize>) -> bool {
        if u == adj.len() { return true; }
        let max_used = color[..u].iter().copied().filter(|&c| c != usize::MAX).max().map_or(0, |m| m + 1);
        for c in 0..k.min(max_used + 1) { // symmetry breaking: never skip a fresh color
            if adj[u].iter().all(|&v| v >= u || color[v] != c) { color[u] = c; if go(adj, k, u + 1, color) { return true; } }
        }
        color[u] = usize::MAX;
        false
    }
    let mut color = vec![usize::MAX; adj.len()];
    go(adj, k, 0, &mut color).then_some(color)
}

/// The chromatic number χ(G) by trying k = 1, 2, 3, …
pub fn chromatic_number(adj: &[Vec<usize>]) -> usize { if adj.is_empty() { 0 } else { (1..).find(|&k| k_coloring(adj, k).is_some()).unwrap() } }

pub fn is_proper(adj: &[Vec<usize>], color: &[usize]) -> bool { adj.iter().enumerate().all(|(u, vs)| vs.iter().all(|&v| color[u] != color[v])) }

pub fn demo() -> String {
    let mut o = String::new();
    let cycle = |n: usize| -> Vec<Vec<usize>> { (0..n).map(|i| vec![(i + 1) % n, (i + n - 1) % n]).collect() };
    for n in [4, 5, 6, 7] { outln!(o, "C{n}: 2-colorable {}, χ = {}", two_color(&cycle(n)).is_some(), chromatic_number(&cycle(n))); }
    let k4: Vec<Vec<usize>> = (0..4).map(|i| (0..4).filter(|&j| j != i).collect()).collect();
    outln!(o, "K4: χ = {}, greedy uses {} colors", chromatic_number(&k4), greedy_coloring(&k4, &[0, 1, 2, 3]).iter().max().unwrap() + 1);
    // Petersen graph: χ = 3
    let mut pet = vec![vec![]; 10];
    for i in 0..5 { for (a, b) in [(i, (i + 1) % 5), (i, i + 5), (i + 5, (i + 2) % 5 + 5)] { pet[a].push(b); pet[b].push(a); } }
    outln!(o, "Petersen graph: χ = {} (bipartite? {})", chromatic_number(&pet), two_color(&pet).is_some());
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
    #[test]
    fn exact_coloring_matches_exhaustive_enumeration() {
        let mut r = Rng::new(412);
        for _ in 0..150 {
            let n = r.range(1, 6);
            let adj = random_graph(&mut r, n, 0.5);
            let chi = (1..=n).find(|&k| (0..k.pow(n as u32)).any(|code| { let c: Vec<usize> = (0..n).map(|i| code / k.pow(i as u32) % k).collect(); is_proper(&adj, &c) })).unwrap();
            assert_eq!(chromatic_number(&adj), chi);
            let col = k_coloring(&adj, chi).unwrap();
            assert!(is_proper(&adj, &col) && col.iter().all(|&c| c < chi));
            assert_eq!(two_color(&adj).is_some(), chi <= 2);
        }
    }
    #[test]
    fn greedy_uses_at_most_max_degree_plus_one() {
        let mut r = Rng::new(413);
        for _ in 0..100 {
            let n = r.range(1, 20);
            let adj = random_graph(&mut r, n, 0.3);
            let mut order: Vec<usize> = (0..n).collect();
            r.shuffle(&mut order);
            let col = greedy_coloring(&adj, &order);
            let delta = adj.iter().map(|a| a.len()).max().unwrap();
            assert!(is_proper(&adj, &col));
            assert!(col.iter().max().unwrap() + 1 <= delta + 1);
        }
    }
    #[test]
    fn known_chromatic_numbers() {
        let cycle = |n: usize| -> Vec<Vec<usize>> { (0..n).map(|i| vec![(i + 1) % n, (i + n - 1) % n]).collect() };
        assert_eq!((chromatic_number(&cycle(6)), chromatic_number(&cycle(7))), (2, 3));
        assert_eq!(chromatic_number(&[]), 0);
    }
}
