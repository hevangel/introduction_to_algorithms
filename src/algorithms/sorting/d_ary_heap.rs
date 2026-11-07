//! @title d-ary Heap
//! @chapter Problem 6-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary A heap whose nodes have d children: shallower tree, cheaper INSERT, costlier EXTRACT-MAX.
//! @order 30
use crate::outln;

/// d-ary max-heap of i64 keys. Children of i are d·i+1 … d·i+d; parent is (i−1)/d.
#[derive(Debug, Clone)]
pub struct DaryHeap {
    pub d: usize,
    pub a: Vec<i64>,
}

impl DaryHeap {
    pub fn new(d: usize) -> Self {
        assert!(d >= 2);
        DaryHeap { d, a: vec![] }
    }
    /// Height of a d-ary heap with n nodes: ⌈log_d(n(d−1)+1)⌉ − 1.
    pub fn height(&self) -> usize {
        let mut h = 0;
        let mut cap = 1usize; // nodes in a full tree of height h
        let mut level = 1usize;
        while cap < self.a.len() {
            level *= self.d;
            cap += level;
            h += 1;
        }
        h
    }
    pub fn insert(&mut self, key: i64) {
        self.a.push(key);
        let mut i = self.a.len() - 1;
        while i > 0 && self.a[(i - 1) / self.d] < self.a[i] {
            self.a.swap(i, (i - 1) / self.d);
            i = (i - 1) / self.d;
        }
    }
    pub fn extract_max(&mut self) -> Option<i64> {
        if self.a.is_empty() { return None; }
        let last = self.a.len() - 1;
        self.a.swap(0, last);
        let m = self.a.pop();
        self.max_heapify(0);
        m
    }
    fn max_heapify(&mut self, mut i: usize) {
        loop {
            let mut largest = i;
            for c in self.d * i + 1..=self.d * i + self.d {
                if c < self.a.len() && self.a[c] > self.a[largest] { largest = c; }
            }
            if largest == i { return; }
            self.a.swap(i, largest);
            i = largest;
        }
    }
    pub fn increase_key(&mut self, mut i: usize, key: i64) {
        assert!(key >= self.a[i]);
        self.a[i] = key;
        while i > 0 && self.a[(i - 1) / self.d] < self.a[i] {
            self.a.swap(i, (i - 1) / self.d);
            i = (i - 1) / self.d;
        }
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    for d in [2, 3, 4, 8] {
        let mut h = DaryHeap::new(d);
        for x in 0..1000 { h.insert((x * 7919) % 1009); }
        outln!(o, "d={d}: height of 1000-node heap = {}", h.height());
    }
    let mut h = DaryHeap::new(3);
    for x in [5, 1, 9, 3, 7] { h.insert(x); }
    let out: Vec<i64> = std::iter::from_fn(|| h.extract_max()).collect();
    outln!(o, "3-ary heap drains as {:?}", out);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn all_arities_sort_descending() {
        let mut r = Rng::new(23);
        for d in 2..7 {
            let v = r.vec(150, -50, 50);
            let mut h = DaryHeap::new(d);
            v.iter().for_each(|&x| h.insert(x));
            let out: Vec<i64> = std::iter::from_fn(|| h.extract_max()).collect();
            let mut e = v;
            e.sort_by(|a, b| b.cmp(a));
            assert_eq!(out, e);
        }
    }
    #[test]
    fn height_matches_formula() {
        for d in 2..6usize {
            let mut h = DaryHeap::new(d);
            for n in 1..200usize {
                h.insert(n as i64);
                // ⌈log_d(n(d−1)+1)⌉ − 1 computed with integers: smallest k with d^k ≥ n(d−1)+1
                let (mut k, mut pw) = (0usize, 1usize);
                while pw < n * (d - 1) + 1 { pw *= d; k += 1; }
                let f = k.saturating_sub(1);
                assert_eq!(h.height(), f, "d={d} n={n}");
            }
        }
    }
    #[test]
    fn increase_key_moves_to_top() {
        let mut h = DaryHeap::new(4);
        for x in 0..20 { h.insert(x); }
        h.increase_key(19, 100);
        assert_eq!(h.extract_max(), Some(100));
    }
}
