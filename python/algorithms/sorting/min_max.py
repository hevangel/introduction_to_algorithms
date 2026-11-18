"""Simultaneous Minimum and Maximum — CLRS 9.1 (1e–4e).

Find both extremes with at most 3⌊n/2⌋ comparisons by pairing elements.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def minimum(a: list):
    """MINIMUM(A) — n−1 comparisons, which is optimal."""
    if not a:
        return None
    m = a[0]
    for x in a[1:]:
        if x < m:
            m = x
    return m


def min_and_max(a: list) -> tuple | None:
    """Returns (min, max, comparisons) using the pairing trick: ≤ 3⌊n/2⌋ comparisons."""
    if not a:
        return None
    cmp = 0
    if len(a) % 2 == 1:
        mn = mx = a[0]
        start = 1
    else:
        cmp += 1
        if a[0] < a[1]:
            mn, mx = a[0], a[1]
        else:
            mn, mx = a[1], a[0]
        start = 2
    i = start
    while i + 1 < len(a):
        cmp += 1
        if a[i] < a[i + 1]:
            small, large = a[i], a[i + 1]
        else:
            small, large = a[i + 1], a[i]
        cmp += 2
        if small < mn:
            mn = small
        if large > mx:
            mx = large
        i += 2
    return mn, mx, cmp


def demo() -> str:
    lines: list[str] = []
    a = [3, 8, 1, 9, 4, 7, 2, 6, 5]
    res = min_and_max(a)
    assert res is not None
    mn, mx, c = res
    lines.append(f"A = {a}")
    lines.append(
        f"min={mn} max={mx} using {c} comparisons (naive: {2 * (len(a) - 1)}, bound 3⌊n/2⌋ = {3 * (len(a) // 2)})"
    )
    b = list(reversed(range(1000)))
    res_b = min_and_max(b)
    assert res_b is not None
    lines.append(f"n=1000: {res_b[2]} comparisons (naive {2 * 999}, bound {1500})")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_correct_and_within_bound(self) -> None:
        r = Rng(40)
        for n in range(1, 100):
            v = r.vec(n, -50, 50)
            res = min_and_max(v)
            assert res is not None
            mn, mx, c = res
            self.assertEqual(mn, min(v))
            self.assertEqual(mx, max(v))
            bound = 3 * (n // 2) if n % 2 == 1 else 3 * n // 2 - 2
            self.assertLessEqual(c, bound, f"n={n} c={c} bound={bound}")
            self.assertEqual(minimum(v), mn)
        self.assertIsNone(min_and_max([]))


if __name__ == "__main__":
    print(demo(), end="")
