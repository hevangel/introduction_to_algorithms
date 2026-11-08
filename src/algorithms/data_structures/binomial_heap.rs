//! @title Binomial Heaps
//! @chapter 19 (2e) / 20 (1e); Problem 19-2 (3e)
//! @editions 1,2,3
//! @summary Mergeable heap built from binomial trees: UNION in O(lg n) by "binary addition" of root lists.
//! @order 200
use crate::outln;

const NIL: usize = usize::MAX;

#[derive(Debug, Clone)]
struct Node {
    key: i64,
    degree: usize,
    p: usize,
    child: usize,
    sibling: usize,
}

/// Binomial min-heap in an arena; roots form a singly linked list sorted by increasing degree.
#[derive(Debug, Clone)]
pub struct BinomialHeap {
    n: Vec<Node>,
    head: usize,
    count: usize,
}

impl Default for BinomialHeap { fn default() -> Self { Self::new() } }

impl BinomialHeap {
    pub fn new() -> Self { BinomialHeap { n: vec![], head: NIL, count: 0 } }
    pub fn len(&self) -> usize { self.count }
    pub fn is_empty(&self) -> bool { self.count == 0 }
    pub fn key(&self, h: usize) -> i64 { self.n[h].key }

    /// BINOMIAL-HEAP-MINIMUM
    pub fn minimum(&self) -> Option<i64> {
        let mut x = self.head;
        let mut best: Option<i64> = None;
        while x != NIL {
            best = Some(best.map_or(self.n[x].key, |b| b.min(self.n[x].key)));
            x = self.n[x].sibling;
        }
        best
    }
    /// Degrees of the root trees (the binary representation of n).
    pub fn root_degrees(&self) -> Vec<usize> {
        let (mut v, mut x) = (vec![], self.head);
        while x != NIL { v.push(self.n[x].degree); x = self.n[x].sibling; }
        v
    }

    /// Problem 19-2(a): for every root tree, (root degree, number of nodes, height, number of nodes at each depth, degrees of the root's children left to right).
    pub fn tree_shapes(&self) -> Vec<(usize, usize, usize, Vec<usize>, Vec<usize>)> {
        let mut out = vec![];
        let mut r = self.head;
        while r != NIL {
            let mut per_depth: Vec<usize> = vec![];
            let mut stack = vec![(r, 0usize)];
            while let Some((x, d)) = stack.pop() {
                if per_depth.len() <= d { per_depth.resize(d + 1, 0); }
                per_depth[d] += 1;
                let mut c = self.n[x].child;
                while c != NIL { stack.push((c, d + 1)); c = self.n[c].sibling; }
            }
            let mut kids = vec![];
            let mut c = self.n[r].child;
            while c != NIL { kids.push(self.n[c].degree); c = self.n[c].sibling; }
            out.push((self.n[r].degree, per_depth.iter().sum(), per_depth.len() - 1, per_depth, kids));
            r = self.n[r].sibling;
        }
        out
    }

    /// BINOMIAL-LINK(y, z): make y the leftmost child of z.
    fn link(&mut self, y: usize, z: usize) {
        self.n[y].p = z;
        self.n[y].sibling = self.n[z].child;
        self.n[z].child = y;
        self.n[z].degree += 1;
    }
    /// BINOMIAL-HEAP-MERGE: merge two root lists into one sorted by degree.
    fn merge_lists(&mut self, mut a: usize, mut b: usize) -> usize {
        let (mut head, mut tail) = (NIL, NIL);
        while a != NIL || b != NIL {
            let pick;
            if b == NIL || (a != NIL && self.n[a].degree <= self.n[b].degree) { pick = a; a = self.n[a].sibling; }
            else { pick = b; b = self.n[b].sibling; }
            if head == NIL { head = pick; } else { self.n[tail].sibling = pick; }
            tail = pick;
        }
        head
    }
    /// BINOMIAL-HEAP-UNION on two root lists living in this arena.
    fn union_lists(&mut self, h1: usize, h2: usize) -> usize {
        let mut head = self.merge_lists(h1, h2);
        if head == NIL { return NIL; }
        let (mut prev, mut x) = (NIL, head);
        let mut next = self.n[x].sibling;
        while next != NIL {
            let after = self.n[next].sibling;
            if self.n[x].degree != self.n[next].degree || (after != NIL && self.n[after].degree == self.n[x].degree) {
                prev = x; // cases 1 and 2: move on
                x = next;
            } else if self.n[x].key <= self.n[next].key {
                self.n[x].sibling = after; // case 3: next becomes x's child
                self.link(next, x);
            } else {
                if prev == NIL { head = next; } else { self.n[prev].sibling = next; } // case 4: x becomes next's child
                self.link(x, next);
                x = next;
            }
            next = self.n[x].sibling;
        }
        head
    }
    /// BINOMIAL-HEAP-UNION(H1, H2): O(lg n). Handles of `other` are shifted by the returned offset.
    pub fn union(&mut self, other: BinomialHeap) -> usize {
        let off = self.n.len();
        let shift = |i: usize| if i == NIL { NIL } else { i + off };
        for mut nd in other.n {
            nd.p = shift(nd.p);
            nd.child = shift(nd.child);
            nd.sibling = shift(nd.sibling);
            self.n.push(nd);
        }
        let oh = shift(other.head);
        self.head = self.union_lists(self.head, oh);
        self.count += other.count;
        off
    }
    /// BINOMIAL-HEAP-INSERT: union with a one-node heap. Returns the node's handle.
    pub fn insert(&mut self, key: i64) -> usize {
        let x = self.n.len();
        self.n.push(Node { key, degree: 0, p: NIL, child: NIL, sibling: NIL });
        self.head = self.union_lists(self.head, x);
        self.count += 1;
        x
    }
    /// BINOMIAL-HEAP-EXTRACT-MIN: returns (handle, key).
    pub fn extract_min(&mut self) -> Option<(usize, i64)> {
        if self.head == NIL { return None; }
        let (mut best, mut best_prev) = (self.head, NIL);
        let (mut prev, mut x) = (NIL, self.head);
        while x != NIL {
            if self.n[x].key < self.n[best].key { best = x; best_prev = prev; }
            prev = x;
            x = self.n[x].sibling;
        }
        if best_prev == NIL { self.head = self.n[best].sibling; } else { self.n[best_prev].sibling = self.n[best].sibling; }
        // reverse the children list: it becomes the root list of a new heap
        let (mut rev, mut c) = (NIL, self.n[best].child);
        while c != NIL {
            let next = self.n[c].sibling;
            self.n[c].sibling = rev;
            self.n[c].p = NIL;
            rev = c;
            c = next;
        }
        self.n[best].child = NIL;
        self.head = self.union_lists(self.head, rev);
        self.count -= 1;
        Some((best, self.n[best].key))
    }
    /// BINOMIAL-HEAP-DECREASE-KEY: bubble the smaller key up by exchanging keys with ancestors.
    /// (As in the book the key *values* move; a handle names a position in the tree.)
    pub fn decrease_key(&mut self, x: usize, k: i64) -> Result<(), &'static str> {
        if k > self.n[x].key { return Err("new key is greater than current key"); }
        self.n[x].key = k;
        let mut y = x;
        let mut z = self.n[y].p;
        while z != NIL && self.n[y].key < self.n[z].key {
            let t = self.n[y].key;
            self.n[y].key = self.n[z].key;
            self.n[z].key = t;
            y = z;
            z = self.n[y].p;
        }
        Ok(())
    }
    /// BINOMIAL-HEAP-DELETE: decrease to −∞ and extract.
    pub fn delete(&mut self, x: usize) {
        self.decrease_key(x, i64::MIN).unwrap();
        self.extract_min();
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut h = BinomialHeap::new();
    for k in [12, 7, 25, 15, 28, 33, 41] { h.insert(k); }
    outln!(o, "7 inserts: root degrees = {:?} (7 = 111₂ → trees B0, B1, B2)", h.root_degrees());
    h.insert(1);
    outln!(o, "8th insert: root degrees = {:?} (8 = 1000₂ → single B3)", h.root_degrees());
    outln!(o, "minimum = {:?}", h.minimum());
    let mut other = BinomialHeap::new();
    for k in [3, 9, 6] { other.insert(k); }
    h.union(other);
    outln!(o, "after union with {{3,9,6}}: size {} degrees {:?}", h.len(), h.root_degrees());
    let mut out = vec![];
    while let Some((_, k)) = h.extract_min() { out.push(k); }
    outln!(o, "extract-min order: {:?}", out);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn binary_degrees(mut n: usize) -> Vec<usize> {
        let mut v = vec![];
        let mut bit = 0;
        while n > 0 { if n & 1 == 1 { v.push(bit); } n >>= 1; bit += 1; }
        v
    }
    fn binom(n: usize, k: usize) -> usize { (0..k).fold(1, |acc, i| acc * (n - i) / (i + 1)) }
    #[test]
    fn binomial_tree_properties_of_problem_19_2() {
        let mut r = Rng::new(95);
        for _ in 0..60 {
            let mut h = BinomialHeap::new();
            let n = r.range(1, 300);
            for _ in 0..n { h.insert(r.range(0, 1000) as i64); }
            for _ in 0..r.range(0, n / 2) { h.extract_min(); }
            let n = h.len();
            let shapes = h.tree_shapes();
            // (b) one tree for every 1-bit of n, so at most ⌊lg n⌋ + 1 trees
            assert_eq!(shapes.iter().map(|s| s.0).collect::<Vec<_>>(), binary_degrees(n));
            assert!(shapes.len() <= (n as f64).log2().floor() as usize + 1);
            for (k, size, height, per_depth, kids) in shapes {
                assert_eq!(size, 1 << k, "B_k has 2^k nodes");
                assert_eq!(height, k, "B_k has height k");
                assert_eq!(per_depth, (0..=k).map(|i| binom(k, i)).collect::<Vec<_>>(), "C(k, i) nodes at depth i");
                assert_eq!(kids, (0..k).rev().collect::<Vec<_>>(), "children of the root have degrees k−1, …, 0");
            }
        }
    }
    #[test]
    fn root_list_mirrors_binary_representation() {
        let mut h = BinomialHeap::new();
        for k in 0..200 {
            h.insert(k * 31 % 17);
            assert_eq!(h.root_degrees(), binary_degrees(h.len()));
        }
    }
    #[test]
    fn heapsort_and_union() {
        let mut r = Rng::new(82);
        let (a, b) = (r.vec(150, -100, 100), r.vec(77, -100, 100));
        let (mut ha, mut hb) = (BinomialHeap::new(), BinomialHeap::new());
        a.iter().for_each(|&k| { ha.insert(k); });
        b.iter().for_each(|&k| { hb.insert(k); });
        ha.union(hb);
        assert_eq!(ha.root_degrees(), binary_degrees(227));
        let out: Vec<i64> = std::iter::from_fn(|| ha.extract_min().map(|x| x.1)).collect();
        let mut e: Vec<i64> = a.into_iter().chain(b).collect();
        e.sort();
        assert_eq!(out, e);
    }
    #[test]
    fn decrease_key_and_delete_keep_multiset_semantics() {
        let mut r = Rng::new(83);
        let mut h = BinomialHeap::new();
        let mut alive: Vec<usize> = vec![];
        for _ in 0..2000 {
            match r.range(0, 3) {
                0 | 1 => alive.push(h.insert(r.range(0, 5000) as i64)),
                2 if !alive.is_empty() => {
                    let i = r.range(0, alive.len() - 1);
                    h.decrease_key(alive[i], h.key(alive[i]) - r.range(0, 300) as i64).unwrap();
                }
                _ if !alive.is_empty() => {
                    let (hd, _) = h.extract_min().unwrap();
                    alive.retain(|&a| a != hd);
                }
                _ => {}
            }
            assert_eq!(h.len(), alive.len());
            assert_eq!(h.minimum(), alive.iter().map(|&a| h.key(a)).min());
        }
        let i = alive.len() / 2;
        let victim = alive.remove(i);
        h.delete(victim);
        assert_eq!(h.len(), alive.len());
    }
}
