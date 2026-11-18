"""k-Center Clustering — CLRS Companion extension (Ch 35; related to 4e §33.1 clustering) (3,4).

Choose k centers to minimise the largest distance from any point to its nearest center: the farthest-first traversal is a 2-approximation, and nothing better is possible unless P = NP.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Pt = tuple[int, int]


def _d2(a: Pt, b: Pt) -> int:
    return (a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2


def radius2(points: list[Pt], centers: list[int]) -> int:
    """Squared radius of a set of centers: max over points of min squared distance to a center."""
    if not points:
        return 0
    return max(min(_d2(p, points[c]) for c in centers) for p in points)


def farthest_first(points: list[Pt], k: int) -> list[int]:
    """Gonzalez's farthest-first traversal: 2-approximation in O(nk)."""
    centers = [0]
    dist = [_d2(p, points[0]) for p in points]
    target = min(k, len(points))
    while len(centers) < target:
        nxt = max(range(len(points)), key=lambda i: dist[i])
        centers.append(nxt)
        for i in range(len(points)):
            dist[i] = min(dist[i], _d2(points[i], points[nxt]))
    return centers


def optimal_centers(points: list[Pt], k: int) -> tuple[int, list[int]]:
    """Optimal choice of k centers among input points by exhaustive search (small inputs)."""
    target = min(k, len(points))
    best_r = math.inf
    best_c: list[int] = []
    chosen: list[int] = []

    def go(start: int) -> None:
        nonlocal best_r, best_c
        if len(chosen) == target:
            r = radius2(points, chosen)
            if r < best_r:
                best_r = r
                best_c = chosen[:]
            return
        for i in range(start, len(points)):
            chosen.append(i)
            go(i + 1)
            chosen.pop()

    go(0)
    return (int(best_r), best_c)


def demo() -> str:
    pts: list[Pt] = [(0, 0), (1, 1), (0, 2), (10, 0), (11, 1), (10, 2), (5, 10), (6, 11), (4, 9)]
    lines = []
    for k in range(1, 5):
        g = farthest_first(pts, k)
        opt, oc = optimal_centers(pts, k)
        lines.append(
            f"k = {k}: greedy centers {g} radius {math.sqrt(radius2(pts, g)):.2f}; "
            f"optimal {oc} radius {math.sqrt(opt):.2f}"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_two_approximation_against_exhaustive_optimum(self) -> None:
        r = Rng(416)
        for _ in range(200):
            n = r.range(1, 9)
            pts = [(r.range(0, 30), r.range(0, 30)) for _ in range(n)]
            k = r.range(1, min(n, 4))
            g = farthest_first(pts, k)
            self.assertEqual(len(g), k)
            opt, _ = optimal_centers(pts, k)
            self.assertLessEqual(radius2(pts, g), 4 * opt)

    def test_well_separated_clusters_are_found(self) -> None:
        pts: list[Pt] = [(0, 0), (1, 0), (100, 0), (101, 1), (0, 100), (1, 101)]
        g = farthest_first(pts, 3)
        self.assertLessEqual(radius2(pts, g), 2)
        self.assertEqual(radius2(pts, farthest_first(pts, 6)), 0)


if __name__ == "__main__":
    print(demo(), end="")
