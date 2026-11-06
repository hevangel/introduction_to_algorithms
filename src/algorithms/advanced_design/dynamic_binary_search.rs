//! @title Dynamic Binary Search
//! @chapter Problem 17-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Keep ⌈lg(n+1)⌉ sorted arrays of sizes 2^i (like a binary counter): SEARCH in O(lg² n), INSERT in O(lg n) amortised, DELETE too.
//! @order 235
use crate::outln;

/// The logarithmic method: array `A_i` is either empty or sorted with exactly 2^i elements, according to the binary representation of n.
#[derive(Debug, Clone, Default)]
pub struct DynamicSet { arrays: Vec<Vec<i64>>, n: usize }

fn merge(a: &[i64], b: &[i64]) -> Vec<i64> {
    let (mut i, mut j, mut out) = (0, 0, Vec::with_capacity(a.len() + b.len()));
    while i < a.len() || j < b.len() { if j >= b.len() || (i < a.len() && a[i] <= b[j]) { out.push(a[i]); i += 1; } else { out.push(b[j]); j += 1; } }
    out
}

impl DynamicSet {
    pub fn new() -> Self { Self::default() }
    pub fn len(&self) -> usize { self.n }
    pub fn is_empty(&self) -> bool { self.n == 0 }
    pub fn sizes(&self) -> Vec<usize> { self.arrays.iter().map(|a| a.len()).collect() }
    /// SEARCH: binary-search each non-empty array, smallest to largest: O(lg² n).
    pub fn search(&self, x: i64) -> Option<(usize, usize)> {
        self.arrays.iter().enumerate().find_map(|(i, a)| a.binary_search(&x).ok().map(|p| (i, p)))
    }
    pub fn contains(&self, x: i64) -> bool { self.search(x).is_some() }
    /// INSERT: carry like a binary counter — merge A_0, A_1, … until an empty slot is found. O(lg n) amortised.
    pub fn insert(&mut self, x: i64) {
        let mut carry = vec![x];
        let mut i = 0;
        loop {
            if i == self.arrays.len() { self.arrays.push(vec![]); }
            if self.arrays[i].is_empty() { self.arrays[i] = carry; break; }
            carry = merge(&self.arrays[i], &carry);
            self.arrays[i].clear();
            i += 1;
        }
        self.n += 1;
    }
    /// DELETE (part (d)): remove x from A_i, then refill the hole with one element y taken from the smallest non-empty array A_j
    /// and split the rest of A_j into arrays of sizes 2^0 … 2^(j−1). Worst case Θ(n), amortised O(lg n).
    pub fn delete(&mut self, x: i64) -> bool {
        let Some((i, pos)) = self.search(x) else { return false };
        let j = self.arrays.iter().position(|a| !a.is_empty()).unwrap();
        self.arrays[i].remove(pos);
        if i != j {
            let y = self.arrays[j].pop().unwrap();
            let p = self.arrays[i].partition_point(|&v| v < y);
            self.arrays[i].insert(p, y);              // A_i is full again (size 2^i)
        }
        // the remaining 2^j − 1 elements of A_j become arrays of sizes 1, 2, 4, …, 2^(j−1)
        let rest = std::mem::take(&mut self.arrays[j]);
        let mut start = 0;
        for k in 0..j { let len = 1 << k; self.arrays[k] = rest[start..start + len].to_vec(); start += len; }
        self.n -= 1;
        true
    }
    /// Invariant: |A_i| ∈ {0, 2^i}, A_i non-empty iff bit i of n is set, and every array is sorted.
    pub fn validate(&self) -> bool {
        self.arrays.iter().enumerate().all(|(i, a)| a.windows(2).all(|w| w[0] <= w[1]) && a.len() == if self.n >> i & 1 == 1 { 1 << i } else { 0 })
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut s = DynamicSet::new();
    for x in [5, 9, 1, 7, 3, 8, 2] { s.insert(x); outln!(o, "insert {x}: array sizes {:?}", s.sizes()); }
    outln!(o, "search(7) = {:?}, search(4) = {:?}", s.search(7), s.search(4));
    s.delete(9);
    outln!(o, "after delete(9): sizes {:?}, contains 9? {}", s.sizes(), s.contains(9));
    outln!(o, "n = {} stored in {} arrays — the binary representation of n", s.len(), s.sizes().iter().filter(|&&x| x > 0).count());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_a_btreeset_and_keeps_the_binary_counter_invariant() {
        let mut s = DynamicSet::new();
        let mut m = std::collections::BTreeSet::new();
        let mut r = Rng::new(336);
        for _ in 0..4000 {
            let x = r.range(0, 300) as i64;
            match r.range(0, 2) {
                0 | 1 => if m.insert(x) { s.insert(x); },
                _ => assert_eq!(s.delete(x), m.remove(&x)),
            }
            assert!(s.validate());
            assert_eq!(s.len(), m.len());
            assert_eq!(s.contains(x), m.contains(&x));
        }
    }
    #[test]
    fn insert_sizes_follow_n_in_binary() {
        let mut s = DynamicSet::new();
        for k in 1..=37 { s.insert(k * 7 % 101); assert_eq!(s.sizes().iter().filter(|&&x| x > 0).count() as u32, (k as u32).count_ones()); }
    }
}
