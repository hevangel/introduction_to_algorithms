//! @title 2-3-4 Trees
//! @chapter Problem 18-2 (2e–4e)
//! @editions 2,3,4
//! @summary B-trees of minimum degree 2: every node has 1–3 keys; top-down insertion splits full 3-key nodes on the way down (a single pass, O(lg n)).
//! @order 55
use crate::outln;

#[derive(Debug, Clone, Default)]
struct Node { keys: Vec<i64>, kids: Vec<Node> }

impl Node {
    fn leaf(&self) -> bool { self.kids.is_empty() }
    /// Split the full (3-key) child i: its middle key moves up into `self`.
    fn split_child(&mut self, i: usize) {
        let child = &mut self.kids[i];
        let right_keys = child.keys.split_off(2);
        let right_kids = if child.leaf() { vec![] } else { child.kids.split_off(2) };
        let mid = child.keys.pop().unwrap();
        self.keys.insert(i, mid);
        self.kids.insert(i + 1, Node { keys: right_keys, kids: right_kids });
    }
}

/// A 2-3-4 tree of distinct integer keys.
#[derive(Debug, Clone, Default)]
pub struct Tree234 { root: Node, len: usize }

impl Tree234 {
    pub fn new() -> Self { Tree234::default() }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn contains(&self, k: i64) -> bool {
        let mut n = &self.root;
        loop {
            match n.keys.binary_search(&k) {
                Ok(_) => return true,
                Err(i) => { if n.leaf() { return false; } n = &n.kids[i]; }
            }
        }
    }
    /// Insert k (returns false if already present). One top-down pass: every full node met on the way is split first, so a split never propagates upward.
    pub fn insert(&mut self, k: i64) -> bool {
        if self.contains(k) { return false; }
        if self.root.keys.len() == 3 { // the root is full: the tree grows in height here (and only here)
            let old = std::mem::take(&mut self.root);
            self.root = Node { keys: vec![], kids: vec![old] };
            self.root.split_child(0);
        }
        let mut n = &mut self.root;
        loop {
            if n.leaf() { let pos = n.keys.binary_search(&k).unwrap_err(); n.keys.insert(pos, k); break; }
            let mut i = n.keys.binary_search(&k).unwrap_err();
            if n.kids[i].keys.len() == 3 {
                n.split_child(i);
                if k > n.keys[i] { i += 1; }
            }
            n = &mut n.kids[i];
        }
        self.len += 1;
        true
    }
    pub fn to_vec(&self) -> Vec<i64> {
        fn walk(n: &Node, out: &mut Vec<i64>) {
            for (i, &k) in n.keys.iter().enumerate() { if !n.leaf() { walk(&n.kids[i], out); } out.push(k); }
            if !n.leaf() { walk(n.kids.last().unwrap(), out); }
        }
        let mut v = vec![];
        walk(&self.root, &mut v);
        v
    }
    pub fn height(&self) -> usize { let (mut h, mut n) = (0, &self.root); while !n.leaf() { n = &n.kids[0]; h += 1; } h }
    /// Check the 2-3-4 invariants: 1–3 keys per node (except an empty root leaf), sorted keys, key/child counts, all leaves at equal depth.
    pub fn validate(&self) -> bool {
        fn go(n: &Node, is_root: bool, lo: Option<i64>, hi: Option<i64>) -> Option<usize> {
            if !(1..=3).contains(&n.keys.len()) && !(is_root && n.keys.is_empty() && n.leaf()) { return None; }
            if n.keys.windows(2).any(|w| w[0] >= w[1]) { return None; }
            if lo.is_some_and(|l| n.keys.first().is_some_and(|&k| k <= l)) || hi.is_some_and(|h| n.keys.last().is_some_and(|&k| k >= h)) { return None; }
            if n.leaf() { return Some(0); }
            if n.kids.len() != n.keys.len() + 1 { return None; }
            let mut depth = None;
            for (i, c) in n.kids.iter().enumerate() {
                let l = if i == 0 { lo } else { Some(n.keys[i - 1]) };
                let h = if i == n.keys.len() { hi } else { Some(n.keys[i]) };
                let d = go(c, false, l, h)?;
                if depth.is_some_and(|x| x != d) { return None; }
                depth = Some(d);
            }
            depth.map(|d| d + 1)
        }
        go(&self.root, true, None, None).is_some()
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = Tree234::new();
    for k in [10, 20, 30, 40, 50, 60, 70, 80, 90, 5, 15, 25, 35] { t.insert(k); }
    outln!(o, "inserted 13 keys: {:?}", t.to_vec());
    outln!(o, "height {}, valid 2-3-4 tree: {}, contains 35: {}, contains 36: {}", t.height(), t.validate(), t.contains(35), t.contains(36));
    let mut big = Tree234::new();
    for k in 0..100_000i64 { big.insert(k); }
    outln!(o, "100000 sequential keys: height {} (lg n = 16.6), valid: {}", big.height(), big.validate());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    use std::collections::BTreeSet;
    #[test]
    fn behaves_like_a_sorted_set() {
        let mut r = Rng::new(444);
        for _ in 0..100 {
            let (mut t, mut s) = (Tree234::new(), BTreeSet::new());
            for _ in 0..r.range(0, 300) {
                let k = r.range(0, 150) as i64;
                assert_eq!(t.insert(k), s.insert(k));
            }
            assert_eq!(t.to_vec(), s.iter().copied().collect::<Vec<_>>());
            assert_eq!(t.len(), s.len());
            assert!(t.validate());
            for k in 0..160 { assert_eq!(t.contains(k), s.contains(&k)); }
        }
    }
    #[test]
    fn height_is_logarithmic_for_sorted_input() {
        let mut t = Tree234::new();
        for k in 0..5000i64 { t.insert(k); assert!(k % 500 != 0 || t.validate()); }
        let h = t.height();
        assert!(h as f64 <= (5001f64).log2().ceil() && h >= 5, "height {h}");
    }
}
