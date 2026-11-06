//! @title Articulation Points, Bridges & Biconnected Components
//! @chapter Problem 22-2 (3e) / 20-2 (4e) / 23-2 (1e–2e)
//! @editions 1,2,3,4
//! @summary Low-link DFS finds cut vertices, bridges and the biconnected components of an undirected graph in Θ(V+E).
//! @order 60
use crate::outln;

#[derive(Debug, Clone, PartialEq)]
pub struct Result_ {
    pub articulation_points: Vec<usize>,
    /// Edge ids of bridges.
    pub bridges: Vec<usize>,
    /// Each biconnected component as a list of edge ids.
    pub components: Vec<Vec<usize>>,
}

/// Analyse an undirected multigraph given as an edge list (self-loops are ignored).
pub fn analyze(n: usize, edges: &[(usize, usize)]) -> Result_ {
    struct C<'a> {
        adj: Vec<Vec<(usize, usize)>>,
        disc: Vec<usize>,
        low: Vec<usize>,
        t: usize,
        estack: Vec<usize>,
        ap: Vec<bool>,
        bridges: Vec<usize>,
        comps: Vec<Vec<usize>>,
        _e: &'a [(usize, usize)],
    }
    fn go(c: &mut C, u: usize, pe: usize) {
        c.t += 1;
        c.disc[u] = c.t;
        c.low[u] = c.t;
        let mut children = 0;
        for i in 0..c.adj[u].len() {
            let (v, id) = c.adj[u][i];
            if id == pe { continue; }
            if c.disc[v] == 0 {
                children += 1;
                c.estack.push(id);
                go(c, v, id);
                c.low[u] = c.low[u].min(c.low[v]);
                if c.low[v] >= c.disc[u] {
                    if pe != usize::MAX { c.ap[u] = true; }
                    let mut comp = vec![];
                    loop {
                        let e = c.estack.pop().unwrap();
                        comp.push(e);
                        if e == id { break; }
                    }
                    comp.sort();
                    c.comps.push(comp);
                }
                if c.low[v] > c.disc[u] { c.bridges.push(id); }
            } else if c.disc[v] < c.disc[u] {
                c.estack.push(id);
                c.low[u] = c.low[u].min(c.disc[v]);
            }
        }
        if pe == usize::MAX && children > 1 { c.ap[u] = true; }
    }
    let mut adj = vec![vec![]; n];
    for (id, &(a, b)) in edges.iter().enumerate() {
        if a != b { adj[a].push((b, id)); adj[b].push((a, id)); }
    }
    let mut c = C { adj, disc: vec![0; n], low: vec![0; n], t: 0, estack: vec![], ap: vec![false; n], bridges: vec![], comps: vec![], _e: edges };
    for s in 0..n { if c.disc[s] == 0 { go(&mut c, s, usize::MAX); } }
    let mut bridges = c.bridges;
    bridges.sort();
    let mut comps = c.comps;
    comps.sort();
    Result_ { articulation_points: (0..n).filter(|&v| c.ap[v]).collect(), bridges, components: comps }
}

pub fn demo() -> String {
    let mut o = String::new();
    // two triangles joined by the bridge (2,3), plus a pendant vertex 6 hanging off 5
    let edges = [(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 5), (5, 3), (5, 6)];
    let r = analyze(7, &edges);
    outln!(o, "edges: {:?}", edges);
    outln!(o, "articulation points: {:?}", r.articulation_points);
    outln!(o, "bridges            : {:?}", r.bridges.iter().map(|&b| edges[b]).collect::<Vec<_>>());
    for c in &r.components { outln!(o, "biconnected component: {:?}", c.iter().map(|&e| edges[e]).collect::<Vec<_>>()); }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn components(n: usize, edges: &[(usize, usize)], skip_v: Option<usize>, skip_e: Option<usize>) -> usize {
        let mut p: Vec<usize> = (0..n).collect();
        fn f(p: &mut Vec<usize>, x: usize) -> usize { if p[x] != x { let r = f(p, p[x]); p[x] = r; } p[x] }
        for (i, &(a, b)) in edges.iter().enumerate() {
            if Some(i) == skip_e || Some(a) == skip_v || Some(b) == skip_v { continue; }
            let (x, y) = (f(&mut p, a), f(&mut p, b));
            p[x] = y;
        }
        (0..n).filter(|&v| Some(v) != skip_v && f(&mut p, v) == v).count()
    }
    #[test]
    fn book_style_example() {
        let r = analyze(7, &[(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 5), (5, 3), (5, 6)]);
        assert_eq!(r.articulation_points, vec![2, 3, 5]);
        assert_eq!(r.bridges, vec![3, 7]);
        assert_eq!(r.components.len(), 4);
    }
    #[test]
    fn matches_removal_definition_on_random_graphs() {
        let mut rg = Rng::new(126);
        for _ in 0..80 {
            let n = rg.range(1, 9);
            let edges: Vec<(usize, usize)> = (0..rg.range(0, 14)).map(|_| (rg.range(0, n - 1), rg.range(0, n - 1))).collect();
            let r = analyze(n, &edges);
            let base = components(n, &edges, None, None);
            for v in 0..n {
                let is_ap = components(n, &edges, Some(v), None) > base;
                assert_eq!(r.articulation_points.contains(&v), is_ap, "vertex {v} in {edges:?}");
            }
            for e in 0..edges.len() {
                if edges[e].0 == edges[e].1 { continue; }
                let is_bridge = components(n, &edges, None, Some(e)) > base;
                assert_eq!(r.bridges.contains(&e), is_bridge, "edge {e} in {edges:?}");
            }
            // every non-loop edge lies in exactly one biconnected component
            let mut seen: Vec<usize> = r.components.iter().flatten().copied().collect();
            seen.sort();
            let want: Vec<usize> = (0..edges.len()).filter(|&e| edges[e].0 != edges[e].1).collect();
            assert_eq!(seen, want);
        }
    }
}
