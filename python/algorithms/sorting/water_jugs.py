"""Matching Water Jugs — CLRS Problem 8-4 (1e–4e) (1,2,3,4).

Match n red jugs with n blue jugs when only red-vs-blue comparisons are allowed: a quicksort-style randomized algorithm needs Θ(n lg n) expected comparisons, and Ω(n lg n) are required.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def match_jugs(red: list[int], blue: list[int], rng: Rng) -> tuple[list[tuple[int, int]], int]:
    """Return (pairs (red idx, blue idx), red-vs-blue comparisons used)."""
    cmp = 0
    out: list[tuple[int, int]] = []
    stack: list[tuple[list[int], list[int]]] = [(list(range(len(red))), list(range(len(blue))))]
    while stack:
        r, b = stack.pop()
        if not r:
            continue
        pivot_red = r[rng.range(0, len(r) - 1)]
        bs: list[int] = []
        bl: list[int] = []
        bm: int | None = None
        for j in b:
            cmp += 1
            if red[pivot_red] > blue[j]:
                bs.append(j)
            elif red[pivot_red] < blue[j]:
                bl.append(j)
            else:
                bm = j
        assert bm is not None, "every red jug has a matching blue jug"
        out.append((pivot_red, bm))
        rs: list[int] = []
        rl: list[int] = []
        for i in r:
            if i == pivot_red:
                continue
            cmp += 1
            if red[i] < blue[bm]:
                rs.append(i)
            else:
                rl.append(i)
        stack.append((rs, bs))
        stack.append((rl, bl))
    return out, cmp


def demo() -> str:
    lines: list[str] = []
    rng = Rng(7)
    red = [5, 2, 9, 4, 7]
    blue = [7, 4, 2, 5, 9]
    pairs, c = match_jugs(red, blue, rng)
    lines.append(f"red {red}, blue {blue}")
    lines.append(f"pairs (red idx, blue idx): {pairs}, {c} comparisons")
    for n in [100, 1000, 10000]:
        caps = list(range(n))
        rng.shuffle(caps)
        other = list(caps)
        rng.shuffle(other)
        _, c = match_jugs(caps, other, rng)
        lines.append(f"n = {n:>6}: {c} comparisons = {c / (n * math.log2(n)):.2f} · n lg n")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_every_pair_has_equal_capacity_and_comparisons_are_n_log_n(self) -> None:
        rng = Rng(440)
        for n in [1, 2, 3, 10, 100, 1000]:
            red = [x * 3 + 1 for x in range(n)]
            blue = list(red)
            rng.shuffle(red)
            rng.shuffle(blue)
            pairs, c = match_jugs(red, blue, rng)
            self.assertEqual(len(pairs), n)
            ur = [False] * n
            ub = [False] * n
            for i, j in pairs:
                self.assertEqual(red[i], blue[j])
                ur[i] = True
                ub[j] = True
            self.assertTrue(all(ur) and all(ub))
            if n > 1:
                self.assertLessEqual(c, 6.0 * n * math.log2(n) + 4.0)


if __name__ == "__main__":
    print(demo(), end="")
