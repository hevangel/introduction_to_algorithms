"""Young Tableaux — CLRS Problem 6-3 (1e–4e) (1,2,3,4).

A sorted m×n matrix acting as a priority queue: EXTRACT-MIN in O(m+n), sorting n² numbers in O(n³).
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

INF = 1 << 62


class YoungTableau:
    """m×n Young tableau: rows and columns are sorted ascending; INF marks empty cells."""

    def __init__(self, m: int, n: int) -> None:
        self.m = m
        self.n = n
        self.a = [[INF] * n for _ in range(m)]

    def is_empty(self) -> bool:
        return self.a[0][0] == INF

    def is_full(self) -> bool:
        return self.a[self.m - 1][self.n - 1] != INF

    def extract_min(self) -> int | None:
        if self.is_empty():
            return None
        mn = self.a[0][0]
        self.a[0][0] = INF
        i, j = 0, 0
        while True:
            best = (i, j)
            if i + 1 < self.m and self.a[i + 1][j] < self.a[best[0]][best[1]]:
                best = (i + 1, j)
            if j + 1 < self.n and self.a[i][j + 1] < self.a[best[0]][best[1]]:
                best = (i, j + 1)
            if best == (i, j):
                break
            self.a[i][j], self.a[best[0]][best[1]] = self.a[best[0]][best[1]], self.a[i][j]
            i, j = best
        return mn

    def insert(self, x: int) -> None:
        if self.is_full():
            raise OverflowError("tableau is full")
        i, j = self.m - 1, self.n - 1
        self.a[i][j] = x
        while True:
            best = (i, j)
            if i > 0 and self.a[i - 1][j] > self.a[best[0]][best[1]]:
                best = (i - 1, j)
            if j > 0 and self.a[i][j - 1] > self.a[best[0]][best[1]]:
                best = (i, j - 1)
            if best == (i, j):
                break
            self.a[i][j], self.a[best[0]][best[1]] = self.a[best[0]][best[1]], self.a[i][j]
            i, j = best

    def contains(self, x: int) -> bool:
        i, j = 0, self.n - 1
        while i < self.m and j >= 0:
            v = self.a[i][j]
            if v == x:
                return True
            if v > x:
                j -= 1
            else:
                i += 1
        return False


def sort_with_tableau(v: list[int]) -> list[int]:
    n = max(1, math.ceil(math.sqrt(len(v))))
    t = YoungTableau(n, n)
    for x in v:
        t.insert(x)
    out: list[int] = []
    while (x := t.extract_min()) is not None:
        out.append(x)
    return out


def demo() -> str:
    lines: list[str] = []
    t = YoungTableau(4, 4)
    for x in [9, 16, 3, 2, 4, 8, 5, 14, 12]:
        t.insert(x)
    for row in t.a:
        lines.append("\t".join("∞" if x == INF else str(x) for x in row))
    lines.append(f"contains 8? {t.contains(8)}   contains 7? {t.contains(7)}")
    lines.append(f"extract-min x3: {[t.extract_min() for _ in range(3)]}")
    lines.append(f"sorted 12 numbers: {sort_with_tableau([7, 3, 11, 1, 9, 5, 12, 2, 8, 4, 10, 6])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _valid(self, t: YoungTableau) -> bool:
        return all(
            (j + 1 >= t.n or t.a[i][j] <= t.a[i][j + 1]) and (i + 1 >= t.m or t.a[i][j] <= t.a[i + 1][j])
            for i in range(t.m)
            for j in range(t.n)
        )

    def test_insert_keeps_invariant_and_extract_sorts(self) -> None:
        r = Rng(24)
        v = r.vec(25, -40, 40)
        t = YoungTableau(5, 5)
        for x in v:
            t.insert(x)
            self.assertTrue(self._valid(t))
        self.assertTrue(t.is_full())
        with self.assertRaises(OverflowError):
            t.insert(0)
        out: list[int] = []
        while (x := t.extract_min()) is not None:
            self.assertTrue(self._valid(t))
            out.append(x)
        self.assertEqual(out, sorted(v))

    def test_membership(self) -> None:
        t = YoungTableau(3, 4)
        for x in [10, 2, 7, 5, 9]:
            t.insert(x)
        for x in [10, 2, 7, 5, 9]:
            self.assertTrue(t.contains(x))
        self.assertFalse(t.contains(6))

    def test_sorts_non_square_counts(self) -> None:
        r = Rng(25)
        v = r.vec(37, 0, 100)
        self.assertEqual(sort_with_tableau(v), sorted(v))


if __name__ == "__main__":
    print(demo(), end="")
