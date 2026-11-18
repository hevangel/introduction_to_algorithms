"""Parallel Prefix (Scan) — CLRS 30.2 (1e) (Editions: 1).

All prefix sums in ceil(lg n) steps (recursive doubling) or with O(n) work (up-sweep / down-sweep).
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest
from typing import Callable, TypeVar

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

T = TypeVar("T")


def prefix_doubling(x: list[T], op: Callable[[T, T], T]) -> tuple[list[T], int]:
    """Hillis-Steele inclusive scan: ceil(lg n) rounds, Theta(n lg n) work."""
    n = len(x)
    cur = list(x)
    step = 1
    rounds = 0
    while step < n:
        old = list(cur)
        for i in range(step, n):
            cur[i] = op(old[i - step], old[i])
        step *= 2
        rounds += 1
    return cur, rounds


def prefix_work_efficient(
    x: list[int], op: Callable[[int, int], int], identity: int
) -> tuple[list[int], int, int]:
    """Work-efficient (Blelloch) exclusive scan with an identity element: 2 lg n rounds, O(n) work."""
    orig_len = len(x)
    n = 1
    while n < orig_len:
        n *= 2
    a = list(x) + [identity] * (n - orig_len)
    rounds = 0
    ops = 0
    d = 1
    while d < n:
        for i in range(0, n, 2 * d):
            a[i + 2 * d - 1] = op(a[i + d - 1], a[i + 2 * d - 1])
            ops += 1
        d *= 2
        rounds += 1
    a[n - 1] = identity
    while d > 1:
        d //= 2
        for i in range(0, n, 2 * d):
            t = a[i + d - 1]
            a[i + d - 1] = a[i + 2 * d - 1]
            a[i + 2 * d - 1] = op(t, a[i + 2 * d - 1])
            ops += 1
        rounds += 1
    return a[:orig_len], rounds, ops


def demo() -> str:
    lines: list[str] = []
    x = list(range(1, 9))
    p, rounds = prefix_doubling(x, lambda a, b: a + b)
    lines.append(f"x                    = {x}")
    lines.append(f"inclusive scan (doubling) = {p} in {rounds} rounds")
    e, r2, ops = prefix_work_efficient(x, lambda a, b: a + b, 0)
    lines.append(f"exclusive scan (Blelloch) = {e} in {r2} rounds, {ops} operations")
    m = [3, 1, 4, 1, 5, 9, 2, 6]
    lines.append(f"prefix maxima        = {prefix_doubling(m, max)[0]}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_sequential_scans(self) -> None:
        r = Rng(161)
        for n in [0, 1, 2, 7, 8, 33, 100]:
            x = r.vec(n, -20, 20)
            seq = list(x)
            for i in range(1, n):
                seq[i] += seq[i - 1]
            p, rounds = prefix_doubling(x, lambda a, b: a + b)
            self.assertEqual(p, seq)
            self.assertEqual(rounds, 0 if n <= 1 else math.ceil(math.log2(n)))
            e, _, ops = prefix_work_efficient(x, lambda a, b: a + b, 0)
            excl = [0 if i == 0 else seq[i - 1] for i in range(n)]
            self.assertEqual(e, excl)
            pow2 = 1
            while pow2 < n:
                pow2 *= 2
            self.assertLessEqual(ops, 2 * max(pow2, 1))

    def test_works_for_any_associative_operator(self) -> None:
        x = [3, 1, 4, 1, 5, 9, 2, 6]
        self.assertEqual(prefix_doubling(x, max)[0], [3, 3, 4, 4, 5, 9, 9, 9])
        self.assertEqual(prefix_doubling(x, min)[0], [3, 1, 1, 1, 1, 1, 1, 1])


if __name__ == "__main__":
    print(demo(), end="")
