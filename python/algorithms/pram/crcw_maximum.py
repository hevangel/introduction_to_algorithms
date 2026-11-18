"""Constant-Time Maximum on a CRCW PRAM — CLRS 30.3 (1e) (Editions: 1).

With n^2 processors and concurrent writes, find the maximum in O(1); contrast with the Theta(lg n) EREW tree.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def crcw_maximum(a: list[int]) -> tuple[int, int, int] | None:
    """Common-CRCW maximum in 2 parallel steps with n^2 processors.

    Returns (maximum, processors used, steps used), or None if empty.
    """
    n = len(a)
    if n == 0:
        return None
    m = [True] * n
    for i in range(n):
        for j in range(n):
            if a[i] < a[j]:
                m[i] = False
    result: int | None = None
    for i in range(n):
        if m[i]:
            result = a[i]
    if result is None:
        return None
    return (result, n * n, 2)


def erew_maximum(a: list[int]) -> tuple[int, int] | None:
    """EREW tree reduction: ceil(lg n) rounds with n/2 processors. Returns (max, rounds)."""
    if not a:
        return None
    cur = list(a)
    rounds = 0
    while len(cur) > 1:
        cur = [max(cur[i : i + 2]) for i in range(0, len(cur), 2)]
        rounds += 1
    return (cur[0], rounds)


def demo() -> str:
    lines: list[str] = []
    a = [7, 3, 9, 1, 9, 4, 2, 8]
    lines.append(f"A = {a}")
    lines.append(f"CRCW: (max, processors, steps) = {crcw_maximum(a)}")
    lines.append(f"EREW: (max, rounds)            = {erew_maximum(a)}")
    for n in [8, 64, 1024]:
        v = list(reversed(range(n)))
        erew_res = erew_maximum(v)
        assert erew_res is not None
        lines.append(
            f"n = {n:>4}: CRCW 2 steps with {n * n:>7} processors; "
            f"EREW {erew_res[1]} steps with {n // 2} processors"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_both_find_the_maximum(self) -> None:
        r = Rng(162)
        for n in range(1, 40):
            v = r.vec(n, -9, 9)
            want = max(v)
            crcw = crcw_maximum(v)
            self.assertIsNotNone(crcw)
            assert crcw is not None
            self.assertEqual(crcw[0], want)
            erew = erew_maximum(v)
            self.assertIsNotNone(erew)
            assert erew is not None
            m, rounds = erew
            self.assertEqual(m, want)
            self.assertEqual(rounds, math.ceil(math.log2(n)))
        self.assertIsNone(crcw_maximum([]))
        self.assertIsNone(erew_maximum([]))


if __name__ == "__main__":
    print(demo(), end="")
