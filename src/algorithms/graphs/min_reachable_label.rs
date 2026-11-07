//! @title Minimum Reachable Label
//! @chapter Problem 22-4 (1e–4e) / 20-4
//! @editions 1,2,3,4
//! @summary For every vertex v find the smallest-labelled vertex reachable from v, in O(V+E): sweep labels upward and search the transpose graph.
//! @order 55
use crate::outln;

/// `label[v]` are distinct vertex labels. Returns `min_label[v]` = min{ label[u] : u reachable from v } (v itself counts).
pub fn min_reachable_label(adj: &[Vec<usize>], label: &[usize]) -> Vec<usize> {
    let n = adj.len();
    let mut radj = vec![vec![]; n]; // transpose graph
    for (u, vs) in adj.iter().enumerate() { for &v in vs { radj[v].push(u); } }
    let mut by_label: Vec<usize> = (0..n).collect();
    by_label.sort_by_key(|&v| label[v]);
    let mut best = vec![usize::MAX; n];
    for &s in &by_label {
        // everything that can reach s and is not yet assigned has s as its minimum reachable label
        if best[s] != usize::MAX { continue; }
        let mut st = vec![s];
        best[s] = label[s];
        while let Some(u) = st.pop() { for &p in &radj[u] { if best[p] == usize::MAX { best[p] = label[s]; st.push(p); } } }
    }
    best
}

pub fn demo() -> String {
    let mut o = String::new();
    let adj = vec![vec![1], vec![2], vec![0, 3], vec![], vec![3], vec![4]];
    let label = [5, 2, 4, 6, 1, 3];
    outln!(o, "edges {:?}, labels {:?}", adj, label);
    outln!(o, "minimum reachable label per vertex: {:?}", min_reachable_label(&adj, &label));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_search_from_every_vertex() {
        let mut r = Rng::new(354);
        for _ in 0..200 {
            let n = r.range(1, 14);
            let adj: Vec<Vec<usize>> = (0..n).map(|u| (0..n).filter(|&v| u != v && r.f64() < 0.15).collect()).collect();
            let mut label: Vec<usize> = (0..n).collect();
            r.shuffle(&mut label);
            let got = min_reachable_label(&adj, &label);
            for s in 0..n {
                let (mut seen, mut st) = (vec![false; n], vec![s]);
                seen[s] = true;
                while let Some(u) = st.pop() { for &v in &adj[u] { if !seen[v] { seen[v] = true; st.push(v); } } }
                assert_eq!(got[s], (0..n).filter(|&v| seen[v]).map(|v| label[v]).min().unwrap());
            }
        }
    }
}
