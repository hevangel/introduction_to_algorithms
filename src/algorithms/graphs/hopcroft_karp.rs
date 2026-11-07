//! @title Hopcroft-Karp Matching
//! @chapter 25.1 (4e)
//! @editions 4
//! @summary Find many shortest vertex-disjoint augmenting paths per phase: O(E√V) maximum bipartite matching.
//! @order 200
use crate::outln;
use std::collections::VecDeque;

const NIL: usize = usize::MAX;

/// Returns (matching size, partner of each left vertex, partner of each right vertex). `adj[l]` lists right neighbours.
pub fn hopcroft_karp(left: usize, right: usize, adj: &[Vec<usize>]) -> (usize, Vec<usize>, Vec<usize>) {
    let (mut ml, mut mr) = (vec![NIL; left], vec![NIL; right]);
    let mut size = 0;
    loop {
        // BFS builds layers of the graph of alternating paths starting from free left vertices
        let mut dist = vec![usize::MAX; left];
        let mut q = VecDeque::new();
        for l in 0..left { if ml[l] == NIL { dist[l] = 0; q.push_back(l); } }
        let mut found = false;
        while let Some(l) = q.pop_front() {
            for &r in &adj[l] {
                match mr[r] {
                    NIL => found = true,
                    l2 if dist[l2] == usize::MAX => { dist[l2] = dist[l] + 1; q.push_back(l2); }
                    _ => {}
                }
            }
        }
        if !found { break; }
        // DFS finds a maximal set of vertex-disjoint shortest augmenting paths
        fn dfs(l: usize, adj: &[Vec<usize>], dist: &mut Vec<usize>, ml: &mut Vec<usize>, mr: &mut Vec<usize>) -> bool {
            for i in 0..adj[l].len() {
                let r = adj[l][i];
                let ok = match mr[r] {
                    NIL => true,
                    l2 => dist[l2] == dist[l] + 1 && dfs(l2, adj, dist, ml, mr),
                };
                if ok { ml[l] = r; mr[r] = l; return true; }
            }
            dist[l] = usize::MAX; // dead end: don't revisit this phase
            false
        }
        for l in 0..left {
            if ml[l] == NIL && dfs(l, adj, &mut dist, &mut ml, &mut mr) { size += 1; }
        }
    }
    (size, ml, mr)
}

pub fn demo() -> String {
    let mut o = String::new();
    let adj = vec![vec![0, 1], vec![0], vec![1, 2], vec![2, 3], vec![3]];
    let (size, ml, _) = hopcroft_karp(5, 4, &adj);
    outln!(o, "adjacency (left → right): {:?}", adj);
    outln!(o, "maximum matching size = {}", size);
    for (l, &r) in ml.iter().enumerate() { if r != NIL { outln!(o, "  left {l} — right {r}"); } else { outln!(o, "  left {l} unmatched"); } }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn agrees_with_flow_based_matching() {
        let mut r = Rng::new(141);
        for _ in 0..100 {
            let (l, rr) = (r.range(1, 12), r.range(1, 12));
            let mut edges: Vec<(usize, usize)> = (0..r.range(0, 40)).map(|_| (r.range(0, l - 1), r.range(0, rr - 1))).collect();
            edges.sort();
            edges.dedup();
            let mut adj = vec![vec![]; l];
            for &(a, b) in &edges { adj[a].push(b); }
            let (size, ml, mr) = hopcroft_karp(l, rr, &adj);
            assert_eq!(size, super::super::maximum_bipartite_matching::maximum_matching(l, rr, &edges).len());
            for a in 0..l { if ml[a] != NIL { assert_eq!(mr[ml[a]], a); assert!(adj[a].contains(&ml[a])); } }
            assert_eq!(ml.iter().filter(|&&x| x != NIL).count(), size);
        }
    }
    #[test]
    fn perfect_matching_found() {
        let n = 50;
        let adj: Vec<Vec<usize>> = (0..n).map(|i| vec![i, (i + 1) % n]).collect();
        assert_eq!(hopcroft_karp(n, n, &adj).0, n);
    }
}
