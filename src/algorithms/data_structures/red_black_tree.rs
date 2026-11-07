//! @title Red-Black Trees
//! @chapter 13 (1e–4e)
//! @editions 1,2,3,4
//! @summary Self-balancing BST: rotations, RB-INSERT and RB-DELETE with fix-up keep height ≤ 2 lg(n+1).
//! @order 120
use crate::outln;

/// Index of the shared sentinel `T.nil` (always black).
pub const NIL: usize = 0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color { Red, Black }
use Color::*;

#[derive(Debug, Clone)]
struct Node {
    key: i64,
    /// Auxiliary value (interval high endpoint for interval trees; equals `key` for plain trees).
    aux: i64,
    color: Color,
    left: usize,
    right: usize,
    p: usize,
    /// Subtree size (for order-statistic queries).
    size: usize,
    /// Maximum `aux` in the subtree (for interval queries).
    max: i64,
}

/// Red-black tree in an arena, following the book's pseudocode with a sentinel `T.nil`.
/// Each node also maintains `size` and `max(aux)` — the augmentations of Chapter 14 — through rotations,
/// insertions and deletions.
#[derive(Debug, Clone)]
pub struct RedBlackTree {
    n: Vec<Node>,
    root: usize,
    free: Vec<usize>,
    len: usize,
    /// Number of rotations performed so far (Problem 17-4).
    pub rotations: usize,
    /// Number of node recolourings (colour changes) performed so far (Problem 17-4).
    pub recolorings: usize,
}

impl Default for RedBlackTree { fn default() -> Self { Self::new() } }

impl RedBlackTree {
    pub fn new() -> Self {
        let nil = Node { key: 0, aux: i64::MIN, color: Black, left: NIL, right: NIL, p: NIL, size: 0, max: i64::MIN };
        RedBlackTree { n: vec![nil], root: NIL, free: vec![], len: 0, rotations: 0, recolorings: 0 }
    }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn root(&self) -> usize { self.root }
    pub fn key(&self, x: usize) -> i64 { self.n[x].key }
    pub fn aux(&self, x: usize) -> i64 { self.n[x].aux }
    pub fn left(&self, x: usize) -> usize { self.n[x].left }
    pub fn right(&self, x: usize) -> usize { self.n[x].right }
    pub fn parent(&self, x: usize) -> usize { self.n[x].p }
    /// Number of nodes in the subtree rooted at x (0 for NIL).
    pub fn size(&self, x: usize) -> usize { self.n[x].size }
    /// Maximum auxiliary value in the subtree rooted at x (i64::MIN for NIL).
    pub fn max_aux(&self, x: usize) -> i64 { self.n[x].max }
    pub fn color(&self, x: usize) -> Color { self.n[x].color }

    fn update(&mut self, x: usize) {
        if x == NIL { return; }
        let (l, r) = (self.n[x].left, self.n[x].right);
        self.n[x].size = self.n[l].size + self.n[r].size + 1;
        self.n[x].max = self.n[x].aux.max(self.n[l].max).max(self.n[r].max);
    }
    fn update_path(&mut self, mut x: usize) {
        while x != NIL { self.update(x); x = self.n[x].p; }
    }

    /// LEFT-ROTATE(T, x)
    fn set_color(&mut self, x: usize, c: Color) { if self.n[x].color != c { self.n[x].color = c; if x != NIL { self.recolorings += 1; } } }
    fn left_rotate(&mut self, x: usize) {
        self.rotations += 1;
        let y = self.n[x].right;
        self.n[x].right = self.n[y].left;
        if self.n[y].left != NIL { let l = self.n[y].left; self.n[l].p = x; }
        self.n[y].p = self.n[x].p;
        let xp = self.n[x].p;
        if xp == NIL { self.root = y; }
        else if x == self.n[xp].left { self.n[xp].left = y; }
        else { self.n[xp].right = y; }
        self.n[y].left = x;
        self.n[x].p = y;
        self.update(x);
        self.update(y);
    }
    /// RIGHT-ROTATE(T, x)
    fn right_rotate(&mut self, x: usize) {
        self.rotations += 1;
        let y = self.n[x].left;
        self.n[x].left = self.n[y].right;
        if self.n[y].right != NIL { let r = self.n[y].right; self.n[r].p = x; }
        self.n[y].p = self.n[x].p;
        let xp = self.n[x].p;
        if xp == NIL { self.root = y; }
        else if x == self.n[xp].right { self.n[xp].right = y; }
        else { self.n[xp].left = y; }
        self.n[y].right = x;
        self.n[x].p = y;
        self.update(x);
        self.update(y);
    }

    /// RB-INSERT(T, z) for a plain key.
    pub fn insert(&mut self, key: i64) -> usize { self.insert_with_aux(key, key) }

    /// RB-INSERT with an auxiliary value (used by interval trees: key = low endpoint, aux = high endpoint).
    pub fn insert_with_aux(&mut self, key: i64, aux: i64) -> usize {
        let node = Node { key, aux, color: Red, left: NIL, right: NIL, p: NIL, size: 1, max: aux };
        let z = if let Some(i) = self.free.pop() { self.n[i] = node; i } else { self.n.push(node); self.n.len() - 1 };
        let (mut y, mut x) = (NIL, self.root);
        while x != NIL {
            y = x;
            x = if key < self.n[x].key { self.n[x].left } else { self.n[x].right };
        }
        self.n[z].p = y;
        if y == NIL { self.root = z; }
        else if key < self.n[y].key { self.n[y].left = z; }
        else { self.n[y].right = z; }
        self.update_path(y);
        self.insert_fixup(z);
        self.len += 1;
        z
    }

    fn insert_fixup(&mut self, mut z: usize) {
        while self.n[self.n[z].p].color == Red {
            let pz = self.n[z].p;
            let gz = self.n[pz].p;
            if pz == self.n[gz].left {
                let y = self.n[gz].right; // uncle
                if self.n[y].color == Red {
                    self.set_color(pz, Black); // case 1
                    self.set_color(y, Black);
                    self.set_color(gz, Red);
                    z = gz;
                } else {
                    if z == self.n[pz].right { z = pz; self.left_rotate(z); } // case 2
                    let pz = self.n[z].p;
                    let gz = self.n[pz].p;
                    self.set_color(pz, Black); // case 3
                    self.set_color(gz, Red);
                    self.right_rotate(gz);
                }
            } else {
                let y = self.n[gz].left;
                if self.n[y].color == Red {
                    self.set_color(pz, Black);
                    self.set_color(y, Black);
                    self.set_color(gz, Red);
                    z = gz;
                } else {
                    if z == self.n[pz].left { z = pz; self.right_rotate(z); }
                    let pz = self.n[z].p;
                    let gz = self.n[pz].p;
                    self.set_color(pz, Black);
                    self.set_color(gz, Red);
                    self.left_rotate(gz);
                }
            }
        }
        let r = self.root;
        self.set_color(r, Black);
    }

    /// Handle of a node with this key, or NIL.
    pub fn find(&self, key: i64) -> usize {
        let mut x = self.root;
        while x != NIL && key != self.n[x].key {
            x = if key < self.n[x].key { self.n[x].left } else { self.n[x].right };
        }
        x
    }
    /// Handle of a node with this key that also satisfies `pred` (duplicates may sit on both sides).
    pub fn find_where(&self, key: i64, pred: &dyn Fn(&Self, usize) -> bool) -> usize {
        fn rec(t: &RedBlackTree, x: usize, key: i64, pred: &dyn Fn(&RedBlackTree, usize) -> bool) -> usize {
            if x == NIL { return NIL; }
            if key < t.n[x].key { return rec(t, t.n[x].left, key, pred); }
            if key > t.n[x].key { return rec(t, t.n[x].right, key, pred); }
            if pred(t, x) { return x; }
            let l = rec(t, t.n[x].left, key, pred);
            if l != NIL { l } else { rec(t, t.n[x].right, key, pred) }
        }
        rec(self, self.root, key, pred)
    }
    pub fn minimum(&self, mut x: usize) -> usize {
        while self.n[x].left != NIL { x = self.n[x].left; }
        x
    }
    pub fn maximum(&self, mut x: usize) -> usize {
        while self.n[x].right != NIL { x = self.n[x].right; }
        x
    }
    pub fn successor(&self, mut x: usize) -> usize {
        if self.n[x].right != NIL { return self.minimum(self.n[x].right); }
        let mut y = self.n[x].p;
        while y != NIL && x == self.n[y].right { x = y; y = self.n[y].p; }
        y
    }

    /// RB-TRANSPLANT: note that it assigns `v.p` even when `v` is the sentinel (the delete fix-up relies on that).
    fn transplant(&mut self, u: usize, v: usize) {
        let up = self.n[u].p;
        if up == NIL { self.root = v; }
        else if u == self.n[up].left { self.n[up].left = v; }
        else { self.n[up].right = v; }
        self.n[v].p = up;
    }

    /// RB-DELETE(T, z) given a node handle.
    pub fn delete(&mut self, z: usize) {
        assert_ne!(z, NIL);
        let mut y = z;
        let mut y_orig = self.n[y].color;
        let x;
        let start; // lowest node whose subtree changed (for size/max maintenance)
        if self.n[z].left == NIL {
            x = self.n[z].right;
            start = self.n[z].p;
            self.transplant(z, x);
        } else if self.n[z].right == NIL {
            x = self.n[z].left;
            start = self.n[z].p;
            self.transplant(z, x);
        } else {
            y = self.minimum(self.n[z].right);
            y_orig = self.n[y].color;
            x = self.n[y].right;
            if self.n[y].p == z {
                self.n[x].p = y;
                start = y;
            } else {
                start = self.n[y].p;
                self.transplant(y, x);
                self.n[y].right = self.n[z].right;
                let r = self.n[y].right;
                self.n[r].p = y;
            }
            self.transplant(z, y);
            self.n[y].left = self.n[z].left;
            let l = self.n[y].left;
            self.n[l].p = y;
            self.set_color(y, self.n[z].color);
        }
        self.update_path(start);
        if y_orig == Black { self.delete_fixup(x); }
        self.n[NIL].p = NIL;
        self.free.push(z);
        self.len -= 1;
    }

    fn delete_fixup(&mut self, mut x: usize) {
        while x != self.root && self.n[x].color == Black {
            let xp = self.n[x].p;
            if x == self.n[xp].left {
                let mut w = self.n[xp].right;
                if self.n[w].color == Red {
                    self.set_color(w, Black); // case 1
                    self.set_color(xp, Red);
                    self.left_rotate(xp);
                    w = self.n[self.n[x].p].right;
                }
                let (wl, wr) = (self.n[w].left, self.n[w].right);
                if self.n[wl].color == Black && self.n[wr].color == Black {
                    self.set_color(w, Red); // case 2
                    x = self.n[x].p;
                } else {
                    if self.n[wr].color == Black {
                        self.set_color(wl, Black); // case 3
                        self.set_color(w, Red);
                        self.right_rotate(w);
                        w = self.n[self.n[x].p].right;
                    }
                    let xp = self.n[x].p; // case 4
                    self.set_color(w, self.n[xp].color);
                    self.set_color(xp, Black);
                    let wr = self.n[w].right;
                    self.set_color(wr, Black);
                    self.left_rotate(xp);
                    x = self.root;
                }
            } else {
                let mut w = self.n[xp].left;
                if self.n[w].color == Red {
                    self.set_color(w, Black);
                    self.set_color(xp, Red);
                    self.right_rotate(xp);
                    w = self.n[self.n[x].p].left;
                }
                let (wl, wr) = (self.n[w].left, self.n[w].right);
                if self.n[wl].color == Black && self.n[wr].color == Black {
                    self.set_color(w, Red);
                    x = self.n[x].p;
                } else {
                    if self.n[wl].color == Black {
                        self.set_color(wr, Black);
                        self.set_color(w, Red);
                        self.left_rotate(w);
                        w = self.n[self.n[x].p].left;
                    }
                    let xp = self.n[x].p;
                    self.set_color(w, self.n[xp].color);
                    self.set_color(xp, Black);
                    let wl = self.n[w].left;
                    self.set_color(wl, Black);
                    self.right_rotate(xp);
                    x = self.root;
                }
            }
        }
        self.set_color(x, Black);
        self.set_color(NIL, Black);
    }

    /// Delete one node with this key. Returns whether a node was removed.
    pub fn delete_key(&mut self, key: i64) -> bool {
        let z = self.find(key);
        if z == NIL { return false; }
        self.delete(z);
        true
    }

    pub fn inorder(&self) -> Vec<i64> {
        let (mut out, mut st, mut x) = (vec![], vec![], self.root);
        while x != NIL || !st.is_empty() {
            while x != NIL { st.push(x); x = self.n[x].left; }
            let y = st.pop().unwrap();
            out.push(self.n[y].key);
            x = self.n[y].right;
        }
        out
    }
    pub fn height(&self) -> usize {
        fn h(t: &RedBlackTree, x: usize) -> usize { if x == NIL { 0 } else { 1 + h(t, t.n[x].left).max(h(t, t.n[x].right)) } }
        h(self, self.root)
    }

    /// Checks all five red-black properties plus parent links, ordering, size and max augmentation.
    /// Returns the black-height on success.
    pub fn validate(&self) -> Result<usize, String> {
        if self.n[NIL].color != Black { return Err("nil is not black".into()); }
        if self.root != NIL && self.n[self.root].color != Black { return Err("root is red".into()); }
        fn rec(t: &RedBlackTree, x: usize, lo: i64, hi: i64) -> Result<usize, String> {
            if x == NIL { return Ok(1); }
            let nd = &t.n[x];
            if nd.key < lo || nd.key > hi { return Err(format!("order violated at {}", nd.key)); }
            if nd.color == Red && (t.n[nd.left].color == Red || t.n[nd.right].color == Red) { return Err(format!("red-red at {}", nd.key)); }
            for c in [nd.left, nd.right] { if c != NIL && t.n[c].p != x { return Err(format!("bad parent link below {}", nd.key)); } }
            if nd.size != t.n[nd.left].size + t.n[nd.right].size + 1 { return Err(format!("bad size at {}", nd.key)); }
            if nd.max != nd.aux.max(t.n[nd.left].max).max(t.n[nd.right].max) { return Err(format!("bad max at {}", nd.key)); }
            let l = rec(t, nd.left, lo, nd.key)?;
            let r = rec(t, nd.right, nd.key, hi)?;
            if l != r { return Err(format!("black-height mismatch at {}", nd.key)); }
            Ok(l + (nd.color == Black) as usize)
        }
        rec(self, self.root, i64::MIN, i64::MAX)
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = RedBlackTree::new();
    for k in [41, 38, 31, 12, 19, 8] {
        t.insert(k);
        outln!(o, "insert {:>2} -> height {} black-height {}", k, t.height(), t.validate().unwrap());
    }
    outln!(o, "inorder = {:?}", t.inorder());
    t.delete_key(8);
    t.delete_key(12);
    t.delete_key(19);
    outln!(o, "after deleting 8, 12, 19: {:?} (valid: {})", t.inorder(), t.validate().is_ok());
    let mut cost = RedBlackTree::new();
    for k in 0..10_000 { cost.insert(k); }
    let (ri, ci) = (cost.rotations, cost.recolorings);
    for k in 0..10_000 { cost.delete_key(k); }
    outln!(o, "10000 sorted insertions: {ri} rotations, {ci} recolourings; 10000 deletions: {} rotations, {} recolourings", cost.rotations - ri, cost.recolorings - ci);
    let mut big = RedBlackTree::new();
    for k in 0..100_000 { big.insert(k); }
    outln!(o, "100000 ascending inserts: height = {} (bound 2·lg(n+1) = {:.1})", big.height(), 2.0 * (100_001f64).log2());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    /// Problem 17-4: structural changes per operation. Insertion needs at most 2 rotations, deletion at most 3, and the number of
    /// recolourings is O(1) amortised over any sequence of operations starting from an empty tree.
    #[test]
    fn restructuring_cost_matches_problem_17_4() {
        let mut r = Rng::new(490);
        for pattern in 0..3 {
            let mut t = RedBlackTree::new();
            let keys: Vec<i64> = match pattern { 0 => (0..3000).collect(), 1 => (0..3000).rev().collect(), _ => r.vec(3000, 0, 1_000_000) };
            for &k in &keys {
                let before = t.rotations;
                t.insert(k);
                assert!(t.rotations - before <= 2, "an insertion uses at most two rotations");
            }
            let total_ops = keys.len() * 2;
            for &k in &keys {
                let before = t.rotations;
                t.delete_key(k);
                assert!(t.rotations - before <= 3, "a deletion uses at most three rotations");
            }
            assert!(t.is_empty() && t.validate().is_ok());
            assert!(t.recolorings <= 6 * total_ops, "recolourings {} for {total_ops} operations", t.recolorings);
        }
    }
    #[test]
    fn invariants_hold_through_random_inserts_and_deletes() {
        let mut t = RedBlackTree::new();
        let mut model: Vec<i64> = vec![];
        let mut r = Rng::new(70);
        for step in 0..6000 {
            let k = r.range(0, 300) as i64;
            if r.range(0, 2) < 2 {
                t.insert(k);
                model.push(k);
            } else {
                let removed = t.delete_key(k);
                let pos = model.iter().position(|&x| x == k);
                assert_eq!(removed, pos.is_some());
                if let Some(p) = pos { model.remove(p); }
            }
            if step % 7 == 0 { t.validate().unwrap(); }
        }
        model.sort();
        assert_eq!(t.inorder(), model);
        assert_eq!(t.len(), model.len());
        t.validate().unwrap();
    }
    #[test]
    fn height_bound_on_sorted_input() {
        let mut t = RedBlackTree::new();
        for k in 0..5000 { t.insert(k); }
        assert!(t.height() as f64 <= 2.0 * (5001f64).log2());
        for k in 0..5000 { assert!(t.delete_key(k)); t.validate().unwrap(); if k > 50 { break; } }
    }
    #[test]
    fn delete_everything() {
        let mut t = RedBlackTree::new();
        let mut r = Rng::new(71);
        let keys = r.vec(400, 0, 1000);
        for &k in &keys { t.insert(k); }
        for &k in &keys { assert!(t.delete_key(k)); t.validate().unwrap(); }
        assert!(t.is_empty());
        assert_eq!(t.root(), NIL);
    }
    #[test]
    fn successor_and_minimum() {
        let mut t = RedBlackTree::new();
        for k in [5, 1, 9, 3, 7] { t.insert(k); }
        let m = t.minimum(t.root());
        assert_eq!(t.key(m), 1);
        assert_eq!(t.key(t.successor(m)), 3);
        assert_eq!(t.key(t.maximum(t.root())), 9);
    }
}
