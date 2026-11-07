//! @title Linked Lists
//! @chapter 10.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Doubly linked list with a sentinel (LIST-SEARCH, LIST-INSERT, LIST-DELETE) plus in-place reversal of a singly linked list.
//! @order 20
use crate::outln;

const NIL: usize = 0; // index of the sentinel L.nil

/// Circular doubly linked list with a sentinel, stored in an arena so no `unsafe` is needed.
#[derive(Debug)]
pub struct LinkedList {
    key: Vec<i64>,
    next: Vec<usize>,
    prev: Vec<usize>,
    free: Vec<usize>,
}

impl Default for LinkedList { fn default() -> Self { Self::new() } }

impl LinkedList {
    pub fn new() -> Self {
        // the sentinel points to itself
        LinkedList { key: vec![0], next: vec![NIL], prev: vec![NIL], free: vec![] }
    }
    fn alloc(&mut self, k: i64) -> usize {
        if let Some(x) = self.free.pop() {
            self.key[x] = k;
            x
        } else {
            self.key.push(k);
            self.next.push(NIL);
            self.prev.push(NIL);
            self.key.len() - 1
        }
    }
    /// LIST-INSERT(L, x): splice a new node onto the front. Returns its handle.
    pub fn insert(&mut self, k: i64) -> usize {
        let x = self.alloc(k);
        self.next[x] = self.next[NIL];
        self.prev[self.next[NIL]] = x;
        self.next[NIL] = x;
        self.prev[x] = NIL;
        x
    }
    /// LIST-SEARCH(L, k): handle of the first node with key k.
    pub fn search(&self, k: i64) -> Option<usize> {
        let mut x = self.next[NIL];
        while x != NIL && self.key[x] != k { x = self.next[x]; }
        if x == NIL { None } else { Some(x) }
    }
    /// LIST-DELETE(L, x): O(1) given a handle (thanks to the sentinel there are no boundary cases).
    pub fn delete(&mut self, x: usize) {
        assert_ne!(x, NIL);
        self.next[self.prev[x]] = self.next[x];
        self.prev[self.next[x]] = self.prev[x];
        self.free.push(x);
    }
    pub fn to_vec(&self) -> Vec<i64> {
        let mut v = vec![];
        let mut x = self.next[NIL];
        while x != NIL { v.push(self.key[x]); x = self.next[x]; }
        v
    }
}

/// Singly linked list node.
#[derive(Debug)]
pub struct Node { pub key: i64, pub next: Option<Box<Node>> }

pub fn from_slice(a: &[i64]) -> Option<Box<Node>> {
    let mut head = None;
    for &k in a.iter().rev() { head = Some(Box::new(Node { key: k, next: head })); }
    head
}
pub fn to_vec(mut h: &Option<Box<Node>>) -> Vec<i64> {
    let mut v = vec![];
    while let Some(n) = h { v.push(n.key); h = &n.next; }
    v
}
/// Exercise 10.2-7: reverse a singly linked list in Θ(n) time with O(1) extra storage.
pub fn reverse(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev = None;
    while let Some(mut n) = head {
        head = n.next.take();
        n.next = prev;
        prev = Some(n);
    }
    prev
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut l = LinkedList::new();
    for k in [9, 16, 4, 1] { l.insert(k); }
    outln!(o, "after inserting 9,16,4,1 at the head: {:?}", l.to_vec());
    let x = l.search(4).unwrap();
    l.delete(x);
    outln!(o, "after deleting key 4 via its handle:   {:?}", l.to_vec());
    outln!(o, "search(99) = {:?}", l.search(99));
    let s = from_slice(&[1, 2, 3, 4]);
    outln!(o, "singly list {:?} reversed -> {:?}", to_vec(&s), to_vec(&reverse(s)));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert_search_delete() {
        let mut l = LinkedList::new();
        let hs: Vec<usize> = (1..=5).map(|k| l.insert(k)).collect();
        assert_eq!(l.to_vec(), vec![5, 4, 3, 2, 1]);
        l.delete(hs[2]); // key 3
        l.delete(hs[4]); // key 5 (head)
        l.delete(hs[0]); // key 1 (tail)
        assert_eq!(l.to_vec(), vec![4, 2]);
        assert_eq!(l.search(3), None);
        assert!(l.search(2).is_some());
        let h = l.insert(7); // reuses a freed slot
        assert_eq!(l.to_vec(), vec![7, 4, 2]);
        assert_eq!(l.search(7), Some(h));
    }
    #[test]
    fn empty_list() {
        let l = LinkedList::new();
        assert!(l.to_vec().is_empty());
        assert_eq!(l.search(1), None);
    }
    #[test]
    fn reverse_singly() {
        assert_eq!(to_vec(&reverse(from_slice(&[1, 2, 3, 4, 5]))), vec![5, 4, 3, 2, 1]);
        assert_eq!(to_vec(&reverse(None)), Vec::<i64>::new());
    }
}
