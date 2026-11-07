//! @title Scapegoat Trees
//! @chapter Problem 17-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary An α-weight-balanced BST with no balance fields: rebuild the subtree of the "scapegoat" when an insertion lands too deep. O(lg n) amortised.
//! @order 230
use crate::outln;

const ALPHA: f64 = 0.7;

#[derive(Debug)]
struct Node { key: i64, size: usize, l: Link, r: Link }
type Link = Option<Box<Node>>;

fn size(t: &Link) -> usize { t.as_ref().map_or(0, |n| n.size) }
fn flatten(t: Link, out: &mut Vec<i64>) { if let Some(n) = t { let n = *n; flatten(n.l, out); out.push(n.key); flatten(n.r, out); } }
fn build(keys: &[i64]) -> Link {
    if keys.is_empty() { return None; }
    let m = keys.len() / 2;
    Some(Box::new(Node { key: keys[m], size: keys.len(), l: build(&keys[..m]), r: build(&keys[m + 1..]) }))
}
/// REBUILD(x): flatten the subtree into sorted order and rebuild it perfectly balanced in Θ(size).
fn rebuild(t: Link) -> Link { let mut v = vec![]; flatten(t, &mut v); build(&v) }
fn depth_limit(n: usize) -> usize { ((n.max(1) as f64).ln() / (1.0 / ALPHA).ln()).floor() as usize }

struct St { inserted: bool, deep: bool, rebuilds: usize }

fn ins(t: &mut Link, key: i64, depth: usize, limit: usize, st: &mut St) {
    let mut scapegoat = false;
    match t {
        None => { *t = Some(Box::new(Node { key, size: 1, l: None, r: None })); st.inserted = true; st.deep = depth > limit; }
        Some(n) => {
            if key == n.key { return; }
            let left = key < n.key;
            if left { ins(&mut n.l, key, depth + 1, limit, st); } else { ins(&mut n.r, key, depth + 1, limit, st); }
            if !st.inserted { return; }
            n.size += 1;
            if st.deep {
                let child = if left { size(&n.l) } else { size(&n.r) };
                if child as f64 > ALPHA * n.size as f64 { scapegoat = true; st.deep = false; } // first ancestor that is not α-weight-balanced
            }
        }
    }
    if scapegoat { *t = rebuild(t.take()); st.rebuilds += 1; }
}

fn take_min(t: Link) -> (Link, i64) {
    let mut n = t.unwrap();
    match n.l.take() {
        None => (n.r.take(), n.key),
        Some(l) => { let (rest, m) = take_min(Some(l)); n.l = rest; n.size = 1 + size(&n.l) + size(&n.r); (Some(n), m) }
    }
}
fn del(t: Link, key: i64) -> (Link, bool) {
    let Some(mut n) = t else { return (None, false) };
    let removed;
    if key < n.key { let (c, r) = del(n.l.take(), key); n.l = c; removed = r; }
    else if key > n.key { let (c, r) = del(n.r.take(), key); n.r = c; removed = r; }
    else {
        match (n.l.take(), n.r.take()) {
            (None, r) => return (r, true),
            (l, None) => return (l, true),
            (l, Some(r)) => { let (rest, m) = take_min(Some(r)); n.key = m; n.l = l; n.r = rest; removed = true; }
        }
    }
    n.size = 1 + size(&n.l) + size(&n.r);
    (Some(n), removed)
}

/// Scapegoat tree with α = 0.7.
#[derive(Debug, Default)]
pub struct ScapegoatTree { root: Link, n: usize, max_n: usize, pub rebuilds: usize }

impl ScapegoatTree {
    pub fn new() -> Self { Self::default() }
    pub fn len(&self) -> usize { self.n }
    pub fn is_empty(&self) -> bool { self.n == 0 }
    pub fn insert(&mut self, key: i64) -> bool {
        let mut st = St { inserted: false, deep: false, rebuilds: 0 };
        ins(&mut self.root, key, 0, depth_limit(self.n + 1), &mut st);
        if st.inserted { self.n += 1; self.max_n = self.max_n.max(self.n); self.rebuilds += st.rebuilds; }
        st.inserted
    }
    pub fn delete(&mut self, key: i64) -> bool {
        let (r, removed) = del(self.root.take(), key);
        self.root = r;
        if removed {
            self.n -= 1;
            if (self.n as f64) < ALPHA * self.max_n as f64 { self.root = rebuild(self.root.take()); self.max_n = self.n; self.rebuilds += 1; } // lazy global rebuild
        }
        removed
    }
    pub fn contains(&self, key: i64) -> bool { let mut x = &self.root; while let Some(n) = x { if key == n.key { return true; } x = if key < n.key { &n.l } else { &n.r }; } false }
    pub fn height(&self) -> usize { fn h(t: &Link) -> usize { t.as_ref().map_or(0, |n| 1 + h(&n.l).max(h(&n.r))) } h(&self.root) }
    pub fn max_depth_allowed(&self) -> usize { depth_limit(self.max_n) + 1 }
    pub fn inorder(&self) -> Vec<i64> { fn go(t: &Link, o: &mut Vec<i64>) { if let Some(n) = t { go(&n.l, o); o.push(n.key); go(&n.r, o); } } let mut v = vec![]; go(&self.root, &mut v); v }
    pub fn validate(&self) -> bool {
        fn go(t: &Link, lo: Option<i64>, hi: Option<i64>) -> Option<usize> {
            let Some(n) = t else { return Some(0) };
            if lo.is_some_and(|l| n.key <= l) || hi.is_some_and(|h| n.key >= h) { return None; }
            let s = go(&n.l, lo, Some(n.key))? + go(&n.r, Some(n.key), hi)? + 1;
            if s == n.size { Some(s) } else { None }
        }
        go(&self.root, None, None) == Some(self.n)
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = ScapegoatTree::new();
    for k in 0..10_000 { t.insert(k); }
    outln!(o, "10000 ascending insertions: height {} (≤ log_{{1/α}} n + 1 = {}), {} partial rebuilds", t.height(), t.max_depth_allowed(), t.rebuilds);
    for k in (0..10_000).filter(|k| k % 3 != 0) { t.delete(k); }
    outln!(o, "after deleting two thirds: n = {}, height {}, total rebuilds {}, valid = {}", t.len(), t.height(), t.rebuilds, t.validate());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn behaves_like_a_set_and_stays_shallow() {
        let mut t = ScapegoatTree::new();
        let mut s = std::collections::BTreeSet::new();
        let mut r = Rng::new(335);
        for step in 0..6000 {
            let k = r.range(0, 400) as i64;
            if r.range(0, 5) < 3 { assert_eq!(t.insert(k), s.insert(k)); } else { assert_eq!(t.delete(k), s.remove(&k)); }
            assert_eq!(t.contains(k), s.contains(&k));
            if step % 50 == 0 { assert!(t.validate()); }
            assert!(t.height() <= t.max_depth_allowed() + 2, "height {} limit {}", t.height(), t.max_depth_allowed());
        }
        assert_eq!(t.inorder(), s.iter().copied().collect::<Vec<_>>());
    }
    #[test]
    fn sorted_insertions_trigger_rebuilds_and_logarithmic_height() {
        let mut t = ScapegoatTree::new();
        for k in 0..20_000 { t.insert(k); }
        assert!(t.rebuilds > 0);
        assert!(t.height() <= t.max_depth_allowed() + 2);
        assert!(t.height() < 40, "{}", t.height());
        assert!(t.validate());
    }
}
