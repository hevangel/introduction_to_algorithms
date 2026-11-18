"""Point in Polygon — CLRS 35.1 (1e) / 33.1 exercises (2e–3e) (1,2,3).

Ray casting (even–odd rule) and the winding number decide whether a point lies inside a simple polygon in O(n), using only exact cross products.
"""

from __future__ import annotations

from enum import Enum
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Pt = tuple[int, int]


class Where(Enum):
    INSIDE = "Inside"
    BOUNDARY = "Boundary"
    OUTSIDE = "Outside"


def cross(p0: Pt, p1: Pt, p2: Pt) -> int:
    return (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1])


def _on_segment(a: Pt, b: Pt, p: Pt) -> bool:
    return (
        cross(a, b, p) == 0
        and min(a[0], b[0]) <= p[0] <= max(a[0], b[0])
        and min(a[1], b[1]) <= p[1] <= max(a[1], b[1])
    )


def locate_even_odd(poly: list[Pt], p: Pt) -> Where:
    """Even–odd rule: count the edges crossed by the horizontal ray going right from p."""
    n = len(poly)
    inside = False
    for i in range(n):
        a, b = poly[i], poly[(i + 1) % n]
        if _on_segment(a, b, p):
            return Where.BOUNDARY
        if (a[1] > p[1]) != (b[1] > p[1]):
            left = cross(a, b, p) > 0
            if left == (b[1] > a[1]):
                inside = not inside
    return Where.INSIDE if inside else Where.OUTSIDE


def locate_winding(poly: list[Pt], p: Pt) -> Where:
    """Winding number: signed number of times the polygon wraps around p (non-zero => inside)."""
    n = len(poly)
    wn = 0
    for i in range(n):
        a, b = poly[i], poly[(i + 1) % n]
        if _on_segment(a, b, p):
            return Where.BOUNDARY
        if a[1] <= p[1]:
            if b[1] > p[1] and cross(a, b, p) > 0:
                wn += 1
        elif b[1] <= p[1] and cross(a, b, p) < 0:
            wn -= 1
    return Where.INSIDE if wn != 0 else Where.OUTSIDE


def area2(poly: list[Pt]) -> int:
    """Twice the signed area (shoelace formula): positive for counter-clockwise polygons."""
    n = len(poly)
    return sum(poly[i][0] * poly[(i + 1) % n][1] - poly[(i + 1) % n][0] * poly[i][1] for i in range(n))


def demo() -> str:
    poly: list[Pt] = [(0, 0), (8, 0), (8, 8), (4, 3), (0, 8)]
    lines = [f"polygon {poly}, 2×area = {area2(poly)}"]
    for p in [(4, 1), (4, 5), (1, 7), (4, 3), (9, 4), (0, 4), (6, 6)]:
        lines.append(
            f"{p}: even-odd {locate_even_odd(poly, p).value}, winding {locate_winding(poly, p).value}"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_rectangles_and_triangles_against_direct_tests(self) -> None:
        r = Rng(409)
        for _ in range(300):
            x0, y0 = r.range(0, 10), r.range(0, 10)
            w, h = r.range(1, 10), r.range(1, 10)
            rect = [(x0, y0), (x0 + w, y0), (x0 + w, y0 + h), (x0, y0 + h)]
            p = (r.range(0, 24) - 2, r.range(0, 24) - 2)
            if x0 < p[0] < x0 + w and y0 < p[1] < y0 + h:
                want = Where.INSIDE
            elif x0 <= p[0] <= x0 + w and y0 <= p[1] <= y0 + h:
                want = Where.BOUNDARY
            else:
                want = Where.OUTSIDE
            self.assertEqual(locate_even_odd(rect, p), want)
            self.assertEqual(locate_winding(rect, p), want)
            rev = list(reversed(rect))
            self.assertEqual(locate_even_odd(rev, p), want)
            self.assertEqual(locate_winding(rev, p), want)

    def test_rules_agree_on_random_star_shaped_polygons(self) -> None:
        r = Rng(410)
        for _ in range(100):
            n = r.range(3, 12)
            poly = []
            for i in range(n):
                a = (i + 0.5 * r.f64()) / n * math.tau
                rad = 5.0 + 20.0 * r.f64()
                poly.append((round(rad * math.cos(a)), round(rad * math.sin(a))))
            if area2(poly) <= 0:
                continue
            for _ in range(30):
                p = (r.range(0, 60) - 30, r.range(0, 60) - 30)
                self.assertEqual(locate_even_odd(poly, p), locate_winding(poly, p), f"{poly} {p}")
            self.assertEqual(locate_even_odd(poly, (0, 0)), Where.INSIDE)

    def test_concave_notch_and_area(self) -> None:
        poly: list[Pt] = [(0, 0), (8, 0), (8, 8), (4, 3), (0, 8)]
        self.assertEqual(locate_even_odd(poly, (4, 5)), Where.OUTSIDE)
        self.assertEqual(locate_even_odd(poly, (4, 1)), Where.INSIDE)
        self.assertEqual(locate_winding(poly, (4, 3)), Where.BOUNDARY)
        self.assertEqual(area2([(0, 0), (4, 0), (4, 3)]), 12)


if __name__ == "__main__":
    print(demo(), end="")
