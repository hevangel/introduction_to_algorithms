"""Building a Heap by Insertion vs. BUILD-MAX-HEAP — CLRS Problem 6-1 (1e–4e).

BUILD-MAX-HEAP′ inserts elements one by one: Θ(n lg n) worst case, and it can produce a different heap than the Θ(n) bottom-up build.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def build_by_insertion(a: list[int]) -> int:
    """BUILD-MAX-HEAP′: repeatedly MAX-HEAP-INSERT (sift each new element up). Returns swap count."""
    swaps = 0
    for i in range(1, len(a)):
        j = i
        while j > 0:
            p = (j - 1) // 2
            if a[p] < a[j]:
                a[j], a[p] = a[p], a[j]
                j = p
                swaps += 1
            else:
                break
    return swaps


def build_by_heapify(a: list[int]) -> int:
    """BUILD-MAX-HEAP (Floyd): sift down from the last internal node. Returns swap count (≤ n)."""
    n = len(a)
    swaps = 0
    for start in range(n // 2 - 1, -1, -1):
        i = start
        while True:
            l, r = 2 * i + 1, 2 * i + 2
            m = i
            if l < n and a[l] > a[m]:
                m = l
            if r < n and a[r] > a[m]:
                m = r
            if m == i:
                break
            a[i], a[m] = a[m], a[i]
            swaps += 1
            i = m
    return swaps


def is_max_heap(a: list[int]) -> bool:
    return all(a[(i - 1) // 2] >= a[i] for i in range(1, len(a)))


def demo() -> str:
    lines: list[str] = []
    x, y = [1, 2, 3], [1, 2, 3]
    build_by_insertion(x)
    build_by_heapify(y)
    lines.append(f"input [1,2,3]: insertion build → {x}, bottom-up build → {y} (different heaps!)")
    for k in [8, 12, 16]:
        n = 1 << k
        asc = list(range(n))
        s1 = build_by_insertion(list(asc))
        s2 = build_by_heapify(list(asc))
        lines.append(
            f"n = {n:>6} ascending input: insertion {s1:>8} swaps (≈ n lg n = {n * k}), "
            f"bottom-up {s2:>6} swaps (≤ n)"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_both_build_valid_heaps_but_not_always_the_same(self) -> None:
        r = Rng(309)
        differ = 0
        for n in range(100):
            v = r.vec(n, -50, 50)
            a, b = list(v), list(v)
            build_by_insertion(a)
            build_by_heapify(b)
            self.assertTrue(is_max_heap(a) and is_max_heap(b))
            if a != b:
                differ += 1
        self.assertGreater(differ, 10)
        x, y = [1, 2, 3], [1, 2, 3]
        build_by_insertion(x)
        build_by_heapify(y)
        self.assertEqual((x, y), ([3, 1, 2], [3, 2, 1]))

    def test_worst_case_gap(self) -> None:
        n = 1 << 12
        asc = list(range(n))
        s1 = build_by_insertion(list(asc))
        s2 = build_by_heapify(list(asc))
        self.assertTrue(s1 > 3 * s2 and s2 <= n, f"{s1} {s2}")


if __name__ == "__main__":
    print(demo(), end="")
