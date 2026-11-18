"""Priority Queue (Binary Heap) — CLRS 6.5 (1e–4e).

INSERT, MAXIMUM, EXTRACT-MAX, INCREASE-KEY and DELETE on a heap; also merging k sorted lists.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class PriorityQueue:
    """A max-priority queue over any comparable key, stored as a binary max-heap."""

    def __init__(self) -> None:
        self._a: list = []

    def __len__(self) -> int:
        return len(self._a)

    def is_empty(self) -> bool:
        return len(self._a) == 0

    def maximum(self):
        """HEAP-MAXIMUM, O(1)."""
        return self._a[0] if self._a else None

    def _sift_up(self, i: int) -> None:
        while i > 0:
            p = (i - 1) // 2
            if self._a[p] < self._a[i]:
                self._a[i], self._a[p] = self._a[p], self._a[i]
                i = p
            else:
                break

    def _sift_down(self, i: int) -> None:
        n = len(self._a)
        while True:
            l, r = 2 * i + 1, 2 * i + 2
            m = i
            if l < n and self._a[l] > self._a[m]:
                m = l
            if r < n and self._a[r] > self._a[m]:
                m = r
            if m == i:
                return
            self._a[i], self._a[m] = self._a[m], self._a[i]
            i = m

    def insert(self, x) -> None:
        """MAX-HEAP-INSERT, O(lg n)."""
        self._a.append(x)
        self._sift_up(len(self._a) - 1)

    def extract_max(self):
        """HEAP-EXTRACT-MAX, O(lg n)."""
        if not self._a:
            return None
        self._a[0], self._a[-1] = self._a[-1], self._a[0]
        m = self._a.pop()
        if self._a:
            self._sift_down(0)
        return m

    def increase_key(self, i: int, key) -> None:
        """HEAP-INCREASE-KEY(A, i, key): `key` must not be smaller than the current key."""
        if key < self._a[i]:
            raise ValueError("new key is smaller than current key")
        self._a[i] = key
        self._sift_up(i)

    def delete(self, i: int):
        """HEAP-DELETE(A, i) (Exercise 6.5-8), O(lg n)."""
        self._a[i], self._a[-1] = self._a[-1], self._a[i]
        x = self._a.pop()
        if i < len(self._a):
            self._sift_up(i)
            self._sift_down(i)
        return x


class _Reverse:
    def __init__(self, val: tuple[int, int, int]) -> None:
        self.val = val

    def __lt__(self, other: _Reverse) -> bool:
        return self.val > other.val

    def __gt__(self, other: _Reverse) -> bool:
        return self.val < other.val


def merge_sorted_lists(lists: list[list[int]]) -> list[int]:
    """Exercise 6.5-9: merge k sorted lists in O(n lg k) with a min-priority queue."""
    pq = PriorityQueue()
    for li, lst in enumerate(lists):
        if lst:
            pq.insert(_Reverse((lst[0], li, 0)))
    out: list[int] = []
    while (item := pq.extract_max()) is not None:
        v, li, pos = item.val
        out.append(v)
        if pos + 1 < len(lists[li]):
            pq.insert(_Reverse((lists[li][pos + 1], li, pos + 1)))
    return out


def demo() -> str:
    lines: list[str] = []
    pq = PriorityQueue()
    for x in [15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1]:
        pq.insert(x)
    lines.append(f"maximum            = {pq.maximum()}")
    pq.increase_key(8, 20)
    lines.append(f"after increase_key = {pq.maximum()}")
    drained = []
    while (m := pq.extract_max()) is not None:
        drained.append(m)
    lines.append(f"extract-max order  = {drained}")
    merged = merge_sorted_lists([[1, 4, 9], [2, 3, 10, 11], [], [0, 8]])
    lines.append(f"merge of 4 sorted lists = {merged}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_extracts_in_descending_order(self) -> None:
        r = Rng(22)
        v = r.vec(200, -100, 100)
        pq = PriorityQueue()
        for x in v:
            pq.insert(x)
        out = []
        while (m := pq.extract_max()) is not None:
            out.append(m)
        self.assertEqual(out, sorted(v, reverse=True))

    def test_increase_key_and_delete(self) -> None:
        pq = PriorityQueue()
        for x in [5, 3, 8, 1]:
            pq.insert(x)
        with self.assertRaises(ValueError):
            pq.increase_key(3, 0)
        pq.increase_key(3, 9)
        self.assertEqual(pq.maximum(), 9)
        d = pq.delete(0)
        self.assertEqual(d, 9)
        self.assertEqual(pq.maximum(), 8)
        self.assertEqual(len(pq), 3)

    def test_k_way_merge(self) -> None:
        lists = [[1, 5, 9], [2, 6], [0, 3, 4, 7, 8]]
        self.assertEqual(merge_sorted_lists(lists), list(range(10)))


if __name__ == "__main__":
    print(demo(), end="")
