"""Convex Layers — CLRS Problem 33-1 (2e–3e) (2,3).

Peel off the convex hull again and again ("onion peeling"): the layers of a point set, computed in O(n²) simply or O(n lg n) with more care.
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


def _cross(p0: Pt, p1: Pt, p2: Pt) -> int:
    return (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1])


def _graham_scan(points: list[Pt]) -> list[Pt]:
    pts = sorted(set(points))
    if len(pts) < 3:
        return pts
    p0 = min(pts, key=lambda p: (p[1], p[0]))
    rest = [p for p in pts if p != p0]
    dist2 = lambda p: (p[0] - p0[0]) ** 2 + (p[1] - p0[1]) ** 2

    def cmp_pts(a: Pt, b: Pt) -> int:
        c = _cross(p0, a, b)
        if c != 0:
            return -1 if c > 0 else 1
        return (dist2(a) > dist2(b)) - (dist2(a) < dist2(b))

    rest.sort(key=cmp_to_key(cmp_pts))
    filtered: list[Pt] = []
    for p in rest:
        if filtered and _cross(p0, filtered[-1], p) == 0:
            filtered[-1] = p
        else:
            filtered.append(p)
    stack = [p0]
    for p in filtered:
        while len(stack) >= 2 and _cross(stack[-2], stack[-1], p) <= 0:
            stack.pop()
        stack.append(p)
    return stack


def _hull_area2(h: list[Pt]) -> int:
    n = len(h)
    return sum(h[i][0] * h[(i + 1) % n][1] - h[(i + 1) % n][0] * h[i][1] for i in range(n))


def convex_layers(points: list[Pt]) -> list[list[Pt]]:
    """Onion-peel convex hulls of a point set."""
    rest = sorted(set(points))
    layers: list[list[Pt]] = []
    while rest:
        layer = _graham_scan(rest)
        if not layer:
            layer = rest[:]
        layer_set = set(layer)
        rest = [p for p in rest if p not in layer_set]
        layers.append(layer)
    return layers


def depths(points: list[Pt]) -> list[int]:
    """Depth of each point (index of its layer), in input order."""
    layers = convex_layers(points)
    layer_of = {p: i for i, l in enumerate(layers) for p in l}
    return [layer_of[p] for p in points]


def demo() -> str:
    pts: list[Pt] = [(0, 0), (10, 0), (10, 10), (0, 10), (2, 2), (8, 2), (8, 8), (2, 8), (5, 5), (4, 6), (6, 4)]
    lines = [
        f"layer {i}: {l} (2×area = {_hull_area2(l)})"
        for i, l in enumerate(convex_layers(pts))
    ]
    lines.append(f"depth of each input point: {depths(pts)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    @staticmethod
    def _random_points(r: Rng, n: int) -> list[Pt]:
        return sorted({(r.range(0, 30), r.range(0, 30)) for _ in range(n)})

    def test_layers_partition_the_points_and_nest(self) -> None:
        r = Rng(405)
        for _ in range(100):
            n = r.range(1, 40)
            pts = self._random_points(r, n)
            layers = convex_layers(pts)
            all_pts = sorted(p for l in layers for p in l)
            self.assertEqual(all_pts, pts)
            for k in range(len(layers) - 1):
                outer, inner = layers[k], layers[k + 1]
                if len(outer) >= 3:
                    for p in inner:
                        for i in range(len(outer)):
                            self.assertGreaterEqual(_cross(outer[i], outer[(i + 1) % len(outer)], p), 0)
                if len(outer) >= 3 and len(inner) >= 3:
                    self.assertLess(_hull_area2(inner), _hull_area2(outer))

    def test_first_layer_is_the_set_of_extreme_points(self) -> None:
        r = Rng(406)
        for _ in range(100):
            n = r.range(3, 14)
            pts = self._random_points(r, n)
            full = _hull_area2(_graham_scan(pts))
            extreme = [
                p
                for p in pts
                if _hull_area2(_graham_scan([q for q in pts if q != p])) < full
            ]
            layer0 = sorted(convex_layers(pts)[0])
            if full > 0:
                self.assertEqual(layer0, extreme)

    def test_nested_squares(self) -> None:
        pts: list[Pt] = [(0, 0), (9, 0), (9, 9), (0, 9), (2, 2), (7, 2), (7, 7), (2, 7), (4, 4), (5, 4), (5, 5), (4, 5)]
        self.assertEqual(len(convex_layers(pts)), 3)


if __name__ == "__main__":
    print(demo(), end="")
