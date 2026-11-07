//! @title 2-3-4 Trees
//! @chapter Problem 18-2 (2e–4e)
//! @editions 2,3,4
//! @summary B-trees of minimum degree 2 (1–3 keys per node): top-down insertion in one pass, plus Problem 18-2's JOIN and SPLIT in O(lg n) using stored heights.
//! @order 55
use crate::outln;

#[derive(Debug, Clone, Default)]
struct Node { keys: Vec<i64>, kids: Vec<Node> }

impl Node {
    fn height(&self) -> usize { let (mut h, mut n) = (0, self); while !n.leaf() { n = &n.kids[0]; h += 1; } h }
    /// Hang key `k` and subtree `t` (of height `ht`) onto the right (or left) boundary spine of `self` (height `h`) at the level just above `t`.
    /// Returns Some((middle key, new right sibling)) when `self` overflowed to four keys and had to be split.
    fn attach(&mut self, k: i64, t: Node, h: usize, ht: usize, left: bool) -> Option<(i64, Node)> {
        if h == ht + 1 {
            if left { self.keys.insert(0, k); self.kids.insert(0, t); } else { self.keys.push(k); self.kids.push(t); }
        } else {
            let i = if left { 0 } else { self.kids.len() - 1 };
            if let Some((m, right)) = self.kids[i].attach(k, t, h - 1, ht, left) {
                self.keys.insert(i, m);
                self.kids.insert(i + 1, right);
            }
        }
        if self.keys.len() == 4 { // overfull: split around the median (second of four keys is kept left, third goes up)
            let right_keys = self.keys.split_off(3);
            let mid = self.keys.pop().unwrap();
            let right_kids = if self.leaf() { vec![] } else { self.kids.split_off(3) };
            return Some((mid, Node { keys: right_keys, kids: right_kids }));
        }
        None
    }
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
pub struct Tree234 { root: Node }

impl Tree234 {
    pub fn new() -> Self { Tree234::default() }
    /// Number of keys (counted by a traversal).
    pub fn len(&self) -> usize { fn c(n: &Node) -> usize { n.keys.len() + n.kids.iter().map(c).sum::<usize>() } c(&self.root) }
    pub fn is_empty(&self) -> bool { self.root.keys.is_empty() }
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
        true
    }

    // ---------- Problem 18-2: JOIN and SPLIT ----------

    /// Build a tree from a list of sorted keys by repeated insertion (helper for tests and demos).
    pub fn from_keys(keys: &[i64]) -> Self { let mut t = Tree234::new(); for &k in keys { t.insert(k); } t }

    /// JOIN(T1, k, T2): every key of T1 < k < every key of T2. Walks down the taller tree's boundary spine to the node whose children have the
    /// height of the shorter tree, hangs `k` and the shorter tree there, and splits overfull nodes on the way back up: O(|h1 − h2| + 1).
    pub fn join(t1: Tree234, k: i64, t2: Tree234) -> Tree234 {
        if t1.is_empty() { let mut t = t2; t.insert(k); return t; }
        if t2.is_empty() { let mut t = t1; t.insert(k); return t; }
        let (h1, h2) = (t1.root.height(), t2.root.height());
        let root = if h1 == h2 {
            Node { keys: vec![k], kids: vec![t1.root, t2.root] }
        } else if h1 > h2 {
            let mut r = t1.root;
            match r.attach(k, t2.root, h1, h2, false) { Some((m, right)) => Node { keys: vec![m], kids: vec![r, right] }, None => r }
        } else {
            let mut r = t2.root;
            match r.attach(k, t1.root, h2, h1, true) { Some((m, right)) => Node { keys: vec![m], kids: vec![r, right] }, None => r }
        };
        Tree234 { root }
    }

    /// SPLIT(T, k): returns (keys < k, whether k was present, keys > k). Recurses down the search path and JOINs the subtrees hanging
    /// to the left and to the right of it; the joins telescope, giving O(lg n) in total.
    pub fn split(self, k: i64) -> (Tree234, bool, Tree234) { split_node(self.root, k) }
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

fn split_node(n: Node, k: i64) -> (Tree234, bool, Tree234) {
    let tree = |node: Node| Tree234 { root: node };
    let mut kids: Vec<Node> = n.kids;
    let keys = n.keys;
    if kids.is_empty() { // leaf: partition the keys
        let found = keys.contains(&k);
        let (lo, hi): (Vec<i64>, Vec<i64>) = (keys.iter().copied().filter(|&x| x < k).collect(), keys.iter().copied().filter(|&x| x > k).collect());
        return (tree(Node { keys: lo, kids: vec![] }), found, tree(Node { keys: hi, kids: vec![] }));
    }
    let (i, found, mut left, mut right) = match keys.binary_search(&k) {
        Ok(i) => (i, true, tree(std::mem::take(&mut kids[i])), tree(std::mem::take(&mut kids[i + 1]))),
        Err(i) => { let (l, f, r) = split_node(std::mem::take(&mut kids[i]), k); (i, f, l, r) }
    };
    // everything left of the search path: kids[0..i] with separators keys[0..i]; right of it: kids[i+1..] with keys[i..] (or keys[i+1..] if k was a key here)
    for j in (0..i).rev() { left = Tree234::join(tree(std::mem::take(&mut kids[j])), keys[j], left); }
    let (first_sep, first_kid) = if keys.binary_search(&k).is_ok() { (i + 1, i + 2) } else { (i, i + 1) };
    for (sep, kid) in (first_sep..keys.len()).zip(first_kid..kids.len()) { right = Tree234::join(right, keys[sep], tree(std::mem::take(&mut kids[kid]))); }
    (left, found, right)
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = Tree234::new();
    for k in [10, 20, 30, 40, 50, 60, 70, 80, 90, 5, 15, 25, 35] { t.insert(k); }
    outln!(o, "inserted 13 keys: {:?}", t.to_vec());
    outln!(o, "height {}, valid 2-3-4 tree: {}, contains 35: {}, contains 36: {}", t.height(), t.validate(), t.contains(35), t.contains(36));
    let mut big = Tree234::new();
    for k in 0..100_000i64 { big.insert(k); }
    let (lo, found, hi) = Tree234::from_keys(&(1..=40).collect::<Vec<_>>()).split(17);
    outln!(o, "split at 17: {} keys below (valid {}), found {found}, {} keys above (valid {})", lo.len(), lo.validate(), hi.len(), hi.validate());
    let joined = Tree234::join(lo, 17, hi);
    outln!(o, "joined back: {:?} … height {} valid {}", &joined.to_vec()[..5], joined.height(), joined.validate());
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
    fn all_valid_and_equal(t: &Tree234, want: &[i64]) -> bool { t.validate() && t.to_vec() == want }
    #[test]
    fn split_and_join_agree_with_sorted_vectors() {
        let mut r = Rng::new(445);
        for _ in 0..200 {
            let n = r.range(0, 120);
            let mut keys: Vec<i64> = (0..n).map(|_| r.range(0, 300) as i64).collect();
            keys.sort();
            keys.dedup();
            let k = r.range(0, 300) as i64;
            let (lo, found, hi) = Tree234::from_keys(&keys).split(k);
            let (a, b): (Vec<i64>, Vec<i64>) = (keys.iter().copied().filter(|&x| x < k).collect(), keys.iter().copied().filter(|&x| x > k).collect());
            assert_eq!(found, keys.contains(&k));
            assert!(all_valid_and_equal(&lo, &a), "low {a:?}");
            assert!(all_valid_and_equal(&hi, &b), "high {b:?}");
            // join the pieces back, also with trees of very different heights
            let back = Tree234::join(lo, k, hi);
            let mut want = a.clone();
            want.push(k);
            want.extend(&b);
            assert!(all_valid_and_equal(&back, &want));
        }
    }
    #[test]
    fn joining_trees_of_very_different_heights() {
        for (small, big) in [(0usize, 500usize), (1, 500), (3, 500), (500, 2), (500, 0), (7, 7), (40, 2000)] {
            let left: Vec<i64> = (0..small as i64).collect();
            let right: Vec<i64> = (small as i64 + 1..small as i64 + 1 + big as i64).collect();
            let t = Tree234::join(Tree234::from_keys(&left), small as i64, Tree234::from_keys(&right));
            assert_eq!(t.to_vec(), (0..(small + big + 1) as i64).collect::<Vec<_>>());
            assert!(t.validate(), "{small} {big}");
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
