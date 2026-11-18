"""Optimal Polygon Triangulation — CLRS 16.4 (1e) (1e).

Minimum-weight triangulation of a convex polygon: the matrix-chain structure in geometry.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

Point = tuple[float, float]


def _dist(a: Point, b: Point) -> float:
    return math.hypot(a[0] - b[0], a[1] - b[1])


def perimeter(p: list[Point], i: int, k: int, j: int) -> float:
    """Weight of triangle (i, k, j): its perimeter."""
    return _dist(p[i], p[k]) + _dist(p[k], p[j]) + _dist(p[i], p[j])


def optimal_triangulation(p: list[Point]) -> tuple[float, list[tuple[int, int, int]]]:
    """Minimum total triangle perimeter over all triangulations of convex polygon p[0..n]."""
    n = len(p)
    if n < 3:
        return 0.0, []
    t = [[0.0] * n for _ in range(n)]
    s = [[0] * n for _ in range(n)]
    for length in range(2, n):
        for i in range(n - length):
            j = i + length
            t[i][j] = math.inf
            for k in range(i + 1, j):
                q = t[i][k] + t[k][j] + perimeter(p, i, k, j)
                if q < t[i][j]:
                    t[i][j] = q
                    s[i][j] = k

    tris: list[tuple[int, int, int]] = []

    def collect(i: int, j: int) -> None:
        if j - i < 2:
            return
        k = s[i][j]
        tris.append((i, k, j))
        collect(i, k)
        collect(k, j)

    collect(0, n - 1)
    return t[0][n - 1], tris


def count_triangulations(n: int) -> int:
    """Number of triangulations of an n-gon: the Catalan number C_{n-2}."""
    m = n - 2
    c = 1
    for i in range(m):
        c = c * 2 * (2 * i + 1) // (i + 2)
    return c


def demo() -> str:
    lines: list[str] = []
    hexagon = [
        (
            math.cos(2.0 * math.pi * k / 6.0) * 3.0 + 0.3 * (k % 2),
            math.sin(2.0 * math.pi * k / 6.0) * 2.0,
        )
        for k in range(6)
    ]
    cost, tris = optimal_triangulation(hexagon)
    lines.append(f"hexagon: optimal total perimeter = {cost:.4f}, triangles = {tris}")
    lines.append(
        f"number of triangulations of an n-gon (Catalan): "
        f"n=6 → {count_triangulations(6)}, n=10 → {count_triangulations(10)}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _brute(self, p: list[Point], i: int, j: int) -> float:
        if j - i < 2:
            return 0.0
        return min(
            self._brute(p, i, k) + self._brute(p, k, j) + perimeter(p, i, k, j)
            for k in range(i + 1, j)
        )

    def test_triangle_and_square(self) -> None:
        tri = [(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)]
        self.assertLess(abs(optimal_triangulation(tri)[0] - 12.0), 1e-9)
        sq = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
        cost, tris = optimal_triangulation(sq)
        self.assertLess(abs(cost - 2.0 * (2.0 + math.sqrt(2.0))), 1e-9)
        self.assertEqual(len(tris), 2)

    def test_matches_recursive_and_catalan(self) -> None:
        pts = [
            (
                math.cos(2.0 * math.pi * k / 8.0) * (1.0 + (0.1 * k) % 0.3),
                math.sin(2.0 * math.pi * k / 8.0),
            )
            for k in range(8)
        ]
        c, tris = optimal_triangulation(pts)
        self.assertLess(abs(c - self._brute(pts, 0, 7)), 1e-9)
        self.assertEqual(len(tris), 6)
        self.assertEqual(count_triangulations(3), 1)
        self.assertEqual(count_triangulations(4), 2)
        self.assertEqual(count_triangulations(6), 14)


if __name__ == "__main__":
    print(demo(), end="")
