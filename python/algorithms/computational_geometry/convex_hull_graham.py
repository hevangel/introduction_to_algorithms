"""Convex Hull: Graham's Scan — CLRS 33.3 (1e–3e) (1,2,3).

Sort points by polar angle around the lowest point, then discard non-left turns with a stack: O(n lg n).
"""

from __future__ import annotations

from functools import cmp_to_key
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Pt = tuple[int, int]


def cross(p0: Pt, p1: Pt, p2: Pt) -> int:
    return (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1])


def graham_scan(points: list[Pt]) -> list[Pt]:
    """GRAHAM-SCAN(Q): CCW hull vertices starting at lowest-then-leftmost point (collinear points dropped)."""
    pts = sorted(set(points))
    if len(pts) < 3:
        return pts
    p0 = min(pts, key=lambda p: (p[1], p[0]))
    rest = [p for p in pts if p != p0]
    dist2 = lambda p: (p[0] - p0[0]) ** 2 + (p[1] - p0[1]) ** 2

    def cmp_pts(a: Pt, b: Pt) -> int:
        c = cross(p0, a, b)
        if c > 0:
            return -1
        if c < 0:
            return 1
        da, db = dist2(a), dist2(b)
        return (da > db) - (da < db)

    rest.sort(key=cmp_to_key(cmp_pts))
    filtered: list[Pt] = []
    for p in rest:
        if filtered and cross(p0, filtered[-1], p) == 0:
            filtered[-1] = p
        else:
            filtered.append(p)
    stack = [p0]
    for p in filtered:
        while len(stack) >= 2 and cross(stack[-2], stack[-1], p) <= 0:
            stack.pop()
        stack.append(p)
    return stack


def hull_area2(h: list[Pt]) -> int:
    n = len(h)
    return sum(h[i][0] * h[(i + 1) % n][1] - h[(i + 1) % n][0] * h[i][1] for i in range(n))


def demo() -> str:
    pts: list[Pt] = [(0, 3), (1, 1), (2, 2), (4, 4), (0, 0), (1, 2), (3, 1), (3, 3), (2, 0), (4, 0)]
    h = graham_scan(pts)
    r = Rng(7)
    big: list[Pt] = [(r.range(0, 1000), r.range(0, 1000)) for _ in range(10_000)]
    return "\n".join([
        f"points: {pts}",
        f"convex hull (CCW from the lowest point): {h}",
        f"hull has {len(h)} of {len(pts)} points; doubled area = {hull_area2(h)}",
        f"10000 random points in a square: hull size {len(graham_scan(big))}",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_square_with_interior_and_collinear_points(self) -> None:
        pts = [(0, 0), (2, 0), (4, 0), (4, 4), (0, 4), (2, 2), (1, 1), (4, 2), (0, 2)]
        self.assertEqual(graham_scan(pts), [(0, 0), (4, 0), (4, 4), (0, 4)])

    def test_hull_contains_all_points_and_is_convex(self) -> None:
        r = Rng(232)
        for _ in range(200):
            n = r.range(0, 40)
            pts = [(r.range(0, 15), r.range(0, 15)) for _ in range(n)]
            h = graham_scan(pts)
            if len(h) >= 3:
                for i in range(len(h)):
                    self.assertGreater(cross(h[i], h[(i + 1) % len(h)], h[(i + 2) % len(h)]), 0)
                for p in pts:
                    self.assertTrue(
                        all(cross(h[i], h[(i + 1) % len(h)], p) >= 0 for i in range(len(h))),
                        f"{p} outside {h}",
                    )

    def test_degenerate_inputs(self) -> None:
        self.assertEqual(graham_scan([]), [])
        self.assertEqual(graham_scan([(1, 1), (1, 1)]), [(1, 1)])
        self.assertEqual(len(graham_scan([(0, 0), (1, 1), (2, 2), (3, 3)])), 2)


if __name__ == "__main__":
    print(demo(), end="")
