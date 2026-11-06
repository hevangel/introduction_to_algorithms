//! @title Order-Statistic Trees
//! @chapter 14.1–14.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Red-black tree augmented with subtree sizes: OS-SELECT and OS-RANK in O(lg n).
//! @order 130
use super::red_black_tree::{RedBlackTree, NIL};
use crate::outln;

/// Dynamic order-statistic set built on the size-augmented red-black tree.
#[derive(Debug, Clone, Default)]
pub struct OsTree {
    rb: RedBlackTree,
}

impl OsTree {
    pub fn new() -> Self { OsTree { rb: RedBlackTree::new() } }
    pub fn len(&self) -> usize { self.rb.len() }
    pub fn is_empty(&self) -> bool { self.rb.is_empty() }
    pub fn insert(&mut self, key: i64) { self.rb.insert(key); }
    pub fn delete(&mut self, key: i64) -> bool { self.rb.delete_key(key) }

    /// OS-SELECT(root, i): the i-th smallest key (1-based).
    pub fn select(&self, mut i: usize) -> Option<i64> {
        if i == 0 || i > self.len() { return None; }
        let mut x = self.rb.root();
        loop {
            let r = self.rb.size(self.rb.left(x)) + 1;
            if i == r { return Some(self.rb.key(x)); }
            if i < r { x = self.rb.left(x); } else { i -= r; x = self.rb.right(x); }
        }
    }
    /// OS-RANK(T, x): position of (the first node with) `key` in sorted order, 1-based.
    pub fn rank(&self, key: i64) -> Option<usize> {
        let z = self.rb.find(key);
        if z == NIL { return None; }
        let mut r = self.rb.size(self.rb.left(z)) + 1;
        let mut y = z;
        while y != self.rb.root() {
            let p = self.rb.parent(y);
            if y == self.rb.right(p) { r += self.rb.size(self.rb.left(p)) + 1; }
            y = p;
        }
        Some(r)
    }
    /// Number of stored keys ≤ `key` (Exercise 14.1-7 helper).
    pub fn count_le(&self, key: i64) -> usize {
        let (mut x, mut c) = (self.rb.root(), 0);
        while x != NIL {
            if self.rb.key(x) <= key { c += self.rb.size(self.rb.left(x)) + 1; x = self.rb.right(x); } else { x = self.rb.left(x); }
        }
        c
    }
    pub fn validate(&self) -> bool { self.rb.validate().is_ok() }
}

/// Exercise 14.1-7: count inversions in Θ(n lg n) with an order-statistic tree.
pub fn count_inversions(a: &[i64]) -> usize {
    let mut t = OsTree::new();
    let mut inv = 0;
    for (i, &x) in a.iter().enumerate() {
        inv += i - t.count_le(x); // earlier elements that are strictly greater
        t.insert(x);
    }
    inv
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = OsTree::new();
    for k in [26, 17, 41, 14, 21, 30, 47, 10, 16, 19, 21, 28, 38, 7, 12, 14, 20, 35, 39, 3] { t.insert(k); }
    outln!(o, "n = {}", t.len());
    for i in [1, 5, 10, 20] { outln!(o, "OS-SELECT({:>2}) = {:?}", i, t.select(i)); }
    for k in [3, 26, 47] { outln!(o, "OS-RANK({:>2})   = {:?}", k, t.rank(k)); }
    t.delete(26);
    outln!(o, "after deleting 26: OS-SELECT(10) = {:?}, size = {}", t.select(10), t.len());
    outln!(o, "inversions of [2,3,8,6,1] via OS-tree: {}", count_inversions(&[2, 3, 8, 6, 1]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn select_and_rank_match_sorted_vector() {
        let mut t = OsTree::new();
        let mut model: Vec<i64> = vec![];
        let mut r = Rng::new(72);
        for _ in 0..1500 {
            let k = r.range(0, 200) as i64;
            if r.range(0, 2) < 2 {
                if !model.contains(&k) { t.insert(k); model.push(k); model.sort(); }
            } else {
                assert_eq!(t.delete(k), model.contains(&k));
                model.retain(|&x| x != k);
            }
            if let Some(&m) = model.get(model.len() / 2) {
                assert_eq!(t.select(model.len() / 2 + 1), Some(m));
                assert_eq!(t.rank(m), Some(model.len() / 2 + 1));
            }
        }
        assert!(t.validate());
        for (i, &k) in model.iter().enumerate() {
            assert_eq!(t.select(i + 1), Some(k));
            assert_eq!(t.rank(k), Some(i + 1));
        }
        assert_eq!(t.select(0), None);
        assert_eq!(t.select(model.len() + 1), None);
    }
    #[test]
    fn inversions_match_merge_sort_count() {
        let mut r = Rng::new(73);
        for n in [0, 1, 10, 100] {
            let v = r.vec(n, 0, 30);
            assert_eq!(count_inversions(&v), crate::algorithms::foundations::inversions::count_inversions(&v));
        }
    }
}
