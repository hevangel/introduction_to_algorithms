"""Implementing Pointers and Objects — CLRS 10.3 (1e-3e) (Editions: 1,2,3).

Multiple-array list representation with a free list: ALLOCATE-OBJECT / FREE-OBJECT,
plus compaction.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

NIL = -1


class MultiArrayList:
    """Doubly linked list stored in parallel arrays `key`, `next`, `prev`; unused cells form a free list."""

    def __init__(self, capacity: int) -> None:
        self.key: list[int] = [0] * capacity
        self.next: list[int] = [i + 1 if i + 1 < capacity else NIL for i in range(capacity)]
        self.prev: list[int] = [NIL] * capacity
        self.head: int = NIL
        self._free: int = 0 if capacity > 0 else NIL

    def allocate(self) -> int:
        """ALLOCATE-OBJECT: pop a cell from the free list (a stack)."""
        if self._free == NIL:
            raise MemoryError("out of space")
        x = self._free
        self._free = self.next[x]
        return x

    def free_object(self, x: int) -> None:
        """FREE-OBJECT: push a cell onto the free list."""
        self.next[x] = self._free
        self._free = x

    def insert(self, k: int) -> int:
        x = self.allocate()
        self.key[x] = k
        self.next[x] = self.head
        self.prev[x] = NIL
        if self.head != NIL:
            self.prev[self.head] = x
        self.head = x
        return x

    def delete(self, x: int) -> None:
        if self.prev[x] != NIL:
            p = self.prev[x]
            self.next[p] = self.next[x]
        else:
            self.head = self.next[x]
        if self.next[x] != NIL:
            n = self.next[x]
            self.prev[n] = self.prev[x]
        self.free_object(x)

    def to_list(self) -> list[int]:
        v: list[int] = []
        x = self.head
        while x != NIL:
            v.append(self.key[x])
            x = self.next[x]
        return v

    def used_cells(self) -> list[int]:
        """Cells currently in use, in list order."""
        v: list[int] = []
        x = self.head
        while x != NIL:
            v.append(x)
            x = self.next[x]
        return v

    def compact(self) -> None:
        """Exercise 10.3-4: relocate the m list objects to cells 0..m (keeping list order)."""
        cells = self.used_cells()
        cap = len(self.key)
        m = len(cells)
        keys = [self.key[c] for c in cells]
        for i in range(m):
            self.key[i] = keys[i]
            self.next[i] = i + 1 if i + 1 < m else NIL
            self.prev[i] = i - 1 if i > 0 else NIL
        self.head = 0 if m > 0 else NIL
        for i in range(m, cap):
            self.next[i] = i + 1 if i + 1 < cap else NIL
        self._free = m if m < cap else NIL


def demo() -> str:
    lines: list[str] = []
    l = MultiArrayList(8)
    h = [l.insert(k) for k in [9, 16, 4, 1]]
    lines.append(f"list = {l.to_list()}, cells in use (list order) = {l.used_cells()}")
    l.delete(h[1])
    lines.append(f"after deleting 16: list = {l.to_list()}, cells = {l.used_cells()}")
    l.compact()
    lines.append(f"after compaction : list = {l.to_list()}, cells = {l.used_cells()}")
    full = MultiArrayList(2)
    full.insert(1)
    full.insert(2)
    try:
        full.insert(3)
        msg = "ok"
    except MemoryError as e:
        msg = f"error: {e}"
    lines.append(f"third insert into capacity-2 list -> {msg}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_allocate_free_reuse(self) -> None:
        l = MultiArrayList(3)
        a = l.insert(1)
        _b = l.insert(2)
        _c = l.insert(3)
        with self.assertRaises(MemoryError):
            l.insert(4)
        l.delete(a)
        self.assertEqual(l.insert(9), a)
        self.assertEqual(l.to_list(), [9, 3, 2])

    def test_compaction_preserves_order(self) -> None:
        l = MultiArrayList(10)
        hs = [l.insert(k) for k in range(10)]
        for i in (hs[1], hs[4], hs[5], hs[8]):
            l.delete(i)
        before = l.to_list()
        l.compact()
        self.assertEqual(l.to_list(), before)
        self.assertEqual(l.used_cells(), list(range(len(before))))
        for k in range(4):
            l.insert(100 + k)
        with self.assertRaises(MemoryError):
            l.insert(0)


if __name__ == "__main__":
    print(demo(), end="")
