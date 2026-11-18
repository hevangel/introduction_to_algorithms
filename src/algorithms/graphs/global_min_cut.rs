//! @title Undirected Global Minimum Cut
//! @chapter Problem 24-7 (4e)
//! @editions 4
//! @summary Find a minimum-weight nonempty vertex bipartition via both the V−1 s-t max-flow reduction and Stoer–Wagner minimum-cut phase contractions.
//! @order 190
use super::ford_fulkerson::FlowNetwork;
use crate::outln;

/// CLRS Problem 24-7(a)/(b): Find the global minimum cut of an undirected weighted graph
/// by fixing vertex 0 and running |V| - 1 directed s-t max-flow queries `max_flow(0, t)` for t = 1..n-1.
pub fn global_min_cut_flows(n: usize, edges: &[(usize, usize, i64)]) -> (i64, Vec<bool>) {
    assert!(n >= 2);
    let mut best_cut = i64::MAX;
    let mut best_side = vec![false; n];
    for t in 1..n {
        let mut net = FlowNetwork::new(n);
        for &(u, v, w) in edges {
            if u != v && w > 0 {
                net.add_edge(u, v, w);
                net.add_edge(v, u, w);
            }
        }
        let flow = net.edmonds_karp(0, t);
        if flow < best_cut {
            best_cut = flow;
            best_side = net.min_cut_source_side(0);
        }
    }
    (best_cut, best_side)
}

/// Stoer–Wagner global minimum cut via V - 1 maximum-adjacency ordering contraction phases.
/// Returns `(min_cut_weight, bool_partition_mask)`.
pub fn stoer_wagner(n: usize, edges: &[(usize, usize, i64)]) -> (i64, Vec<bool>) {
    assert!(n >= 2);
    let mut adj = vec![vec![0i64; n]; n];
    for &(u, v, w) in edges {
        if u != v {
            adj[u][v] += w;
            adj[v][u] += w;
        }
    }
    let mut alive = vec![true; n];
    let mut group: Vec<Vec<usize>> = (0..n).map(|i| vec![i]).collect();
    let mut best_weight = i64::MAX;
    let mut best_side = vec![false; n];

    for phase in 0..(n - 1) {
        let mut in_a = vec![false; n];
        let mut key = vec![0i64; n];
        let mut prev = usize::MAX;
        let mut last = usize::MAX;
        let remaining = n - phase;

        for _ in 0..remaining {
            let mut best_u = usize::MAX;
            let mut best_k = i64::MIN;
            for u in 0..n {
                if alive[u] && !in_a[u] && key[u] > best_k {
                    best_k = key[u];
                    best_u = u;
                }
            }
            in_a[best_u] = true;
            prev = last;
            last = best_u;
            for v in 0..n {
                if alive[v] && !in_a[v] {
                    key[v] += adj[best_u][v];
                }
            }
        }

        let cut_of_phase = key[last];
        if cut_of_phase < best_weight {
            best_weight = cut_of_phase;
            best_side.fill(false);
            for &orig in &group[last] {
                best_side[orig] = true;
            }
        }

        // Merge `last` into `prev`.
        alive[last] = false;
        let mut merged = std::mem::take(&mut group[last]);
        group[prev].append(&mut merged);
        for v in 0..n {
            if alive[v] && v != prev {
                adj[prev][v] += adj[last][v];
                adj[v][prev] = adj[prev][v];
            }
        }
    }
    (best_weight, best_side)
}

pub fn demo() -> String {
    let mut o = String::new();
    // Two dense 4-cliques linked by two thin bridges of total weight 3.
    let mut edges = Vec::new();
    for u in 0..4 {
        for v in (u + 1)..4 {
            edges.push((u, v, 5));
            edges.push((u + 4, v + 4, 5));
        }
    }
    edges.push((1, 4, 1));
    edges.push((2, 6, 2));
    let (c_flow, side_f) = global_min_cut_flows(8, &edges);
    let (c_sw, side_sw) = stoer_wagner(8, &edges);
    outln!(o, "V-1 flows global min-cut = {c_flow}, partition = {:?}", side_f);
    outln!(o, "Stoer-Wagner min-cut     = {c_sw}, partition = {:?}", side_sw);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn both_algorithms_match_2_to_n_cut_enumeration() {
        let mut r = Rng::new(2407);
        for _ in 0..50 {
            let n = r.range(2, 10);
            let m = r.range(0, 22);
            let edges: Vec<(usize, usize, i64)> = (0..m)
                .map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(1, 20) as i64))
                .collect();

            let (c1, s1) = global_min_cut_flows(n, &edges);
            let (c2, s2) = stoer_wagner(n, &edges);

            let eval_cut = |side: &[bool]| -> i64 {
                assert!(side.iter().any(|&b| b) && side.iter().any(|&b| !b));
                edges.iter().filter(|&&(u, v, _)| side[u] != side[v]).map(|&(_, _, w)| w).sum()
            };
            assert_eq!(eval_cut(&s1), c1);
            assert_eq!(eval_cut(&s2), c2);

            let mut brute = i64::MAX;
            for mask in 1..((1usize << n) - 1) {
                let side: Vec<bool> = (0..n).map(|i| (mask >> i) & 1 == 1).collect();
                let w: i64 = edges.iter().filter(|&&(u, v, _)| side[u] != side[v]).map(|&(_, _, w)| w).sum();
                brute = brute.min(w);
            }
            assert_eq!(c1, brute);
            assert_eq!(c2, brute);
        }
    }
}
