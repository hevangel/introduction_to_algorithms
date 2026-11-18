"""Convex Hull: Jarvis's March — CLRS 33.3 (1e–3e) (1,2,3).

Gift wrapping: from each hull vertex pick the point with the smallest polar angle; output-sensitive O(nh).
"""

from __future__ import annotations

from functools import cmp_to_key
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Pt = tuple[int, int]


def cross(p0: Pt, p1: Pt, p2: Pt) -> int:
    return (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1])


def jarvis_march(points: list[Pt]) -> list[Pt]:
    """JARVIS-MARCH(Q): hull vertices in CCW order starting at lowest-leftmost point (collinear points omitted)."""
    pts = sorted(set(points))
    if len(pts) < 3:
        return pts
    start = min(pts, key=lambda p: (p[1], p[0]))
    dist2 = lambda a, b: (a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2
    hull = [start]
    cur = start
    while True:
        nxt = pts[1] if pts[0] == cur else pts[0]
        for p in pts:
            if p == cur:
                continue
            c = cross(cur, nxt, p)
            if c < 0 or (c == 0 and dist2(cur, p) > dist2(cur, nxt)):
                nxt = p
        if nxt == start:
            break
        hull.append(nxt)
        cur = nxt
    return hull


def _graham_scan(points: list[Pt]) -> list[Pt]:
    pts = sorted(set(points))
    if len(pts) < 3:
        return pts
    p0 = min(pts, key=lambda p: (p[1], p[0]))
    rest = [p for p in pts if p != p0]
    dist2 = lambda p: (p[0] - p0[0]) ** 2 + (p[1] - p0[1]) ** 2

    def cmp_pts(a: Pt, b: Pt) -> int:
        c = cross(p0, a, b)
        if c != 0:
            return -1 if c > 0 else 1
        return (dist2(a) > dist2(b)) - (dist2(a) < dist2(b))

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


def demo() -> str:
    pts: list[Pt] = [(0, 3), (1, 1), (2, 2), (4, 4), (0, 0), (1, 2), (3, 1), (3, 3), (2, 0), (4, 0)]
    circle = [
        (round(1000.0 * math.cos(math.radians(d))), round(1000.0 * math.sin(math.radians(d))))
        for d in range(0, 360, 10)
    ]
    return "\n".join([
        f"Jarvis hull   : {jarvis_march(pts)}",
        f"Graham hull   : {_graham_scan(pts)}",
        f"points on a circle (h ≈ n): hull size {len(jarvis_march(circle))} of {len(circle)}",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_agrees_with_graham_scan(self) -> None:
        r = Rng(233)
        for _ in range(300):
            n = r.range(0, 40)
            pts = [(r.range(0, 12), r.range(0, 12)) for _ in range(n)]
            self.assertEqual(jarvis_march(pts), _graham_scan(pts), f"{pts}")

    def test_collinear_boundary_points_dropped(self) -> None:
        self.assertEqual(
            jarvis_march([(0, 0), (2, 0), (4, 0), (4, 4), (0, 4), (2, 2)]),
            [(0, 0), (4, 0), (4, 4), (0, 4)],
        )


if __name__ == "__main__":
    print(demo(), end="")
