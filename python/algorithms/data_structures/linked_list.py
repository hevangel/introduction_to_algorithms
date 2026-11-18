"""Linked Lists — CLRS 10.2 (1e-4e) (Editions: 1,2,3,4).

Doubly linked list with a sentinel (LIST-SEARCH, LIST-INSERT, LIST-DELETE) plus in-place reversal of a singly linked list.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

_NIL = 0  # index of the sentinel L.nil


class LinkedList:
    """Circular doubly linked list with a sentinel, stored in an arena (index handles)."""

    def __init__(self) -> None:
        # the sentinel points to itself
        self.key: list[int] = [0]
        self.next: list[int] = [_NIL]
        self.prev: list[int] = [_NIL]
        self.free: list[int] = []

    def _alloc(self, k: int) -> int:
        if self.free:
            x = self.free.pop()
            self.key[x] = k
            return x
        self.key.append(k)
        self.next.append(_NIL)
        self.prev.append(_NIL)
        return len(self.key) - 1

    def insert(self, k: int) -> int:
        """LIST-INSERT(L, x): splice a new node onto the front. Returns its handle."""
        x = self._alloc(k)
        self.next[x] = self.next[_NIL]
        self.prev[self.next[_NIL]] = x
        self.next[_NIL] = x
        self.prev[x] = _NIL
        return x

    def search(self, k: int) -> int | None:
        """LIST-SEARCH(L, k): handle of the first node with key k."""
        x = self.next[_NIL]
        while x != _NIL and self.key[x] != k:
            x = self.next[x]
        return None if x == _NIL else x

    def delete(self, x: int) -> None:
        """LIST-DELETE(L, x): O(1) given a handle (the sentinel removes boundary cases)."""
        assert x != _NIL
        self.next[self.prev[x]] = self.next[x]
        self.prev[self.next[x]] = self.prev[x]
        self.free.append(x)

    def to_vec(self) -> list[int]:
        v: list[int] = []
        x = self.next[_NIL]
        while x != _NIL:
            v.append(self.key[x])
            x = self.next[x]
        return v


@dataclass
class Node:
    """Singly linked list node."""

    key: int
    next: Node | None = None


def from_slice(a: list[int]) -> Node | None:
    head: Node | None = None
    for k in reversed(a):
        head = Node(k, head)
    return head


def to_vec(h: Node | None) -> list[int]:
    v: list[int] = []
    while h is not None:
        v.append(h.key)
        h = h.next
    return v


def reverse(head: Node | None) -> Node | None:
    """Exercise 10.2-7: reverse a singly linked list in Theta(n) time with O(1) extra storage."""
    prev: Node | None = None
    while head is not None:
        nxt = head.next
        head.next = prev
        prev = head
        head = nxt
    return prev


def _dbg(v: int | None) -> str:
    return "None" if v is None else f"Some({v})"


def demo() -> str:
    lines: list[str] = []
    l = LinkedList()
    for k in [9, 16, 4, 1]:
        l.insert(k)
    lines.append(f"after inserting 9,16,4,1 at the head: {l.to_vec()}")
    x = l.search(4)
    assert x is not None
    l.delete(x)
    lines.append(f"after deleting key 4 via its handle:   {l.to_vec()}")
    lines.append(f"search(99) = {_dbg(l.search(99))}")
    s = from_slice([1, 2, 3, 4])
    before = to_vec(s)
    lines.append(f"singly list {before} reversed -> {to_vec(reverse(s))}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_insert_search_delete(self) -> None:
        l = LinkedList()
        hs = [l.insert(k) for k in range(1, 6)]
        self.assertEqual(l.to_vec(), [5, 4, 3, 2, 1])
        l.delete(hs[2])  # key 3
        l.delete(hs[4])  # key 5 (head)
        l.delete(hs[0])  # key 1 (tail)
        self.assertEqual(l.to_vec(), [4, 2])
        self.assertIsNone(l.search(3))
        self.assertIsNotNone(l.search(2))
        h = l.insert(7)  # reuses a freed slot
        self.assertEqual(l.to_vec(), [7, 4, 2])
        self.assertEqual(l.search(7), h)

    def test_empty_list(self) -> None:
        l = LinkedList()
        self.assertEqual(l.to_vec(), [])
        self.assertIsNone(l.search(1))

    def test_reverse_singly(self) -> None:
        self.assertEqual(to_vec(reverse(from_slice([1, 2, 3, 4, 5]))), [5, 4, 3, 2, 1])
        self.assertEqual(to_vec(reverse(None)), [])


if __name__ == "__main__":
    print(demo(), end="")
