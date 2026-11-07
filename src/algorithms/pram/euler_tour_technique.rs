//! @title Euler-Tour Technique on Trees
//! @chapter 30.1–30.2 (1e) application
//! @editions 1
//! @summary Compute every vertex's depth in O(lg n) PRAM time by listing the Euler tour and running a parallel prefix over ±1.
//! @order 40
use crate::outln;

/// Depth of every vertex of the rooted tree given by `children`, computed with the Euler-tour technique:
/// 1. build the tour as a linked list of directed edges; 2. weight down-edges +1 and up-edges −1;
/// 3. scan the list by pointer jumping (O(lg n) rounds); depth(v) = prefix sum at the edge entering v.
/// Returns (depths, pointer-jumping rounds).
pub fn depths(children: &[Vec<usize>], root: usize) -> (Vec<usize>, usize) {
    let n = children.len();
    if n == 1 { return (vec![0], 0); }
    let mut parent = vec![usize::MAX; n];
    for (p, cs) in children.iter().enumerate() { for &c in cs { parent[c] = p; } }
    let adj: Vec<Vec<usize>> = (0..n).map(|v| {
        let mut a = vec![];
        if parent[v] != usize::MAX { a.push(parent[v]); }
        a.extend(&children[v]);
        a
    }).collect();
    // directed-edge ids: 2v = (parent[v] → v) "down", 2v+1 = (v → parent[v]) "up"
    let edge = |a: usize, b: usize| if parent[b] == a { 2 * b } else { 2 * a + 1 };
    let ends = |e: usize| { let v = e / 2; if e % 2 == 0 { (parent[v], v) } else { (v, parent[v]) } };
    let mut next: Vec<Option<usize>> = vec![None; 2 * n];
    let mut valid = vec![false; 2 * n];
    for v in 0..n { if v != root { valid[2 * v] = true; valid[2 * v + 1] = true; } }
    for e in (0..2 * n).filter(|&e| valid[e]) {
        let (u, v) = ends(e);
        let pos = adj[v].iter().position(|&x| x == u).unwrap();
        let w = adj[v][(pos + 1) % adj[v].len()];
        next[e] = Some(edge(v, w)); // the tour continues from v along the next edge in v's circular adjacency order
    }
    // the tour is a cycle starting with root → first child; cut it before that edge
    let start = edge(root, children[root][0]);
    let last = (0..2 * n).find(|&e| next[e] == Some(start)).unwrap();
    next[last] = None;
    let weight: Vec<i64> = (0..2 * n).map(|e| if !valid[e] { 0 } else if e % 2 == 0 { 1 } else { -1 }).collect();
    // suffix sums over the list by pointer jumping
    let (mut suf, mut nxt, mut rounds) = (weight.clone(), next, 0);
    while nxt.iter().any(|x| x.is_some()) {
        let (s_old, n_old) = (suf.clone(), nxt.clone());
        for i in 0..2 * n { if let Some(j) = n_old[i] { suf[i] = s_old[i] + s_old[j]; nxt[i] = n_old[j]; } }
        rounds += 1;
    }
    let total: i64 = weight.iter().sum(); // = 0 for a closed tour
    let mut depth = vec![0usize; n];
    for v in 0..n { if v != root { let e = 2 * v; depth[v] = (total - suf[e] + weight[e]) as usize; } }
    (depth, rounds)
}

pub fn demo() -> String {
    let mut o = String::new();
    //         0
    //       / | \
    //      1  2  3
    //     / \     \
    //    4   5     6
    //        |
    //        7
    let children = vec![vec![1, 2, 3], vec![4, 5], vec![], vec![6], vec![], vec![7], vec![], vec![]];
    let (d, rounds) = depths(&children, 0);
    outln!(o, "children lists: {:?}", children);
    outln!(o, "depths via Euler tour + list scan: {:?} ({} pointer-jumping rounds over {} tour edges)", d, rounds, 2 * 7);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn sequential_depths(children: &[Vec<usize>], root: usize) -> Vec<usize> {
        let mut d = vec![0; children.len()];
        let mut st = vec![root];
        while let Some(u) = st.pop() { for &c in &children[u] { d[c] = d[u] + 1; st.push(c); } }
        d
    }
    #[test]
    fn matches_sequential_depths_on_random_trees() {
        let mut r = Rng::new(163);
        for n in [1usize, 2, 3, 8, 30, 100] {
            let mut children = vec![vec![]; n];
            for v in 1..n { children[r.range(0, v - 1)].push(v); }
            let (d, rounds) = depths(&children, 0);
            assert_eq!(d, sequential_depths(&children, 0), "n={n}");
            if n > 1 { assert!(rounds <= (2.0 * n as f64).log2().ceil() as usize + 1); }
        }
    }
    #[test]
    fn path_and_star() {
        let path: Vec<Vec<usize>> = (0..6).map(|i| if i < 5 { vec![i + 1] } else { vec![] }).collect();
        assert_eq!(depths(&path, 0).0, vec![0, 1, 2, 3, 4, 5]);
        let star = vec![vec![1, 2, 3, 4], vec![], vec![], vec![], vec![]];
        assert_eq!(depths(&star, 0).0, vec![0, 1, 1, 1, 1]);
    }
}
