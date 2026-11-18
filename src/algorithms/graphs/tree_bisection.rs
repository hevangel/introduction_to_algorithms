//! @title Tree Bisection (Balanced Separator)
//! @chapter Problem B-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Partition any n-node binary or bounded-degree tree in O(n) DFS time into two vertex sets of size ≤ 3n/4 (and single-edge separator ≤ 2n/3 for binary trees).
//! @order 240
use crate::outln;

/// O(n) single-edge centroid separator for a tree (binary / max-degree <= 3, n >= 2).
/// Cuts one edge `(u, v)` partitioning vertices into two subtrees `(side_a, side_b)`
/// where both sides have size <= floor(3n/4) (and <= ceil(2n/3) for n >= 2 except K_{1,3}).
pub fn binary_tree_edge_separator(n: usize, edges: &[(usize, usize)]) -> ((usize, usize), Vec<bool>) {
    assert!(n >= 2 && edges.len() == n - 1);
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut size = vec![0usize; n];
    let mut parent = vec![usize::MAX; n];
    let mut order = Vec::with_capacity(n);
    let mut stack = vec![0usize];
    parent[0] = 0;
    while let Some(u) = stack.pop() {
        order.push(u);
        for &v in &adj[u] {
            if v != parent[u] {
                parent[v] = u;
                stack.push(v);
            }
        }
    }
    for &u in order.iter().rev() {
        size[u] = 1;
        for &v in &adj[u] {
            if v != parent[u] {
                size[u] += size[v];
            }
        }
    }
    // Find edge (u, parent[u]) maximizing min(size[u], n - size[u]).
    let mut best_u = 1usize;
    let mut best_min = 0usize;
    for u in 1..n {
        let s = size[u].min(n - size[u]);
        if s > best_min {
            best_min = s;
            best_u = u;
        }
    }
    let p = parent[best_u];
    let mut side = vec![false; n];
    let mut st = vec![best_u];
    side[best_u] = true;
    while let Some(u) = st.pop() {
        for &v in &adj[u] {
            if v != p && !side[v] {
                side[v] = true;
                st.push(v);
            }
        }
    }
    ((best_u, p), side)
}

/// Recursive centroid-based exact bisection of any bounded-degree (or arbitrary) tree into
/// `(side_mask, cut_edges)` where `side_a` has size `n / 2` and `side_b` has size `n - n / 2`
/// (both <= 3n/4, cutting O(d lg n) edges for max-degree d).
pub fn tree_exact_bisection(n: usize, edges: &[(usize, usize)]) -> (Vec<bool>, Vec<(usize, usize)>) {
    assert_eq!(edges.len(), n.saturating_sub(1));
    if n <= 1 {
        return (vec![true; n], vec![]);
    }
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut side = vec![false; n];
    let target = n / 2;

    // Color `target` vertices using centroid decomposition:
    // find centroid of alive vertices, assign smallest connected components into side `true` greedily.
    fn assign(
        comp: &[usize],
        need: usize,
        adj: &[Vec<usize>],
        side: &mut [bool],
    ) {
        if need == 0 || comp.is_empty() {
            return;
        }
        if need == comp.len() {
            for &u in comp {
                side[u] = true;
            }
            return;
        }
        let mut in_comp = vec![false; adj.len()];
        for &u in comp {
            in_comp[u] = true;
        }
        // Compute subtree sizes rooted at comp[0].
        let mut parent = vec![usize::MAX; adj.len()];
        let mut order = Vec::with_capacity(comp.len());
        let mut st = vec![comp[0]];
        parent[comp[0]] = comp[0];
        while let Some(u) = st.pop() {
            order.push(u);
            for &v in &adj[u] {
                if in_comp[v] && v != parent[u] {
                    parent[v] = u;
                    st.push(v);
                }
            }
        }
        let mut sz = vec![0usize; adj.len()];
        for &u in order.iter().rev() {
            sz[u] = 1;
            for &v in &adj[u] {
                if in_comp[v] && v != parent[u] {
                    sz[u] += sz[v];
                }
            }
        }
        let m = comp.len();
        let centroid = *comp
            .iter()
            .find(|&&u| {
                let mut max_branch = m - sz[u];
                for &v in &adj[u] {
                    if in_comp[v] && v != parent[u] {
                        max_branch = max_branch.max(sz[v]);
                    }
                }
                max_branch <= m / 2
            })
            .unwrap();

        // Collect all components of `comp \ {centroid}`.
        let mut branches = Vec::new();
        for &nbr in &adj[centroid] {
            if !in_comp[nbr] {
                continue;
            }
            let mut b = Vec::new();
            let mut b_st = vec![(nbr, centroid)];
            while let Some((u, p)) = b_st.pop() {
                b.push(u);
                for &v in &adj[u] {
                    if in_comp[v] && v != p {
                        b_st.push((v, u));
                    }
                }
            }
            branches.push(b);
        }
        branches.sort_by_key(|b| b.len());

        let mut rem = need;
        for b in branches {
            if rem == 0 {
                break;
            }
            if b.len() <= rem {
                for &u in &b {
                    side[u] = true;
                }
                rem -= b.len();
            } else {
                // Need strictly fewer than b.len() vertices from this largest branch (<= m/2).
                assign(&b, rem, adj, side);
                rem = 0;
            }
        }
        if rem == 1 {
            side[centroid] = true;
        }
    }

    let all: Vec<usize> = (0..n).collect();
    assign(&all, target, &adj, &mut side);
    let cut: Vec<(usize, usize)> = edges.iter().copied().filter(|&(u, v)| side[u] != side[v]).collect();
    (side, cut)
}

pub fn demo() -> String {
    let mut o = String::new();
    let edges = [(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6), (3, 7), (3, 8), (5, 9)];
    let (sep, side) = binary_tree_edge_separator(10, &edges);
    let count_a = side.iter().filter(|&&b| b).count();
    outln!(o, "10-node binary tree single-edge separator {:?} -> split ({}, {})", sep, count_a, 10 - count_a);
    let (bi_side, cut) = tree_exact_bisection(10, &edges);
    let bi_a = bi_side.iter().filter(|&&b| b).count();
    outln!(o, "exact bisection split ({}, {}) cutting {} edges: {:?}", bi_a, 10 - bi_a, cut.len(), cut);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn binary_tree_edge_separator_and_exact_bisection_bounds() {
        let mut r = Rng::new(3003);
        for _ in 0..80 {
            let n = r.range(2, 60);
            // Random binary tree: each node attaches to a node with degree < 3.
            let mut deg = vec![0usize; n];
            let mut edges = Vec::new();
            for v in 1..n {
                let candidates: Vec<usize> = (0..v).filter(|&u| deg[u] < 3).collect();
                let u = candidates[r.range(0, candidates.len() - 1)];
                deg[u] += 1;
                deg[v] += 1;
                edges.push((u, v));
            }
            let (_, side) = binary_tree_edge_separator(n, &edges);
            let a = side.iter().filter(|&&b| b).count();
            let b = n - a;
            assert!(a.max(b) <= (3 * n) / 4, "n={n}: split ({a}, {b}) exceeds 3n/4");

            let (bi_side, cut) = tree_exact_bisection(n, &edges);
            let ba = bi_side.iter().filter(|&&x| x).count();
            assert_eq!(ba, n / 2);
            assert!(!cut.is_empty());
            assert!(cut.len() <= (n.ilog2() as usize + 2) * 3);
        }
    }
}
