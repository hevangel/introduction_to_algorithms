"""Randomized Quicksort — CLRS 7.3–7.4 (2e–4e; 1e had a variant in 8.x) (1,2,3,4).

Pick the pivot uniformly at random: expected Θ(n lg n) on every input.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _partition(a: list, lo: int, hi: int) -> int:
    pivot = a[hi]
    i = lo
    for j in range(lo, hi):
        if a[j] <= pivot:
            a[i], a[j] = a[j], a[i]
            i += 1
    a[i], a[hi] = a[hi], a[i]
    return i


def randomized_partition(a: list, rng: Rng, lo: int = 0, hi: int | None = None) -> int:
    """RANDOMIZED-PARTITION: swap a random element into the pivot slot, then PARTITION."""
    if hi is None:
        hi = len(a) - 1
    i = rng.range(lo, hi)
    a[i], a[hi] = a[hi], a[i]
    return _partition(a, lo, hi)


def randomized_quicksort(a: list, rng: Rng) -> None:
    """RANDOMIZED-QUICKSORT (iterative stack to handle arbitrary sizes)."""
    if len(a) <= 1:
        return
    stack = [(0, len(a) - 1)]
    while stack:
        lo, hi = stack.pop()
        if lo < hi:
            q = randomized_partition(a, rng, lo, hi)
            if q - 1 - lo > hi - (q + 1):
                stack.append((lo, q - 1))
                stack.append((q + 1, hi))
            else:
                stack.append((q + 1, hi))
                stack.append((lo, q - 1))


def comparisons(n: int, rng: Rng, sorted_input: bool) -> int:
    """Count comparisons made by randomized quicksort."""
    v = list(range(n))
    if not sorted_input:
        rng.shuffle(v)
    c = 0
    stack = [(0, n - 1)] if n > 1 else []
    while stack:
        lo, hi = stack.pop()
        if lo >= hi:
            continue
        i = rng.range(lo, hi)
        v[i], v[hi] = v[hi], v[i]
        k = lo
        pivot = v[hi]
        for j in range(lo, hi):
            c += 1
            if v[j] <= pivot:
                v[k], v[j] = v[j], v[k]
                k += 1
        v[k], v[hi] = v[hi], v[k]
        stack.append((lo, k - 1))
        stack.append((k + 1, hi))
    return c


def demo() -> str:
    lines: list[str] = []
    r = Rng(5)
    a = [9, 4, 7, 1, 8, 2, 6, 3, 5]
    randomized_quicksort(a, r)
    lines.append(f"sorted: {a}")
    for n in [100, 1000, 10000]:
        c = comparisons(n, r, True)
        lines.append(
            f"n={n:>5}: comparisons on already-sorted input = {c:>7}  "
            f"(2n ln n ≈ {2.0 * n * math.log(n):.0f}, n²/2 = {n * n // 2})"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_sorts(self) -> None:
        r = Rng(29)
        for n in range(80):
            v = r.vec(n, -30, 30)
            e = sorted(v)
            randomized_quicksort(v, r)
            self.assertEqual(v, e)

    def test_sorted_input_is_not_quadratic(self) -> None:
        r = Rng(30)
        n = 5000
        c = comparisons(n, r, True)
        self.assertLess(c, 4.0 * n * math.log(n))


if __name__ == "__main__":
    print(demo(), end="")
