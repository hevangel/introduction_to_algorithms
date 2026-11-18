//! @title Fractional Bipartite Matching & Vertex Labels
//! @chapter Problems 25-4 & 25-5 (4e)
//! @editions 4
//! @summary Round a fractional bipartite matching to an integral matching of ≥ weight in O(E) time via alternating cycles/paths, and recover optimal Egerváry vertex labels from a matching.
//! @order 225
use crate::outln;

/// Problem 25-4: Given a bipartite graph `(left, right, edges)` where each edge is `(u, v, weight)`
/// and a feasible fractional matching `x` (`0 <= x[e] <= 1`, sum of `x` at every vertex <= 1),
/// round `x` into an integral matching (`0` or `1` per edge) with total weight >= original fractional weight.
pub fn round_fractional_matching(
    left: usize,
    right: usize,
    edges: &[(usize, usize, f64)],
    x_init: &[f64],
) -> Vec<usize> {
    const EPS: f64 = 1e-11;
    let m = edges.len();
    assert_eq!(m, x_init.len());
    let mut x = x_init.to_vec();
    let n_total = left + right;
    let mut active: Vec<bool> = x.iter().map(|&v| v > EPS && v < 1.0 - EPS).collect();

    loop {
        let mut deg = vec![0usize; n_total];
        let mut adj = vec![Vec::new(); n_total];
        for (e, &(u, v, _)) in edges.iter().enumerate() {
            if active[e] {
                let ru = u;
                let rv = left + v;
                adj[ru].push((rv, e));
                adj[rv].push((ru, e));
                deg[ru] += 1;
                deg[rv] += 1;
            }
        }
        let Some(start_e) = (0..m).find(|&e| active[e]) else {
            break;
        };
        let _ = start_e;

        // First look for any cycle in the fractional edge subgraph via recursive DFS.
        let mut state = vec![0u8; n_total];
        let mut parent_v = vec![usize::MAX; n_total];
        let mut parent_e = vec![usize::MAX; n_total];
        let mut cycle_edges: Option<Vec<usize>> = None;

        #[allow(clippy::too_many_arguments)]
        fn dfs(
            u: usize,
            pe: usize,
            adj: &[Vec<(usize, usize)>],
            active: &[bool],
            state: &mut [u8],
            parent_v: &mut [usize],
            parent_e: &mut [usize],
            cycle: &mut Option<Vec<usize>>,
        ) {
            state[u] = 1;
            for &(v, e) in &adj[u] {
                if e == pe || !active[e] {
                    continue;
                }
                if state[v] == 1 {
                    let mut cyc = vec![e];
                    let mut cur = u;
                    while cur != v {
                        cyc.push(parent_e[cur]);
                        cur = parent_v[cur];
                    }
                    *cycle = Some(cyc);
                    return;
                } else if state[v] == 0 {
                    parent_v[v] = u;
                    parent_e[v] = e;
                    dfs(v, e, adj, active, state, parent_v, parent_e, cycle);
                    if cycle.is_some() {
                        return;
                    }
                }
            }
            state[u] = 2;
        }

        for s in 0..n_total {
            if state[s] == 0 && deg[s] > 0 {
                dfs(s, usize::MAX, &adj, &active, &mut state, &mut parent_v, &mut parent_e, &mut cycle_edges);
                if cycle_edges.is_some() {
                    break;
                }
            }
        }

        if let Some(cyc) = cycle_edges {
            let even_w: f64 = cyc.iter().step_by(2).map(|&e| edges[e].2).sum();
            let odd_w: f64 = cyc.iter().skip(1).step_by(2).map(|&e| edges[e].2).sum();
            let raise_even = even_w >= odd_w;
            let mut delta = f64::INFINITY;
            for (pos, &e) in cyc.iter().enumerate() {
                let inc = (pos % 2 == 0) == raise_even;
                delta = delta.min(if inc { 1.0 - x[e] } else { x[e] });
            }
            for (pos, &e) in cyc.iter().enumerate() {
                let inc = (pos % 2 == 0) == raise_even;
                x[e] = if inc { x[e] + delta } else { x[e] - delta };
                if x[e] <= EPS {
                    x[e] = 0.0;
                    active[e] = false;
                } else if x[e] >= 1.0 - EPS {
                    x[e] = 1.0;
                    active[e] = false;
                }
            }
            continue;
        }

        // Acyclic fractional forest: take any leaf (deg == 1) and trace maximal path to another leaf.
        let leaf = (0..n_total).find(|&v| deg[v] == 1).unwrap();
        let mut path = Vec::new();
        let mut cur = leaf;
        let mut prev_e = usize::MAX;
        loop {
            let next = adj[cur].iter().find(|&&(_, e)| e != prev_e && active[e]).copied();
            let Some((nxt_v, e)) = next else {
                break;
            };
            path.push(e);
            prev_e = e;
            cur = nxt_v;
        }

        let even_w: f64 = path.iter().step_by(2).map(|&e| edges[e].2).sum();
        let odd_w: f64 = path.iter().skip(1).step_by(2).map(|&e| edges[e].2).sum();
        let raise_even = even_w >= odd_w;
        let mut delta = f64::INFINITY;
        for (pos, &e) in path.iter().enumerate() {
            let inc = (pos % 2 == 0) == raise_even;
            delta = delta.min(if inc { 1.0 - x[e] } else { x[e] });
        }
        for (pos, &e) in path.iter().enumerate() {
            let inc = (pos % 2 == 0) == raise_even;
            x[e] = if inc { x[e] + delta } else { x[e] - delta };
            if x[e] <= EPS {
                x[e] = 0.0;
                active[e] = false;
            } else if x[e] >= 1.0 - EPS {
                x[e] = 1.0;
                active[e] = false;
            }
        }
    }

    (0..m).filter(|&e| x[e] >= 1.0 - 1e-6).collect()
}

/// Problem 25-5: Given a bipartite graph `(left, right, edges)` with non-negative weights
/// and an optimal maximum-weight matching `matched_edges` (indices into `edges`),
/// compute feasible non-negative dual vertex labels `(u_left, v_right)` in O(VE) time via Bellman-Ford
/// on the difference-constraint graph such that u[a] + v[b] >= w for all edges and sum(u) + sum(v) == w(M).
pub fn dual_labels_from_matching(
    left: usize,
    right: usize,
    edges: &[(usize, usize, i64)],
    matched_edges: &[usize],
) -> Option<(Vec<i64>, Vec<i64>)> {
    let n = left + right + 1;
    let s = left + right;
    let mut is_matched_edge = vec![false; edges.len()];
    let mut l_matched = vec![false; left];
    let mut r_matched = vec![false; right];
    for &ei in matched_edges {
        is_matched_edge[ei] = true;
        let (a, b, _) = edges[ei];
        l_matched[a] = true;
        r_matched[b] = true;
    }

    // Variables: y[a] = -u[a] for a in 0..left, v[b] for b in 0..right.
    let mut arc_from = Vec::new();
    let mut arc_to = Vec::new();
    let mut arc_w = Vec::new();
    let mut add_arc = |from: usize, to: usize, w: i64| {
        arc_from.push(from);
        arc_to.push(to);
        arc_w.push(w);
    };

    for a in 0..left {
        add_arc(s, a, 0); // y[a] <= 0 <=> u[a] >= 0
        if !l_matched[a] {
            add_arc(a, s, 0); // u[a] = 0 for unmatched left vertex
        }
    }
    for b in 0..right {
        let rb = left + b;
        add_arc(rb, s, 0); // -v[b] <= 0 <=> v[b] >= 0
        if !r_matched[b] {
            add_arc(s, rb, 0); // v[b] = 0 for unmatched right vertex
        }
    }
    for (ei, &(a, b, w)) in edges.iter().enumerate() {
        let rb = left + b;
        add_arc(rb, a, -w); // y[a] - v[b] <= -w <=> u[a] + v[b] >= w
        if is_matched_edge[ei] {
            add_arc(a, rb, w); // v[b] - y[a] <= w <=> u[a] + v[b] <= w
        }
    }

    const INF: i64 = i64::MAX / 4;
    let mut dist = vec![INF; n];
    dist[s] = 0;
    for _ in 0..n {
        let mut changed = false;
        for i in 0..arc_from.len() {
            let u = arc_from[i];
            let v = arc_to[i];
            if dist[u] < INF / 2 && dist[u] + arc_w[i] < dist[v] {
                dist[v] = dist[u] + arc_w[i];
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for i in 0..arc_from.len() {
        let u = arc_from[i];
        let v = arc_to[i];
        if dist[u] < INF / 2 && dist[u] + arc_w[i] < dist[v] {
            return None; // negative cycle => M was not optimal
        }
    }

    let u_labels: Vec<i64> = (0..left).map(|a| -dist[a]).collect();
    let v_labels: Vec<i64> = (0..right).map(|b| dist[left + b]).collect();
    Some((u_labels, v_labels))
}

pub fn demo() -> String {
    let mut o = String::new();
    // Fractional 6-cycle matching: every edge x = 0.5 with alternating weights.
    let edges = vec![(0, 0, 8.0), (1, 0, 3.0), (1, 1, 7.0), (2, 1, 2.0), (2, 2, 9.0), (0, 2, 4.0)];
    let frac = vec![0.5; 6];
    let frac_wt: f64 = edges.iter().zip(&frac).map(|(e, &x)| e.2 * x).sum();
    let rounded = round_fractional_matching(3, 3, &edges, &frac);
    let round_wt: f64 = rounded.iter().map(|&i| edges[i].2).sum();
    outln!(o, "fractional matching weight = {frac_wt:.1} -> rounded edges {:?}, integral weight = {round_wt:.1}", rounded);

    // Optimal vertex labels from Hungarian optimal matching.
    let w_mat = vec![vec![9, 2, 7], vec![6, 4, 3], vec![5, 8, 1]];
    let (opt_w, assign) = super::hungarian::max_weight_assignment(&w_mat);
    let mut g_edges = Vec::new();
    let mut matched = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            if j == assign[i] {
                matched.push(g_edges.len());
            }
            g_edges.push((i, j, w_mat[i][j]));
        }
    }
    let (ul, vr) = dual_labels_from_matching(3, 3, &g_edges, &matched).unwrap();
    outln!(o, "optimal matching weight = {opt_w}; dual labels u = {:?}, v = {:?} (sum = {})", ul, vr, ul.iter().sum::<i64>() + vr.iter().sum::<i64>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn rounds_arbitrary_fractional_matchings_without_loss() {
        let mut r = Rng::new(2504);
        for _ in 0..60 {
            let l = r.range(2, 6);
            let rr = r.range(2, 6);
            let mut edges = Vec::new();
            for a in 0..l {
                for b in 0..rr {
                    if r.f64() < 0.65 {
                        edges.push((a, b, r.range(1, 20) as f64));
                    }
                }
            }
            if edges.is_empty() {
                continue;
            }
            // Convex combination of 4 random integral matchings always produces a valid fractional matching.
            let mut x = vec![0.0; edges.len()];
            let coeffs = [0.2, 0.3, 0.25, 0.25];
            for &alpha in &coeffs {
                let mut used_l = vec![false; l];
                let mut used_r = vec![false; rr];
                let mut order: Vec<usize> = (0..edges.len()).collect();
                r.shuffle(&mut order);
                for ei in order {
                    let (a, b, _) = edges[ei];
                    if !used_l[a] && !used_r[b] {
                        used_l[a] = true;
                        used_r[b] = true;
                        x[ei] += alpha;
                    }
                }
            }
            let frac_w: f64 = edges.iter().zip(&x).map(|(e, &v)| e.2 * v).sum();
            let chosen = round_fractional_matching(l, rr, &edges, &x);
            let mut used_l = vec![false; l];
            let mut used_r = vec![false; rr];
            let mut int_w = 0.0;
            for &ei in &chosen {
                let (a, b, w) = edges[ei];
                assert!(!used_l[a] && !used_r[b]);
                used_l[a] = true;
                used_r[b] = true;
                int_w += w;
            }
            assert!(int_w + 1e-9 >= frac_w, "{int_w} < {frac_w}");
        }
    }

    #[test]
    fn recovers_feasible_optimal_dual_labels_for_partial_and_full_matchings() {
        let mut r = Rng::new(2505);
        for _ in 0..60 {
            let l = r.range(2, 5);
            let rr = r.range(2, 5);
            let sz = l.max(rr);
            let mut mat = vec![vec![0i64; sz]; sz];
            let mut edges = Vec::new();
            for a in 0..l {
                for b in 0..rr {
                    if r.f64() < 0.7 {
                        let w = r.range(1, 30) as i64;
                        mat[a][b] = w;
                        edges.push((a, b, w));
                    }
                }
            }
            let (opt_w, assign) = super::super::hungarian::max_weight_assignment(&mat);
            let mut matched = Vec::new();
            for (ei, &(a, b, w)) in edges.iter().enumerate() {
                if assign[a] == b && w > 0 {
                    matched.push(ei);
                }
            }
            let (ul, vr) = dual_labels_from_matching(l, rr, &edges, &matched).unwrap();
            assert!(ul.iter().all(|&x| x >= 0) && vr.iter().all(|&x| x >= 0));
            for &(a, b, w) in &edges {
                assert!(ul[a] + vr[b] >= w);
            }
            for &ei in &matched {
                let (a, b, w) = edges[ei];
                assert_eq!(ul[a] + vr[b], w);
            }
            let dual_sum: i64 = ul.iter().sum::<i64>() + vr.iter().sum::<i64>();
            assert_eq!(dual_sum, opt_w);
        }
    }
}
