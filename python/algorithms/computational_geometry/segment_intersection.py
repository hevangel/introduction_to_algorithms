"""Line-Segment Properties & Intersection — CLRS 33.1 (1e–3e) (1,2,3).

Cross products decide turn direction and whether two segments intersect, using only integer arithmetic.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Pt = tuple[int, int]


def cross(p0: Pt, p1: Pt, p2: Pt) -> int:
    """Cross product (p1 - p0) x (p2 - p0): > 0 left turn, < 0 right turn, 0 collinear."""
    return (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1])


def direction(pi: Pt, pj: Pt, pk: Pt) -> int:
    """DIRECTION(pi, pj, pk) = (pk - pi) x (pj - pi)."""
    return -cross(pi, pk, pj)


def on_segment(pi: Pt, pj: Pt, pk: Pt) -> bool:
    """ON-SEGMENT(pi, pj, pk): given pk collinear with segment pi pj, is pk inside its bounding box?"""
    return (
        min(pi[0], pj[0]) <= pk[0] <= max(pi[0], pj[0])
        and min(pi[1], pj[1]) <= pk[1] <= max(pi[1], pj[1])
    )


def segments_intersect(p1: Pt, p2: Pt, p3: Pt, p4: Pt) -> bool:
    """SEGMENTS-INTERSECT(p1, p2, p3, p4): do closed segments p1p2 and p3p4 share at least one point?"""
    d1, d2 = direction(p3, p4, p1), direction(p3, p4, p2)
    d3, d4 = direction(p1, p2, p3), direction(p1, p2, p4)
    if ((d1 > 0 and d2 < 0) or (d1 < 0 and d2 > 0)) and (
        (d3 > 0 and d4 < 0) or (d3 < 0 and d4 > 0)
    ):
        return True
    return (
        (d1 == 0 and on_segment(p3, p4, p1))
        or (d2 == 0 and on_segment(p3, p4, p2))
        or (d3 == 0 and on_segment(p1, p2, p3))
        or (d4 == 0 and on_segment(p1, p2, p4))
    )


def demo() -> str:
    lines = [
        f"cross((0,0),(4,0),(4,4)) = {cross((0, 0), (4, 0), (4, 4))} (left turn)",
        f"cross((0,0),(4,0),(4,-4)) = {cross((0, 0), (4, 0), (4, -4))} (right turn)",
        f"cross((0,0),(2,2),(4,4)) = {cross((0, 0), (2, 2), (4, 4))} (collinear)",
    ]
    cases: list[tuple[str, Pt, Pt, Pt, Pt]] = [
        ("proper crossing        ", (0, 0), (4, 4), (0, 4), (4, 0)),
        ("touching at an endpoint", (0, 0), (2, 2), (2, 2), (4, 0)),
        ("collinear overlapping  ", (0, 0), (4, 0), (2, 0), (6, 0)),
        ("collinear disjoint     ", (0, 0), (1, 0), (2, 0), (3, 0)),
        ("parallel               ", (0, 0), (4, 0), (0, 1), (4, 1)),
    ]
    for name, a, b, c, d in cases:
        lines.append(f"{name}: intersect = {segments_intersect(a, b, c, d)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    @staticmethod
    def _reference(p1: Pt, p2: Pt, p3: Pt, p4: Pt) -> bool:
        r0, r1 = p2[0] - p1[0], p2[1] - p1[1]
        a, b = p4[0] - p3[0], p4[1] - p3[1]
        qx, qy = p3[0] - p1[0], p3[1] - p1[1]
        den = r0 * b - r1 * a
        if den != 0:
            tn, un = qx * b - qy * a, qx * r1 - qy * r0
            if den > 0:
                return 0 <= tn <= den and 0 <= un <= den
            return den <= tn <= 0 and den <= un <= 0
        if qx * r1 - qy * r0 != 0:
            return False
        ov = lambda a1, a2, b1, b2: min(a1, a2) <= max(b1, b2) and min(b1, b2) <= max(a1, a2)
        return ov(p1[0], p2[0], p3[0], p4[0]) and ov(p1[1], p2[1], p3[1], p4[1])

    def test_turn_directions(self) -> None:
        self.assertGreater(cross((0, 0), (4, 0), (4, 4)), 0)
        self.assertLess(cross((0, 0), (4, 0), (4, -4)), 0)
        self.assertEqual(cross((0, 0), (2, 2), (4, 4)), 0)

    def test_named_cases(self) -> None:
        self.assertTrue(segments_intersect((0, 0), (4, 4), (0, 4), (4, 0)))
        self.assertTrue(segments_intersect((0, 0), (2, 2), (2, 2), (4, 0)))
        self.assertTrue(segments_intersect((0, 0), (4, 0), (2, 0), (6, 0)))
        self.assertFalse(segments_intersect((0, 0), (1, 0), (2, 0), (3, 0)))
        self.assertFalse(segments_intersect((0, 0), (4, 0), (0, 1), (4, 1)))
        self.assertFalse(segments_intersect((0, 0), (1, 1), (2, 2), (3, 0)))

    def test_matches_exact_reference_on_small_grids(self) -> None:
        r = Rng(230)
        for _ in range(5000):
            p = lambda: (r.range(0, 6), r.range(0, 6))
            a, b, c, d = p(), p(), p(), p()
            if a == b or c == d:
                continue
            self.assertEqual(
                segments_intersect(a, b, c, d),
                self._reference(a, b, c, d),
                f"{a} {b} {c} {d}",
            )


if __name__ == "__main__":
    print(demo(), end="")
