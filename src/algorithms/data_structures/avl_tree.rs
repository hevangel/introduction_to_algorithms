//! @title AVL Trees
//! @chapter Problem 13-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary The original height-balanced BST: |height(left) − height(right)| ≤ 1 restored by rotations.
//! @order 150
use crate::outln;

#[derive(Debug, Clone)]
struct Node {
    key: i64,
    h: i32,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}
type Link = Option<Box<Node>>;

fn h(n: &Link) -> i32 { n.as_ref().map_or(0, |n| n.h) }
fn update(n: &mut Node) { n.h = 1 + h(&n.l).max(h(&n.r)); }
fn balance(n: &Node) -> i32 { h(&n.l) - h(&n.r) }

fn rotate_right(mut n: Box<Node>) -> Box<Node> {
    let mut l = n.l.take().unwrap();
    n.l = l.r.take();
    update(&mut n);
    l.r = Some(n);
    update(&mut l);
    l
}
fn rotate_left(mut n: Box<Node>) -> Box<Node> {
    let mut r = n.r.take().unwrap();
    n.r = r.l.take();
    update(&mut n);
    r.l = Some(n);
    update(&mut r);
    r
}
/// BALANCE(x): restore the AVL property at x when its subtrees differ in height by at most 2.
fn rebalance(mut n: Box<Node>) -> Box<Node> {
    update(&mut n);
    let b = balance(&n);
    if b > 1 {
        if balance(n.l.as_ref().unwrap()) < 0 { n.l = Some(rotate_left(n.l.take().unwrap())); }
        return rotate_right(n);
    }
    if b < -1 {
        if balance(n.r.as_ref().unwrap()) > 0 { n.r = Some(rotate_right(n.r.take().unwrap())); }
        return rotate_left(n);
    }
    n
}

fn insert(n: Link, key: i64) -> (Box<Node>, bool) {
    match n {
        None => (Box::new(Node { key, h: 1, l: None, r: None }), true),
        Some(mut x) => {
            let added;
            if key < x.key { let (c, a) = insert(x.l.take(), key); x.l = Some(c); added = a; }
            else if key > x.key { let (c, a) = insert(x.r.take(), key); x.r = Some(c); added = a; }
            else { return (x, false); }
            (rebalance(x), added)
        }
    }
}
fn take_min(mut n: Box<Node>) -> (Link, i64) {
    match n.l.take() {
        None => (n.r.take(), n.key),
        Some(l) => { let (rest, m) = take_min(l); n.l = rest; (Some(rebalance(n)), m) }
    }
}
fn delete(n: Link, key: i64) -> (Link, bool) {
    match n {
        None => (None, false),
        Some(mut x) => {
            let removed;
            if key < x.key { let (c, r) = delete(x.l.take(), key); x.l = c; removed = r; }
            else if key > x.key { let (c, r) = delete(x.r.take(), key); x.r = c; removed = r; }
            else {
                match (x.l.take(), x.r.take()) {
                    (None, r) => return (r, true),
                    (l, None) => return (l, true),
                    (l, Some(r)) => { let (rest, m) = take_min(r); x.key = m; x.l = l; x.r = rest; removed = true; }
                }
            }
            (Some(rebalance(x)), removed)
        }
    }
}

/// An AVL tree storing a set of distinct i64 keys.
#[derive(Debug, Clone, Default)]
pub struct AvlTree {
    root: Link,
    len: usize,
}

impl AvlTree {
    pub fn new() -> Self { AvlTree { root: None, len: 0 } }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn height(&self) -> usize { h(&self.root) as usize }
    pub fn insert(&mut self, key: i64) -> bool {
        let (r, added) = insert(self.root.take(), key);
        self.root = Some(r);
        if added { self.len += 1; }
        added
    }
    pub fn delete(&mut self, key: i64) -> bool {
        let (r, removed) = delete(self.root.take(), key);
        self.root = r;
        if removed { self.len -= 1; }
        removed
    }
    pub fn contains(&self, key: i64) -> bool {
        let mut x = &self.root;
        while let Some(n) = x {
            if key == n.key { return true; }
            x = if key < n.key { &n.l } else { &n.r };
        }
        false
    }
    pub fn inorder(&self) -> Vec<i64> {
        fn rec(n: &Link, out: &mut Vec<i64>) { if let Some(x) = n { rec(&x.l, out); out.push(x.key); rec(&x.r, out); } }
        let mut v = vec![];
        rec(&self.root, &mut v);
        v
    }
    /// Verify ordering, stored heights and the balance condition everywhere.
    pub fn validate(&self) -> bool {
        fn rec(n: &Link, lo: Option<i64>, hi: Option<i64>) -> Option<i32> {
            match n {
                None => Some(0),
                Some(x) => {
                    if lo.is_some_and(|l| x.key <= l) || hi.is_some_and(|u| x.key >= u) { return None; }
                    let (a, b) = (rec(&x.l, lo, Some(x.key))?, rec(&x.r, Some(x.key), hi)?);
                    if (a - b).abs() > 1 || x.h != 1 + a.max(b) { return None; }
                    Some(x.h)
                }
            }
        }
        rec(&self.root, None, None).is_some()
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = AvlTree::new();
    for k in 1..=15 { t.insert(k); }
    outln!(o, "inserting 1..15 in order gives height {} (perfectly balanced would be 4)", t.height());
    let mut big = AvlTree::new();
    for k in 0..100_000 { big.insert(k); }
    outln!(o, "100000 ascending keys: height = {} (AVL bound 1.4405·lg(n+2) ≈ {:.1})", big.height(), 1.4405 * (100_002f64).log2());
    t.delete(8);
    outln!(o, "delete(8): {:?}, valid = {}", t.inorder(), t.validate());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn random_operations_preserve_balance() {
        let mut t = AvlTree::new();
        let mut s = BTreeSet::new();
        let mut r = crate::common::Rng::new(75);
        for step in 0..5000 {
            let k = r.range(0, 250) as i64;
            if r.range(0, 2) < 2 { assert_eq!(t.insert(k), s.insert(k)); } else { assert_eq!(t.delete(k), s.remove(&k)); }
            assert_eq!(t.contains(k), s.contains(&k));
            if step % 11 == 0 { assert!(t.validate()); }
        }
        assert_eq!(t.inorder(), s.iter().copied().collect::<Vec<_>>());
        assert_eq!(t.len(), s.len());
    }
    #[test]
    fn height_is_logarithmic_for_sorted_input() {
        let mut t = AvlTree::new();
        for k in 0..20_000 { t.insert(k); }
        assert!(t.height() as f64 <= 1.4405 * (20_002f64).log2());
        assert!(t.validate());
    }
}
