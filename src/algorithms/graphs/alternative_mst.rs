//! @title Alternative MST Algorithms (MAYBE-MST-A/B/C)
//! @chapter Problem 23-4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Three candidate algorithms: reverse-delete (correct), add-if-acyclic in arbitrary order (wrong), add-edge-and-drop-heaviest-cycle-edge (correct).
//! @order 82
use crate::outln;

pub type Edge = (usize, usize, i64);

fn connected(n: usize, edges: &[Edge], keep: &[bool], skip: Option<usize>) -> bool {
    let mut adj = vec![vec![]; n];
    for (i, &(u, v, _)) in edges.iter().enumerate() { if keep[i] && Some(i) != skip { adj[u].push(v); adj[v].push(u); } }
    let (mut seen, mut st) = (vec![false; n], vec![0]);
    seen[0] = true;
    while let Some(u) = st.pop() { for &v in &adj[u] { if !seen[v] { seen[v] = true; st.push(v); } } }
    seen.iter().all(|&b| b)
}

/// MAYBE-MST-A: sort edges by weight **decreasing**; remove an edge if the graph stays connected without it. Correct (reverse-delete).
pub fn maybe_mst_a(n: usize, edges: &[Edge]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..edges.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(edges[i].2));
    let mut keep = vec![true; edges.len()];
    for i in order { if connected(n, edges, &keep, Some(i)) { keep[i] = false; } }
    (0..edges.len()).filter(|&i| keep[i]).collect()
}

/// MAYBE-MST-B: scan the edges in the *given* (arbitrary) order, adding each edge that does not create a cycle. Returns a spanning tree but not necessarily a minimum one.
pub fn maybe_mst_b(n: usize, edges: &[Edge]) -> Vec<usize> {
    let mut p: Vec<usize> = (0..n).collect();
    fn find(p: &mut Vec<usize>, x: usize) -> usize { let mut r = x; while p[r] != r { r = p[r]; } let mut c = x; while p[c] != r { let nx = p[c]; p[c] = r; c = nx; } r }
    let mut t = vec![];
    for (i, &(u, v, _)) in edges.iter().enumerate() { let (a, b) = (find(&mut p, u), find(&mut p, v)); if a != b { p[a] = b; t.push(i); } }
    t
}

/// MAYBE-MST-C: scan edges in arbitrary order, add each to T and, if that creates a cycle, delete a maximum-weight edge on the cycle. Correct.
pub fn maybe_mst_c(n: usize, edges: &[Edge]) -> Vec<usize> {
    let mut t: Vec<usize> = vec![];
    for i in 0..edges.len() {
        t.push(i);
        // look for a cycle through edge i: a path in T − {i} between its endpoints
        let mut adj = vec![vec![]; n];
        for &j in &t { if j != i { adj[edges[j].0].push((edges[j].1, j)); adj[edges[j].1].push((edges[j].0, j)); } }
        let (s, goal) = (edges[i].0, edges[i].1);
        let mut parent: Vec<Option<(usize, usize)>> = vec![None; n];
        let (mut seen, mut st) = (vec![false; n], vec![s]);
        seen[s] = true;
        while let Some(u) = st.pop() { for &(v, j) in &adj[u] { if !seen[v] { seen[v] = true; parent[v] = Some((u, j)); st.push(v); } } }
        if seen[goal] {
            let mut cycle = vec![i];
            let mut v = goal;
            while let Some((u, j)) = parent[v] { cycle.push(j); v = u; }
            let heaviest = *cycle.iter().max_by_key(|&&j| edges[j].2).unwrap();
            t.retain(|&j| j != heaviest);
        }
    }
    t
}

pub fn weight(edges: &[Edge], t: &[usize]) -> i64 { t.iter().map(|&i| edges[i].2).sum() }

pub fn demo() -> String {
    let mut o = String::new();
    // a triangle listed heaviest first defeats MAYBE-MST-B
    let e: Vec<Edge> = vec![(0, 1, 9), (1, 2, 5), (0, 2, 1)];
    let k = super::mst_kruskal::kruskal(3, &e).0;
    outln!(o, "triangle with weights 9, 5, 1 in that order; MST weight = {k}");
    outln!(o, "MAYBE-MST-A (reverse delete) → {}", weight(&e, &maybe_mst_a(3, &e)));
    outln!(o, "MAYBE-MST-B (arbitrary order, add if acyclic) → {}  ← not minimum!", weight(&e, &maybe_mst_b(3, &e)));
    outln!(o, "MAYBE-MST-C (add, drop heaviest on cycle) → {}", weight(&e, &maybe_mst_c(3, &e)));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn a_and_c_are_correct_and_b_is_not() {
        let mut r = Rng::new(351);
        let mut b_wrong = 0;
        for _ in 0..150 {
            let n = r.range(2, 8);
            let mut edges: Vec<Edge> = (1..n).map(|v| (r.range(0, v - 1), v, r.range(1, 20) as i64)).collect();
            for _ in 0..r.range(0, 10) { let (a, b) = (r.range(0, n - 1), r.range(0, n - 1)); if a != b { edges.push((a, b, r.range(1, 20) as i64)); } }
            r.shuffle(&mut edges);
            let best = super::super::mst_kruskal::kruskal(n, &edges).0;
            let (a, b, c) = (maybe_mst_a(n, &edges), maybe_mst_b(n, &edges), maybe_mst_c(n, &edges));
            assert_eq!(weight(&edges, &a), best);
            assert_eq!(weight(&edges, &c), best);
            for t in [&a, &b, &c] { assert_eq!(t.len(), n - 1); }
            assert!(weight(&edges, &b) >= best);
            if weight(&edges, &b) > best { b_wrong += 1; }
        }
        assert!(b_wrong > 10, "MAYBE-MST-B should fail on many inputs, failed {b_wrong} times");
    }
}
