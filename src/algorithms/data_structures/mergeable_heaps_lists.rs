//! @title Mergeable Heaps with Linked Lists
//! @chapter Problem 10-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Sorted-list vs unsorted-list mergeable heaps: INSERT/MINIMUM/EXTRACT-MIN/UNION cost trade-offs, with operation counts.
//! @order 45
use crate::outln;
use std::collections::VecDeque;

/// (a) Sorted list: MINIMUM and EXTRACT-MIN are O(1); INSERT and UNION are O(n). Counts list-node steps.
#[derive(Debug, Clone, Default)]
pub struct SortedListHeap { items: VecDeque<i64>, pub steps: usize }

impl SortedListHeap {
    pub fn new() -> Self { Self::default() }
    pub fn insert(&mut self, x: i64) {
        let pos = self.items.iter().position(|&y| y >= x).unwrap_or(self.items.len());
        self.steps += pos + 1;
        self.items.insert(pos, x);
    }
    pub fn minimum(&mut self) -> Option<i64> { self.steps += 1; self.items.front().copied() }
    pub fn extract_min(&mut self) -> Option<i64> { self.steps += 1; self.items.pop_front() }
    /// Merge two sorted lists in O(n1 + n2).
    pub fn union(&mut self, other: SortedListHeap) {
        let (a, b) = (std::mem::take(&mut self.items), other.items);
        self.steps += a.len() + b.len();
        let (mut ia, mut ib) = (a.into_iter().peekable(), b.into_iter().peekable());
        while ia.peek().is_some() || ib.peek().is_some() {
            let take_a = match (ia.peek(), ib.peek()) { (Some(x), Some(y)) => x <= y, (Some(_), None) => true, _ => false };
            self.items.push_back(if take_a { ia.next().unwrap() } else { ib.next().unwrap() });
        }
    }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

/// (b) Unsorted list: INSERT and UNION are O(1) (append / splice); MINIMUM and EXTRACT-MIN are O(n).
#[derive(Debug, Clone, Default)]
pub struct UnsortedListHeap { items: Vec<i64>, pub steps: usize }

impl UnsortedListHeap {
    pub fn new() -> Self { Self::default() }
    pub fn insert(&mut self, x: i64) { self.steps += 1; self.items.push(x); }
    pub fn minimum(&mut self) -> Option<i64> { self.steps += self.items.len(); self.items.iter().copied().min() }
    pub fn extract_min(&mut self) -> Option<i64> {
        self.steps += self.items.len();
        let (i, _) = self.items.iter().enumerate().min_by_key(|&(_, &x)| x)?;
        Some(self.items.swap_remove(i))
    }
    /// O(1) in a linked list (concatenation of two lists); here counted as one step.
    pub fn union(&mut self, other: UnsortedListHeap) { self.steps += 1; self.items.extend(other.items); }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = crate::common::Rng::new(1);
    let n = 2000;
    let (xs, ys) = (r.vec(n, 0, 100_000), r.vec(n, 0, 100_000));
    let (mut sa, mut sb) = (SortedListHeap::new(), SortedListHeap::new());
    let (mut ua, mut ub) = (UnsortedListHeap::new(), UnsortedListHeap::new());
    for &x in &xs { sa.insert(x); ua.insert(x); }
    for &y in &ys { sb.insert(y); ub.insert(y); }
    let (ins_s, ins_u) = (sa.steps + sb.steps, ua.steps + ub.steps);
    let (before_s, before_u) = (sa.steps, ua.steps);
    sa.union(sb);
    ua.union(ub);
    outln!(o, "{} inserts: sorted list {} steps (≈ n²/4 each), unsorted list {} steps", 2 * n, ins_s, ins_u);
    outln!(o, "UNION of two lists of {n}: sorted {} steps, unsorted {} step(s)", sa.steps - before_s, ua.steps - before_u);
    let (s0, u0) = (sa.steps, ua.steps);
    for _ in 0..100 { sa.extract_min(); ua.extract_min(); }
    outln!(o, "100 EXTRACT-MINs: sorted {} steps, unsorted {} steps", sa.steps - s0, ua.steps - u0);
    outln!(o, "neither list gives O(lg n) for everything — that is what binary, binomial and Fibonacci heaps achieve");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn both_behave_like_a_priority_queue() {
        let mut r = Rng::new(329);
        let (mut a, mut b, mut model) = (SortedListHeap::new(), UnsortedListHeap::new(), std::collections::BinaryHeap::new());
        for _ in 0..2000 {
            match r.range(0, 3) {
                0 | 1 => { let x = r.range(0, 500) as i64; a.insert(x); b.insert(x); model.push(std::cmp::Reverse(x)); }
                _ => { let w = model.pop().map(|x| x.0); assert_eq!(a.extract_min(), w); assert_eq!(b.extract_min(), w); }
            }
            assert_eq!(a.minimum(), model.peek().map(|x| x.0));
            assert_eq!(b.minimum(), model.peek().map(|x| x.0));
        }
    }
    #[test]
    fn union_semantics_and_costs() {
        let (mut a, mut b) = (SortedListHeap::new(), SortedListHeap::new());
        for x in [5, 1, 9] { a.insert(x); }
        for x in [4, 2, 10] { b.insert(x); }
        a.union(b);
        let out: Vec<i64> = std::iter::from_fn(|| a.extract_min()).collect();
        assert_eq!(out, vec![1, 2, 4, 5, 9, 10]);
        let (mut u, mut v) = (UnsortedListHeap::new(), UnsortedListHeap::new());
        (0..1000).for_each(|x| { u.insert(x); v.insert(x + 1000); });
        let s = u.steps;
        u.union(v);
        assert_eq!(u.steps - s, 1);
        assert_eq!(u.len(), 2000);
    }
}
