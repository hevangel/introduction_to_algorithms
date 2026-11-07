//! @title Disjoint-Set Applications: Off-line Minimum, Depth, Tarjan's LCA
//! @chapter Problems 21-1, 21-2, 21-3 (1e–3e) / 19-1…3 (4e)
//! @editions 1,2,3,4
//! @summary Three classic union-find applications: OFF-LINE-MINIMUM, FIND-DEPTH/GRAFT with weighted union-find, and off-line least common ancestors.
//! @order 215
use crate::outln;

/// An operation of the off-line minimum problem.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op { Insert(usize), Extract }

fn find(p: &mut Vec<usize>, x: usize) -> usize { let mut r = x; while p[r] != r { r = p[r]; } let mut c = x; while p[c] != r { let n = p[c]; p[c] = r; c = n; } r }

/// Problem 21-1, OFF-LINE-MINIMUM: the sequence inserts each of 1..=n exactly once; return `extracted[j]` = the key returned by the j-th EXTRACT-MIN.
pub fn offline_minimum(n: usize, ops: &[Op]) -> Vec<usize> {
    let m = ops.iter().filter(|o| **o == Op::Extract).count();
    let (mut set_of, mut j) = (vec![0usize; n + 1], 0);
    for op in ops { match *op { Op::Insert(i) => set_of[i] = j, Op::Extract => j += 1 } } // K_j holds the keys inserted before the j-th extract
    let mut parent: Vec<usize> = (0..=m).collect();
    let mut extracted = vec![0; m];
    for i in 1..=n {
        let s = find(&mut parent, set_of[i]);
        if s < m {
            extracted[s] = i;          // i is the minimum remaining key at the s-th extraction
            parent[s] = s + 1;         // K_l ← K_l ∪ K_s for the next surviving set l > s, and delete K_s
        }
    }
    extracted
}

/// Problem 21-2: a forest supporting MAKE-TREE, FIND-DEPTH(v) and GRAFT(r, v) in O(α(n)) amortised, using disjoint sets with offsets.
#[derive(Debug, Clone, Default)]
pub struct DepthForest { p: Vec<usize>, d: Vec<i64>, rank: Vec<usize> }

impl DepthForest {
    pub fn new() -> Self { Self::default() }
    pub fn make_tree(&mut self) -> usize { self.p.push(self.p.len()); self.d.push(0); self.rank.push(0); self.p.len() - 1 }
    fn root(&mut self, x: usize) -> usize {
        if self.p[x] == x { return x; }
        let px = self.p[x];
        let r = self.root(px);
        if px != r { self.d[x] += self.d[px]; }   // path compression keeps Σ of offsets up to (excluding) the set root
        self.p[x] = r;
        r
    }
    /// FIND-DEPTH(v): depth of v in its (real) tree.
    pub fn find_depth(&mut self, v: usize) -> i64 { let r = self.root(v); if v == r { self.d[r] } else { self.d[v] + self.d[r] } }
    /// GRAFT(r, v): make the root `r` of one tree a child of node `v` of another tree.
    pub fn graft(&mut self, r: usize, v: usize) {
        let shift = self.find_depth(v) + 1; // every node of r's tree goes down by depth(v) + 1
        let (ra, rb) = (self.root(r), self.root(v));
        if self.rank[ra] <= self.rank[rb] {
            self.p[ra] = rb;
            self.d[ra] += shift - self.d[rb];
            if self.rank[ra] == self.rank[rb] { self.rank[rb] += 1; }
        } else {
            self.d[ra] += shift;
            self.p[rb] = ra;
            self.d[rb] -= self.d[ra];
        }
    }
}

/// Problem 21-3: Tarjan's off-line least-common-ancestors. `children[u]` lists the children of u; each query is a pair of nodes.
pub fn tarjan_lca(children: &[Vec<usize>], root: usize, queries: &[(usize, usize)]) -> Vec<usize> {
    let n = children.len();
    let mut at: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for (i, &(u, v)) in queries.iter().enumerate() { at[u].push((v, i)); at[v].push((u, i)); }
    let (mut parent, mut ancestor): (Vec<usize>, Vec<usize>) = ((0..n).collect(), (0..n).collect());
    let (mut black, mut ans) = (vec![false; n], vec![usize::MAX; queries.len()]);
    // iterative post-order walk: Enter(u) then Exit(child) events
    enum Ev { Enter(usize), AfterChild(usize, usize) }
    let mut stack = vec![Ev::Enter(root)];
    while let Some(ev) = stack.pop() {
        match ev {
            Ev::Enter(u) => {
                ancestor[u] = u;
                let mut evs = vec![];
                for &c in &children[u] { evs.push(Ev::Enter(c)); evs.push(Ev::AfterChild(u, c)); }
                // push in reverse so children are processed in order, each followed by its AfterChild
                for e in evs.into_iter().rev() { stack.push(e); }
                // when all children are done, u is finished: handled by a sentinel
                stack.push(Ev::AfterChild(u, usize::MAX));
                // ordering fix: the sentinel must run LAST, so move it below the children events
                let len = stack.len();
                let sentinel = stack.remove(len - 1);
                let base = len - 1 - 2 * children[u].len();
                stack.insert(base, sentinel);
            }
            Ev::AfterChild(u, c) if c != usize::MAX => {
                let (a, b) = (find(&mut parent, u), find(&mut parent, c));
                parent[b] = a;                 // UNION(u, c)
                let ra = find(&mut parent, u);
                ancestor[ra] = u;
            }
            Ev::AfterChild(u, _) => {
                black[u] = true;
                for &(v, qi) in &at[u] {
                    if black[v] { let r = find(&mut parent, v); ans[qi] = ancestor[r]; }
                }
            }
        }
    }
    ans
}

pub fn demo() -> String {
    let mut o = String::new();
    // Problem 21-1: 4,8,E,3,E,9,2,6,E,E,E,1,7,E,5
    let seq = [Op::Insert(4), Op::Insert(8), Op::Extract, Op::Insert(3), Op::Extract, Op::Insert(9), Op::Insert(2), Op::Insert(6), Op::Extract, Op::Extract, Op::Extract, Op::Insert(1), Op::Insert(7), Op::Extract, Op::Insert(5)];
    outln!(o, "OFF-LINE-MINIMUM extracted = {:?}", offline_minimum(9, &seq));
    let mut f = DepthForest::new();
    let nodes: Vec<usize> = (0..6).map(|_| f.make_tree()).collect();
    f.graft(nodes[1], nodes[0]);
    f.graft(nodes[2], nodes[1]);
    f.graft(nodes[4], nodes[3]);
    f.graft(nodes[3], nodes[2]);
    outln!(o, "depths after grafts: {:?}", nodes.iter().map(|&v| f.find_depth(v)).collect::<Vec<_>>());
    let children = vec![vec![1, 2], vec![3, 4], vec![5], vec![], vec![6], vec![], vec![]];
    outln!(o, "off-line LCA of (3,6), (5,4), (3,4), (6,6): {:?}", tarjan_lca(&children, 0, &[(3, 6), (5, 4), (3, 4), (6, 6)]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn offline_minimum_matches_simulation() {
        let mut r = Rng::new(326);
        for _ in 0..200 {
            let n = r.range(1, 25);
            let mut perm: Vec<usize> = (1..=n).collect();
            r.shuffle(&mut perm);
            let mut ops = vec![];
            for &x in &perm { ops.push(Op::Insert(x)); if r.range(0, 2) == 0 { ops.push(Op::Extract); } }
            // extractions only after at least one insert so the naive simulation never underflows
            let mut heap = std::collections::BinaryHeap::new();
            let mut want = vec![];
            let mut ok = true;
            for op in &ops { match *op { Op::Insert(i) => heap.push(std::cmp::Reverse(i)), Op::Extract => match heap.pop() { Some(std::cmp::Reverse(x)) => want.push(x), None => ok = false } } }
            if ok { assert_eq!(offline_minimum(n, &ops), want, "{ops:?}"); }
        }
    }
    #[test]
    fn depth_forest_matches_naive_parent_arrays() {
        let mut r = Rng::new(327);
        for _ in 0..100 {
            let n = r.range(2, 40);
            let mut f = DepthForest::new();
            let ids: Vec<usize> = (0..n).map(|_| f.make_tree()).collect();
            let mut parent: Vec<Option<usize>> = vec![None; n];
            let depth = |parent: &Vec<Option<usize>>, mut v: usize| { let mut d = 0; while let Some(p) = parent[v] { v = p; d += 1; } d };
            let tree_root = |parent: &Vec<Option<usize>>, mut v: usize| { while let Some(p) = parent[v] { v = p; } v };
            for _ in 0..3 * n {
                let (rt, v) = (r.range(0, n - 1), r.range(0, n - 1));
                if parent[rt].is_none() && tree_root(&parent, v) != rt { parent[rt] = Some(v); f.graft(ids[rt], ids[v]); }
                let q = r.range(0, n - 1);
                assert_eq!(f.find_depth(ids[q]), depth(&parent, q) as i64);
            }
            for q in 0..n { assert_eq!(f.find_depth(ids[q]), depth(&parent, q) as i64); }
        }
    }
    #[test]
    fn tarjan_lca_matches_naive() {
        let mut r = Rng::new(328);
        for _ in 0..100 {
            let n = r.range(1, 40);
            let mut children = vec![vec![]; n];
            let mut parent = vec![usize::MAX; n];
            for v in 1..n { let p = r.range(0, v - 1); children[p].push(v); parent[v] = p; }
            let qs: Vec<(usize, usize)> = (0..20).map(|_| (r.range(0, n - 1), r.range(0, n - 1))).collect();
            let anc = |mut v: usize| { let mut a = vec![v]; while parent[v] != usize::MAX { v = parent[v]; a.push(v); } a };
            let want: Vec<usize> = qs.iter().map(|&(u, v)| { let au = anc(u); *anc(v).iter().find(|x| au.contains(x)).unwrap() }).collect();
            assert_eq!(tarjan_lca(&children, 0, &qs), want);
        }
    }
}
