"""d-ary Heap — CLRS Problem 6-2 (1e–4e).

A heap whose nodes have d children: shallower tree, cheaper INSERT, costlier EXTRACT-MAX.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class DaryHeap:
    """d-ary max-heap of integer keys. Children of i are d·i+1 … d·i+d; parent is (i−1)//d."""

    def __init__(self, d: int) -> None:
        assert d >= 2
        self.d = d
        self.a: list[int] = []

    def height(self) -> int:
        """Height of a d-ary heap with n nodes: ⌈log_d(n(d−1)+1)⌉ − 1."""
        h = 0
        cap = 1
        level = 1
        while cap < len(self.a):
            level *= self.d
            cap += level
            h += 1
        return h

    def insert(self, key: int) -> None:
        self.a.append(key)
        i = len(self.a) - 1
        while i > 0:
            p = (i - 1) // self.d
            if self.a[p] < self.a[i]:
                self.a[i], self.a[p] = self.a[p], self.a[i]
                i = p
            else:
                break

    def extract_max(self) -> int | None:
        if not self.a:
            return None
        self.a[0], self.a[-1] = self.a[-1], self.a[0]
        m = self.a.pop()
        if self.a:
            self._max_heapify(0)
        return m

    def _max_heapify(self, i: int) -> None:
        n = len(self.a)
        while True:
            largest = i
            start = self.d * i + 1
            end = min(self.d * i + self.d + 1, n)
            for c in range(start, end):
                if self.a[c] > self.a[largest]:
                    largest = c
            if largest == i:
                return
            self.a[i], self.a[largest] = self.a[largest], self.a[i]
            i = largest

    def increase_key(self, i: int, key: int) -> None:
        assert key >= self.a[i]
        self.a[i] = key
        while i > 0:
            p = (i - 1) // self.d
            if self.a[p] < self.a[i]:
                self.a[i], self.a[p] = self.a[p], self.a[i]
                i = p
            else:
                break


def demo() -> str:
    lines: list[str] = []
    for d in [2, 3, 4, 8]:
        h = DaryHeap(d)
        for x in range(1000):
            h.insert((x * 7919) % 1009)
        lines.append(f"d={d}: height of 1000-node heap = {h.height()}")
    h = DaryHeap(3)
    for x in [5, 1, 9, 3, 7]:
        h.insert(x)
    out = []
    while (m := h.extract_max()) is not None:
        out.append(m)
    lines.append(f"3-ary heap drains as {out}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_all_arities_sort_descending(self) -> None:
        r = Rng(23)
        for d in range(2, 7):
            v = r.vec(150, -50, 50)
            h = DaryHeap(d)
            for x in v:
                h.insert(x)
            out = []
            while (m := h.extract_max()) is not None:
                out.append(m)
            self.assertEqual(out, sorted(v, reverse=True))

    def test_height_matches_formula(self) -> None:
        for d in range(2, 6):
            h = DaryHeap(d)
            for n in range(1, 200):
                h.insert(n)
                k = 0
                pw = 1
                while pw < n * (d - 1) + 1:
                    pw *= d
                    k += 1
                f = max(k - 1, 0)
                self.assertEqual(h.height(), f, f"d={d} n={n}")

    def test_increase_key_moves_to_top(self) -> None:
        h = DaryHeap(4)
        for x in range(20):
            h.insert(x)
        h.increase_key(19, 100)
        self.assertEqual(h.extract_max(), 100)


if __name__ == "__main__":
    print(demo(), end="")
