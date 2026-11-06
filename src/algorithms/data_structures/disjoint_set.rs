//! @title Disjoint-Set Union (Union-Find)
//! @chapter 21 (3e) / 19 (4e) / 22 (1e, 2e)
//! @editions 1,2,3,4
//! @summary Linked-list sets with weighted union and disjoint-set forests with union by rank + path compression.
//! @order 210
use crate::outln;

/// Disjoint-set forest: union by rank and path compression give O(m α(n)).
#[derive(Debug, Clone, Default)]
pub struct DisjointSetForest {
    p: Vec<usize>,
    rank: Vec<usize>,
    sets: usize,
}

impl DisjointSetForest {
    pub fn new(n: usize) -> Self { DisjointSetForest { p: (0..n).collect(), rank: vec![0; n], sets: n } }
    /// MAKE-SET: add a new singleton, returns its index.
    pub fn make_set(&mut self) -> usize {
        self.p.push(self.p.len());
        self.rank.push(0);
        self.sets += 1;
        self.p.len() - 1
    }
    /// FIND-SET with two-pass path compression.
    pub fn find(&mut self, x: usize) -> usize {
        let mut r = x;
        while self.p[r] != r { r = self.p[r]; }
        let mut c = x;
        while self.p[c] != r { let next = self.p[c]; self.p[c] = r; c = next; }
        r
    }
    /// UNION by rank. Returns false if already in the same set.
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (rx, ry) = (self.find(x), self.find(y));
        if rx == ry { return false; }
        // LINK
        if self.rank[rx] > self.rank[ry] { self.p[ry] = rx; }
        else {
            self.p[rx] = ry;
            if self.rank[rx] == self.rank[ry] { self.rank[ry] += 1; }
        }
        self.sets -= 1;
        true
    }
    pub fn same_set(&mut self, x: usize, y: usize) -> bool { self.find(x) == self.find(y) }
    pub fn set_count(&self) -> usize { self.sets }
}

/// Linked-list representation with the weighted-union heuristic (Section 21.2): the shorter list is appended to the longer.
#[derive(Debug, Clone)]
pub struct LinkedListSets {
    rep: Vec<usize>,
    members: Vec<Vec<usize>>,
    /// Total number of "pointer to the set representative" updates performed.
    pub pointer_updates: usize,
}

impl LinkedListSets {
    pub fn new(n: usize) -> Self { LinkedListSets { rep: (0..n).collect(), members: (0..n).map(|i| vec![i]).collect(), pointer_updates: 0 } }
    pub fn find(&self, x: usize) -> usize { self.rep[x] }
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (mut a, mut b) = (self.rep[x], self.rep[y]);
        if a == b { return false; }
        if self.members[a].len() < self.members[b].len() { std::mem::swap(&mut a, &mut b); }
        let moved = std::mem::take(&mut self.members[b]);
        for &m in &moved { self.rep[m] = a; }
        self.pointer_updates += moved.len();
        self.members[a].extend(moved);
        true
    }
}

/// Section 21.1: connected components of an undirected graph; returns a component label per vertex and the count.
pub fn connected_components(n: usize, edges: &[(usize, usize)]) -> (Vec<usize>, usize) {
    let mut d = DisjointSetForest::new(n);
    for &(u, v) in edges { d.union(u, v); }
    let labels: Vec<usize> = (0..n).map(|v| d.find(v)).collect();
    (labels, d.set_count())
}

pub fn demo() -> String {
    let mut o = String::new();
    let edges = [(0, 1), (1, 2), (3, 4), (5, 6), (6, 7), (7, 5), (8, 8)];
    let (labels, count) = connected_components(9, &edges);
    outln!(o, "edges {:?}", edges);
    outln!(o, "{} connected components; labels = {:?}", count, labels);
    let mut f = DisjointSetForest::new(8);
    outln!(o, "same_set(0,1) before union: {}", f.same_set(0, 1));
    f.union(0, 1);
    f.union(2, 3);
    f.union(1, 3);
    outln!(o, "after unions: same_set(0,2) = {}, same_set(0,4) = {}, sets = {}", f.same_set(0, 2), f.same_set(0, 4), f.set_count());
    let n = 1 << 12;
    let mut l = LinkedListSets::new(n);
    let mut step = 1;
    while step < n {
        for i in (0..n).step_by(2 * step) { l.union(i, i + step); }
        step *= 2;
    }
    outln!(o, "weighted union of {} singletons (balanced merges): {} pointer updates = n·lg n / 2 = {}", n, l.pointer_updates, n * 12 / 2);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn forest_matches_naive_labels() {
        let mut r = Rng::new(84);
        let n = 120;
        let mut f = DisjointSetForest::new(n);
        let mut l = LinkedListSets::new(n);
        let mut label: Vec<usize> = (0..n).collect();
        for _ in 0..300 {
            let (a, b) = (r.range(0, n - 1), r.range(0, n - 1));
            let (la, lb) = (label[a], label[b]);
            assert_eq!(f.union(a, b), la != lb);
            assert_eq!(l.union(a, b), la != lb);
            if la != lb { for x in label.iter_mut() { if *x == lb { *x = la; } } }
            let (c, d) = (r.range(0, n - 1), r.range(0, n - 1));
            assert_eq!(f.same_set(c, d), label[c] == label[d]);
            assert_eq!(l.find(c) == l.find(d), label[c] == label[d]);
        }
        assert_eq!(f.set_count(), label.iter().collect::<std::collections::HashSet<_>>().len());
    }
    #[test]
    fn weighted_union_bound() {
        let n = 1024;
        let mut l = LinkedListSets::new(n);
        let mut r = Rng::new(85);
        for _ in 0..5000 { l.union(r.range(0, n - 1), r.range(0, n - 1)); }
        // every element's pointer changes at most lg n times
        assert!(l.pointer_updates <= n * 10);
    }
    #[test]
    fn components_and_make_set() {
        let (labels, c) = connected_components(6, &[(0, 1), (2, 3), (3, 4)]);
        assert_eq!(c, 3);
        assert_eq!(labels[2], labels[4]);
        assert_ne!(labels[0], labels[2]);
        let mut f = DisjointSetForest::new(0);
        let a = f.make_set();
        let b = f.make_set();
        assert!(f.union(a, b));
        assert_eq!(f.set_count(), 1);
    }
}
