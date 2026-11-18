"""Fibonacci Heaps — CLRS 20 (1e, 2e) / 19 (3e) (Editions: 1,2,3).

Mergeable heap with amortised O(1) insert, union and decrease-key and O(lg n) extract-min.
"""

from __future__ import annotations

from dataclasses import dataclass
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

NIL = -1


@dataclass
class _Node:
    key: int
    degree: int = 0
    mark: bool = False
    p: int = NIL
    child: int = NIL
    left: int = 0
    right: int = 0


class FibonacciHeap:
    """Fibonacci min-heap in an arena. A handle is the arena index returned by insert()."""

    def __init__(self) -> None:
        self._n: list[_Node] = []
        self._min: int = NIL
        self._count: int = 0

    def __len__(self) -> int:
        return self._count

    def is_empty(self) -> bool:
        return self._count == 0

    def key(self, h: int) -> int:
        return self._n[h].key

    def minimum(self) -> int | None:
        """FIB-HEAP-MINIMUM."""
        return None if self._min == NIL else self._n[self._min].key

    def _splice_after(self, m: int, x: int) -> None:
        r = self._n[m].right
        self._n[x].left = m
        self._n[x].right = r
        self._n[m].right = x
        self._n[r].left = x

    def _add_root(self, x: int) -> None:
        self._n[x].p = NIL
        if self._min == NIL:
            self._n[x].left = x
            self._n[x].right = x
            self._min = x
        else:
            self._splice_after(self._min, x)
            if self._n[x].key < self._n[self._min].key:
                self._min = x

    def insert(self, key: int) -> int:
        """FIB-HEAP-INSERT: O(1) — add a new single-node tree to root list."""
        x = len(self._n)
        self._n.append(_Node(key=key, left=x, right=x))
        self._add_root(x)
        self._count += 1
        return x

    def union(self, other: FibonacciHeap) -> int:
        """FIB-HEAP-UNION: O(1) concatenation of root lists. Handles of `other` shift by offset."""
        off = len(self._n)

        def shift(i: int) -> int:
            return NIL if i == NIL else i + off

        for nd in other._n:
            self._n.append(
                _Node(
                    key=nd.key,
                    degree=nd.degree,
                    mark=nd.mark,
                    p=shift(nd.p),
                    child=shift(nd.child),
                    left=shift(nd.left),
                    right=shift(nd.right),
                )
            )
        omin = shift(other._min)
        if omin != NIL:
            if self._min == NIL:
                self._min = omin
            else:
                a, b = self._min, omin
                ar, bl = self._n[a].right, self._n[b].left
                self._n[a].right = b
                self._n[b].left = a
                self._n[bl].right = ar
                self._n[ar].left = bl
                if self._n[b].key < self._n[a].key:
                    self._min = b
        self._count += other._count
        return off

    def extract_min(self) -> tuple[int, int] | None:
        """FIB-HEAP-EXTRACT-MIN: returns (handle, key). Amortised O(lg n)."""
        z = self._min
        if z == NIL:
            return None
        if self._n[z].child != NIL:
            start = self._n[z].child
            kids = [start]
            c = self._n[start].right
            while c != start:
                kids.append(c)
                c = self._n[c].right
            for k in kids:
                self._n[k].p = NIL
                self._splice_after(z, k)
            self._n[z].child = NIL
        l, r = self._n[z].left, self._n[z].right
        if r == z:
            self._min = NIL
        else:
            self._n[l].right = r
            self._n[r].left = l
            self._min = r
            self._consolidate()
        self._count -= 1
        return (z, self._n[z].key)

    def _link(self, y: int, x: int) -> None:
        self._n[y].p = x
        self._n[y].mark = False
        if self._n[x].child == NIL:
            self._n[x].child = y
            self._n[y].left = y
            self._n[y].right = y
        else:
            self._splice_after(self._n[x].child, y)
        self._n[x].degree += 1

    def _consolidate(self) -> None:
        roots = [self._min]
        w = self._n[self._min].right
        while w != self._min:
            roots.append(w)
            w = self._n[w].right
        a = [NIL] * 64
        for w in roots:
            x = w
            d = self._n[x].degree
            while a[d] != NIL:
                y = a[d]
                if self._n[x].key > self._n[y].key:
                    x, y = y, x
                self._link(y, x)
                a[d] = NIL
                d += 1
            a[d] = x
        self._min = NIL
        for x in a:
            if x != NIL:
                self._add_root(x)

    def _cut(self, x: int, y: int) -> None:
        if self._n[x].right == x:
            self._n[y].child = NIL
        else:
            l, r = self._n[x].left, self._n[x].right
            self._n[l].right = r
            self._n[r].left = l
            if self._n[y].child == x:
                self._n[y].child = r
        self._n[y].degree -= 1
        self._n[x].mark = False
        self._add_root(x)

    def _cascading_cut(self, y: int) -> None:
        while True:
            z = self._n[y].p
            if z == NIL:
                return
            if not self._n[y].mark:
                self._n[y].mark = True
                return
            self._cut(y, z)
            y = z

    def decrease_key(self, x: int, k: int) -> None:
        """FIB-HEAP-DECREASE-KEY: amortised O(1)."""
        if k > self._n[x].key:
            raise ValueError("new key is greater than current key")
        self._n[x].key = k
        y = self._n[x].p
        if y != NIL and self._n[x].key < self._n[y].key:
            self._cut(x, y)
            self._cascading_cut(y)
        if self._n[x].key < self._n[self._min].key:
            self._min = x

    def delete(self, x: int) -> None:
        """FIB-HEAP-DELETE: decrease to -infinity then extract the minimum."""
        self.decrease_key(x, -(1 << 63))
        self.extract_min()

    def max_root_degree(self) -> int:
        """Largest degree among roots (bounded by D(n) = floor(log_phi n))."""
        if self._min == NIL:
            return 0
        m = self._n[self._min].degree
        w = self._n[self._min].right
        while w != self._min:
            m = max(m, self._n[w].degree)
            w = self._n[w].right
        return m


def demo() -> str:
    lines: list[str] = []
    h = FibonacciHeap()
    hs = [h.insert(k) for k in [23, 7, 21, 3, 18, 52, 38, 39, 41, 17, 30, 24, 26, 46, 35]]
    lines.append(f"minimum = {h.minimum()} (n = {len(h)})")
    item = h.extract_min()
    lines.append(f"extract-min -> {item[1] if item is not None else None}")
    lines.append(f"max root degree after consolidation = {h.max_root_degree()}")
    h.decrease_key(hs[8], 1)
    lines.append(f"decrease-key(41 -> 1): minimum = {h.minimum()}")
    h.delete(hs[5])
    order: list[int] = []
    while (it := h.extract_min()) is not None:
        order.append(it[1])
    lines.append(f"remaining keys in order: {order}")
    a, b = FibonacciHeap(), FibonacciHeap()
    for k in [5, 9, 2]:
        a.insert(k)
    for k in [4, 1, 8]:
        b.insert(k)
    a.union(b)
    lines.append(f"union of two heaps: minimum = {a.minimum()}, size = {len(a)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_heapsort_via_extract_min(self) -> None:
        r = Rng(80)
        v = r.vec(500, -300, 300)
        h = FibonacciHeap()
        for k in v:
            h.insert(k)
        out: list[int] = []
        while (it := h.extract_min()) is not None:
            out.append(it[1])
        self.assertEqual(out, sorted(v))

    def test_random_decrease_key_and_delete_match_model(self) -> None:
        r = Rng(81)
        h = FibonacciHeap()
        alive: list[int] = []
        for round_idx in range(3000):
            op = r.range(0, 4)
            if op in (0, 1):
                alive.append(h.insert(r.range(0, 10_000)))
            elif op == 2 and alive:
                i = r.range(0, len(alive) - 1)
                nk = h.key(alive[i]) - r.range(0, 500)
                h.decrease_key(alive[i], nk)
            elif op == 3 and alive:
                item = h.extract_min()
                assert item is not None
                hd, k = item
                model_min = min(h.key(a) for a in alive)
                self.assertEqual(k, min(model_min, k))
                alive = [a for a in alive if a != hd]
            elif alive:
                i = r.range(0, len(alive) - 1)
                hd = alive[i]
                alive[i] = alive[-1]
                alive.pop()
                h.delete(hd)
            self.assertEqual(len(h), len(alive), f"round {round_idx}")
            expected_min = min((h.key(a) for a in alive), default=None)
            self.assertEqual(h.minimum(), expected_min)

    def test_degree_bound_and_union(self) -> None:
        h = FibonacciHeap()
        for k in range(4096):
            h.insert(k)
        h.extract_min()
        phi = (1.0 + math.sqrt(5.0)) / 2.0
        self.assertLessEqual(h.max_root_degree(), math.log(4096.0) / math.log(phi))
        a, b = FibonacciHeap(), FibonacciHeap()
        for k in [10, 20, 30]:
            a.insert(k)
        for k in [5, 25]:
            b.insert(k)
        a.union(b)
        out: list[int] = []
        while (it := a.extract_min()) is not None:
            out.append(it[1])
        self.assertEqual(out, [5, 10, 20, 25, 30])

    def test_decrease_key_rejects_larger_key(self) -> None:
        h = FibonacciHeap()
        x = h.insert(5)
        with self.assertRaises(ValueError):
            h.decrease_key(x, 9)


if __name__ == "__main__":
    print(demo(), end="")
