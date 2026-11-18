"""Counting Sort — CLRS 8.2 (1e–4e).

Stable Θ(n+k) sort for integer keys in 0..=k, using a prefix-sum table.
"""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def counting_sort_by_key(a: list, k: int, key: Callable) -> list:
    """Stable counting sort of arbitrary records by an integer key in `0..=k`."""
    c = [0] * (k + 1)
    for x in a:
        c[key(x)] += 1
    for i in range(1, k + 1):
        c[i] += c[i - 1]
    b: list = [None] * len(a)
    for x in reversed(a):
        kx = key(x)
        c[kx] -= 1
        b[c[kx]] = x
    return b


def counting_sort(a: list[int], k: int) -> list[int]:
    """COUNTING-SORT(A, B, k): stable sort of values in `0..=k`."""
    return counting_sort_by_key(a, k, lambda x: x)


class RangeCounter:
    """Exercise 8.2-4: after Θ(n+k) preprocessing, count elements in [lo, hi] in O(1)."""

    def __init__(self, a: list[int], k: int) -> None:
        c = [0] * (k + 2)
        for x in a:
            c[x + 1] += 1
        for i in range(1, len(c)):
            c[i] += c[i - 1]
        self._c = c

    def count(self, lo: int, hi: int) -> int:
        """Number of elements x with lo ≤ x ≤ hi."""
        return self._c[hi + 1] - self._c[lo]


def demo() -> str:
    lines: list[str] = []
    a = [2, 5, 3, 0, 2, 3, 0, 3]
    lines.append(f"A = {a}")
    lines.append(f"B = {counting_sort(a, 5)}")
    rc = RangeCounter(a, 5)
    lines.append(f"elements in [2,3] : {rc.count(2, 3)}")
    recs = [(2, "a"), (1, "b"), (2, "c"), (1, "d")]
    lines.append(f"stability check   : {counting_sort_by_key(recs, 2, lambda r: r[0])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_sorts_and_is_stable(self) -> None:
        r = Rng(33)
        v = r.vec(300, 0, 20)
        e = sorted(v)
        self.assertEqual(counting_sort(v, 20), e)
        recs = [(x, i) for i, x in enumerate(v)]
        out = counting_sort_by_key(recs, 20, lambda rec: rec[0])
        self.assertTrue(
            all(
                out[i][0] < out[i + 1][0] or (out[i][0] == out[i + 1][0] and out[i][1] < out[i + 1][1])
                for i in range(len(out) - 1)
            )
        )

    def test_range_counter(self) -> None:
        a = [2, 5, 3, 0, 2, 3, 0, 3]
        rc = RangeCounter(a, 5)
        self.assertEqual(rc.count(0, 5), 8)
        self.assertEqual(rc.count(2, 3), 5)
        self.assertEqual(rc.count(4, 4), 0)
        self.assertEqual(rc.count(0, 0), 2)


if __name__ == "__main__":
    print(demo(), end="")
