"""Monge Arrays — CLRS Problem 4-6 (2e–4e).

Row-minimum positions of a Monge array are monotone: find all of them by divide and conquer in O(m + n lg m).
"""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def is_monge(a: list[list[int]]) -> bool:
    """Is `a` Monge: a[i][j] + a[k][l] ≤ a[i][l] + a[k][j] for all i<k, j<l?"""
    if not a or not a[0]:
        return True
    return all(
        a[i][j] + a[i + 1][j + 1] <= a[i][j + 1] + a[i + 1][j]
        for i in range(len(a) - 1)
        for j in range(len(a[0]) - 1)
    )


def row_minima(m: int, n: int, f: Callable[[int, int], int]) -> list[int]:
    """Leftmost minimum column of every row of an m×n Monge array, given as an oracle `f(i, j)`."""
    res = [0] * m

    def go(row_lo: int, row_hi: int, col_lo: int, col_hi: int) -> None:
        if row_lo >= row_hi:
            return
        mid = (row_lo + row_hi) // 2
        best = col_lo
        best_val = f(mid, best)
        for j in range(col_lo + 1, col_hi + 1):
            val = f(mid, j)
            if val < best_val:
                best = j
                best_val = val
        res[mid] = best
        go(row_lo, mid, col_lo, best)
        go(mid + 1, row_hi, best, col_hi)

    if m > 0 and n > 0:
        go(0, m, 0, n - 1)
    return res


def row_minima_brute(a: list[list[int]]) -> list[int]:
    """Brute-force Θ(mn) leftmost row minima."""
    return [min(range(len(r)), key=lambda j: r[j]) for r in a]


def demo() -> str:
    lines: list[str] = []
    a = [[(i - 2 * j) ** 2 for j in range(8)] for i in range(8)]
    lines.append(f"is Monge: {is_monge(a)}")
    lines.append(f"row minima (divide & conquer): {row_minima(8, 8, lambda i, j: a[i][j])}")
    lines.append(f"row minima (brute force)     : {row_minima_brute(a)}")
    calls = 0

    def big(i: int, j: int) -> int:
        nonlocal calls
        calls += 1
        return (i - j // 2) ** 2

    row_minima(1000, 1000, big)
    lines.append(
        f"1000×1000 array: {calls} entry evaluations (≈ m + n lg m = {1000 + 1000 * 10}) instead of 10⁶"
    )
    return "\n".join(lines) + "\n"


def _random_monge(m: int, n: int, r: Rng) -> list[list[int]]:
    ri = r.vec(m, 0, 50)
    cj = r.vec(n, 0, 50)
    p = [r.vec(n, 0, 6) for _ in range(m)]
    # Prefix sums of P for fast O(mn) construction with identical values to the double sum
    pref = [[0] * n for _ in range(m)]
    for i in range(m):
        row_sum = 0
        for j in range(n):
            row_sum += p[i][j]
            pref[i][j] = row_sum + (pref[i - 1][j] if i > 0 else 0)
    return [[ri[i] + cj[j] - pref[i][j] for j in range(n)] for i in range(m)]


class Tests(unittest.TestCase):
    def test_matches_brute_force_on_random_monge_arrays(self) -> None:
        r = Rng(306)
        for _ in range(200):
            m, n = r.range(1, 14), r.range(1, 14)
            a = _random_monge(m, n, r)
            self.assertTrue(is_monge(a))
            self.assertEqual(row_minima(m, n, lambda i, j, a=a: a[i][j]), row_minima_brute(a))

    def test_monotone_minima_and_non_monge_detection(self) -> None:
        r = Rng(307)
        a = _random_monge(30, 25, r)
        mins = row_minima_brute(a)
        self.assertTrue(all(mins[i] <= mins[i + 1] for i in range(len(mins) - 1)))
        self.assertFalse(is_monge([[5, 0], [0, 5]]))


if __name__ == "__main__":
    print(demo(), end="")
