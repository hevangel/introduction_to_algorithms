//! @title Pairing Heaps
//! @chapter Problem 19-2 (1e–3e)
//! @editions 1,2,3
//! @summary A self-adjusting heap: meld is O(1), delete-min pairs siblings in two passes (O(lg n) amortised), decrease-key cuts a subtree.
//! @order 195
use crate::outln;

const NIL: usize = usize::MAX;

#[derive(Debug, Clone)]
struct Node { key: i64, child: usize, sibling: usize, prev: usize } // prev: parent if first child, else previous sibling

/// Min pairing heap in an arena; handles are arena indices.
#[derive(Debug, Clone)]
pub struct PairingHeap { n: Vec<Node>, root: usize, len: usize }

impl Default for PairingHeap { fn default() -> Self { Self::new() } }

impl PairingHeap {
    pub fn new() -> Self { PairingHeap { n: vec![], root: NIL, len: 0 } }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn key(&self, h: usize) -> i64 { self.n[h].key }
    pub fn minimum(&self) -> Option<i64> { if self.root == NIL { None } else { Some(self.n[self.root].key) } }
    /// MELD two trees: the one with the larger root becomes the first child of the other. O(1).
    fn meld(&mut self, a: usize, b: usize) -> usize {
        if a == NIL { return b; }
        if b == NIL { return a; }
        let (p, c) = if self.n[a].key <= self.n[b].key { (a, b) } else { (b, a) };
        let first = self.n[p].child;
        self.n[c].sibling = first;
        if first != NIL { self.n[first].prev = c; }
        self.n[c].prev = p;
        self.n[p].child = c;
        p
    }
    pub fn insert(&mut self, key: i64) -> usize {
        self.n.push(Node { key, child: NIL, sibling: NIL, prev: NIL });
        let x = self.n.len() - 1;
        self.root = self.meld(self.root, x);
        self.len += 1;
        x
    }
    /// Meld another heap into this one (its handles are shifted by the returned offset).
    pub fn union(&mut self, other: PairingHeap) -> usize {
        let off = self.n.len();
        let sh = |i: usize| if i == NIL { NIL } else { i + off };
        for mut nd in other.n { nd.child = sh(nd.child); nd.sibling = sh(nd.sibling); nd.prev = sh(nd.prev); self.n.push(nd); }
        self.root = self.meld(self.root, sh(other.root));
        self.len += other.len;
        off
    }
    /// DELETE-MIN: detach the root and combine its children with the two-pass pairing heuristic.
    pub fn extract_min(&mut self) -> Option<(usize, i64)> {
        let r = self.root;
        if r == NIL { return None; }
        let mut kids = vec![];
        let mut c = self.n[r].child;
        while c != NIL { let next = self.n[c].sibling; self.n[c].sibling = NIL; self.n[c].prev = NIL; kids.push(c); c = next; }
        // pass 1: meld pairs left to right
        let mut paired = vec![];
        for ch in kids.chunks(2) { paired.push(if ch.len() == 2 { self.meld(ch[0], ch[1]) } else { ch[0] }); }
        // pass 2: meld right to left
        let mut acc = NIL;
        for &t in paired.iter().rev() { acc = self.meld(t, acc); }
        self.root = acc;
        if acc != NIL { self.n[acc].prev = NIL; }
        self.len -= 1;
        self.n[r].child = NIL;
        Some((r, self.n[r].key))
    }
    /// DECREASE-KEY: cut the node (with its subtree) out of its parent's child list and meld it with the root.
    pub fn decrease_key(&mut self, x: usize, k: i64) -> Result<(), &'static str> {
        if k > self.n[x].key { return Err("new key is greater than current key"); }
        self.n[x].key = k;
        if x == self.root { return Ok(()); }
        let (p, s) = (self.n[x].prev, self.n[x].sibling);
        if self.n[p].child == x { self.n[p].child = s; } else { self.n[p].sibling = s; }
        if s != NIL { self.n[s].prev = p; }
        self.n[x].sibling = NIL;
        self.n[x].prev = NIL;
        self.root = self.meld(self.root, x);
        Ok(())
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut h = PairingHeap::new();
    let hs: Vec<usize> = [8, 3, 5, 9, 1, 7, 2, 6].iter().map(|&k| h.insert(k)).collect();
    outln!(o, "minimum = {:?}, size = {}", h.minimum(), h.len());
    h.decrease_key(hs[3], 0).unwrap(); // 9 → 0
    outln!(o, "after decrease-key(9 → 0): minimum = {:?}", h.minimum());
    let mut order = vec![];
    while let Some((_, k)) = h.extract_min() { order.push(k); }
    outln!(o, "extract-min order: {:?}", order);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn heapsort_and_union() {
        let mut r = Rng::new(324);
        let (a, b) = (r.vec(200, -100, 100), r.vec(80, -100, 100));
        let (mut ha, mut hb) = (PairingHeap::new(), PairingHeap::new());
        a.iter().for_each(|&k| { ha.insert(k); });
        b.iter().for_each(|&k| { hb.insert(k); });
        ha.union(hb);
        let out: Vec<i64> = std::iter::from_fn(|| ha.extract_min().map(|x| x.1)).collect();
        let mut e: Vec<i64> = a.into_iter().chain(b).collect();
        e.sort();
        assert_eq!(out, e);
    }
    #[test]
    fn random_decrease_key_matches_model() {
        let mut r = Rng::new(325);
        let mut h = PairingHeap::new();
        let mut alive: Vec<usize> = vec![];
        for _ in 0..3000 {
            match r.range(0, 3) {
                0 | 1 => alive.push(h.insert(r.range(0, 10_000) as i64)),
                2 if !alive.is_empty() => { let i = r.range(0, alive.len() - 1); let nk = h.key(alive[i]) - r.range(0, 400) as i64; h.decrease_key(alive[i], nk).unwrap(); }
                _ if !alive.is_empty() => { let (hd, k) = h.extract_min().unwrap(); assert_eq!(Some(k), alive.iter().map(|&a| h.key(a)).min()); alive.retain(|&a| a != hd); }
                _ => {}
            }
            assert_eq!(h.len(), alive.len());
            assert_eq!(h.minimum(), alive.iter().map(|&a| h.key(a)).min());
        }
    }
    #[test]
    fn errors_and_empty() {
        let mut h = PairingHeap::new();
        assert!(h.extract_min().is_none());
        let x = h.insert(5);
        assert!(h.decrease_key(x, 6).is_err());
        assert!(h.decrease_key(x, 1).is_ok());
        assert_eq!(h.extract_min(), Some((x, 1)));
    }
}
