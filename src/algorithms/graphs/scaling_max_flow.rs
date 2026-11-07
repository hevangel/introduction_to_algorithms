//! @title Maximum Flow by Capacity Scaling
//! @chapter Problem 26-5 (1e–4e)
//! @editions 1,2,3,4
//! @summary Augment only along paths whose residual capacities are all ≥ Δ, halving Δ each phase: O(E² lg C) regardless of the flow value.
//! @order 175
use super::ford_fulkerson::FlowNetwork;
use crate::outln;
use std::collections::VecDeque;

impl FlowNetwork {
    /// MAX-FLOW-BY-SCALING: with Δ the largest power of 2 ≤ the largest capacity, repeatedly augment along s–t paths in the residual
    /// graph restricted to edges of residual capacity ≥ Δ; when none exists, set Δ ← Δ/2. Returns (max flow, number of augmentations).
    pub fn scaling_max_flow(&mut self, s: usize, t: usize) -> (i64, usize) {
        let cmax = (0..self.cap.len()).step_by(2).map(|e| self.capacity_of(e)).max().unwrap_or(0);
        let mut delta = if cmax > 0 { 1i64 << (63 - (cmax as u64).leading_zeros()) } else { 0 };
        let (mut flow, mut augmentations) = (0, 0);
        while delta >= 1 {
            loop {
                let mut pred = vec![usize::MAX; self.n];
                let mut seen = vec![false; self.n];
                seen[s] = true;
                let mut q = VecDeque::from([s]);
                while let Some(u) = q.pop_front() {
                    for &e in &self.adj[u] { let v = self.to[e]; if !seen[v] && self.cap[e] >= delta { seen[v] = true; pred[v] = e; q.push_back(v); } }
                }
                if !seen[t] { break; }
                let (mut b, mut v) = (i64::MAX, t);
                while v != s { let e = pred[v]; b = b.min(self.cap[e]); v = self.to[e ^ 1]; }
                v = t;
                while v != s { let e = pred[v]; self.cap[e] -= b; self.cap[e ^ 1] += b; v = self.to[e ^ 1]; }
                flow += b;
                augmentations += 1;
            }
            delta /= 2;
        }
        (flow, augmentations)
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    // huge capacities: DFS-based Ford–Fulkerson could take ~10⁹ augmentations; scaling needs only a handful
    let mut g = FlowNetwork::new(4);
    let big = 1_000_000_000;
    g.add_edge(0, 1, big); g.add_edge(0, 2, big); g.add_edge(1, 2, 1); g.add_edge(1, 3, big); g.add_edge(2, 3, big);
    let (f, aug) = g.scaling_max_flow(0, 3);
    outln!(o, "classic bad-case network with capacities 10⁹: max flow {f} found with only {aug} augmentations");
    let mut h = super::ford_fulkerson::clrs_network();
    outln!(o, "CLRS Figure 26.1 network: {:?}", h.scaling_max_flow(0, 5));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn equals_edmonds_karp_on_random_networks() {
        let mut r = Rng::new(367);
        for _ in 0..100 {
            let n = r.range(2, 9);
            let mut g = FlowNetwork::new(n);
            for _ in 0..r.range(1, 25) { let (u, v) = (r.range(0, n - 1), r.range(0, n - 1)); if u != v { g.add_edge(u, v, r.range(0, 1000) as i64); } }
            let (mut a, mut b) = (g.clone(), g.clone());
            let (f, aug) = a.scaling_max_flow(0, n - 1);
            assert_eq!(f, b.edmonds_karp(0, n - 1));
            for v in 1..n - 1 { assert_eq!(a.net_flow(v), 0); }
            assert!(aug <= 40 * g.cap.len(), "augmentations {aug}");
        }
    }
    #[test]
    fn few_augmentations_for_huge_capacities() {
        let mut g = FlowNetwork::new(4);
        for (u, v, c) in [(0, 1, 1i64 << 40), (0, 2, 1 << 40), (1, 2, 1), (1, 3, 1 << 40), (2, 3, 1 << 40)] { g.add_edge(u, v, c); }
        let (f, aug) = g.scaling_max_flow(0, 3);
        assert_eq!(f, 2 << 40);
        assert!(aug < 10, "{aug}");
    }
}
