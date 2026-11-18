"""Image Compression by Seam Carving — CLRS Problem 15-8 (3e), 14-8 (4e) (3e,4e).

Find the vertical seam of minimum total disruption in an m x n grid: O(mn) DP.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def min_seam(d: list[list[int]]) -> tuple[int, list[int]]:
    """Minimum-cost vertical seam through disruption grid d[row][col].

    Returns (cost, column chosen in each row, top to bottom).
    """
    m, n = len(d), len(d[0])
    best = [[sys.maxsize] * n for _ in range(m)]
    best[0] = list(d[0])
    for i in range(1, m):
        for j in range(n):
            lo = max(0, j - 1)
            hi = min(n - 1, j + 1)
            prev = min(best[i - 1][k] for k in range(lo, hi + 1))
            best[i][j] = prev + d[i][j]
    j = min(range(n), key=lambda col: best[m - 1][col])
    cost = best[m - 1][j]
    seam = [j] * m
    for i in range(m - 1, 0, -1):
        lo = max(0, j - 1)
        hi = min(n - 1, j + 1)
        j = min(range(lo, hi + 1), key=lambda k: best[i - 1][k])
        seam[i - 1] = j
    return cost, seam


def remove_seam(d: list[list[int]], seam: list[int]) -> list[list[int]]:
    """Remove a seam from the grid, producing an m x (n - 1) grid."""
    return [[v for j, v in enumerate(row) if j != c] for row, c in zip(d, seam)]


def demo() -> str:
    lines: list[str] = []
    d = [
        [5, 9, 2, 8, 6],
        [3, 1, 7, 4, 9],
        [8, 6, 5, 1, 3],
        [4, 7, 2, 9, 5],
    ]
    cost, seam = min_seam(d)
    lines.append("disruption grid:")
    for row in d:
        lines.append(f"  {row}")
    lines.append(f"minimum seam cost = {cost}, columns by row = {seam}")
    d2 = remove_seam(d, seam)
    lines.append(f"after carving the seam the grid is {len(d2)}×{len(d2[0])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _brute(self, d: list[list[int]], row: int, col: int) -> int:
        n = len(d[0])
        if row + 1 == len(d):
            rest = 0
        else:
            rest = min(
                self._brute(d, row + 1, c)
                for c in range(max(0, col - 1), min(n - 1, col + 1) + 1)
            )
        return d[row][col] + rest

    def test_matches_brute_force_and_seam_is_connected(self) -> None:
        r = Rng(110)
        for _ in range(60):
            m, n = r.range(1, 6), r.range(1, 6)
            d = [r.vec(n, 0, 9) for _ in range(m)]
            cost, seam = min_seam(d)
            self.assertEqual(cost, min(self._brute(d, 0, c) for c in range(n)))
            self.assertEqual(sum(d[i][c] for i, c in enumerate(seam)), cost)
            self.assertTrue(all(abs(seam[i] - seam[i + 1]) <= 1 for i in range(len(seam) - 1)))
            if n > 1:
                self.assertEqual(len(remove_seam(d, seam)[0]), n - 1)


if __name__ == "__main__":
    print(demo(), end="")
