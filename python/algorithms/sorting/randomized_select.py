"""Randomized Select — CLRS 9.2 (1e–4e) (1,2,3,4).

Quickselect: find the i-th smallest element in expected Θ(n) time.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def randomized_select(a: list, i: int, rng: Rng):
    """RANDOMIZED-SELECT(A, p, r, i): the i-th smallest (1-based) element of `a`. Reorders `a`."""
    assert 1 <= i <= len(a)
    lo, hi = 0, len(a) - 1
    while lo < hi:
        p = rng.range(lo, hi)
        a[p], a[hi] = a[hi], a[p]
        pivot = a[hi]
        q = lo
        for j in range(lo, hi):
            if a[j] <= pivot:
                a[q], a[j] = a[j], a[q]
                q += 1
        a[q], a[hi] = a[hi], a[q]
        k = q - lo + 1
        if i == k:
            return a[q]
        if i < k:
            hi = q - 1
        else:
            i -= k
            lo = q + 1
    return a[lo]


def demo() -> str:
    lines: list[str] = []
    r = Rng(11)
    v = [33, 7, 21, 4, 50, 18, 9, 41, 2, 27]
    lines.append(f"A (sorted view) = {sorted(v)}")
    for i in [1, 5, 10]:
        lines.append(f"RANDOMIZED-SELECT i={i:>2} -> {randomized_select(list(v), i, r)}")
    lines.append(f"median (i=(n+1)/2 lower) -> {randomized_select(list(v), 5, r)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_sorted_order_for_every_rank(self) -> None:
        r = Rng(41)
        for _ in range(30):
            v = r.vec(40, -25, 25)
            s = sorted(v)
            for i in range(1, len(v) + 1):
                self.assertEqual(randomized_select(list(v), i, r), s[i - 1])

    def test_singleton(self) -> None:
        r = Rng(1)
        self.assertEqual(randomized_select([5], 1, r), 5)


if __name__ == "__main__":
    print(demo(), end="")
