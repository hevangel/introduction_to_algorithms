//! @title Priority Queue (Binary Heap)
//! @chapter 6.5 (1e–4e)
//! @editions 1,2,3,4
//! @summary INSERT, MAXIMUM, EXTRACT-MAX, INCREASE-KEY and DELETE on a heap; also merging k sorted lists.
//! @order 20
use crate::outln;
use std::cmp::Reverse;

/// A max-priority queue over any `PartialOrd` key, stored as a binary max-heap.
#[derive(Debug, Clone, Default)]
pub struct PriorityQueue<T> {
    a: Vec<T>,
}

impl<T: PartialOrd + Clone> PriorityQueue<T> {
    pub fn new() -> Self { PriorityQueue { a: Vec::new() } }
    pub fn len(&self) -> usize { self.a.len() }
    pub fn is_empty(&self) -> bool { self.a.is_empty() }
    /// HEAP-MAXIMUM, O(1)
    pub fn maximum(&self) -> Option<&T> { self.a.first() }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 && self.a[(i - 1) / 2] < self.a[i] {
            self.a.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }
    fn sift_down(&mut self, mut i: usize) {
        let n = self.a.len();
        loop {
            let (l, r) = (2 * i + 1, 2 * i + 2);
            let mut m = i;
            if l < n && self.a[l] > self.a[m] { m = l; }
            if r < n && self.a[r] > self.a[m] { m = r; }
            if m == i { return; }
            self.a.swap(i, m);
            i = m;
        }
    }
    /// MAX-HEAP-INSERT, O(lg n)
    pub fn insert(&mut self, x: T) {
        self.a.push(x);
        let i = self.a.len() - 1;
        self.sift_up(i);
    }
    /// HEAP-EXTRACT-MAX, O(lg n)
    pub fn extract_max(&mut self) -> Option<T> {
        if self.a.is_empty() { return None; }
        let last = self.a.len() - 1;
        self.a.swap(0, last);
        let m = self.a.pop();
        self.sift_down(0);
        m
    }
    /// HEAP-INCREASE-KEY(A, i, key): `key` must not be smaller than the current key.
    pub fn increase_key(&mut self, i: usize, key: T) -> Result<(), &'static str> {
        if key < self.a[i] { return Err("new key is smaller than current key"); }
        self.a[i] = key;
        self.sift_up(i);
        Ok(())
    }
    /// HEAP-DELETE(A, i) (Exercise 6.5-8), O(lg n)
    pub fn delete(&mut self, i: usize) -> T {
        let last = self.a.len() - 1;
        self.a.swap(i, last);
        let x = self.a.pop().unwrap();
        if i < self.a.len() {
            self.sift_up(i);
            self.sift_down(i);
        }
        x
    }
}

/// Exercise 6.5-9: merge k sorted lists in O(n lg k) with a min-priority queue (max-heap of `Reverse`).
pub fn merge_sorted_lists(lists: &[Vec<i64>]) -> Vec<i64> {
    let mut pq = PriorityQueue::new();
    for (li, l) in lists.iter().enumerate() {
        if !l.is_empty() { pq.insert(Reverse((l[0], li, 0usize))); }
    }
    let mut out = Vec::new();
    while let Some(Reverse((v, li, pos))) = pq.extract_max() {
        out.push(v);
        if pos + 1 < lists[li].len() { pq.insert(Reverse((lists[li][pos + 1], li, pos + 1))); }
    }
    out
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut pq = PriorityQueue::new();
    for x in [15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1] { pq.insert(x); }
    outln!(o, "maximum            = {:?}", pq.maximum());
    pq.increase_key(8, 20).unwrap();
    outln!(o, "after increase_key = {:?}", pq.maximum());
    let drained: Vec<i32> = std::iter::from_fn(|| pq.extract_max()).collect();
    outln!(o, "extract-max order  = {:?}", drained);
    let merged = merge_sorted_lists(&[vec![1, 4, 9], vec![2, 3, 10, 11], vec![], vec![0, 8]]);
    outln!(o, "merge of 4 sorted lists = {:?}", merged);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn extracts_in_descending_order() {
        let mut r = Rng::new(22);
        let v = r.vec(200, -100, 100);
        let mut pq = PriorityQueue::new();
        v.iter().for_each(|&x| pq.insert(x));
        let out: Vec<i64> = std::iter::from_fn(|| pq.extract_max()).collect();
        let mut e = v;
        e.sort_by(|a, b| b.cmp(a));
        assert_eq!(out, e);
    }
    #[test]
    fn increase_key_and_delete() {
        let mut pq = PriorityQueue::new();
        for x in [5, 3, 8, 1] { pq.insert(x); }
        assert!(pq.increase_key(3, 0).is_err());
        pq.increase_key(3, 9).unwrap();
        assert_eq!(pq.maximum(), Some(&9));
        let d = pq.delete(0);
        assert_eq!(d, 9);
        assert_eq!(pq.maximum(), Some(&8));
        assert_eq!(pq.len(), 3);
    }
    #[test]
    fn k_way_merge() {
        let lists = vec![vec![1, 5, 9], vec![2, 6], vec![0, 3, 4, 7, 8]];
        assert_eq!(merge_sorted_lists(&lists), (0..10).collect::<Vec<_>>());
    }
}
