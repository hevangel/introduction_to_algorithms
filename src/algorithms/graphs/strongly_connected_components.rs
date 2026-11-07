//! @title Strongly Connected Components
//! @chapter 22.5 (3e) / 20.5 (4e) / 23.5 (1e–2e)
//! @editions 1,2,3,4
//! @summary Kosaraju's two-pass DFS (CLRS) and Tarjan's one-pass algorithm; the component DAG.
//! @order 50
use super::graph_representations::transpose;
use crate::outln;

/// STRONGLY-CONNECTED-COMPONENTS(G): DFS(G), then DFS(Gᵀ) in decreasing finish order of the first pass.
pub fn strongly_connected_components(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = adj.len();
    let first = super::depth_first_search::dfs(adj);
    let t = transpose(&adj.to_vec());
    let mut seen = vec![false; n];
    let mut comps = vec![];
    for &s in first.finish_order.iter().rev() {
        if seen[s] { continue; }
        let (mut comp, mut stack) = (vec![], vec![s]);
        seen[s] = true;
        while let Some(u) = stack.pop() {
            comp.push(u);
            for &v in &t[u] { if !seen[v] { seen[v] = true; stack.push(v); } }
        }
        comp.sort();
        comps.push(comp);
    }
    comps
}

/// Tarjan's algorithm (one DFS with low-link values).
pub fn tarjan(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    struct S<'a> { adj: &'a [Vec<usize>], idx: Vec<usize>, low: Vec<usize>, on: Vec<bool>, st: Vec<usize>, next: usize, out: Vec<Vec<usize>> }
    fn visit(s: &mut S, u: usize) {
        s.next += 1;
        s.idx[u] = s.next;
        s.low[u] = s.next;
        s.st.push(u);
        s.on[u] = true;
        for i in 0..s.adj[u].len() {
            let v = s.adj[u][i];
            if s.idx[v] == 0 { visit(s, v); s.low[u] = s.low[u].min(s.low[v]); }
            else if s.on[v] { s.low[u] = s.low[u].min(s.idx[v]); }
        }
        if s.low[u] == s.idx[u] {
            let mut comp = vec![];
            loop {
                let w = s.st.pop().unwrap();
                s.on[w] = false;
                comp.push(w);
                if w == u { break; }
            }
            comp.sort();
            s.out.push(comp);
        }
    }
    let n = adj.len();
    let mut s = S { adj, idx: vec![0; n], low: vec![0; n], on: vec![false; n], st: vec![], next: 0, out: vec![] };
    for u in 0..n { if s.idx[u] == 0 { visit(&mut s, u); } }
    s.out
}

/// The component graph G^SCC: vertices are component indices, edges connect different components.
pub fn component_graph(adj: &[Vec<usize>], comps: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut id = vec![0; adj.len()];
    for (c, vs) in comps.iter().enumerate() { for &v in vs { id[v] = c; } }
    let mut g: Vec<Vec<usize>> = vec![vec![]; comps.len()];
    for (u, vs) in adj.iter().enumerate() { for &v in vs { if id[u] != id[v] && !g[id[u]].contains(&id[v]) { g[id[u]].push(id[v]); } } }
    g
}

pub fn demo() -> String {
    let mut o = String::new();
    // Figure 22.9: vertices a..h
    let names = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let edges = [(0, 1), (1, 2), (1, 4), (1, 5), (2, 3), (2, 6), (3, 2), (3, 7), (4, 0), (4, 5), (5, 6), (6, 5), (6, 7), (7, 7)];
    let mut adj = vec![vec![]; 8];
    for &(a, b) in &edges { adj[a].push(b); }
    let show = |cs: &Vec<Vec<usize>>| cs.iter().map(|c| format!("{{{}}}", c.iter().map(|&v| names[v]).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(" ");
    let mut k = strongly_connected_components(&adj);
    outln!(o, "Kosaraju (CLRS): {}", show(&k));
    k.sort();
    let mut t = tarjan(&adj);
    t.sort();
    outln!(o, "Tarjan agrees  : {}", k == t);
    let comps = strongly_connected_components(&adj);
    outln!(o, "component DAG  : {:?}", component_graph(&adj, &comps));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn reach(adj: &[Vec<usize>]) -> Vec<Vec<bool>> {
        let n = adj.len();
        let mut r = vec![vec![false; n]; n];
        for u in 0..n { r[u][u] = true; for &v in &adj[u] { r[u][v] = true; } }
        for k in 0..n { for i in 0..n { for j in 0..n { if r[i][k] && r[k][j] { r[i][j] = true; } } } }
        r
    }
    #[test]
    fn book_figure() {
        let edges = [(0, 1), (1, 2), (1, 4), (1, 5), (2, 3), (2, 6), (3, 2), (3, 7), (4, 0), (4, 5), (5, 6), (6, 5), (6, 7), (7, 7)];
        let mut adj = vec![vec![]; 8];
        for &(a, b) in &edges { adj[a].push(b); }
        let mut c = strongly_connected_components(&adj);
        c.sort();
        assert_eq!(c, vec![vec![0, 1, 4], vec![2, 3], vec![5, 6], vec![7]]);
    }
    #[test]
    fn matches_reachability_definition() {
        let mut rg = Rng::new(125);
        for _ in 0..60 {
            let n = rg.range(1, 12);
            let adj: Vec<Vec<usize>> = (0..n).map(|u| (0..n).filter(|&v| u != v && rg.f64() < 0.18).collect()).collect();
            let r = reach(&adj);
            let mut k = strongly_connected_components(&adj);
            let mut t = tarjan(&adj);
            k.sort();
            t.sort();
            assert_eq!(k, t);
            let mut label = vec![0; n];
            for (c, vs) in k.iter().enumerate() { for &v in vs { label[v] = c; } }
            for u in 0..n { for v in 0..n { assert_eq!(label[u] == label[v], r[u][v] && r[v][u]); } }
            let cg = component_graph(&adj, &k);
            assert!(super::super::topological_sort::topological_sort(&cg).is_some()); // condensation is a DAG
        }
    }
}
