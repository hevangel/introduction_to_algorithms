"""Quicksort — CLRS 7.1–7.2 (2e–4e; 7.1–7.2 with Hoare in 1e).

Partition around a pivot (Lomuto scheme) and recurse: Θ(n lg n) expected, Θ(n²) worst case.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng, is_sorted


def partition(a: list, lo: int = 0, hi: int | None = None) -> int:
    """PARTITION(A, p, r) with pivot A[r] (Lomuto). Returns the pivot's final index in `lo..hi`."""
    if hi is None:
        hi = len(a)
    r = hi - 1
    i = lo
    for j in range(lo, r):
        if a[j] <= a[r]:
            a[i], a[j] = a[j], a[i]
            i += 1
    a[i], a[r] = a[r], a[i]
    return i


def quicksort(a: list) -> None:
    """QUICKSORT(A, p, r)."""
    old_limit = sys.getrecursionlimit()
    if len(a) + 50 > old_limit:
        sys.setrecursionlimit(len(a) + 50)

    def sort_range(lo: int, hi: int) -> None:
        if hi - lo > 1:
            q = partition(a, lo, hi)
            sort_range(lo, q)
            sort_range(q + 1, hi)

    try:
        sort_range(0, len(a))
    finally:
        if len(a) + 50 > old_limit:
            sys.setrecursionlimit(old_limit)


def demo() -> str:
    lines: list[str] = []
    a = [2, 8, 7, 1, 3, 5, 6, 4]
    lines.append(f"input            : {a}")
    b = list(a)
    q = partition(b)
    lines.append(f"after PARTITION  : {b} (pivot 4 now at index {q})")
    quicksort(a)
    lines.append(f"quicksort result : {a}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_partition_invariant(self) -> None:
        a = [2, 8, 7, 1, 3, 5, 6, 4]
        q = partition(a)
        self.assertEqual(q, 3)
        self.assertTrue(all(x <= a[q] for x in a[:q]) and all(x > a[q] for x in a[q + 1 :]))

    def test_sorts_random_and_adversarial_inputs(self) -> None:
        r = Rng(26)
        for n in range(100):
            v = r.vec(n, -20, 20)
            e = sorted(v)
            quicksort(v)
            self.assertEqual(v, e)
        sorted_arr = list(range(2000))
        quicksort(sorted_arr)
        self.assertTrue(is_sorted(sorted_arr))


if __name__ == "__main__":
    print(demo(), end="")
