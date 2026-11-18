"""Hoare Partition Quicksort — CLRS 7.1 (1e); Problem 7-1 (2e–4e).

The original two-pointer partition scheme: fewer swaps than Lomuto.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def hoare_partition(a: list, lo: int = 0, hi: int | None = None) -> int:
    """HOARE-PARTITION: pivot is a[lo]. Returns j with a[lo..=j] <= pivot <= a[j+1..hi] and j < hi - 1."""
    if hi is None:
        hi = len(a)
    x = a[lo]
    i = lo - 1
    j = hi
    while True:
        while True:
            j -= 1
            if a[j] <= x:
                break
        while True:
            i += 1
            if a[i] >= x:
                break
        if i < j:
            a[i], a[j] = a[j], a[i]
        else:
            return j


def hoare_quicksort(a: list) -> None:
    """QUICKSORT using HOARE-PARTITION (recurse on A[lo..=q] and A[q+1..hi])."""

    def sort_range(lo: int, hi: int) -> None:
        if hi - lo > 1:
            q = hoare_partition(a, lo, hi)
            sort_range(lo, q + 1)
            sort_range(q + 1, hi)

    sort_range(0, len(a))


def demo() -> str:
    lines: list[str] = []
    a = [13, 19, 9, 5, 12, 8, 7, 4, 11, 2, 6, 21]
    lines.append(f"input    : {a}")
    b = list(a)
    q = hoare_partition(b)
    lines.append(f"partition: {b} split after index {q} (pivot value 13)")
    hoare_quicksort(a)
    lines.append(f"sorted   : {a}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_partition_splits_correctly(self) -> None:
        r = Rng(27)
        for n in range(2, 50):
            v = r.vec(n, -10, 10)
            pivot = v[0]
            q = hoare_partition(v)
            self.assertLess(q, n - 1)
            lmax = max(v[: q + 1])
            rmin = min(v[q + 1 :])
            self.assertLessEqual(lmax, rmin)
            self.assertGreaterEqual(lmax, min(pivot, lmax))

    def test_sorts(self) -> None:
        r = Rng(28)
        for n in range(80):
            v = r.vec(n, -5, 5)
            e = sorted(v)
            hoare_quicksort(v)
            self.assertEqual(v, e)


if __name__ == "__main__":
    print(demo(), end="")
