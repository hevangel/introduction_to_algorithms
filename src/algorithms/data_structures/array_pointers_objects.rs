//! @title Implementing Pointers and Objects
//! @chapter 10.3 (1e–3e)
//! @editions 1,2,3
//! @summary Multiple-array list representation with a free list: ALLOCATE-OBJECT / FREE-OBJECT, plus compaction.
//! @order 30
use crate::outln;

pub const NIL: usize = usize::MAX;

/// Doubly linked list stored in parallel arrays `key`, `next`, `prev`; unused cells form a free list.
#[derive(Debug, Clone)]
pub struct MultiArrayList {
    pub key: Vec<i64>,
    pub next: Vec<usize>,
    pub prev: Vec<usize>,
    pub head: usize,
    free: usize,
}

impl MultiArrayList {
    pub fn new(capacity: usize) -> Self {
        let next = (0..capacity).map(|i| if i + 1 < capacity { i + 1 } else { NIL }).collect();
        MultiArrayList { key: vec![0; capacity], next, prev: vec![NIL; capacity], head: NIL, free: if capacity > 0 { 0 } else { NIL } }
    }
    /// ALLOCATE-OBJECT: pop a cell from the free list (a stack).
    pub fn allocate(&mut self) -> Result<usize, &'static str> {
        if self.free == NIL { return Err("out of space"); }
        let x = self.free;
        self.free = self.next[x];
        Ok(x)
    }
    /// FREE-OBJECT: push a cell onto the free list.
    pub fn free_object(&mut self, x: usize) {
        self.next[x] = self.free;
        self.free = x;
    }
    pub fn insert(&mut self, k: i64) -> Result<usize, &'static str> {
        let x = self.allocate()?;
        self.key[x] = k;
        self.next[x] = self.head;
        self.prev[x] = NIL;
        if self.head != NIL { self.prev[self.head] = x; }
        self.head = x;
        Ok(x)
    }
    pub fn delete(&mut self, x: usize) {
        if self.prev[x] != NIL { let p = self.prev[x]; self.next[p] = self.next[x]; } else { self.head = self.next[x]; }
        if self.next[x] != NIL { let n = self.next[x]; self.prev[n] = self.prev[x]; }
        self.free_object(x);
    }
    pub fn to_vec(&self) -> Vec<i64> {
        let mut v = vec![];
        let mut x = self.head;
        while x != NIL { v.push(self.key[x]); x = self.next[x]; }
        v
    }
    /// Cells currently in use, in list order.
    pub fn used_cells(&self) -> Vec<usize> {
        let mut v = vec![];
        let mut x = self.head;
        while x != NIL { v.push(x); x = self.next[x]; }
        v
    }
    /// Exercise 10.3-4: relocate the m list objects to cells 0..m (keeping list order) so the rest is one contiguous free block.
    pub fn compact(&mut self) {
        let cells = self.used_cells();
        let cap = self.key.len();
        let m = cells.len();
        let keys: Vec<i64> = cells.iter().map(|&c| self.key[c]).collect();
        for i in 0..m {
            self.key[i] = keys[i];
            self.next[i] = if i + 1 < m { i + 1 } else { NIL };
            self.prev[i] = if i > 0 { i - 1 } else { NIL };
        }
        self.head = if m > 0 { 0 } else { NIL };
        for i in m..cap { self.next[i] = if i + 1 < cap { i + 1 } else { NIL }; }
        self.free = if m < cap { m } else { NIL };
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut l = MultiArrayList::new(8);
    let h: Vec<usize> = [9, 16, 4, 1].iter().map(|&k| l.insert(k).unwrap()).collect();
    outln!(o, "list = {:?}, cells in use (list order) = {:?}", l.to_vec(), l.used_cells());
    l.delete(h[1]);
    outln!(o, "after deleting 16: list = {:?}, cells = {:?}", l.to_vec(), l.used_cells());
    l.compact();
    outln!(o, "after compaction : list = {:?}, cells = {:?}", l.to_vec(), l.used_cells());
    let mut full = MultiArrayList::new(2);
    full.insert(1).unwrap(); full.insert(2).unwrap();
    outln!(o, "third insert into capacity-2 list -> {:?}", full.insert(3));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allocate_free_reuse() {
        let mut l = MultiArrayList::new(3);
        let a = l.insert(1).unwrap();
        let _b = l.insert(2).unwrap();
        let _c = l.insert(3).unwrap();
        assert!(l.insert(4).is_err());
        l.delete(a);
        assert_eq!(l.insert(9).unwrap(), a); // freed cell is reused first
        assert_eq!(l.to_vec(), vec![9, 3, 2]);
    }
    #[test]
    fn compaction_preserves_order() {
        let mut l = MultiArrayList::new(10);
        let hs: Vec<usize> = (0..10).map(|k| l.insert(k).unwrap()).collect();
        for &i in &[hs[1], hs[4], hs[5], hs[8]] { l.delete(i); }
        let before = l.to_vec();
        l.compact();
        assert_eq!(l.to_vec(), before);
        assert_eq!(l.used_cells(), (0..before.len()).collect::<Vec<_>>());
        // free cells are now cells m..cap and can be allocated
        for k in 0..4 { l.insert(100 + k).unwrap(); }
        assert!(l.insert(0).is_err());
    }
}
