"""Merge Sort — CLRS 2.3 (1e–4e).

Divide-and-conquer sort: split, sort halves recursively, merge in linear time.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def merge(a: list, p: int, q: int, r: int) -> None:
    """MERGE(A, p, q, r): merges sorted A[p..=q] and A[q+1..=r]."""
    left = a[p : q + 1]
    right = a[q + 1 : r + 1]
    i, j = 0, 0
    for k in range(p, r + 1):
        if j >= len(right) or (i < len(left) and left[i] <= right[j]):
            a[k] = left[i]
            i += 1
        else:
            a[k] = right[j]
            j += 1


def _merge_sort_range(a: list, p: int, r: int) -> None:
    if p < r:
        q = (p + r) // 2
        _merge_sort_range(a, p, q)
        _merge_sort_range(a, q + 1, r)
        merge(a, p, q, r)


def merge_sort(a: list) -> None:
    """MERGE-SORT(A, 1, n)."""
    if len(a) > 1:
        _merge_sort_range(a, 0, len(a) - 1)


def merge_sort_bottom_up(a: list) -> None:
    """Bottom-up (iterative) merge sort — no recursion."""
    n = len(a)
    w = 1
    while w < n:
        p = 0
        while p + w < n:
            q = p + w - 1
            r = min(p + 2 * w - 1, n - 1)
            merge(a, p, q, r)
            p += 2 * w
        w *= 2


def demo() -> str:
    lines: list[str] = []
    a = [38, 27, 43, 3, 9, 82, 10]
    lines.append(f"input         : {a}")
    b = list(a)
    merge_sort(a)
    lines.append(f"merge_sort    : {a}")
    merge_sort_bottom_up(b)
    lines.append(f"bottom-up     : {b}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_both_variants_sort(self) -> None:
        r = Rng(3)
        for n in range(70):
            v = r.vec(n, -20, 20)
            e = sorted(v)
            a, b = list(v), list(v)
            merge_sort(a)
            merge_sort_bottom_up(b)
            self.assertEqual(a, e)
            self.assertEqual(b, e)

    def test_merge_combines_two_sorted_runs(self) -> None:
        a = [1, 4, 9, 2, 3, 10]
        merge(a, 0, 2, 5)
        self.assertEqual(a, [1, 2, 3, 4, 9, 10])


if __name__ == "__main__":
    print(demo(), end="")
