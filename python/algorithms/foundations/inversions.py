"""Counting Inversions — CLRS Problem 2-4 (1e–4e).

Count out-of-order pairs in Θ(n lg n) by augmenting merge sort.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def count_inversions_naive(a: list) -> int:
    """Brute-force Θ(n²) count of pairs i < j with a[i] > a[j]."""
    c = 0
    for i in range(len(a)):
        for j in range(i + 1, len(a)):
            if a[i] > a[j]:
                c += 1
    return c


def _sort_count(a: list, lo: int, hi: int) -> int:
    if hi - lo < 2:
        return 0
    mid = (lo + hi) // 2
    inv = _sort_count(a, lo, mid) + _sort_count(a, mid, hi)
    left = a[lo:mid]
    right = a[mid:hi]
    i, j = 0, 0
    for k in range(lo, hi):
        if j >= len(right) or (i < len(left) and left[i] <= right[j]):
            a[k] = left[i]
            i += 1
        else:
            inv += len(left) - i
            a[k] = right[j]
            j += 1
    return inv


def count_inversions(a: list) -> int:
    """Θ(n lg n) count; sorts a copy while merging."""
    v = list(a)
    return _sort_count(v, 0, len(v))


def demo() -> str:
    lines: list[str] = []
    a = [2, 3, 8, 6, 1]
    lines.append(f"A = {a}")
    lines.append(f"naive inversions     = {count_inversions_naive(a)}")
    lines.append(f"merge-sort inversions = {count_inversions(a)}")
    r = list(reversed(range(10)))
    lines.append(f"reverse of 0..10 has {count_inversions(r)} inversions (= 10·9/2)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_example(self) -> None:
        self.assertEqual(count_inversions([2, 3, 8, 6, 1]), 5)

    def test_matches_naive(self) -> None:
        r = Rng(5)
        for n in range(60):
            v = r.vec(n, 0, 15)
            self.assertEqual(count_inversions(v), count_inversions_naive(v))

    def test_extremes(self) -> None:
        n = 20
        self.assertEqual(count_inversions(list(range(n))), 0)
        self.assertEqual(count_inversions(list(reversed(range(n)))), n * (n - 1) // 2)


if __name__ == "__main__":
    print(demo(), end="")
