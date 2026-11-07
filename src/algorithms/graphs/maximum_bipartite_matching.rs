//! @title Maximum Bipartite Matching via Flow
//! @chapter 26.3 (3e) / 24.3 (4e) / 27.3 (1e–2e: 26.3)
//! @editions 1,2,3,4
//! @summary Reduce matching to max flow with unit capacities; integrality gives a maximum matching in O(VE).
//! @order 190
use super::ford_fulkerson::FlowNetwork;
use crate::outln;

/// Maximum matching in a bipartite graph with `left` and `right` vertices; `edges` are (l, r) pairs.
/// Builds the flow network s → L → R → t with unit capacities and runs Edmonds-Karp.
pub fn maximum_matching(left: usize, right: usize, edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let (s, t) = (left + right, left + right + 1);
    let mut g = FlowNetwork::new(left + right + 2);
    for l in 0..left { g.add_edge(s, l, 1); }
    for r in 0..right { g.add_edge(left + r, t, 1); }
    let ids: Vec<usize> = edges.iter().map(|&(l, r)| g.add_edge(l, left + r, 1)).collect();
    g.edmonds_karp(s, t);
    let mut m: Vec<(usize, usize)> = edges.iter().zip(&ids).filter(|(_, &id)| g.flow_on(id) == 1).map(|(&e, _)| e).collect();
    m.sort();
    m
}

pub fn is_matching(edges: &[(usize, usize)], m: &[(usize, usize)], left: usize, right: usize) -> bool {
    let (mut l, mut r) = (vec![false; left], vec![false; right]);
    m.iter().all(|e| edges.contains(e) && !std::mem::replace(&mut l[e.0], true) && !std::mem::replace(&mut r[e.1], true))
}

pub fn demo() -> String {
    let mut o = String::new();
    // five workers (left) and five jobs (right)
    let edges = [(0, 0), (0, 1), (1, 0), (1, 2), (2, 1), (2, 2), (2, 3), (3, 3), (3, 4), (4, 3)];
    let m = maximum_matching(5, 5, &edges);
    outln!(o, "edges: {:?}", edges);
    outln!(o, "maximum matching of size {}: {:?}", m.len(), m);
    outln!(o, "valid matching? {}", is_matching(&edges, &m, 5, 5));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn brute(l: usize, edges: &[(usize, usize)], i: usize, used: &mut Vec<bool>) -> usize {
        if i == l { return 0; }
        let mut best = brute(l, edges, i + 1, used);
        for &(a, b) in edges {
            if a == i && !used[b] { used[b] = true; best = best.max(1 + brute(l, edges, i + 1, used)); used[b] = false; }
        }
        best
    }
    #[test]
    fn matches_brute_force() {
        let mut r = Rng::new(140);
        for _ in 0..80 {
            let (l, rr) = (r.range(1, 6), r.range(1, 6));
            let mut edges: Vec<(usize, usize)> = (0..r.range(0, 14)).map(|_| (r.range(0, l - 1), r.range(0, rr - 1))).collect();
            edges.sort();
            edges.dedup();
            let m = maximum_matching(l, rr, &edges);
            assert!(is_matching(&edges, &m, l, rr));
            assert_eq!(m.len(), brute(l, &edges, 0, &mut vec![false; rr]));
        }
    }
    #[test]
    fn hall_violation_limits_size() {
        // three left vertices all adjacent only to right vertex 0
        assert_eq!(maximum_matching(3, 2, &[(0, 0), (1, 0), (2, 0)]).len(), 1);
        assert!(maximum_matching(0, 0, &[]).is_empty());
    }
}
