//! @title 2-3-4 Heaps
//! @chapter Problem 19-4 (3e)
//! @editions 3,4
//! @summary Mergeable heaps built on 2-3-4 trees whose leaves hold the keys in no particular order and whose internal nodes store the smallest key below them: MINIMUM, DECREASE-KEY, INSERT, DELETE, EXTRACT-MIN and UNION all in O(lg n).
//! @order 190
use crate::outln;

const NIL: usize = usize::MAX;

#[derive(Debug, Clone)]
struct Node { small: i64, kids: Vec<usize>, p: usize, leaf: bool }

/// A 2-3-4 heap in an arena. Only leaves hold keys; an internal node has 2–4 children and caches the minimum key below it (`small`).
/// A handle is the arena index of a leaf as returned by [`insert`](Self::insert).
#[derive(Debug, Clone)]
pub struct Heap234 { n: Vec<Node>, root: usize, len: usize }

impl Default for Heap234 { fn default() -> Self { Self::new() } }

impl Heap234 {
    pub fn new() -> Self { Heap234 { n: vec![], root: NIL, len: 0 } }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn key(&self, h: usize) -> i64 { self.n[h].small }
    fn height(&self, mut x: usize) -> usize { let mut h = 0; while !self.n[x].leaf { x = self.n[x].kids[0]; h += 1; } h }
    fn new_internal(&mut self, kids: Vec<usize>) -> usize {
        let id = self.n.len();
        self.n.push(Node { small: i64::MAX, kids: kids.clone(), p: NIL, leaf: false });
        for k in kids { self.n[k].p = id; }
        self.recompute(id);
        id
    }
    fn recompute(&mut self, x: usize) { self.n[x].small = self.n[x].kids.iter().map(|&k| self.n[k].small).min().unwrap_or(i64::MAX); }
    fn recompute_up(&mut self, mut x: usize) { while x != NIL { self.recompute(x); x = self.n[x].p; } }
    /// Split a node with 4 or 5 children into two nodes of 2 and 2–3 children; returns the new right sibling (parent not yet set).
    fn split(&mut self, x: usize) -> usize {
        let half = self.n[x].kids.len() / 2;
        let right_kids = self.n[x].kids.split_off(half);
        let y = self.new_internal(right_kids);
        self.recompute(x);
        y
    }
    /// MINIMUM: walk down choosing the child with the smallest `small`. Returns (leaf handle, key). O(lg n).
    pub fn minimum(&self) -> Option<(usize, i64)> {
        if self.root == NIL { return None; }
        let mut x = self.root;
        while !self.n[x].leaf { x = *self.n[x].kids.iter().min_by_key(|&&k| self.n[k].small).unwrap(); }
        Some((x, self.n[x].small))
    }
    /// INSERT: top-down; every full (4-child) node on the way is split first, then the new leaf is attached to a node of height 1. O(lg n).
    pub fn insert(&mut self, key: i64) -> usize {
        let leaf = self.n.len();
        self.n.push(Node { small: key, kids: vec![], p: NIL, leaf: true });
        self.len += 1;
        if self.root == NIL { self.root = leaf; return leaf; }
        if self.n[self.root].leaf { let r = self.root; self.root = self.new_internal(vec![r, leaf]); return leaf; }
        if self.n[self.root].kids.len() == 4 { let r = self.root; let y = self.split(r); self.root = self.new_internal(vec![r, y]); }
        let mut x = self.root;
        let mut h = self.height(x);
        loop {
            self.n[x].small = self.n[x].small.min(key);
            if h == 1 { self.n[x].kids.push(leaf); self.n[leaf].p = x; return leaf; }
            let mut c = self.n[x].kids[0];
            if self.n[c].kids.len() == 4 {
                let y = self.split(c);
                self.n[x].kids.insert(1, y);
                self.n[y].p = x;
            }
            c = self.n[x].kids[0];
            x = c;
            h -= 1;
        }
    }
    /// DECREASE-KEY: lower the leaf's key and update `small` on the path to the root. O(lg n).
    pub fn decrease_key(&mut self, h: usize, k: i64) {
        assert!(self.n[h].leaf && k <= self.n[h].small);
        self.n[h].small = k;
        let mut x = self.n[h].p;
        while x != NIL && k < self.n[x].small { self.n[x].small = k; x = self.n[x].p; }
    }
    /// DELETE a leaf: remove it from its parent, then repair underfull nodes bottom-up by borrowing from or merging with a sibling (as in B-tree deletion).
    pub fn delete(&mut self, h: usize) {
        assert!(self.n[h].leaf);
        self.len -= 1;
        let p = self.n[h].p;
        if p == NIL { self.root = NIL; return; }
        self.n[p].kids.retain(|&k| k != h);
        self.n[h].p = NIL;
        let mut x = p;
        loop {
            if self.n[x].kids.len() >= 2 { self.recompute_up(x); return; }
            if x == self.root { // a root with a single child disappears and the tree gets shorter
                let c = self.n[x].kids[0];
                self.root = c;
                self.n[c].p = NIL;
                return;
            }
            let parent = self.n[x].p;
            let pos = self.n[parent].kids.iter().position(|&k| k == x).unwrap();
            let sib_pos = if pos + 1 < self.n[parent].kids.len() { pos + 1 } else { pos - 1 };
            let s = self.n[parent].kids[sib_pos];
            if self.n[s].kids.len() >= 3 { // borrow one child from the sibling
                let moved = if sib_pos > pos { self.n[s].kids.remove(0) } else { self.n[s].kids.pop().unwrap() };
                self.n[x].kids.push(moved);
                self.n[moved].p = x;
                self.recompute(x);
                self.recompute(s);
                self.recompute_up(parent);
                return;
            }
            // merge x's only child into the two-child sibling and discard x
            let only = self.n[x].kids[0];
            self.n[s].kids.push(only);
            self.n[only].p = s;
            self.n[x].kids.clear();
            self.n[parent].kids.retain(|&k| k != x);
            self.recompute(s);
            x = parent;
        }
    }
    /// EXTRACT-MIN = MINIMUM + DELETE. Returns the key.
    pub fn extract_min(&mut self) -> Option<i64> { let (h, k) = self.minimum()?; self.delete(h); Some(k) }
    /// UNION (Problem 18-2(b) without a separating key): hang the shorter tree under the right spine of the taller one at the matching height,
    /// splitting overfull nodes upward. O(lg n). Handles of `other` are shifted by the returned offset.
    pub fn union(&mut self, other: Heap234) -> usize {
        let off = self.n.len();
        if other.root == NIL { return off; }
        let shift = |i: usize| if i == NIL { NIL } else { i + off };
        for mut nd in other.n { nd.p = shift(nd.p); for k in nd.kids.iter_mut() { *k = shift(*k); } self.n.push(nd); }
        let r2 = shift(other.root);
        let len = self.len + other.len;
        if self.root == NIL { self.root = r2; self.len = len; return off; }
        let r1 = self.root;
        let (h1, h2) = (self.height(r1), self.height(r2));
        if h1 == h2 { self.root = self.new_internal(vec![r1, r2]); }
        else {
            let (tall, short, hs, ht) = if h1 > h2 { (r1, r2, h2, h1) } else { (r2, r1, h1, h2) };
            let mut x = tall;
            for _ in 0..ht - hs - 1 { x = *self.n[x].kids.last().unwrap(); }
            self.n[x].kids.push(short);
            self.n[short].p = x;
            let mut root = tall;
            while self.n[x].kids.len() == 5 { // overfull: split and push the new sibling into the parent
                let y = self.split(x);
                if x == root { root = self.new_internal(vec![x, y]); break; }
                let parent = self.n[x].p;
                let pos = self.n[parent].kids.iter().position(|&k| k == x).unwrap();
                self.n[parent].kids.insert(pos + 1, y);
                self.n[y].p = parent;
                x = parent;
            }
            self.recompute_up(x);
            self.root = root;
            self.n[root].p = NIL;
        }
        self.len = len;
        off
    }
    /// All 2-3-4 heap invariants: equal leaf depth, 2–4 children per internal node, correct `small`, consistent parents, leaf count = len.
    pub fn validate(&self) -> bool {
        if self.root == NIL { return self.len == 0; }
        fn go(h: &Heap234, x: usize, parent: usize) -> Option<(usize, usize)> {
            if h.n[x].p != parent { return None; }
            if h.n[x].leaf { return Some((0, 1)); }
            let k = &h.n[x].kids;
            if !(2..=4).contains(&k.len()) || h.n[x].small != k.iter().map(|&c| h.n[c].small).min().unwrap() { return None; }
            let (mut depth, mut leaves) = (None, 0);
            for &c in k { let (d, l) = go(h, c, x)?; if depth.is_some_and(|e| e != d) { return None; } depth = Some(d); leaves += l; }
            Some((depth.unwrap() + 1, leaves))
        }
        go(self, self.root, NIL).is_some_and(|(_, leaves)| leaves == self.len)
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut h = Heap234::new();
    let handles: Vec<usize> = [41, 7, 33, 19, 52, 3, 28, 65, 11, 90, 14].iter().map(|&k| h.insert(k)).collect();
    outln!(o, "minimum = {:?}, size = {}, valid = {}", h.minimum().map(|m| m.1), h.len(), h.validate());
    h.decrease_key(handles[4], 1);
    outln!(o, "decrease-key(52 → 1): minimum = {:?}", h.minimum().map(|m| m.1));
    h.delete(handles[2]);
    let mut other = Heap234::new();
    for k in [100, 2, 50] { other.insert(k); }
    h.union(other);
    let mut order = vec![];
    while let Some(k) = h.extract_min() { order.push(k); }
    outln!(o, "after deleting 33 and uniting with {{100, 2, 50}}, extract-min yields {:?}", order);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn random_operations_match_a_model_and_keep_the_invariants() {
        let mut r = Rng::new(514);
        for _ in 0..20 {
            let mut h = Heap234::new();
            let mut model: Vec<(usize, i64)> = vec![];
            for step in 0..600 {
                match r.range(0, 6) {
                    0 | 1 | 2 => { let k = r.range(0, 1000) as i64; model.push((h.insert(k), k)); }
                    3 if !model.is_empty() => { let i = r.range(0, model.len() - 1); let nk = model[i].1 - r.range(0, 300) as i64; h.decrease_key(model[i].0, nk); model[i].1 = nk; }
                    4 if !model.is_empty() => { let i = r.range(0, model.len() - 1); let (hd, _) = model.swap_remove(i); h.delete(hd); }
                    5 if !model.is_empty() => { let k = h.extract_min().unwrap(); let (idx, _) = model.iter().enumerate().min_by_key(|(_, m)| m.1).unwrap(); assert_eq!(k, model[idx].1); model.remove(idx); }
                    _ => { // union with a freshly built heap of random size
                        let mut o = Heap234::new();
                        let local: Vec<(usize, i64)> = (0..r.range(0, 40)).map(|_| { let k = r.range(0, 1000) as i64; (o.insert(k), k) }).collect();
                        let off = h.union(o);
                        model.extend(local.into_iter().map(|(hd, k)| (hd + off, k)));
                    }
                }
                assert_eq!(h.len(), model.len());
                assert!(h.validate(), "invariants violated at step {step}");
                assert_eq!(h.minimum().map(|m| m.1), model.iter().map(|m| m.1).min());
            }
            let mut drained = vec![];
            while let Some(k) = h.extract_min() { drained.push(k); }
            let mut want: Vec<i64> = model.iter().map(|m| m.1).collect();
            want.sort();
            assert_eq!(drained, want);
        }
    }
    #[test]
    fn height_is_logarithmic_and_union_of_very_different_sizes_works() {
        let mut big = Heap234::new();
        for k in 0..5000 { big.insert(k); }
        assert!(big.validate());
        let h = big.height(big.root);
        assert!(h as f64 <= (5000f64).log2() && h >= 6, "height {h}");
        let mut small = Heap234::new();
        small.insert(-1);
        big.union(small);
        let mut tiny = Heap234::new();
        tiny.insert(-5);
        tiny.union(big);
        assert!(tiny.validate());
        assert_eq!((tiny.len(), tiny.minimum().map(|m| m.1)), (5002, Some(-5)));
    }
}
