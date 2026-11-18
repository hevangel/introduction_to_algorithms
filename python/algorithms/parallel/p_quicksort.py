"""Parallel Randomized Quicksort — CLRS 27.3, Problem 27-6 (3e) / 26.3, Problem 26-6 (4e) (Editions: 3,4).

Spawn the two recursive calls after a serial partition: work Theta(n lg n), span Theta(n)
expected for a serial partition — a parallel partition would give Theta(lg^2 n).
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import threading
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

_MASK64 = (1 << 64) - 1


def _partition(a: list[int], lo: int, hi: int, rng: Rng) -> int:
    p = rng.range(lo, hi - 1)
    a[p], a[hi - 1] = a[hi - 1], a[p]
    pivot = a[hi - 1]
    i = lo
    for j in range(lo, hi - 1):
        if a[j] < pivot:
            a[i], a[j] = a[j], a[i]
            i += 1
    a[i], a[hi - 1] = a[hi - 1], a[i]
    return i


def _p_quicksort_range(a: list[int], lo: int, hi: int, seed: int, cutoff: int) -> int:
    n = hi - lo
    if n <= 1:
        return 0
    if n <= cutoff:
        a[lo:hi] = sorted(a[lo:hi])
        return 0
    rng = Rng(seed)
    q = _partition(a, lo, hi, rng)
    dl = [0]
    left_seed = (seed * 3 + 1) & _MASK64
    right_seed = (seed * 5 + 7) & _MASK64

    def run_left() -> None:
        dl[0] = _p_quicksort_range(a, lo, q, left_seed, cutoff)

    t = threading.Thread(target=run_left)
    t.start()
    dr = _p_quicksort_range(a, q + 1, hi, right_seed, cutoff)
    t.join()
    return 1 + max(dl[0], dr)


def p_quicksort(a: list[int], seed: int, cutoff: int) -> int:
    """P-QUICKSORT: recursive calls run in parallel on disjoint slices. Returns recursion depth."""
    return _p_quicksort_range(a, 0, len(a), seed, cutoff)


def demo() -> str:
    lines: list[str] = []
    r = Rng(9)
    for n in [1_000, 100_000]:
        a = r.vec(n, 0, 1_000_000)
        expect = sorted(a)
        depth = p_quicksort(a, 1, 2_000)
        lines.append(
            f"n = {n:>7}: sorted correctly {str(a == expect).lower()}, "
            f"parallel recursion depth {depth} (~= 2*lg n = {2.0 * math.log2(n):.0f})"
        )
    lines.append(
        "with a serial partition the span is Theta(n) (the first partition alone costs n); parallelising the partition gives Theta(lg^2 n)"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_sorts_random_sorted_reversed_and_duplicate_heavy_inputs(self) -> None:
        r = Rng(431)
        for _ in range(40):
            n = r.range(0, 3000)
            a = r.vec(n, 0, 50)
            e = sorted(a)
            p_quicksort(a, 7, 100)
            self.assertEqual(a, e)
        s = list(range(5000))
        p_quicksort(s, 3, 64)
        self.assertEqual(s, list(range(5000)))
        d = list(reversed(range(5000)))
        p_quicksort(d, 3, 64)
        self.assertEqual(d, list(range(5000)))

    def test_depth_is_logarithmic_with_high_probability(self) -> None:
        r = Rng(432)
        a = r.vec(50_000, 0, 1 << 30)
        depth = p_quicksort(a, 11, 500)
        self.assertLess(depth, 80)


if __name__ == "__main__":
    print(demo(), end="")
