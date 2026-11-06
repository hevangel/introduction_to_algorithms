//! @title Young Tableaux
//! @chapter Problem 6-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary A sorted m×n matrix acting as a priority queue: EXTRACT-MIN in O(m+n), sorting n² numbers in O(n³).
//! @order 40
use crate::outln;

pub const INF: i64 = i64::MAX;

/// m×n Young tableau: rows and columns are sorted ascending; `INF` marks empty cells.
#[derive(Debug, Clone)]
pub struct YoungTableau {
    pub m: usize,
    pub n: usize,
    pub a: Vec<Vec<i64>>,
}

impl YoungTableau {
    pub fn new(m: usize, n: usize) -> Self {
        YoungTableau { m, n, a: vec![vec![INF; n]; m] }
    }
    pub fn is_empty(&self) -> bool { self.a[0][0] == INF }
    pub fn is_full(&self) -> bool { self.a[self.m - 1][self.n - 1] != INF }

    /// EXTRACT-MIN: remove the corner element and let the hole slide to the bottom right, O(m+n).
    pub fn extract_min(&mut self) -> Option<i64> {
        if self.is_empty() { return None; }
        let min = self.a[0][0];
        self.a[0][0] = INF;
        let (mut i, mut j) = (0, 0);
        loop {
            let mut best = (i, j);
            if i + 1 < self.m && self.a[i + 1][j] < self.a[best.0][best.1] { best = (i + 1, j); }
            if j + 1 < self.n && self.a[i][j + 1] < self.a[best.0][best.1] { best = (i, j + 1); }
            if best == (i, j) { break; }
            let tmp = self.a[i][j];
            self.a[i][j] = self.a[best.0][best.1];
            self.a[best.0][best.1] = tmp;
            (i, j) = best;
        }
        Some(min)
    }

    /// INSERT into the (necessarily empty) bottom-right cell, then bubble towards the top-left, O(m+n).
    pub fn insert(&mut self, x: i64) -> Result<(), &'static str> {
        if self.is_full() { return Err("tableau is full"); }
        let (mut i, mut j) = (self.m - 1, self.n - 1);
        self.a[i][j] = x;
        loop {
            let mut best = (i, j);
            if i > 0 && self.a[i - 1][j] > self.a[best.0][best.1] { best = (i - 1, j); }
            if j > 0 && self.a[i][j - 1] > self.a[best.0][best.1] { best = (i, j - 1); }
            if best == (i, j) { break; }
            let tmp = self.a[i][j];
            self.a[i][j] = self.a[best.0][best.1];
            self.a[best.0][best.1] = tmp;
            (i, j) = best;
        }
        Ok(())
    }

    /// Membership test in O(m+n): walk from the top-right corner.
    pub fn contains(&self, x: i64) -> bool {
        let (mut i, mut j) = (0isize, self.n as isize - 1);
        while i < self.m as isize && j >= 0 {
            let v = self.a[i as usize][j as usize];
            if v == x { return true; }
            if v > x { j -= 1 } else { i += 1 }
        }
        false
    }
}

/// Sort n² numbers with an n×n tableau in O(n³).
pub fn sort_with_tableau(v: &[i64]) -> Vec<i64> {
    let n = (v.len() as f64).sqrt().ceil() as usize;
    let n = n.max(1);
    let mut t = YoungTableau::new(n, n);
    for &x in v { t.insert(x).unwrap(); }
    let mut out = Vec::with_capacity(v.len());
    while let Some(x) = t.extract_min() { out.push(x); }
    out
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut t = YoungTableau::new(4, 4);
    for x in [9, 16, 3, 2, 4, 8, 5, 14, 12] { t.insert(x).unwrap(); }
    for row in &t.a {
        let r: Vec<String> = row.iter().map(|&x| if x == INF { "∞".into() } else { x.to_string() }).collect();
        outln!(o, "{}", r.join("\t"));
    }
    outln!(o, "contains 8? {}   contains 7? {}", t.contains(8), t.contains(7));
    outln!(o, "extract-min x3: {:?}", (0..3).map(|_| t.extract_min().unwrap()).collect::<Vec<_>>());
    outln!(o, "sorted 12 numbers: {:?}", sort_with_tableau(&[7, 3, 11, 1, 9, 5, 12, 2, 8, 4, 10, 6]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn valid(t: &YoungTableau) -> bool {
        (0..t.m).all(|i| (0..t.n).all(|j| (j + 1 >= t.n || t.a[i][j] <= t.a[i][j + 1]) && (i + 1 >= t.m || t.a[i][j] <= t.a[i + 1][j])))
    }
    #[test]
    fn insert_keeps_invariant_and_extract_sorts() {
        let mut r = Rng::new(24);
        let v = r.vec(25, -40, 40);
        let mut t = YoungTableau::new(5, 5);
        for &x in &v {
            t.insert(x).unwrap();
            assert!(valid(&t));
        }
        assert!(t.is_full());
        assert!(t.insert(0).is_err());
        let mut out = vec![];
        while let Some(x) = t.extract_min() {
            assert!(valid(&t));
            out.push(x);
        }
        let mut e = v;
        e.sort();
        assert_eq!(out, e);
    }
    #[test]
    fn membership() {
        let mut t = YoungTableau::new(3, 4);
        for x in [10, 2, 7, 5, 9] { t.insert(x).unwrap(); }
        for x in [10, 2, 7, 5, 9] { assert!(t.contains(x)); }
        assert!(!t.contains(6));
    }
    #[test]
    fn sorts_non_square_counts() {
        let mut r = Rng::new(25);
        let v = r.vec(37, 0, 100);
        let mut e = v.clone();
        e.sort();
        assert_eq!(sort_with_tableau(&v), e);
    }
}
