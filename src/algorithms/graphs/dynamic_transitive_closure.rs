//! @title Transitive Closure of a Dynamic Graph
//! @chapter Problem 25-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Maintain the closure under edge insertions in O(V²) per insertion, and O(V³) total for any sequence.
//! @order 145
use crate::outln;

/// The transitive closure (reachability matrix) of a growing directed graph.
#[derive(Debug, Clone)]
pub struct IncrementalClosure { n: usize, reach: Vec<Vec<bool>>, /// Inner-loop steps performed so far (to illustrate the O(V³) total bound).
    pub work: usize }

impl IncrementalClosure {
    pub fn new(n: usize) -> Self { let mut reach = vec![vec![false; n]; n]; for i in 0..n { reach[i][i] = true; } IncrementalClosure { n, reach, work: 0 } }
    pub fn reachable(&self, u: usize, v: usize) -> bool { self.reach[u][v] }
    pub fn matrix(&self) -> &Vec<Vec<bool>> { &self.reach }
    /// INSERT-EDGE(u, v): every vertex i that reaches u now reaches everything v reaches. The inner loop runs only for pairs (i, v) that
    /// become newly reachable — each pair at most once — so the total is O(V²·V) = O(V³).
    pub fn add_edge(&mut self, u: usize, v: usize) {
        for i in 0..self.n {
            self.work += 1;
            if self.reach[i][u] && !self.reach[i][v] {
                for j in 0..self.n { self.work += 1; if self.reach[v][j] { self.reach[i][j] = true; } }
            }
        }
    }
}

/// Recompute the closure from scratch with Warshall's algorithm (for comparison).
pub fn warshall(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<bool>> {
    let mut t = vec![vec![false; n]; n];
    for i in 0..n { t[i][i] = true; }
    for &(u, v) in edges { t[u][v] = true; }
    for k in 0..n { for i in 0..n { for j in 0..n { t[i][j] = t[i][j] || (t[i][k] && t[k][j]); } } }
    t
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut c = IncrementalClosure::new(6);
    for (u, v) in [(0, 1), (1, 2), (3, 4), (2, 3), (5, 0)] {
        c.add_edge(u, v);
        let pairs = (0..6).flat_map(|a| (0..6).map(move |b| (a, b))).filter(|&(a, b)| a != b && c.reachable(a, b)).count();
        outln!(o, "after inserting {u}→{v}: {pairs} reachable ordered pairs");
    }
    let mut r = crate::common::Rng::new(1);
    let n = 60;
    let mut big = IncrementalClosure::new(n);
    for _ in 0..n * n { big.add_edge(r.range(0, n - 1), r.range(0, n - 1)); }
    outln!(o, "{} insertions into a {n}-vertex graph cost {} steps (V³ = {})", n * n, big.work, n * n * n);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_recomputation_after_every_insertion() {
        let mut r = Rng::new(365);
        for _ in 0..50 {
            let n = r.range(1, 10);
            let mut c = IncrementalClosure::new(n);
            let mut edges = vec![];
            for _ in 0..r.range(0, 25) {
                let (u, v) = (r.range(0, n - 1), r.range(0, n - 1));
                edges.push((u, v));
                c.add_edge(u, v);
                assert_eq!(c.matrix(), &warshall(n, &edges));
            }
        }
    }
    #[test]
    fn total_work_is_cubic_even_for_many_insertions() {
        let n = 40;
        let mut c = IncrementalClosure::new(n);
        let mut r = Rng::new(366);
        let inserts = 4 * n * n;
        for _ in 0..inserts { c.add_edge(r.range(0, n - 1), r.range(0, n - 1)); }
        // one outer step per (insertion, vertex) plus at most V inner steps for each of the ≤ V² newly reachable pairs
        assert!(c.work <= inserts * n + n * n * n, "{}", c.work);
    }
}
