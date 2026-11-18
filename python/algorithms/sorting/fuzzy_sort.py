"""Fuzzy Sorting of Intervals — CLRS Problem 7-6 (2e–4e).

Order intervals so that points c₁ ≤ … ≤ cₙ with cᵢ ∈ Iᵢ exist: quicksort around the intersection of the pivot's overlapping intervals.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Interval = tuple[int, int]


def fuzzy_sort(a: list[Interval], rng: Rng) -> list[Interval]:
    """FUZZY-SORT: returns a permutation of intervals such that some non-decreasing point selection exists."""
    if len(a) <= 1:
        return list(a)
    pivot = a[rng.range(0, len(a) - 1)]
    lo, hi = pivot
    for x, y in a:
        if x <= hi and y >= lo:
            if x > lo:
                lo = x
            if y < hi:
                hi = y
    left: list[Interval] = []
    mid: list[Interval] = []
    right: list[Interval] = []
    for x, y in a:
        if y < lo:
            left.append((x, y))
        elif x > hi:
            right.append((x, y))
        else:
            mid.append((x, y))
    return fuzzy_sort(left, rng) + mid + fuzzy_sort(right, rng)


def is_fuzzy_sorted(a: list[Interval]) -> bool:
    """Is there a non-decreasing choice of points c_i ∈ I_i?"""
    c: float = -math.inf
    for x, y in a:
        if x > c:
            c = x
        if c > y:
            return False
    return True


def demo() -> str:
    lines: list[str] = []
    r = Rng(2)
    ivs = [(8, 10), (1, 3), (4, 6), (2, 5), (9, 12), (5, 7), (0, 1)]
    s = fuzzy_sort(ivs, r)
    lines.append(f"intervals : {ivs}")
    lines.append(f"fuzzy sort: {s}  (valid: {is_fuzzy_sorted(s)})")
    same = [(i % 7, 100 + i % 5) for i in range(100_000)]
    lines.append(
        f"100000 mutually overlapping intervals: all land in the middle bucket after one pass, "
        f"valid = {is_fuzzy_sorted(fuzzy_sort(same, r))}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_output_is_a_permutation_and_fuzzy_sorted(self) -> None:
        r = Rng(310)
        for _ in range(500):
            n = r.range(0, 30)
            ivs = [(a := r.range(0, 60), a + r.range(0, 12)) for _ in range(n)]
            s = fuzzy_sort(ivs, r)
            self.assertTrue(is_fuzzy_sorted(s), f"{ivs} -> {s}")
            self.assertEqual(sorted(ivs), sorted(s))

    def test_detector_and_degenerate_cases(self) -> None:
        self.assertFalse(is_fuzzy_sorted([(5, 6), (1, 2)]))
        self.assertTrue(is_fuzzy_sorted([(1, 5), (2, 6), (3, 4)]))
        r = Rng(1)
        self.assertEqual(fuzzy_sort([], r), [])
        all_overlap = [(i % 10, 50 + i % 3) for i in range(1000)]
        self.assertTrue(is_fuzzy_sorted(fuzzy_sort(all_overlap, r)))


if __name__ == "__main__":
    print(demo(), end="")
