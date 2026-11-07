//! @title Binary Search Trees
//! @chapter 12.1–12.3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Inorder walk, search, min/max, successor/predecessor, insert and delete (TRANSPLANT) in O(h).
//! @order 100
use crate::outln;

pub const NIL: usize = usize::MAX;

#[derive(Debug, Clone)]
pub struct Node { pub key: i64, pub left: usize, pub right: usize, pub p: usize }

/// Binary search tree in an arena; nodes keep parent pointers as in the book.
#[derive(Debug, Clone, Default)]
pub struct Bst {
    pub nodes: Vec<Node>,
    pub root: usize,
    len: usize,
}

impl Bst {
    pub fn new() -> Self { Bst { nodes: vec![], root: NIL, len: 0 } }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }

    /// TREE-INSERT(T, z): O(h). Returns the handle of the new node.
    pub fn insert(&mut self, key: i64) -> usize {
        let z = self.nodes.len();
        self.nodes.push(Node { key, left: NIL, right: NIL, p: NIL });
        let (mut y, mut x) = (NIL, self.root);
        while x != NIL {
            y = x;
            x = if key < self.nodes[x].key { self.nodes[x].left } else { self.nodes[x].right };
        }
        self.nodes[z].p = y;
        if y == NIL { self.root = z; }
        else if key < self.nodes[y].key { self.nodes[y].left = z; }
        else { self.nodes[y].right = z; }
        self.len += 1;
        z
    }
    /// ITERATIVE-TREE-SEARCH
    pub fn search(&self, key: i64) -> usize {
        let mut x = self.root;
        while x != NIL && key != self.nodes[x].key {
            x = if key < self.nodes[x].key { self.nodes[x].left } else { self.nodes[x].right };
        }
        x
    }
    /// TREE-SEARCH (recursive)
    pub fn search_recursive(&self, x: usize, key: i64) -> usize {
        if x == NIL || key == self.nodes[x].key { return x; }
        if key < self.nodes[x].key { self.search_recursive(self.nodes[x].left, key) } else { self.search_recursive(self.nodes[x].right, key) }
    }
    pub fn minimum(&self, mut x: usize) -> usize {
        while self.nodes[x].left != NIL { x = self.nodes[x].left; }
        x
    }
    pub fn maximum(&self, mut x: usize) -> usize {
        while self.nodes[x].right != NIL { x = self.nodes[x].right; }
        x
    }
    /// TREE-SUCCESSOR: next key in sorted order (NIL if x is the maximum).
    pub fn successor(&self, mut x: usize) -> usize {
        if self.nodes[x].right != NIL { return self.minimum(self.nodes[x].right); }
        let mut y = self.nodes[x].p;
        while y != NIL && x == self.nodes[y].right { x = y; y = self.nodes[y].p; }
        y
    }
    pub fn predecessor(&self, mut x: usize) -> usize {
        if self.nodes[x].left != NIL { return self.maximum(self.nodes[x].left); }
        let mut y = self.nodes[x].p;
        while y != NIL && x == self.nodes[y].left { x = y; y = self.nodes[y].p; }
        y
    }
    /// INORDER-TREE-WALK (recursive, as in the book).
    pub fn inorder_walk(&self, x: usize, out: &mut Vec<i64>) {
        if x != NIL {
            self.inorder_walk(self.nodes[x].left, out);
            out.push(self.nodes[x].key);
            self.inorder_walk(self.nodes[x].right, out);
        }
    }
    /// Inorder keys using successor links (no recursion, safe for degenerate trees).
    pub fn keys(&self) -> Vec<i64> {
        let mut v = vec![];
        if self.root == NIL { return v; }
        let mut x = self.minimum(self.root);
        while x != NIL { v.push(self.nodes[x].key); x = self.successor(x); }
        v
    }
    fn transplant(&mut self, u: usize, v: usize) {
        let up = self.nodes[u].p;
        if up == NIL { self.root = v; }
        else if u == self.nodes[up].left { self.nodes[up].left = v; }
        else { self.nodes[up].right = v; }
        if v != NIL { self.nodes[v].p = up; }
    }
    /// TREE-DELETE(T, z): the node's storage stays in the arena but is unlinked.
    pub fn delete(&mut self, z: usize) {
        if self.nodes[z].left == NIL {
            let r = self.nodes[z].right;
            self.transplant(z, r);
        } else if self.nodes[z].right == NIL {
            let l = self.nodes[z].left;
            self.transplant(z, l);
        } else {
            let y = self.minimum(self.nodes[z].right);
            if self.nodes[y].p != z {
                let yr = self.nodes[y].right;
                self.transplant(y, yr);
                self.nodes[y].right = self.nodes[z].right;
                let r = self.nodes[y].right;
                self.nodes[r].p = y;
            }
            self.transplant(z, y);
            self.nodes[y].left = self.nodes[z].left;
            let l = self.nodes[y].left;
            self.nodes[l].p = y;
        }
        self.len -= 1;
    }
    pub fn delete_key(&mut self, key: i64) -> bool {
        let z = self.search(key);
        if z == NIL { return false; }
        self.delete(z);
        true
    }
    /// Height = number of edges on the longest root-to-leaf path (−1 for empty).
    pub fn height(&self) -> isize {
        if self.root == NIL { return -1; }
        let mut level = vec![self.root];
        let mut h = -1;
        while !level.is_empty() {
            h += 1;
            level = level.iter().flat_map(|&x| [self.nodes[x].left, self.nodes[x].right]).filter(|&c| c != NIL).collect();
        }
        h
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = Bst::new();
    for k in [15, 6, 18, 3, 7, 17, 20, 2, 4, 13, 9] { t.insert(k); }
    let mut w = vec![];
    t.inorder_walk(t.root, &mut w);
    outln!(o, "inorder walk      : {:?}", w);
    outln!(o, "height            : {}", t.height());
    let (mn, mx) = (t.minimum(t.root), t.maximum(t.root));
    outln!(o, "minimum / maximum : {} / {}", t.nodes[mn].key, t.nodes[mx].key);
    let x = t.search(13);
    outln!(o, "successor(13)     : {}, predecessor(13): {}", t.nodes[t.successor(x)].key, t.nodes[t.predecessor(x)].key);
    t.delete_key(6);
    outln!(o, "after delete(6)   : {:?}", t.keys());
    t.delete_key(15);
    outln!(o, "after delete(15)  : {:?}", t.keys());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn matches_btreeset_under_random_operations() {
        let mut t = Bst::new();
        let mut s = BTreeSet::new();
        let mut r = crate::common::Rng::new(60);
        for _ in 0..4000 {
            let k = r.range(0, 150) as i64;
            match r.range(0, 2) {
                0 => if s.insert(k) { t.insert(k); },
                1 => assert_eq!(t.delete_key(k), s.remove(&k)),
                _ => assert_eq!(t.search(k) != NIL, s.contains(&k)),
            }
            assert_eq!(t.keys(), s.iter().copied().collect::<Vec<_>>());
        }
        assert_eq!(t.len(), s.len());
    }
    #[test]
    fn successor_predecessor_min_max() {
        let mut t = Bst::new();
        for k in [8, 3, 10, 1, 6, 14] { t.insert(k); }
        let n = |k| t.search(k);
        assert_eq!(t.nodes[t.successor(n(6))].key, 8);
        assert_eq!(t.nodes[t.predecessor(n(8))].key, 6);
        assert_eq!(t.successor(n(14)), NIL);
        assert_eq!(t.predecessor(n(1)), NIL);
        assert_eq!(t.nodes[t.minimum(t.root)].key, 1);
        assert_eq!(t.nodes[t.maximum(t.root)].key, 14);
        assert_eq!(t.search_recursive(t.root, 10), n(10));
    }
    #[test]
    fn degenerate_tree_has_linear_height() {
        let mut t = Bst::new();
        for k in 0..100 { t.insert(k); }
        assert_eq!(t.height(), 99);
        assert_eq!(Bst::new().height(), -1);
    }
}
