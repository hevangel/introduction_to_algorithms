"""Stooge Sort — CLRS Problem 7-3 (2e–4e) (2,3,4).

A deliberately slow recursive sort: Θ(n^(lg 3/lg 1.5)) ≈ Θ(n^2.71).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def stooge_sort(a: list, lo: int = 0, hi: int | None = None) -> None:
    """STOOGE-SORT(A, i, j) in place."""
    if hi is None:
        hi = len(a) - 1
    n = hi - lo + 1
    if n < 2:
        return
    if a[lo] > a[hi]:
        a[lo], a[hi] = a[hi], a[lo]
    if n >= 3:
        k = n // 3
        stooge_sort(a, lo, hi - k)
        stooge_sort(a, lo + k, hi)
        stooge_sort(a, lo, hi - k)


def calls(n: int) -> int:
    """Number of recursive calls for input size n (grows as n^2.71)."""
    if n < 3:
        return 1
    return 1 + 3 * calls(n - n // 3)


def demo() -> str:
    lines: list[str] = []
    a = [8, 3, 9, 1, 7, 2, 6]
    stooge_sort(a)
    lines.append(f"sorted: {a}")
    for n in [10, 100, 1000, 10000]:
        lines.append(f"n={n:>5}: {calls(n):>12} recursive calls (n^2.71 ≈ {n ** 2.7095:.0f})")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_sorts_every_size(self) -> None:
        r = Rng(32)
        for n in range(45):
            v = r.vec(n, -10, 10)
            e = sorted(v)
            stooge_sort(v)
            self.assertEqual(v, e)

    def test_call_count_grows_superquadratically(self) -> None:
        self.assertGreater(calls(2000) / calls(1000), 6.0)


if __name__ == "__main__":
    print(demo(), end="")
