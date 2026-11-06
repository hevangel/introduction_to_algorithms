//! @title B-Trees
//! @chapter 18 (3e, 4e) / 19 (1e, 2e)
//! @editions 1,2,3,4
//! @summary Disk-friendly multiway search trees: split-on-the-way-down insertion and full deletion with merge/borrow.
//! @order 170
use crate::outln;

#[derive(Debug, Clone)]
struct Node {
    keys: Vec<i64>,
    children: Vec<Node>,
    leaf: bool,
}

impl Node {
    fn leaf() -> Node { Node { keys: vec![], children: vec![], leaf: true } }
    fn min_key(&self) -> i64 { if self.leaf { self.keys[0] } else { self.children[0].min_key() } }
    fn max_key(&self) -> i64 { if self.leaf { *self.keys.last().unwrap() } else { self.children.last().unwrap().max_key() } }
}

/// B-tree of minimum degree `t`: every node but the root has between t−1 and 2t−1 keys.
#[derive(Debug, Clone)]
pub struct BTree {
    t: usize,
    root: Node,
    len: usize,
}

fn split_child(x: &mut Node, i: usize, t: usize) {
    // B-TREE-SPLIT-CHILD: child i is full (2t−1 keys); its median moves up into x
    let y = &mut x.children[i];
    let mut z = Node { keys: y.keys.split_off(t), children: vec![], leaf: y.leaf };
    let median = y.keys.pop().unwrap();
    if !y.leaf { z.children = y.children.split_off(t); }
    x.keys.insert(i, median);
    x.children.insert(i + 1, z);
}

fn insert_nonfull(x: &mut Node, k: i64, t: usize) {
    let mut i = x.keys.partition_point(|&a| a < k);
    if x.leaf {
        x.keys.insert(i, k);
    } else {
        if x.children[i].keys.len() == 2 * t - 1 {
            split_child(x, i, t);
            if k > x.keys[i] { i += 1; }
        }
        insert_nonfull(&mut x.children[i], k, t);
    }
}

fn merge(x: &mut Node, i: usize) {
    let right = x.children.remove(i + 1);
    let sep = x.keys.remove(i);
    let left = &mut x.children[i];
    left.keys.push(sep);
    left.keys.extend(right.keys);
    left.children.extend(right.children);
}
fn borrow_from_prev(x: &mut Node, i: usize) {
    let (k, c) = {
        let sib = &mut x.children[i - 1];
        (sib.keys.pop().unwrap(), if sib.leaf { None } else { sib.children.pop() })
    };
    let sep = std::mem::replace(&mut x.keys[i - 1], k);
    let child = &mut x.children[i];
    child.keys.insert(0, sep);
    if let Some(c) = c { child.children.insert(0, c); }
}
fn borrow_from_next(x: &mut Node, i: usize) {
    let (k, c) = {
        let sib = &mut x.children[i + 1];
        (sib.keys.remove(0), if sib.leaf { None } else { Some(sib.children.remove(0)) })
    };
    let sep = std::mem::replace(&mut x.keys[i], k);
    let child = &mut x.children[i];
    child.keys.push(sep);
    if let Some(c) = c { child.children.push(c); }
}

fn delete(x: &mut Node, k: i64, t: usize) -> bool {
    let i = x.keys.partition_point(|&a| a < k);
    if i < x.keys.len() && x.keys[i] == k {
        if x.leaf { x.keys.remove(i); return true; }                       // case 1
        if x.children[i].keys.len() >= t {                                  // case 2a: predecessor
            let pred = x.children[i].max_key();
            x.keys[i] = pred;
            return delete(&mut x.children[i], pred, t);
        }
        if x.children[i + 1].keys.len() >= t {                              // case 2b: successor
            let succ = x.children[i + 1].min_key();
            x.keys[i] = succ;
            return delete(&mut x.children[i + 1], succ, t);
        }
        merge(x, i);                                                        // case 2c
        return delete(&mut x.children[i], k, t);
    }
    if x.leaf { return false; }
    let mut i = i;
    if x.children[i].keys.len() < t {                                       // case 3: ensure child has ≥ t keys
        if i > 0 && x.children[i - 1].keys.len() >= t { borrow_from_prev(x, i); }
        else if i < x.keys.len() && x.children[i + 1].keys.len() >= t { borrow_from_next(x, i); }
        else if i < x.keys.len() { merge(x, i); }
        else { merge(x, i - 1); i -= 1; }
    }
    delete(&mut x.children[i], k, t)
}

impl BTree {
    pub fn new(t: usize) -> Self {
        assert!(t >= 2);
        BTree { t, root: Node::leaf(), len: 0 }
    }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    /// B-TREE-SEARCH
    pub fn contains(&self, k: i64) -> bool {
        let mut x = &self.root;
        loop {
            let i = x.keys.partition_point(|&a| a < k);
            if i < x.keys.len() && x.keys[i] == k { return true; }
            if x.leaf { return false; }
            x = &x.children[i];
        }
    }
    /// B-TREE-INSERT: split full nodes on the way down so only one pass is needed.
    pub fn insert(&mut self, k: i64) -> bool {
        if self.contains(k) { return false; }
        if self.root.keys.len() == 2 * self.t - 1 {
            let old = std::mem::replace(&mut self.root, Node { keys: vec![], children: vec![], leaf: false });
            self.root.children.push(old);
            split_child(&mut self.root, 0, self.t);
        }
        insert_nonfull(&mut self.root, k, self.t);
        self.len += 1;
        true
    }
    /// B-TREE-DELETE
    pub fn delete(&mut self, k: i64) -> bool {
        let removed = delete(&mut self.root, k, self.t);
        if self.root.keys.is_empty() && !self.root.leaf { self.root = self.root.children.remove(0); }
        if removed { self.len -= 1; }
        removed
    }
    pub fn height(&self) -> usize {
        let (mut h, mut x) = (0, &self.root);
        while !x.leaf { h += 1; x = &x.children[0]; }
        h
    }
    pub fn inorder(&self) -> Vec<i64> {
        fn rec(x: &Node, out: &mut Vec<i64>) {
            for (i, &k) in x.keys.iter().enumerate() {
                if !x.leaf { rec(&x.children[i], out); }
                out.push(k);
            }
            if !x.leaf { rec(x.children.last().unwrap(), out); }
        }
        let mut v = vec![];
        rec(&self.root, &mut v);
        v
    }
    /// Check key counts, ordering and uniform leaf depth.
    pub fn validate(&self) -> bool {
        fn rec(x: &Node, t: usize, is_root: bool, lo: Option<i64>, hi: Option<i64>) -> Option<usize> {
            if x.keys.len() > 2 * t - 1 || (!is_root && x.keys.len() < t - 1) { return None; }
            if x.keys.windows(2).any(|w| w[0] >= w[1]) { return None; }
            if x.keys.first().zip(lo).is_some_and(|(k, l)| *k <= l) || x.keys.last().zip(hi).is_some_and(|(k, h)| *k >= h) { return None; }
            if x.leaf { return if x.children.is_empty() { Some(0) } else { None }; }
            if x.children.len() != x.keys.len() + 1 { return None; }
            let mut depth = None;
            for (i, c) in x.children.iter().enumerate() {
                let clo = if i == 0 { lo } else { Some(x.keys[i - 1]) };
                let chi = if i == x.keys.len() { hi } else { Some(x.keys[i]) };
                let d = rec(c, t, false, clo, chi)?;
                if depth.is_some_and(|p| p != d) { return None; }
                depth = Some(d);
            }
            depth.map(|d| d + 1)
        }
        (self.root.keys.is_empty() && self.root.leaf) || rec(&self.root, self.t, true, None, None).is_some()
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = BTree::new(3);
    for k in 1..=20 { t.insert(k); }
    outln!(o, "B-tree (t=3) with keys 1..=20: height {}, valid {}", t.height(), t.validate());
    outln!(o, "contains 7? {}   contains 25? {}", t.contains(7), t.contains(25));
    for k in [6, 13, 7, 4, 2, 16] { t.delete(k); }
    outln!(o, "after deleting 6,13,7,4,2,16: {:?}", t.inorder());
    outln!(o, "valid {}, height {}", t.validate(), t.height());
    let mut big = BTree::new(50);
    for k in 0..1_000_000i64 { big.insert((k * 7919) % 1_000_003); }
    outln!(o, "t=50 with 1,000,000 keys: height {} → at most {} node reads per search", big.height(), big.height() + 1);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn matches_btreeset_for_several_degrees() {
        for t in [2, 3, 5, 8] {
            let mut b = BTree::new(t);
            let mut s = BTreeSet::new();
            let mut r = crate::common::Rng::new(77 + t as u64);
            for step in 0..4000 {
                let k = r.range(0, 400) as i64;
                if r.range(0, 5) < 3 { assert_eq!(b.insert(k), s.insert(k)); } else { assert_eq!(b.delete(k), s.remove(&k)); }
                assert_eq!(b.contains(k), s.contains(&k));
                if step % 13 == 0 { assert!(b.validate(), "t={t} step={step}"); }
            }
            assert_eq!(b.inorder(), s.iter().copied().collect::<Vec<_>>());
            assert_eq!(b.len(), s.len());
        }
    }
    #[test]
    fn delete_everything_collapses_height() {
        let mut b = BTree::new(2);
        for k in 0..200 { b.insert(k); }
        assert!(b.height() >= 3);
        for k in 0..200 { assert!(b.delete(k)); assert!(b.validate()); }
        assert!(b.is_empty());
        assert_eq!(b.height(), 0);
    }
    #[test]
    fn height_bound() {
        let t = 4;
        let mut b = BTree::new(t);
        let n = 5000;
        for k in 0..n { b.insert(k); }
        // h ≤ log_t((n+1)/2)
        assert!(b.height() as f64 <= (((n + 1) / 2) as f64).log(t as f64) + 1e-9);
    }
}
