//! @title Stacks, Queues & Deques
//! @chapter 10.1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Array-based stack, circular queue and deque with overflow/underflow detection; queue from two stacks.
//! @order 10
use crate::outln;

/// Fixed-capacity stack (PUSH / POP / STACK-EMPTY).
#[derive(Debug)]
pub struct Stack<T> {
    s: Vec<Option<T>>,
    top: usize,
}
impl<T> Stack<T> {
    pub fn with_capacity(n: usize) -> Self { Stack { s: (0..n).map(|_| None).collect(), top: 0 } }
    pub fn is_empty(&self) -> bool { self.top == 0 }
    pub fn push(&mut self, x: T) -> Result<(), &'static str> {
        if self.top == self.s.len() { return Err("overflow"); }
        self.s[self.top] = Some(x);
        self.top += 1;
        Ok(())
    }
    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.top == 0 { return Err("underflow"); }
        self.top -= 1;
        Ok(self.s[self.top].take().unwrap())
    }
}

/// Fixed-capacity circular queue (ENQUEUE / DEQUEUE).
#[derive(Debug)]
pub struct Queue<T> {
    q: Vec<Option<T>>,
    head: usize,
    len: usize,
}
impl<T> Queue<T> {
    pub fn with_capacity(n: usize) -> Self { Queue { q: (0..n).map(|_| None).collect(), head: 0, len: 0 } }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn enqueue(&mut self, x: T) -> Result<(), &'static str> {
        if self.len == self.q.len() { return Err("overflow"); }
        let tail = (self.head + self.len) % self.q.len();
        self.q[tail] = Some(x);
        self.len += 1;
        Ok(())
    }
    pub fn dequeue(&mut self) -> Result<T, &'static str> {
        if self.len == 0 { return Err("underflow"); }
        let x = self.q[self.head].take().unwrap();
        self.head = (self.head + 1) % self.q.len();
        self.len -= 1;
        Ok(x)
    }
}

/// Double-ended queue on a circular buffer (Exercise 10.1-5).
#[derive(Debug)]
pub struct Deque<T> {
    q: Vec<Option<T>>,
    head: usize,
    len: usize,
}
impl<T> Deque<T> {
    pub fn with_capacity(n: usize) -> Self { Deque { q: (0..n).map(|_| None).collect(), head: 0, len: 0 } }
    pub fn len(&self) -> usize { self.len }
    pub fn push_back(&mut self, x: T) -> Result<(), &'static str> {
        if self.len == self.q.len() { return Err("overflow"); }
        let i = (self.head + self.len) % self.q.len();
        self.q[i] = Some(x);
        self.len += 1;
        Ok(())
    }
    pub fn push_front(&mut self, x: T) -> Result<(), &'static str> {
        if self.len == self.q.len() { return Err("overflow"); }
        self.head = (self.head + self.q.len() - 1) % self.q.len();
        self.q[self.head] = Some(x);
        self.len += 1;
        Ok(())
    }
    pub fn pop_front(&mut self) -> Result<T, &'static str> {
        if self.len == 0 { return Err("underflow"); }
        let x = self.q[self.head].take().unwrap();
        self.head = (self.head + 1) % self.q.len();
        self.len -= 1;
        Ok(x)
    }
    pub fn pop_back(&mut self) -> Result<T, &'static str> {
        if self.len == 0 { return Err("underflow"); }
        let i = (self.head + self.len - 1) % self.q.len();
        self.len -= 1;
        Ok(self.q[i].take().unwrap())
    }
}

/// Exercise 10.1-6: a queue implemented with two stacks (amortised O(1) per operation).
#[derive(Debug, Default)]
pub struct QueueFromTwoStacks<T> {
    inbox: Vec<T>,
    outbox: Vec<T>,
}
impl<T> QueueFromTwoStacks<T> {
    pub fn new() -> Self { QueueFromTwoStacks { inbox: vec![], outbox: vec![] } }
    pub fn enqueue(&mut self, x: T) { self.inbox.push(x); }
    pub fn dequeue(&mut self) -> Option<T> {
        if self.outbox.is_empty() {
            while let Some(x) = self.inbox.pop() { self.outbox.push(x); }
        }
        self.outbox.pop()
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut s = Stack::with_capacity(3);
    for x in 1..=4 { outln!(o, "push {x} -> {:?}", s.push(x)); }
    outln!(o, "pop -> {:?}, pop -> {:?}", s.pop(), s.pop());
    let mut q = Queue::with_capacity(3);
    q.enqueue('a').unwrap(); q.enqueue('b').unwrap(); q.enqueue('c').unwrap();
    outln!(o, "queue full, enqueue 'd' -> {:?}", q.enqueue('d'));
    outln!(o, "dequeue -> {:?}; enqueue 'd' -> {:?} (wraps around)", q.dequeue(), q.enqueue('d'));
    let mut d = Deque::with_capacity(4);
    d.push_back(2).unwrap(); d.push_front(1).unwrap(); d.push_back(3).unwrap();
    outln!(o, "deque pop_front={:?} pop_back={:?}", d.pop_front(), d.pop_back());
    let mut t = QueueFromTwoStacks::new();
    for x in 1..=3 { t.enqueue(x); }
    outln!(o, "two-stack queue: {:?} {:?}", t.dequeue(), t.dequeue());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    #[test]
    fn stack_lifo_and_errors() {
        let mut s = Stack::with_capacity(2);
        assert!(s.is_empty());
        s.push(1).unwrap(); s.push(2).unwrap();
        assert_eq!(s.push(3), Err("overflow"));
        assert_eq!(s.pop(), Ok(2));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("underflow"));
    }
    #[test]
    fn queue_and_deque_match_std() {
        let mut q = Queue::with_capacity(5);
        let mut m: VecDeque<u32> = VecDeque::new();
        let mut d = Deque::with_capacity(5);
        let mut md: VecDeque<u32> = VecDeque::new();
        let mut r = crate::common::Rng::new(50);
        for i in 0..2000u32 {
            match r.range(0, 5) {
                0 => assert_eq!(q.enqueue(i).is_ok(), if m.len() < 5 { m.push_back(i); true } else { false }),
                1 => assert_eq!(q.dequeue().ok(), m.pop_front()),
                2 => assert_eq!(d.push_front(i).is_ok(), if md.len() < 5 { md.push_front(i); true } else { false }),
                3 => assert_eq!(d.push_back(i).is_ok(), if md.len() < 5 { md.push_back(i); true } else { false }),
                4 => assert_eq!(d.pop_front().ok(), md.pop_front()),
                _ => assert_eq!(d.pop_back().ok(), md.pop_back()),
            }
        }
        assert_eq!(q.len(), m.len());
        assert_eq!(d.len(), md.len());
    }
    #[test]
    fn two_stack_queue_is_fifo() {
        let mut t = QueueFromTwoStacks::new();
        for i in 0..5 { t.enqueue(i); }
        assert_eq!(t.dequeue(), Some(0));
        t.enqueue(5);
        let rest: Vec<i32> = std::iter::from_fn(|| t.dequeue()).collect();
        assert_eq!(rest, vec![1, 2, 3, 4, 5]);
    }
}
