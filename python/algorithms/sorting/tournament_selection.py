"""Second Smallest via a Tournament — CLRS Exercise 9.1-1 (1e–4e) (1,2,3,4).

Find the second smallest of n elements with n + ⌈lg n⌉ − 2 comparisons using a knockout tournament.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def second_smallest(a: list[int]) -> tuple[int, int]:
    """Return (second smallest, comparisons used). Needs at least two elements."""
    assert len(a) >= 2
    comps = 0

    def tournament(lo: int, hi: int) -> tuple[int, list[int]]:
        nonlocal comps
        if hi - lo == 1:
            return (a[lo], [])
        mid = (lo + hi) // 2
        lw, lbeat = tournament(lo, mid)
        rw, rbeat = tournament(mid, hi)
        comps += 1
        if lw <= rw:
            lbeat.append(rw)
            return (lw, lbeat)
        rbeat.append(lw)
        return (rw, rbeat)

    _, beaten = tournament(0, len(a))
    best = beaten[0]
    for x in beaten[1:]:
        comps += 1
        if x < best:
            best = x
    return (best, comps)


def demo() -> str:
    lines: list[str] = []
    v = [27, 4, 91, 15, 8, 63, 2, 40]
    s, c = second_smallest(v)
    lines.append(f"A = {v}: second smallest = {s} using {c} comparisons (n + ⌈lg n⌉ − 2 = {8 + 3 - 2})")
    for n in [10, 100, 1000, 100_000]:
        a = list(range(n - 1, -1, -1))
        s, c = second_smallest(a)
        bound = n + math.ceil(math.log2(n)) - 2
        lines.append(f"n = {n:>6}: second smallest {s}, comparisons {c} ≤ n + ⌈lg n⌉ − 2 = {bound}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_correct_and_within_the_optimal_bound(self) -> None:
        r = Rng(314)
        for n in range(2, 200):
            v = r.vec(n, -100, 100)
            s = sorted(v)
            x, c = second_smallest(v)
            self.assertEqual(x, s[1])
            self.assertLessEqual(c, n + math.ceil(math.log2(n)) - 2)

    def test_power_of_two_is_exact(self) -> None:
        v = list(range(15, -1, -1))
        self.assertEqual(second_smallest(v), (1, 16 + 4 - 2))


if __name__ == "__main__":
    print(demo(), end="")
