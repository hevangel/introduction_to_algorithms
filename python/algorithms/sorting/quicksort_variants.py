"""Quicksort Variants — CLRS Problems 7-2, 7-4, 7-5 (2e–4e).

Three-way partitioning for equal keys, tail-recursion elimination with O(lg n) stack, median-of-3 pivots.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng, is_sorted


def quicksort_three_way(a: list) -> None:
    """Problem 7-2: three-way partition (Dutch national flag). Equal keys are never recursed on."""

    def sort_range(lo: int, hi: int) -> None:
        if hi - lo <= 1:
            return
        x = a[lo + (hi - lo) // 2]
        lt, i, gt = lo, lo, hi
        while i < gt:
            if a[i] < x:
                a[lt], a[i] = a[i], a[lt]
                lt += 1
                i += 1
            elif a[i] > x:
                gt -= 1
                a[i], a[gt] = a[gt], a[i]
            else:
                i += 1
        sort_range(lo, lt)
        sort_range(gt, hi)

    sort_range(0, len(a))


def _lomuto(a: list, lo: int, hi: int) -> int:
    r = hi - 1
    pivot = a[r]
    i = lo
    for j in range(lo, r):
        if a[j] <= pivot:
            a[i], a[j] = a[j], a[i]
            i += 1
    a[i], a[r] = a[r], a[i]
    return i


def quicksort_tail_recursive(a: list) -> int:
    """Problem 7-4: TAIL-RECURSIVE-QUICKSORT — recurse into smaller part, loop on larger."""
    max_depth = 0

    def go(lo: int, hi: int, depth: int) -> None:
        nonlocal max_depth
        if depth > max_depth:
            max_depth = depth
        while hi - lo > 1:
            q = _lomuto(a, lo, hi)
            left_len = q - lo
            right_len = hi - (q + 1)
            if left_len < right_len:
                go(lo, q, depth + 1)
                lo = q + 1
            else:
                go(q + 1, hi, depth + 1)
                hi = q

    go(0, len(a), 1)
    return max_depth


def quicksort_median_of_three(a: list) -> None:
    """Problem 7-5: median-of-3 pivot selection."""

    def sort_range(lo: int, hi: int) -> None:
        if hi - lo <= 1:
            return
        mid = lo + (hi - lo) // 2
        last = hi - 1
        if (a[lo] <= a[mid]) == (a[mid] <= a[last]):
            m = mid
        elif (a[mid] <= a[lo]) == (a[lo] <= a[last]):
            m = lo
        else:
            m = last
        a[m], a[last] = a[last], a[m]
        q = _lomuto(a, lo, hi)
        sort_range(lo, q)
        sort_range(q + 1, hi)

    sort_range(0, len(a))


def demo() -> str:
    lines: list[str] = []
    a = [3, 1, 3, 2, 3, 1, 2, 3, 3]
    quicksort_three_way(a)
    lines.append(f"three-way on many duplicates: {a}")
    big = list(range(4000))
    depth = quicksort_tail_recursive(big)
    lines.append(f"tail-recursive quicksort on 4000 sorted keys: max stack depth = {depth} (lg n ≈ 12)")
    c = list(reversed(range(50)))
    quicksort_median_of_three(c)
    lines.append(f"median-of-3 on reverse input sorted: {is_sorted(c)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_all_variants_sort(self) -> None:
        r = Rng(31)
        for n in range(90):
            v = r.vec(n, -4, 4)
            e = sorted(v)
            a, b, c = list(v), list(v), list(v)
            quicksort_three_way(a)
            quicksort_tail_recursive(b)
            quicksort_median_of_three(c)
            self.assertEqual(a, e)
            self.assertEqual(b, e)
            self.assertEqual(c, e)

    def test_stack_depth_is_logarithmic_on_worst_case_input(self) -> None:
        v = list(range(4000))
        d = quicksort_tail_recursive(v)
        self.assertLessEqual(d, 20, f"depth {d}")

    def test_three_way_handles_all_equal(self) -> None:
        v = [7] * 100_000
        quicksort_three_way(v)
        self.assertTrue(all(x == 7 for x in v))


if __name__ == "__main__":
    print(demo(), end="")
