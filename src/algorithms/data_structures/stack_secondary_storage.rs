//! @title Stacks on Secondary Storage
//! @chapter Problem 18-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Keep one page in memory and a push/pop pattern at a page boundary thrashes (Θ(1) I/Os per operation); two pages give O(1/m) amortised I/Os.
//! @order 175
use crate::outln;
use std::collections::VecDeque;

/// A stack of `len` items stored in pages of `m` items on disk, with an LRU buffer of `b` pages in memory.
/// After every operation the page holding the top element must be resident. Counts page reads and write-backs.
#[derive(Debug)]
pub struct PagedStack {
    m: usize,
    b: usize,
    len: usize,
    resident: VecDeque<(usize, bool)>, // (page, dirty), front = most recently used
    allocated: usize,                  // pages 0..allocated exist on disk
    pub reads: usize,
    pub writes: usize,
}

impl PagedStack {
    pub fn new(m: usize, b: usize) -> Self { PagedStack { m, b, len: 0, resident: VecDeque::new(), allocated: 0, reads: 0, writes: 0 } }
    pub fn accesses(&self) -> usize { self.reads + self.writes }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    fn touch(&mut self, page: usize, dirty: bool) {
        if let Some(i) = self.resident.iter().position(|&(p, _)| p == page) {
            let (p, d) = self.resident.remove(i).unwrap();
            self.resident.push_front((p, d || dirty));
            return;
        }
        if page >= self.allocated { self.allocated = page + 1; } else { self.reads += 1; } // a brand-new page needs no disk read
        if self.resident.len() == self.b { if let Some((_, d)) = self.resident.pop_back() { if d { self.writes += 1; } } }
        self.resident.push_front((page, dirty));
    }
    pub fn push(&mut self) { self.len += 1; self.touch((self.len - 1) / self.m, true); }
    pub fn pop(&mut self) {
        assert!(self.len > 0);
        self.touch((self.len - 1) / self.m, true);
        self.len -= 1;
        if self.len > 0 { self.touch((self.len - 1) / self.m, false); } // the new top must be in memory
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let m = 64;
    for b in [1usize, 2, 3] {
        // fill exactly to a page boundary, then alternate push/pop 10000 times
        let mut s = PagedStack::new(m, b);
        for _ in 0..m { s.push(); }
        let before = s.accesses();
        for _ in 0..10_000 { s.push(); s.pop(); }
        outln!(o, "page size {m}, {b} page(s) in memory: {:>6} disk accesses for 20000 operations at a page boundary", s.accesses() - before);
    }
    let mut s = PagedStack::new(m, 2);
    for _ in 0..100_000 { s.push(); }
    outln!(o, "100000 pushes with 2 buffered pages: {} accesses (n/m = {})", s.accesses(), 100_000 / m);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_buffered_page_thrashes_at_a_boundary() {
        let m = 16;
        let mut s = PagedStack::new(m, 1);
        for _ in 0..m { s.push(); }
        let base = s.accesses();
        for _ in 0..1000 { s.push(); s.pop(); }
        assert!(s.accesses() - base >= 2000, "{}", s.accesses() - base);
    }
    #[test]
    fn two_pages_give_constant_amortised_cost() {
        let m = 16;
        let mut s = PagedStack::new(m, 2);
        for _ in 0..m { s.push(); }
        let base = s.accesses();
        for _ in 0..1000 { s.push(); s.pop(); }
        assert!(s.accesses() - base <= 4, "{}", s.accesses() - base);
        // n pushes cost about n/m page writes; popping everything reads each page back once and writes it again: ≈ 3n/m in total
        let mut t = PagedStack::new(m, 2);
        for _ in 0..1600 { t.push(); }
        for _ in 0..1600 { t.pop(); }
        assert!(t.accesses() <= 4 * 1600 / m + 8, "{}", t.accesses());
    }
}
