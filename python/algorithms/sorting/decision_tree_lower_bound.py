"""Comparison-Sort Lower Bound — CLRS 8.1 (1e–4e).

Any comparison sort needs ⌈lg n!⌉ comparisons in the worst case: measure real sorts against the decision-tree bound.
"""

from __future__ import annotations

from collections.abc import Callable
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

_COUNT = 0


class Counted:
    """A key whose every ordering comparison is counted — turns any generic sort into a decision-tree probe."""

    def __init__(self, val: int) -> None:
        self.val = val

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Counted):
            return NotImplemented
        global _COUNT
        _COUNT += 1
        return self.val == other.val

    def __lt__(self, other: Counted) -> bool:
        global _COUNT
        _COUNT += 1
        return self.val < other.val

    def __le__(self, other: Counted) -> bool:
        global _COUNT
        _COUNT += 1
        return self.val <= other.val

    def __gt__(self, other: Counted) -> bool:
        global _COUNT
        _COUNT += 1
        return self.val > other.val

    def __ge__(self, other: Counted) -> bool:
        global _COUNT
        _COUNT += 1
        return self.val >= other.val


def all_permutations(n: int) -> list[list[int]]:
    if n == 0:
        return [[]]
    out: list[list[int]] = []
    for p in all_permutations(n - 1):
        for pos in range(len(p) + 1):
            q = list(p)
            q.insert(pos, n - 1)
            out.append(q)
    return out


def lower_bound(n: int) -> int:
    """⌈lg(n!)⌉: the minimum height of a binary tree with n! leaves."""
    f = math.factorial(n)
    return (f - 1).bit_length() if f > 1 else 0


def measure(sort_fn: Callable[[list[Counted]], None], n: int) -> tuple[int, float]:
    """(worst-case comparisons, average comparisons) of `sort_fn` over all n! input orders."""
    global _COUNT
    perms = all_permutations(n)
    worst = 0
    total = 0
    for p in perms:
        v = [Counted(x) for x in p]
        _COUNT = 0
        sort_fn(v)
        c = _COUNT
        assert all(v[i].val < v[i + 1].val for i in range(len(v) - 1))
        if c > worst:
            worst = c
        total += c
    return worst, total / len(perms)


def _insertion_sort(a: list[Counted]) -> None:
    for j in range(1, len(a)):
        key = a[j]
        i = j
        while i > 0 and a[i - 1] > key:
            a[i] = a[i - 1]
            i -= 1
        a[i] = key


def _merge_sort(a: list[Counted]) -> None:
    def sort_range(p: int, r: int) -> None:
        if p < r:
            q = (p + r) // 2
            sort_range(p, q)
            sort_range(q + 1, r)
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

    if len(a) > 1:
        sort_range(0, len(a) - 1)


def _heapsort(a: list[Counted]) -> None:
    def max_heapify(i: int, heap_size: int) -> None:
        while True:
            l, r = 2 * i + 1, 2 * i + 2
            largest = i
            if l < heap_size and a[l] > a[largest]:
                largest = l
            if r < heap_size and a[r] > a[largest]:
                largest = r
            if largest == i:
                return
            a[i], a[largest] = a[largest], a[i]
            i = largest

    n = len(a)
    for i in range(n // 2 - 1, -1, -1):
        max_heapify(i, n)
    size = n
    while size > 1:
        a[0], a[size - 1] = a[size - 1], a[0]
        size -= 1
        max_heapify(0, size)


def _quicksort(a: list[Counted]) -> None:
    def sort_range(lo: int, hi: int) -> None:
        if hi - lo > 1:
            r = hi - 1
            i = lo
            for j in range(lo, r):
                if a[j] <= a[r]:
                    a[i], a[j] = a[j], a[i]
                    i += 1
            a[i], a[r] = a[r], a[i]
            sort_range(lo, i)
            sort_range(i + 1, hi)

    sort_range(0, len(a))


def demo() -> str:
    lines: list[str] = []
    lines.append("  n | ⌈lg n!⌉ | insertion w/avg | merge w/avg | heapsort w/avg | quicksort w/avg")
    for n in range(2, 8):
        ins = measure(_insertion_sort, n)
        mrg = measure(_merge_sort, n)
        hp = measure(_heapsort, n)
        qs = measure(_quicksort, n)
        lines.append(
            f"{n:>3} | {lower_bound(n):>7} | {ins[0]:>6} {ins[1]:>7.2f} | "
            f"{mrg[0]:>5} {mrg[1]:>6.2f} | {hp[0]:>6} {hp[1]:>7.2f} | {qs[0]:>7} {qs[1]:>7.2f}"
        )
    lines.append("lg(n!) = n lg n − n lg e + O(lg n)  ⇒  Ω(n lg n); 2^h ≥ n! forces height h ≥ lg n!")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_bound_values(self) -> None:
        self.assertEqual([lower_bound(n) for n in range(1, 9)], [0, 1, 3, 5, 7, 10, 13, 16])

    def test_no_comparison_sort_beats_the_decision_tree_bound(self) -> None:
        sorts = [_insertion_sort, _merge_sort, _heapsort, _quicksort]
        for n in range(2, 8):
            lg_fact = sum(math.log2(i) for i in range(1, n + 1))
            for s in sorts:
                worst, avg = measure(s, n)
                self.assertGreaterEqual(worst, lower_bound(n), f"worst case n={n}")
                self.assertGreaterEqual(avg, lg_fact - 1e-9, f"average case n={n}: {avg} < {lg_fact}")

    def test_merge_sort_is_tight_for_four_elements(self) -> None:
        self.assertEqual(measure(_merge_sort, 4)[0], 5)


if __name__ == "__main__":
    print(demo(), end="")
