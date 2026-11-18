//! @title Widest Augmenting Path Max Flow
//! @chapter Problem 24-6 (4e)
//! @editions 4
//! @summary Choose the maximum-bottleneck s-t augmenting path per step via modified Dijkstra in O(E lg C) augmentations.
//! @order 185
use super::ford_fulkerson::FlowNetwork;
use crate::outln;
use std::collections::BinaryHeap;

/// Compute maximum s-t flow by repeatedly augmenting along a residual path with maximum bottleneck capacity.
/// Returns `(max_flow, augmentation_count, final_network)`.
pub fn widest_augmenting_path_flow(
    n: usize,
    edges: &[(usize, usize, i64)],
    s: usize,
    t: usize,
) -> (i64, usize, FlowNetwork) {
    let mut net = FlowNetwork::new(n);
    for &(u, v, c) in edges {
        net.add_edge(u, v, c);
    }
    let mut total_flow = 0i64;
    let mut augmentations = 0usize;

    loop {
        let mut best_bot = vec![0i64; n];
        let mut pred_edge = vec![usize::MAX; n];
        best_bot[s] = i64::MAX;
        let mut pq: BinaryHeap<(i64, usize)> = BinaryHeap::new();
        pq.push((i64::MAX, s));

        while let Some((b, u)) = pq.pop() {
            if b < best_bot[u] {
                continue;
            }
            if u == t {
                break;
            }
            for &e in &net.adj[u] {
                let cap = net.cap[e];
                if cap <= 0 {
                    continue;
                }
                let v = net.to[e];
                let cand = b.min(cap);
                if cand > best_bot[v] {
                    best_bot[v] = cand;
                    pred_edge[v] = e;
                    pq.push((cand, v));
                }
            }
        }
        let b = best_bot[t];
        if b == 0 {
            break;
        }
        let mut v = t;
        while v != s {
            let e = pred_edge[v];
            net.cap[e] -= b;
            net.cap[e ^ 1] += b;
            v = net.to[e ^ 1];
        }
        total_flow += b;
        augmentations += 1;
    }

    (total_flow, augmentations, net)
}

pub fn demo() -> String {
    let mut o = String::new();
    // Large-capacity vs small-capacity bypass to highlight O(E lg C) augmentations.
    let edges = [
        (0, 1, 1000),
        (1, 2, 1),
        (0, 2, 1000),
        (1, 3, 1000),
        (2, 3, 1000),
    ];
    let (flow, augs, _) = widest_augmenting_path_flow(4, &edges, 0, 3);
    outln!(o, "widest augmenting path: max_flow = {flow} in {augs} augmentations");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn matches_edmonds_karp_on_random_capacitated_graphs() {
        let mut r = Rng::new(2406);
        for _ in 0..80 {
            let n = r.range(2, 14);
            let m = r.range(1, 36);
            let edges: Vec<(usize, usize, i64)> = (0..m)
                .map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(1, 10_000) as i64))
                .collect();
            let (s, t) = (0, n - 1);
            let (wap_flow, augs, net_w) = widest_augmenting_path_flow(n, &edges, s, t);

            let mut ek = FlowNetwork::new(n);
            for &(u, v, c) in &edges {
                ek.add_edge(u, v, c);
            }
            let ek_flow = ek.edmonds_karp(s, t);
            assert_eq!(wap_flow, ek_flow);
            for u in 1..n - 1 {
                assert_eq!(net_w.net_flow(u), 0);
            }
            if wap_flow > 0 {
                assert!(augs >= 1);
            }
        }
    }

    #[test]
    fn avoids_bad_unit_shortcut_diamond() {
        let edges = [(0, 1, 1_000_000), (0, 2, 1_000_000), (1, 2, 1), (1, 3, 1_000_000), (2, 3, 1_000_000)];
        let (flow, augs, _) = widest_augmenting_path_flow(4, &edges, 0, 3);
        assert_eq!(flow, 2_000_000);
        assert_eq!(augs, 2);
    }
}
