//! @title Updating a Maximum Flow
//! @chapter Problem 26-4 (1e–4e)
//! @editions 1,2,3,4
//! @summary After one edge's capacity changes by 1, restore a maximum flow with a single augmenting-path search: O(E) instead of recomputing.
//! @order 185
use super::ford_fulkerson::FlowNetwork;
use crate::outln;
use std::collections::VecDeque;

impl FlowNetwork {
    /// One augmenting-path search in the residual graph (BFS); pushes the bottleneck. Returns the amount pushed (0 if none).
    fn augment_once(&mut self, s: usize, t: usize) -> i64 {
        let (mut pred, mut seen) = (vec![usize::MAX; self.n], vec![false; self.n]);
        seen[s] = true;
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() { for &e in &self.adj[u] { let v = self.to[e]; if !seen[v] && self.cap[e] > 0 { seen[v] = true; pred[v] = e; q.push_back(v); } } }
        if !seen[t] { return 0; }
        let (mut b, mut v) = (i64::MAX, t);
        while v != s { let e = pred[v]; b = b.min(self.cap[e]); v = self.to[e ^ 1]; }
        v = t;
        while v != s { let e = pred[v]; self.cap[e] -= b; self.cap[e ^ 1] += b; v = self.to[e ^ 1]; }
        b
    }
    /// BFS along edges that carry flow, from `from` to `goal`; returns the forward edge ids on the path.
    fn flow_path(&self, from: usize, goal: usize) -> Option<Vec<usize>> {
        let (mut pred, mut seen) = (vec![usize::MAX; self.n], vec![false; self.n]);
        seen[from] = true;
        let mut q = VecDeque::from([from]);
        while let Some(u) = q.pop_front() {
            for &e in &self.adj[u] { if e % 2 == 0 && self.cap[e ^ 1] > 0 { let v = self.to[e]; if !seen[v] { seen[v] = true; pred[v] = e; q.push_back(v); } } }
        }
        if !seen[goal] { return None; }
        let (mut path, mut v) = (vec![], goal);
        while v != from { let e = pred[v]; path.push(e); v = self.to[e ^ 1]; }
        Some(path)
    }
    /// Increase the capacity of edge `id` by one and restore maximality with one augmentation: O(E). Returns the new flow value.
    pub fn increase_capacity(&mut self, id: usize, s: usize, t: usize) -> i64 {
        self.cap[id] += 1;
        self.augment_once(s, t);
        self.net_flow(s)
    }
    /// Decrease the capacity of edge `id` by one (it must be ≥ 1). If the edge was saturated, cancel one unit of flow through it
    /// (along a flow-carrying cycle, or s ⇝ u and v ⇝ t paths), then augment once. O(E). Returns the new flow value.
    pub fn decrease_capacity(&mut self, id: usize, s: usize, t: usize) -> i64 {
        assert!(id % 2 == 0 && self.capacity_of(id) >= 1);
        if self.cap[id] == 0 { // the edge is saturated: the unit that used to cross it must be rerouted
            let (u, v) = (self.to[id ^ 1], self.to[id]);
            let mut cancel = vec![id];
            if let Some(cycle) = self.flow_path(v, u) { cancel.extend(cycle); } // the unit lies on a flow cycle
            else {
                cancel.extend(self.flow_path(s, u).expect("flow conservation guarantees a flow path into u"));
                cancel.extend(self.flow_path(v, t).expect("and one out of v"));
            }
            for e in cancel { self.cap[e] += 1; self.cap[e ^ 1] -= 1; }
        }
        self.cap[id] -= 1;
        self.augment_once(s, t);
        self.net_flow(s)
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut g = super::ford_fulkerson::clrs_network();
    let ids: Vec<usize> = (0..g.cap.len()).step_by(2).collect();
    let f0 = g.edmonds_karp(0, 5);
    outln!(o, "initial maximum flow = {f0}");
    outln!(o, "increase the capacity of v4→t (edge {}) by 1: flow = {}", ids[8], g.increase_capacity(ids[8], 0, 5));
    outln!(o, "decrease the capacity of v1→v3 (edge {}) by 1: flow = {}", ids[2], g.decrease_capacity(ids[2], 0, 5));
    let mut fresh = FlowNetwork::new(6);
    for (k, (u, v, c)) in [(0, 1, 16), (0, 2, 13), (1, 3, 11), (2, 1, 4), (2, 4, 14), (3, 2, 9), (3, 5, 20), (4, 3, 7), (4, 5, 5)].into_iter().enumerate() { let _ = k; fresh.add_edge(u, v, c); }
    outln!(o, "recomputed from scratch on the updated network: {}", fresh.edmonds_karp(0, 5));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn rebuild(n: usize, caps: &[(usize, usize, i64)]) -> FlowNetwork { let mut g = FlowNetwork::new(n); for &(u, v, c) in caps { g.add_edge(u, v, c); } g }
    #[test]
    fn single_unit_updates_agree_with_recomputation() {
        let mut r = Rng::new(371);
        for _ in 0..100 {
            let n = r.range(2, 8);
            let mut model: Vec<(usize, usize, i64)> = (0..r.range(1, 15)).filter_map(|_| { let (u, v) = (r.range(0, n - 1), r.range(0, n - 1)); (u != v).then(|| (u, v, r.range(1, 6) as i64)) }).collect();
            if model.is_empty() { continue; }
            let mut g = rebuild(n, &model);
            g.edmonds_karp(0, n - 1);
            for _ in 0..12 {
                let e = r.range(0, model.len() - 1);
                let got = if r.range(0, 1) == 0 { model[e].2 += 1; g.increase_capacity(2 * e, 0, n - 1) } else if model[e].2 >= 1 { model[e].2 -= 1; g.decrease_capacity(2 * e, 0, n - 1) } else { continue };
                let want = rebuild(n, &model).edmonds_karp(0, n - 1);
                assert_eq!(got, want, "model {model:?}");
                for v in 1..n - 1 { assert_eq!(g.net_flow(v), 0, "conservation"); }
            }
        }
    }
}
