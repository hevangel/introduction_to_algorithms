"""Closest Pair of Points — CLRS 33.4 (1e–3e) (1,2,3).

Divide and conquer on x with a y-sorted strip: only 7 comparisons per point, O(n lg n).
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


def closest_pair_brute(p: list[Pt]) -> tuple[int, Pt, Pt] | None:
    """Theta(n^2) brute force: (squared distance, pair)."""
    best: tuple[int, Pt, Pt] | None = None
    for i in range(len(p)):
        for j in range(i + 1, len(p)):
            d = _d2(p[i], p[j])
            if best is None or d < best[0]:
                best = (d, p[i], p[j])
    return best


def closest_pair(points: list[Pt]) -> tuple[int, Pt, Pt] | None:
    """Divide and conquer: returns (squared distance, a, b) of a closest pair, or None if < 2 points."""
    if len(points) < 2:
        return None
    px = sorted(points)

    def rec(sub: list[Pt]) -> tuple[tuple[int, Pt, Pt], list[Pt]]:
        n = len(sub)
        if n <= 3:
            ys = sorted(sub, key=lambda p: (p[1], p[0]))
            res = closest_pair_brute(sub)
            assert res is not None
            return (res, ys)
        mid = n // 2
        midx = sub[mid][0]
        dl, yl = rec(sub[:mid])
        dr, yr = rec(sub[mid:])
        best = dl if dl[0] <= dr[0] else dr
        ys: list[Pt] = []
        i = j = 0
        while i < len(yl) or j < len(yr):
            if j >= len(yr) or (i < len(yl) and (yl[i][1], yl[i][0]) <= (yr[j][1], yr[j][0])):
                ys.append(yl[i])
                i += 1
            else:
                ys.append(yr[j])
                j += 1
        strip = [p for p in ys if (p[0] - midx) ** 2 < best[0]]
        for a in range(len(strip)):
            for b in range(a + 1, len(strip)):
                if (strip[b][1] - strip[a][1]) ** 2 >= best[0]:
                    break
                d = _d2(strip[a], strip[b])
                if d < best[0]:
                    best = (d, strip[a], strip[b])
        return (best, ys)

    return rec(px)[0]


def demo() -> str:
    pts: list[Pt] = [(2, 3), (12, 30), (40, 50), (5, 1), (12, 10), (3, 4)]
    res = closest_pair(pts)
    assert res is not None
    d, a, b = res
    r = Rng(3)
    big: list[Pt] = [(r.range(0, 1_000_000), r.range(0, 1_000_000)) for _ in range(20_000)]
    d_big = closest_pair(big)
    assert d_big is not None
    return "\n".join([
        f"points {pts}",
        f"closest pair {a} – {b} at distance √{d} = {math.sqrt(d):.4f}",
        f"20000 random points: squared distance {d_big[0]} (n lg n divide and conquer)",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_matches_brute_force(self) -> None:
        r = Rng(234)
        for _ in range(300):
            n = r.range(0, 60)
            pts = [(r.range(0, 50), r.range(0, 50)) for _ in range(n)]
            cp = closest_pair(pts)
            br = closest_pair_brute(pts)
            self.assertEqual(None if cp is None else cp[0], None if br is None else br[0], f"{pts}")

    def test_duplicates_and_degenerate(self) -> None:
        res0 = closest_pair([(1, 1), (5, 5), (1, 1)])
        assert res0 is not None
        self.assertEqual(res0[0], 0)
        self.assertIsNone(closest_pair([(1, 1)]))
        res25 = closest_pair([(0, 0), (3, 4)])
        assert res25 is not None
        self.assertEqual(res25[0], 25)
        line = [(i * 7, 0) for i in range(100)]
        res49 = closest_pair(line)
        assert res49 is not None
        self.assertEqual(res49[0], 49)


if __name__ == "__main__":
    print(demo(), end="")
