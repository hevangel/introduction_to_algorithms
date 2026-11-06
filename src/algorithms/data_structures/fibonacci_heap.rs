//! @title Fibonacci Heaps
//! @chapter 20 (1e, 2e) / 19 (3e)
//! @editions 1,2,3
//! @summary Mergeable heap with amortised O(1) insert, union and decrease-key and O(lg n) extract-min.
//! @order 180
use crate::outln;

const NIL: usize = usize::MAX;

#[derive(Debug, Clone)]
struct Node {
    key: i64,
    degree: usize,
    mark: bool,
    p: usize,
    child: usize,
    left: usize,
    right: usize,
}

/// Fibonacci min-heap in an arena. A *handle* is the arena index returned by [`insert`](Self::insert).
#[derive(Debug, Clone)]
pub struct FibonacciHeap {
    n: Vec<Node>,
    min: usize,
    count: usize,
}

impl Default for FibonacciHeap { fn default() -> Self { Self::new() } }

impl FibonacciHeap {
    pub fn new() -> Self { FibonacciHeap { n: vec![], min: NIL, count: 0 } }
    pub fn len(&self) -> usize { self.count }
    pub fn is_empty(&self) -> bool { self.count == 0 }
    pub fn key(&self, h: usize) -> i64 { self.n[h].key }
    /// FIB-HEAP-MINIMUM
    pub fn minimum(&self) -> Option<i64> { if self.min == NIL { None } else { Some(self.n[self.min].key) } }

    fn splice_after(&mut self, m: usize, x: usize) {
        let r = self.n[m].right;
        self.n[x].left = m;
        self.n[x].right = r;
        self.n[m].right = x;
        self.n[r].left = x;
    }
    fn add_root(&mut self, x: usize) {
        self.n[x].p = NIL;
        if self.min == NIL {
            self.n[x].left = x;
            self.n[x].right = x;
            self.min = x;
        } else {
            let m = self.min;
            self.splice_after(m, x);
            if self.n[x].key < self.n[self.min].key { self.min = x; }
        }
    }
    /// FIB-HEAP-INSERT: O(1) — add a new single-node tree to the root list (lazy: no consolidation).
    pub fn insert(&mut self, key: i64) -> usize {
        let x = self.n.len();
        self.n.push(Node { key, degree: 0, mark: false, p: NIL, child: NIL, left: x, right: x });
        self.add_root(x);
        self.count += 1;
        x
    }
    /// FIB-HEAP-UNION: O(1) concatenation of root lists. Handles of `other` are shifted by the returned offset.
    pub fn union(&mut self, other: FibonacciHeap) -> usize {
        let off = self.n.len();
        let shift = |i: usize| if i == NIL { NIL } else { i + off };
        for mut nd in other.n {
            nd.p = shift(nd.p);
            nd.child = shift(nd.child);
            nd.left = shift(nd.left);
            nd.right = shift(nd.right);
            self.n.push(nd);
        }
        let omin = shift(other.min);
        if omin != NIL {
            if self.min == NIL { self.min = omin; } else {
                let (a, b) = (self.min, omin);
                let (ar, bl) = (self.n[a].right, self.n[b].left);
                self.n[a].right = b;
                self.n[b].left = a;
                self.n[bl].right = ar;
                self.n[ar].left = bl;
                if self.n[b].key < self.n[a].key { self.min = b; }
            }
        }
        self.count += other.count;
        off
    }
    /// FIB-HEAP-EXTRACT-MIN: returns (handle, key). Amortised O(lg n).
    pub fn extract_min(&mut self) -> Option<(usize, i64)> {
        let z = self.min;
        if z == NIL { return None; }
        // promote z's children to the root list
        if self.n[z].child != NIL {
            let start = self.n[z].child;
            let mut kids = vec![start];
            let mut c = self.n[start].right;
            while c != start { kids.push(c); c = self.n[c].right; }
            for k in kids {
                self.n[k].p = NIL;
                self.splice_after(z, k);
            }
            self.n[z].child = NIL;
        }
        let (l, r) = (self.n[z].left, self.n[z].right);
        if r == z {
            self.min = NIL;
        } else {
            self.n[l].right = r;
            self.n[r].left = l;
            self.min = r;
            self.consolidate();
        }
        self.count -= 1;
        Some((z, self.n[z].key))
    }
    fn link(&mut self, y: usize, x: usize) {
        self.n[y].p = x;
        self.n[y].mark = false;
        if self.n[x].child == NIL {
            self.n[x].child = y;
            self.n[y].left = y;
            self.n[y].right = y;
        } else {
            let c = self.n[x].child;
            self.splice_after(c, y);
        }
        self.n[x].degree += 1;
    }
    /// CONSOLIDATE: link roots of equal degree until all root degrees are distinct.
    fn consolidate(&mut self) {
        let mut roots = vec![self.min];
        let mut w = self.n[self.min].right;
        while w != self.min { roots.push(w); w = self.n[w].right; }
        let mut a = vec![NIL; 64];
        for w in roots {
            let mut x = w;
            let mut d = self.n[x].degree;
            while a[d] != NIL {
                let mut y = a[d];
                if self.n[x].key > self.n[y].key { std::mem::swap(&mut x, &mut y); }
                self.link(y, x);
                a[d] = NIL;
                d += 1;
            }
            a[d] = x;
        }
        self.min = NIL;
        for x in a.into_iter().filter(|&x| x != NIL) { self.add_root(x); }
    }
    fn cut(&mut self, x: usize, y: usize) {
        if self.n[x].right == x {
            self.n[y].child = NIL;
        } else {
            let (l, r) = (self.n[x].left, self.n[x].right);
            self.n[l].right = r;
            self.n[r].left = l;
            if self.n[y].child == x { self.n[y].child = r; }
        }
        self.n[y].degree -= 1;
        self.n[x].mark = false;
        self.add_root(x);
    }
    fn cascading_cut(&mut self, mut y: usize) {
        loop {
            let z = self.n[y].p;
            if z == NIL { return; }
            if !self.n[y].mark { self.n[y].mark = true; return; }
            self.cut(y, z);
            y = z;
        }
    }
    /// FIB-HEAP-DECREASE-KEY: amortised O(1).
    pub fn decrease_key(&mut self, x: usize, k: i64) -> Result<(), &'static str> {
        if k > self.n[x].key { return Err("new key is greater than current key"); }
        self.n[x].key = k;
        let y = self.n[x].p;
        if y != NIL && self.n[x].key < self.n[y].key {
            self.cut(x, y);
            self.cascading_cut(y);
        }
        if self.n[x].key < self.n[self.min].key { self.min = x; }
        Ok(())
    }
    /// FIB-HEAP-DELETE: decrease to −∞ then extract the minimum.
    pub fn delete(&mut self, x: usize) {
        self.decrease_key(x, i64::MIN).unwrap();
        self.extract_min();
    }
    /// Largest degree among roots (bounded by D(n) = ⌊log_φ n⌋).
    pub fn max_root_degree(&self) -> usize {
        if self.min == NIL { return 0; }
        let mut m = self.n[self.min].degree;
        let mut w = self.n[self.min].right;
        while w != self.min { m = m.max(self.n[w].degree); w = self.n[w].right; }
        m
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut h = FibonacciHeap::new();
    let hs: Vec<usize> = [23, 7, 21, 3, 18, 52, 38, 39, 41, 17, 30, 24, 26, 46, 35].iter().map(|&k| h.insert(k)).collect();
    outln!(o, "minimum = {:?} (n = {})", h.minimum(), h.len());
    outln!(o, "extract-min -> {:?}", h.extract_min().map(|x| x.1));
    outln!(o, "max root degree after consolidation = {}", h.max_root_degree());
    h.decrease_key(hs[8], 1).unwrap(); // 41 -> 1
    outln!(o, "decrease-key(41 → 1): minimum = {:?}", h.minimum());
    h.delete(hs[5]); // delete 52
    let mut order = vec![];
    while let Some((_, k)) = h.extract_min() { order.push(k); }
    outln!(o, "remaining keys in order: {:?}", order);
    let (mut a, mut b) = (FibonacciHeap::new(), FibonacciHeap::new());
    for k in [5, 9, 2] { a.insert(k); }
    for k in [4, 1, 8] { b.insert(k); }
    a.union(b);
    outln!(o, "union of two heaps: minimum = {:?}, size = {}", a.minimum(), a.len());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn heapsort_via_extract_min() {
        let mut r = Rng::new(80);
        let v = r.vec(500, -300, 300);
        let mut h = FibonacciHeap::new();
        for &k in &v { h.insert(k); }
        let mut out = vec![];
        while let Some((_, k)) = h.extract_min() { out.push(k); }
        let mut e = v;
        e.sort();
        assert_eq!(out, e);
    }
    #[test]
    fn random_decrease_key_and_delete_match_model() {
        let mut r = Rng::new(81);
        let mut h = FibonacciHeap::new();
        let mut alive: Vec<usize> = vec![];
        for round in 0..3000 {
            match r.range(0, 4) {
                0 | 1 => alive.push(h.insert(r.range(0, 10_000) as i64)),
                2 if !alive.is_empty() => {
                    let i = r.range(0, alive.len() - 1);
                    let nk = h.key(alive[i]) - r.range(0, 500) as i64;
                    h.decrease_key(alive[i], nk).unwrap();
                }
                3 if !alive.is_empty() => {
                    let (hd, k) = h.extract_min().unwrap();
                    let model_min = alive.iter().map(|&a| h.key(a)).min().unwrap();
                    assert_eq!(k, model_min.min(k)); // k was the minimum before removal
                    alive.retain(|&a| a != hd);
                }
                _ if !alive.is_empty() => {
                    let i = r.range(0, alive.len() - 1);
                    let hd = alive.swap_remove(i);
                    h.delete(hd);
                }
                _ => {}
            }
            assert_eq!(h.len(), alive.len(), "round {round}");
            assert_eq!(h.minimum(), alive.iter().map(|&a| h.key(a)).min());
        }
    }
    #[test]
    fn degree_bound_and_union() {
        let mut h = FibonacciHeap::new();
        for k in 0..4096 { h.insert(k); }
        h.extract_min();
        assert!(h.max_root_degree() as f64 <= (4096f64).ln() / ((1.0 + 5f64.sqrt()) / 2.0).ln());
        let mut a = FibonacciHeap::new();
        let mut b = FibonacciHeap::new();
        for k in [10, 20, 30] { a.insert(k); }
        for k in [5, 25] { b.insert(k); }
        a.union(b);
        let out: Vec<i64> = std::iter::from_fn(|| a.extract_min().map(|x| x.1)).collect();
        assert_eq!(out, vec![5, 10, 20, 25, 30]);
    }
    #[test]
    fn decrease_key_rejects_larger_key() {
        let mut h = FibonacciHeap::new();
        let x = h.insert(5);
        assert!(h.decrease_key(x, 9).is_err());
    }
}
