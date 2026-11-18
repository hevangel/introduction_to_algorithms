//! @title Weighted Set Cover, Maximal Matching & 0-1 Knapsack FPTAS
//! @chapter Problems 35-3, 35-4, 35-6 & 35-7 (2e–4e)
//! @editions 1,2,3,4
//! @summary Classic approximation algorithms from Chapter 35 problems: H(n)-approx weighted greedy set cover, 2-approx maximal matching, 2-approx max spanning tree, and (1−ε) 0-1 knapsack FPTAS.
//! @order 75
use crate::outln;

/// Problem 35-3: Weighted greedy set cover (`sets[i] = (weight_i, elements_i)` over 0..universe).
/// Picks the set minimizing `weight_i / newly_covered_elements` -> H(d)-approximation.
pub fn weighted_greedy_set_cover(
    universe: usize,
    sets: &[(f64, Vec<usize>)],
) -> Option<(f64, Vec<usize>)> {
    let mut uncovered = vec![true; universe];
    let mut left = universe;
    let mut total_w = 0.0f64;
    let mut chosen = Vec::new();
    let mut used = vec![false; sets.len()];

    while left > 0 {
        let mut best_idx = None;
        let mut best_ratio = f64::INFINITY;
        for (i, (w, elems)) in sets.iter().enumerate() {
            if used[i] {
                continue;
            }
            let gain = elems.iter().filter(|&&e| uncovered[e]).count();
            if gain > 0 {
                let ratio = *w / gain as f64;
                if ratio < best_ratio {
                    best_ratio = ratio;
                    best_idx = Some(i);
                }
            }
        }
        let best = best_idx?;
        used[best] = true;
        total_w += sets[best].0;
        for &e in &sets[best].1 {
            if uncovered[e] {
                uncovered[e] = false;
                left -= 1;
            }
        }
        chosen.push(best);
    }
    Some((total_w, chosen))
}

/// Problem 35-4: Arbitrary maximal matching (`|M| >= |M*| / 2`) in linear time.
pub fn maximal_matching(n: usize, edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut matched = vec![false; n];
    let mut m = Vec::new();
    for &(u, v) in edges {
        if u != v && !matched[u] && !matched[v] {
            matched[u] = true;
            matched[v] = true;
            m.push((u, v));
        }
    }
    m
}

/// Problem 35-6: Maximum spanning tree / forest via Kruskal's algorithm sorted by descending weight.
pub fn max_spanning_tree(n: usize, edges: &[(usize, usize, i64)]) -> (i64, Vec<(usize, usize, i64)>) {
    let mut sorted = edges.to_vec();
    sorted.sort_by_key(|e| std::cmp::Reverse(e.2));
    let mut parent: Vec<usize> = (0..n).collect();
    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    let mut total = 0i64;
    let mut tree = Vec::new();
    for (u, v, w) in sorted {
        let ru = find(&mut parent, u);
        let rv = find(&mut parent, v);
        if ru != rv {
            parent[ru] = rv;
            total += w;
            tree.push((u, v, w));
        }
    }
    (total, tree)
}

/// Problem 35-7: Fully Polynomial-Time Approximation Scheme (FPTAS) for 0-1 Knapsack.
/// Guarantees value >= (1 - eps) * OPT in O(n^3 / eps) time.
pub fn knapsack_fptas(items: &[(usize, u64)], cap: usize, eps: f64) -> (u64, Vec<usize>) {
    assert!(eps > 0.0 && eps < 1.0);
    let feasible: Vec<(usize, usize, u64)> = items
        .iter()
        .enumerate()
        .filter(|&(_, &(w, _))| w <= cap)
        .map(|(idx, &(w, v))| (idx, w, v))
        .collect();
    if feasible.is_empty() {
        return (0, vec![]);
    }
    let n = feasible.len();
    let v_max = feasible.iter().map(|&(_, _, v)| v).max().unwrap_or(0);
    if v_max == 0 {
        return (0, vec![]);
    }
    let scale = (eps * v_max as f64) / n as f64;
    let scaled: Vec<usize> = feasible
        .iter()
        .map(|&(_, _, v)| (v as f64 / scale).floor() as usize)
        .collect();
    let max_v_sum: usize = scaled.iter().sum();

    const INF: usize = usize::MAX / 4;
    let mut dp = vec![INF; max_v_sum + 1];
    let mut parent_val = vec![vec![None; max_v_sum + 1]; n];
    dp[0] = 0;

    for (i, &(_, w, _)) in feasible.iter().enumerate() {
        let sv = scaled[i];
        for v in (sv..=max_v_sum).rev() {
            if dp[v - sv] + w < dp[v] {
                dp[v] = dp[v - sv] + w;
                parent_val[i][v] = Some(i);
            }
        }
    }

    let best_sv = (0..=max_v_sum)
        .rfind(|&v| dp[v] <= cap)
        .unwrap_or(0);

    let mut chosen = Vec::new();
    let mut cur_v = best_sv;
    for i in (0..n).rev() {
        if parent_val[i][cur_v].is_some() {
            chosen.push(feasible[i].0);
            cur_v -= scaled[i];
        }
    }
    chosen.reverse();
    let orig_val: u64 = chosen.iter().map(|&idx| items[idx].1).sum();
    (orig_val, chosen)
}

pub fn demo() -> String {
    let mut o = String::new();
    let sets = vec![(1.0, vec![0, 1]), (1.5, vec![1, 2, 3]), (1.2, vec![2, 3])];
    outln!(o, "weighted greedy set cover -> {:?}", weighted_greedy_set_cover(4, &sets));
    outln!(o, "maximal matching          -> {:?}", maximal_matching(4, &[(0, 1), (1, 2), (2, 3)]));
    outln!(o, "max spanning tree         -> {:?}", max_spanning_tree(4, &[(0, 1, 3), (1, 2, 8), (2, 3, 5), (0, 3, 7)]));
    let items = [(10, 60), (20, 100), (30, 120)];
    outln!(o, "0-1 knapsack FPTAS (eps=0.1) -> {:?}", knapsack_fptas(&items, 50, 0.1));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn weighted_cover_maximal_matching_and_max_st_bounds() {
        let mut r = Rng::new(3503);
        for _ in 0..40 {
            let u = r.range(2, 10);
            let m = r.range(2, 9);
            let sets: Vec<(f64, Vec<usize>)> = (0..m)
                .map(|_| (r.range(1, 20) as f64, (0..u).filter(|_| r.f64() < 0.4).collect()))
                .collect();
            let greedy = weighted_greedy_set_cover(u, &sets);
            let mut opt = f64::INFINITY;
            for mask in 1..(1usize << m) {
                let mut cov = vec![false; u];
                let mut w = 0.0;
                for (i, (wi, s)) in sets.iter().enumerate() {
                    if (mask >> i) & 1 == 1 {
                        w += *wi;
                        for &e in s {
                            cov[e] = true;
                        }
                    }
                }
                if cov.iter().all(|&c| c) {
                    opt = opt.min(w);
                }
            }
            if opt.is_finite() {
                let (gw, _) = greedy.unwrap();
                let d = sets.iter().map(|s| s.1.len()).max().unwrap().max(1);
                let hd: f64 = (1..=d).map(|k| 1.0 / k as f64).sum();
                assert!(gw <= opt * hd + 1e-9);
            } else {
                assert!(greedy.is_none());
            }
        }

        // Maximal matching >= OPT/2 against Hopcroft-Karp
        for _ in 0..40 {
            let (l, rr) = (r.range(1, 8), r.range(1, 8));
            let mut adj = vec![Vec::new(); l];
            let mut edges = Vec::new();
            for a in 0..l {
                for b in 0..rr {
                    if r.f64() < 0.4 {
                        adj[a].push(b);
                        edges.push((a, l + b));
                    }
                }
            }
            let (opt_m, _, _) = crate::algorithms::graphs::hopcroft_karp::hopcroft_karp(l, rr, &adj);
            let mm = maximal_matching(l + rr, &edges);
            assert!(2 * mm.len() >= opt_m);
        }

        let (tw, te) = max_spanning_tree(4, &[(0, 1, 3), (1, 2, 8), (2, 3, 5), (0, 3, 7), (0, 2, 2)]);
        assert_eq!(tw, 8 + 7 + 5);
        assert_eq!(te.len(), 3);
    }

    #[test]
    fn knapsack_fptas_meets_one_minus_epsilon_guarantee() {
        let mut r = Rng::new(3507);
        for _ in 0..40 {
            let n = r.range(1, 10);
            let items: Vec<(usize, u64)> = (0..n).map(|_| (r.range(1, 15), r.range(5, 200) as u64)).collect();
            let cap = r.range(5, 45);
            let (opt, _) = crate::algorithms::advanced_design::knapsack_01::knapsack_01(&items, cap);
            for eps in [0.5, 0.25, 0.1] {
                let (approx, chosen) = knapsack_fptas(&items, cap, eps);
                let w_sum: usize = chosen.iter().map(|&i| items[i].0).sum();
                let v_sum: u64 = chosen.iter().map(|&i| items[i].1).sum();
                assert!(w_sum <= cap);
                assert_eq!(v_sum, approx);
                assert!(approx <= opt);
                assert!((approx as f64) + 1e-6 >= (1.0 - eps) * opt as f64, "{approx} < (1-{eps})*{opt}");
            }
        }
    }
}
