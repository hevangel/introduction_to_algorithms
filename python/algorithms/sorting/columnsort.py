"""Columnsort — CLRS Problem 8-7 (2e–4e).

Leighton's 8-step sorting network on an r×s matrix (r ≥ 2(s−1)², s | r): four column sorts plus fixed permutations.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def columnsort(data: list[int], r: int, s: int) -> list[int]:
    """Columnsort of `r·s` numbers viewed as an r-row, s-column matrix (column-major)."""
    assert len(data) == r * s
    assert r % 2 == 0 and r % s == 0 and r >= 2 * (s - 1) * (s - 1), (
        "columnsort needs r even, s | r, r ≥ 2(s−1)²"
    )

    def sort_columns(v: list) -> None:
        for start in range(0, len(v), r):
            v[start : start + r] = sorted(v[start : start + r])

    a = list(data)
    sort_columns(a)  # 1. sort each column
    t = [0] * (r * s)  # 2. "transpose": pick up column-major, lay down row-major
    for k, x in enumerate(a):
        t[(k % s) * r + k // s] = x
    a = t
    sort_columns(a)  # 3. sort each column
    t = [0] * (r * s)  # 4. inverse of step 2
    for k in range(r * s):
        t[k] = a[(k % s) * r + k // s]
    a = t
    sort_columns(a)  # 5. sort each column
    shifted: list = [-math.inf] * (r // 2) + a + [math.inf] * (r // 2)  # 6. shift by r/2
    sort_columns(shifted)  # 7. sort each column
    return shifted[r // 2 : r // 2 + r * s]  # 8. unshift


def min_rows(s: int) -> int:
    """Smallest legal number of rows for s columns: the least even multiple of s ≥ 2(s−1)²."""
    r = max(2 * (s - 1) * (s - 1), 2)
    while r % s != 0 or r % 2 != 0:
        r += 1
    return r


def demo() -> str:
    lines: list[str] = []
    rng = Rng(5)
    for s in [2, 3, 4, 6]:
        r = min_rows(s)
        v = rng.vec(r * s, 0, 99)
        e = sorted(v)
        lines.append(f"s = {s}, r = {r} ({r * s} numbers): columnsort == sort: {columnsort(v, r, s) == e}")
    v = list(reversed(range(36)))
    lines.append(f"reverse 35..0 as 12×3: {columnsort(v, 12, 3)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_sorts_random_inputs_for_legal_shapes(self) -> None:
        rng = Rng(311)
        for s in range(2, 8):
            r = min_rows(s)
            for _ in range(40):
                v = rng.vec(r * s, -20, 20)
                e = sorted(v)
                self.assertEqual(columnsort(v, r, s), e, f"r={r} s={s}")
            rev = list(reversed(range(r * s)))
            self.assertEqual(columnsort(rev, r, s), list(range(r * s)))

    def test_taller_matrices_also_work_and_bad_shapes_are_rejected(self) -> None:
        rng = Rng(312)
        v = rng.vec(40 * 4, 0, 1000)
        e = sorted(v)
        self.assertEqual(columnsort(v, 40, 4), e)
        self.assertEqual(columnsort([0] * 12, 6, 2), [0] * 12)  # legal shape: r=6 even, 2 | 6, 6 >= 2
        with self.assertRaises(AssertionError):
            columnsort([0] * 12, 4, 3)  # s does not divide r
        self.assertEqual(min_rows(3), 12)


if __name__ == "__main__":
    print(demo(), end="")
