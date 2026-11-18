"""Maximal Layers — CLRS Problem 33-2 (2e–3e) (2,3).

Peel off the maximal (non-dominated) points repeatedly; layer numbers come from one sort and a binary search over the layers' topmost y-values: O(n lg n).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Pt = tuple[int, int]


def maximal_layers(points: list[Pt]) -> list[int]:
    """Point p dominates q if p != q, p.x >= q.x and p.y >= q.y. Layer 0 = maximal points. O(n lg n)."""
    order = sorted(range(len(points)), key=lambda i: points[i], reverse=True)
    top: list[int] = []  # non-increasing in layer index k
    layer = [0] * len(points)
    for i in order:
        y = points[i][1]
        lo, hi = 0, len(top)
        while lo < hi:
            mid = (lo + hi) // 2
            if top[mid] >= y:
                lo = mid + 1
            else:
                hi = mid
        k = lo
        if k == len(top):
            top.append(y)
        else:
            top[k] = max(top[k], y)
        layer[i] = k
    return layer


def maximal_layers_naive(points: list[Pt]) -> list[int]:
    """Quadratic reference: repeatedly remove all maximal points."""
    layer = [-1] * len(points)
    left = list(range(len(points)))
    k = 0
    while left:
        maximal = [
            i
            for i in left
            if not any(
                j != i and points[j][0] >= points[i][0] and points[j][1] >= points[i][1]
                for j in left
            )
        ]
        max_set = set(maximal)
        for i in maximal:
            layer[i] = k
        left = [i for i in left if i not in max_set]
        k += 1
    return layer


def demo() -> str:
    pts: list[Pt] = [(1, 9), (3, 7), (2, 5), (5, 6), (6, 2), (4, 3), (8, 1), (7, 4), (9, 0), (2, 2)]
    return "\n".join([
        f"points {pts}",
        f"layers (fast)  {maximal_layers(pts)}",
        f"layers (naive) {maximal_layers_naive(pts)}",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_fast_equals_naive(self) -> None:
        r = Rng(407)
        for _ in range(300):
            pts = sorted({(r.range(0, 12), r.range(0, 12)) for _ in range(r.range(0, 30))})
            r.shuffle(pts)
            self.assertEqual(maximal_layers(pts), maximal_layers_naive(pts), f"{pts}")

    def test_staircase_has_one_layer_and_chain_has_many(self) -> None:
        stair = [(i, 10 - i) for i in range(10)]
        self.assertTrue(all(l == 0 for l in maximal_layers(stair)))
        chain = [(i, i) for i in range(10)]
        self.assertEqual(maximal_layers(chain), list(reversed(range(10))))


if __name__ == "__main__":
    print(demo(), end="")
