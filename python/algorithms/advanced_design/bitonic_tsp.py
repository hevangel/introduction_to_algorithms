"""Bitonic Euclidean Traveling Salesman — CLRS Problem 15-3 (1e–4e) (1e,2e,3e,4e).

The shortest bitonic closed tour of points sorted by x, found in O(n^2) by DP.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Point = tuple[float, float]


def _dist(a: Point, b: Point) -> float:
    return math.hypot(a[0] - b[0], a[1] - b[1])


def bitonic_tour_length(points: list[Point]) -> float:
    """Length of the shortest bitonic tour of points (sorted internally by x)."""
    p = sorted(points, key=lambda pt: pt[0])
    n = len(p)
    if n <= 1:
        return 0.0
    if n == 2:
        return 2.0 * _dist(p[0], p[1])
    b = [[math.inf] * n for _ in range(n)]
    b[0][1] = _dist(p[0], p[1])
    for j in range(2, n):
        for i in range(0, j - 1):
            b[i][j] = b[i][j - 1] + _dist(p[j - 1], p[j])
        b[j - 1][j] = min(b[k][j - 1] + _dist(p[k], p[j]) for k in range(j - 1))
    return b[n - 2][n - 1] + _dist(p[n - 2], p[n - 1])


def bitonic_tour_brute_force(points: list[Point]) -> float:
    """Exhaustive check: assign each interior point to the upper or lower path (2^(n-2) tours)."""
    p = sorted(points, key=lambda pt: pt[0])
    n = len(p)
    if n < 3:
        return bitonic_tour_length(points)
    best = math.inf
    for mask in range(1 << (n - 2)):
        up = [0]
        down = [0]
        for i in range(1, n - 1):
            if (mask >> (i - 1)) & 1:
                up.append(i)
            else:
                down.append(i)
        up.append(n - 1)
        down.append(n - 1)
        length = sum(_dist(p[up[k]], p[up[k + 1]]) for k in range(len(up) - 1)) + sum(
            _dist(p[down[k]], p[down[k + 1]]) for k in range(len(down) - 1)
        )
        best = min(best, length)
    return best


def demo() -> str:
    lines: list[str] = []
    pts = [(0.0, 6.0), (1.0, 0.0), (2.0, 3.0), (5.0, 4.0), (6.0, 1.0), (7.0, 5.0), (8.0, 2.0)]
    lines.append(f"points: {pts}")
    lines.append(f"shortest bitonic tour (DP)    = {bitonic_tour_length(pts):.4f}")
    lines.append(f"shortest bitonic tour (brute) = {bitonic_tour_brute_force(pts):.4f}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_dp_matches_brute_force(self) -> None:
        r = Rng(108)
        for n in range(3, 11):
            pts = [(i + r.f64() * 0.5, r.f64() * 10.0) for i in range(n)]
            self.assertLess(abs(bitonic_tour_length(pts) - bitonic_tour_brute_force(pts)), 1e-9, f"n={n}")

    def test_tiny_cases(self) -> None:
        self.assertEqual(bitonic_tour_length([]), 0.0)
        self.assertEqual(bitonic_tour_length([(1.0, 1.0)]), 0.0)
        self.assertLess(abs(bitonic_tour_length([(0.0, 0.0), (3.0, 4.0)]) - 10.0), 1e-9)
        expected = 3.0 + math.hypot(2.0, 4.0) + math.hypot(1.0, 4.0)
        self.assertLess(abs(bitonic_tour_length([(0.0, 0.0), (3.0, 0.0), (1.0, 4.0)]) - expected), 1e-9)


if __name__ == "__main__":
    print(demo(), end="")
