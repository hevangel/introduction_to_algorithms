//! @title Treaps
//! @chapter Problem 13-4 (1e–4e)
//! @editions 1,2,3,4
//! @summary BST on keys, heap on random priorities: a randomly built tree regardless of insertion order.
//! @order 160
use crate::common::Rng;
use crate::outln;

#[derive(Debug, Clone)]
struct Node {
    key: i64,
    pri: u64,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}
type Link = Option<Box<Node>>;

fn rotate_right(mut n: Box<Node>) -> Box<Node> {
    let mut l = n.l.take().unwrap();
    n.l = l.r.take();
    l.r = Some(n);
    l
}
fn rotate_left(mut n: Box<Node>) -> Box<Node> {
    let mut r = n.r.take().unwrap();
    n.r = r.l.take();
    r.l = Some(n);
    r
}
fn insert(n: Link, key: i64, pri: u64) -> Box<Node> {
    match n {
        None => Box::new(Node { key, pri, l: None, r: None }),
        Some(mut x) => {
            if key < x.key {
                x.l = Some(insert(x.l.take(), key, pri));
                if x.l.as_ref().unwrap().pri > x.pri { return rotate_right(x); }
            } else if key > x.key {
                x.r = Some(insert(x.r.take(), key, pri));
                if x.r.as_ref().unwrap().pri > x.pri { return rotate_left(x); }
            }
            x
        }
    }
}
/// Rotate the root down until it becomes a leaf, then cut it off.
fn remove_root(mut n: Box<Node>) -> Link {
    match (n.l.is_some(), n.r.is_some()) {
        (false, false) => None,
        (true, false) => n.l.take(),
        (false, true) => n.r.take(),
        (true, true) => {
            if n.l.as_ref().unwrap().pri > n.r.as_ref().unwrap().pri {
                let mut root = rotate_right(n);
                root.r = remove_root(root.r.take().unwrap());
                Some(root)
            } else {
                let mut root = rotate_left(n);
                root.l = remove_root(root.l.take().unwrap());
                Some(root)
            }
        }
    }
}
fn delete(n: Link, key: i64) -> (Link, bool) {
    match n {
        None => (None, false),
        Some(mut x) => {
            if key < x.key { let (c, d) = delete(x.l.take(), key); x.l = c; (Some(x), d) }
            else if key > x.key { let (c, d) = delete(x.r.take(), key); x.r = c; (Some(x), d) }
            else { (remove_root(x), true) }
        }
    }
}

/// A treap: a binary search tree on `key` that is simultaneously a max-heap on random `priority`.
#[derive(Debug, Clone)]
pub struct Treap {
    root: Link,
    len: usize,
    rng: Rng,
}

impl Treap {
    pub fn new(seed: u64) -> Self { Treap { root: None, len: 0, rng: Rng::new(seed) } }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn contains(&self, key: i64) -> bool {
        let mut x = &self.root;
        while let Some(n) = x {
            if key == n.key { return true; }
            x = if key < n.key { &n.l } else { &n.r };
        }
        false
    }
    pub fn insert(&mut self, key: i64) -> bool {
        if self.contains(key) { return false; }
        let pri = self.rng.next_u64();
        self.root = Some(insert(self.root.take(), key, pri));
        self.len += 1;
        true
    }
    pub fn delete(&mut self, key: i64) -> bool {
        let (r, d) = delete(self.root.take(), key);
        self.root = r;
        if d { self.len -= 1; }
        d
    }
    pub fn height(&self) -> usize {
        fn h(n: &Link) -> usize { n.as_ref().map_or(0, |x| 1 + h(&x.l).max(h(&x.r))) }
        h(&self.root)
    }
    pub fn inorder(&self) -> Vec<i64> {
        fn rec(n: &Link, o: &mut Vec<i64>) { if let Some(x) = n { rec(&x.l, o); o.push(x.key); rec(&x.r, o); } }
        let mut v = vec![];
        rec(&self.root, &mut v);
        v
    }
    /// Check BST order on keys and heap order on priorities.
    pub fn validate(&self) -> bool {
        fn rec(n: &Link, lo: Option<i64>, hi: Option<i64>, parent_pri: u64) -> bool {
            match n {
                None => true,
                Some(x) => !(lo.is_some_and(|l| x.key <= l) || hi.is_some_and(|u| x.key >= u) || x.pri > parent_pri)
                    && rec(&x.l, lo, Some(x.key), x.pri) && rec(&x.r, Some(x.key), hi, x.pri),
            }
        }
        rec(&self.root, None, None, u64::MAX)
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = Treap::new(1);
    for k in 0..10_000 { t.insert(k); }
    outln!(o, "10000 keys inserted in ascending order: height = {} (a plain BST would have height 9999; 2·lg n ≈ {:.0})", t.height(), 2.0 * (10_000f64).log2());
    outln!(o, "valid treap: {}", t.validate());
    for k in (0..10_000).step_by(2) { t.delete(k); }
    outln!(o, "after deleting even keys: size = {}, height = {}, valid = {}", t.len(), t.height(), t.validate());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn behaves_like_a_set() {
        let mut t = Treap::new(5);
        let mut s = BTreeSet::new();
        let mut r = Rng::new(76);
        for _ in 0..4000 {
            let k = r.range(0, 300) as i64;
            if r.range(0, 2) < 2 { assert_eq!(t.insert(k), s.insert(k)); } else { assert_eq!(t.delete(k), s.remove(&k)); }
            assert_eq!(t.contains(k), s.contains(&k));
        }
        assert!(t.validate());
        assert_eq!(t.inorder(), s.iter().copied().collect::<Vec<_>>());
    }
    #[test]
    fn sorted_input_still_gives_logarithmic_height() {
        let mut t = Treap::new(6);
        for k in 0..5000 { t.insert(k); }
        assert!(t.height() < 60, "{}", t.height());
    }
}
