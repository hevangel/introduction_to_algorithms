"""Stacks, Queues & Deques — CLRS 10.1 (1e-4e) (Editions: 1,2,3,4).

Array-based stack, circular queue and deque with overflow/underflow detection; queue from two stacks.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest
from collections import deque
from typing import Any

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

# A Result is modelled as ("Ok", value) or ("Err", message), like Rust's Result.
Result = tuple[str, Any]
_OK_UNIT: Result = ("Ok", None)


def _dbg(v: Any) -> str:
    """Rust `{:?}`-style rendering of the values used by the demo."""
    if v is None:
        return "None"
    if isinstance(v, tuple) and len(v) == 2 and v[0] in ("Ok", "Err"):
        tag, inner = v
        if tag == "Ok":
            return "Ok(())" if inner is None else f"Ok({_dbg(inner)})"
        return f'Err("{inner}")'
    if isinstance(v, str):
        return f"'{v}'"
    return str(v)


def _some(v: Any) -> str:
    return "None" if v is None else f"Some({_dbg(v)})"


class Stack:
    """Fixed-capacity stack (PUSH / POP / STACK-EMPTY)."""

    def __init__(self, capacity: int) -> None:
        self._s: list[Any] = [None] * capacity
        self._top = 0

    @classmethod
    def with_capacity(cls, n: int) -> Stack:
        return cls(n)

    def is_empty(self) -> bool:
        return self._top == 0

    def push(self, x: Any) -> Result:
        if self._top == len(self._s):
            return ("Err", "overflow")
        self._s[self._top] = x
        self._top += 1
        return _OK_UNIT

    def pop(self) -> Result:
        if self._top == 0:
            return ("Err", "underflow")
        self._top -= 1
        x = self._s[self._top]
        self._s[self._top] = None
        return ("Ok", x)


class Queue:
    """Fixed-capacity circular queue (ENQUEUE / DEQUEUE)."""

    def __init__(self, capacity: int) -> None:
        self._q: list[Any] = [None] * capacity
        self._head = 0
        self._len = 0

    @classmethod
    def with_capacity(cls, n: int) -> Queue:
        return cls(n)

    def __len__(self) -> int:
        return self._len

    def is_empty(self) -> bool:
        return self._len == 0

    def enqueue(self, x: Any) -> Result:
        if self._len == len(self._q):
            return ("Err", "overflow")
        tail = (self._head + self._len) % len(self._q)
        self._q[tail] = x
        self._len += 1
        return _OK_UNIT

    def dequeue(self) -> Result:
        if self._len == 0:
            return ("Err", "underflow")
        x = self._q[self._head]
        self._q[self._head] = None
        self._head = (self._head + 1) % len(self._q)
        self._len -= 1
        return ("Ok", x)


class Deque:
    """Double-ended queue on a circular buffer (Exercise 10.1-5)."""

    def __init__(self, capacity: int) -> None:
        self._q: list[Any] = [None] * capacity
        self._head = 0
        self._len = 0

    @classmethod
    def with_capacity(cls, n: int) -> Deque:
        return cls(n)

    def __len__(self) -> int:
        return self._len

    def push_back(self, x: Any) -> Result:
        if self._len == len(self._q):
            return ("Err", "overflow")
        i = (self._head + self._len) % len(self._q)
        self._q[i] = x
        self._len += 1
        return _OK_UNIT

    def push_front(self, x: Any) -> Result:
        if self._len == len(self._q):
            return ("Err", "overflow")
        self._head = (self._head + len(self._q) - 1) % len(self._q)
        self._q[self._head] = x
        self._len += 1
        return _OK_UNIT

    def pop_front(self) -> Result:
        if self._len == 0:
            return ("Err", "underflow")
        x = self._q[self._head]
        self._q[self._head] = None
        self._head = (self._head + 1) % len(self._q)
        self._len -= 1
        return ("Ok", x)

    def pop_back(self) -> Result:
        if self._len == 0:
            return ("Err", "underflow")
        i = (self._head + self._len - 1) % len(self._q)
        self._len -= 1
        x = self._q[i]
        self._q[i] = None
        return ("Ok", x)


class QueueFromTwoStacks:
    """Exercise 10.1-6: a queue implemented with two stacks (amortised O(1) per operation)."""

    def __init__(self) -> None:
        self._inbox: list[Any] = []
        self._outbox: list[Any] = []

    def enqueue(self, x: Any) -> None:
        self._inbox.append(x)

    def dequeue(self) -> Any | None:
        if not self._outbox:
            while self._inbox:
                self._outbox.append(self._inbox.pop())
        return self._outbox.pop() if self._outbox else None


def demo() -> str:
    lines: list[str] = []
    s = Stack.with_capacity(3)
    for x in range(1, 5):
        lines.append(f"push {x} -> {_dbg(s.push(x))}")
    a = s.pop()
    b = s.pop()
    lines.append(f"pop -> {_dbg(a)}, pop -> {_dbg(b)}")
    q = Queue.with_capacity(3)
    q.enqueue("a")
    q.enqueue("b")
    q.enqueue("c")
    lines.append(f"queue full, enqueue 'd' -> {_dbg(q.enqueue('d'))}")
    dq = q.dequeue()
    en = q.enqueue("d")
    lines.append(f"dequeue -> {_dbg(dq)}; enqueue 'd' -> {_dbg(en)} (wraps around)")
    d = Deque.with_capacity(4)
    d.push_back(2)
    d.push_front(1)
    d.push_back(3)
    pf = d.pop_front()
    pb = d.pop_back()
    lines.append(f"deque pop_front={_dbg(pf)} pop_back={_dbg(pb)}")
    t = QueueFromTwoStacks()
    for x in range(1, 4):
        t.enqueue(x)
    t1 = t.dequeue()
    t2 = t.dequeue()
    lines.append(f"two-stack queue: {_some(t1)} {_some(t2)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_stack_lifo_and_errors(self) -> None:
        s = Stack.with_capacity(2)
        self.assertTrue(s.is_empty())
        s.push(1)
        s.push(2)
        self.assertEqual(s.push(3), ("Err", "overflow"))
        self.assertEqual(s.pop(), ("Ok", 2))
        self.assertEqual(s.pop(), ("Ok", 1))
        self.assertEqual(s.pop(), ("Err", "underflow"))

    def test_queue_and_deque_match_std(self) -> None:
        q = Queue.with_capacity(5)
        m: deque[int] = deque()
        d = Deque.with_capacity(5)
        md: deque[int] = deque()
        r = Rng(50)

        def ok(res: Result) -> int | None:
            return res[1] if res[0] == "Ok" else None

        for i in range(2000):
            op = r.range(0, 5)
            if op == 0:
                if len(m) < 5:
                    m.append(i)
                    expect = True
                else:
                    expect = False
                self.assertEqual(q.enqueue(i)[0] == "Ok", expect)
            elif op == 1:
                self.assertEqual(ok(q.dequeue()), m.popleft() if m else None)
            elif op == 2:
                if len(md) < 5:
                    md.appendleft(i)
                    expect = True
                else:
                    expect = False
                self.assertEqual(d.push_front(i)[0] == "Ok", expect)
            elif op == 3:
                if len(md) < 5:
                    md.append(i)
                    expect = True
                else:
                    expect = False
                self.assertEqual(d.push_back(i)[0] == "Ok", expect)
            elif op == 4:
                self.assertEqual(ok(d.pop_front()), md.popleft() if md else None)
            else:
                self.assertEqual(ok(d.pop_back()), md.pop() if md else None)
        self.assertEqual(len(q), len(m))
        self.assertEqual(len(d), len(md))

    def test_two_stack_queue_is_fifo(self) -> None:
        t = QueueFromTwoStacks()
        for i in range(5):
            t.enqueue(i)
        self.assertEqual(t.dequeue(), 0)
        t.enqueue(5)
        rest = []
        while (x := t.dequeue()) is not None:
            rest.append(x)
        self.assertEqual(rest, [1, 2, 3, 4, 5])


if __name__ == "__main__":
    print(demo(), end="")
