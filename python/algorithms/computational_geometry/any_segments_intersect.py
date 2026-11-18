"""Sweep Line: Do Any Two Segments Intersect? — CLRS 33.2 (1e–3e) (1,2,3).

Shamos-Hoey sweep: keep segments ordered by y along a moving vertical line and test only neighbours: O(n lg n) probes.
"""

from __future__ import annotations

from bisect import bisect_right
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Pt = tuple[int, int]
Seg = tuple[Pt, Pt]


def _direction(pi: Pt, pj: Pt, pk: Pt) -> int:
    return (pk[0] - pi[0]) * (pj[1] - pi[1]) - (pj[0] - pi[0]) * (pk[1] - pi[1])


def _on_segment(pi: Pt, pj: Pt, pk: Pt) -> bool:
    return (
        min(pi[0], pj[0]) <= pk[0] <= max(pi[0], pj[0])
        and min(pi[1], pj[1]) <= pk[1] <= max(pi[1], pj[1])
    )


def segments_intersect(p1: Pt, p2: Pt, p3: Pt, p4: Pt) -> bool:
    d1, d2 = _direction(p3, p4, p1), _direction(p3, p4, p2)
    d3, d4 = _direction(p1, p2, p3), _direction(p1, p2, p4)
    if ((d1 > 0 and d2 < 0) or (d1 < 0 and d2 > 0)) and (
        (d3 > 0 and d4 < 0) or (d3 < 0 and d4 > 0)
    ):
        return True
    return (
        (d1 == 0 and _on_segment(p3, p4, p1))
        or (d2 == 0 and _on_segment(p3, p4, p2))
        or (d3 == 0 and _on_segment(p1, p2, p3))
        or (d4 == 0 and _on_segment(p1, p2, p4))
    )


def _cmp_at(a: Seg, b: Seg, x: int) -> int:
    def num_den(s: Seg) -> tuple[int, int]:
        (x1, y1), (x2, y2) = s
        den = x2 - x1
        return (y1 * den + (y2 - y1) * (x - x1), den)

    na, da = num_den(a)
    nb, db = num_den(b)
    lhs, rhs = na * db, nb * da
    return (lhs > rhs) - (lhs < rhs)


def any_segments_intersect(segs: list[Seg]) -> bool:
    """ANY-SEGMENTS-INTERSECT(S): O(n lg n) sweep; falls back to O(n^2) if any segment is vertical."""
    s = [(p, q) if p <= q else (q, p) for p, q in segs]
    if any(p[0] == q[0] for p, q in s):
        return any(
            segments_intersect(s[i][0], s[i][1], s[j][0], s[j][1])
            for i in range(len(s))
            for j in range(i + 1, len(s))
        )
    ev: list[tuple[int, int, int, int]] = []
    for i, (p, q) in enumerate(s):
        ev.append((p[0], 0, p[1], i))
        ev.append((q[0], 1, q[1], i))
    ev.sort()
    t: list[int] = []
    hit = lambda a, b: segments_intersect(s[a][0], s[a][1], s[b][0], s[b][1])
    for x, kind, _, i in ev:
        if kind == 0:
            lo, hi = 0, len(t)
            while lo < hi:
                mid = (lo + hi) // 2
                if _cmp_at(s[t[mid]], s[i], x) <= 0:
                    lo = mid + 1
                else:
                    hi = mid
            pos = lo
            if pos > 0 and hit(t[pos - 1], i):
                return True
            if pos < len(t) and hit(t[pos], i):
                return True
            t.insert(pos, i)
        else:
            pos = t.index(i)
            if pos > 0 and pos + 1 < len(t) and hit(t[pos - 1], t[pos + 1]):
                return True
            t.pop(pos)
    return False


def demo() -> str:
    disjoint: list[Seg] = [
        ((0, 0), (10, 1)),
        ((1, 5), (9, 6)),
        ((2, 10), (8, 12)),
        ((0, 20), (10, 22)),
    ]
    crossing = [*disjoint, ((3, -2), (7, 11))]
    return "\n".join([
        f"four stacked non-crossing segments: {any_segments_intersect(disjoint)}",
        f"adding a segment that crosses several: {any_segments_intersect(crossing)}",
        "the sweep keeps at most n segments in the status and probes O(n) neighbour pairs → O(n lg n) with a balanced tree",
    ]) + "\n"


class Tests(unittest.TestCase):
    @staticmethod
    def _brute(s: list[Seg]) -> bool:
        return any(
            segments_intersect(s[i][0], s[i][1], s[j][0], s[j][1])
            for i in range(len(s))
            for j in range(i + 1, len(s))
        )

    def test_agrees_with_all_pairs_on_random_inputs(self) -> None:
        r = Rng(231)
        yes = no = 0
        for _ in range(600):
            n = r.range(1, 9)
            xs = list(range(2 * n))
            r.shuffle(xs)
            segs = [((xs[2 * i], r.range(0, 1000)), (xs[2 * i + 1], r.range(0, 1000))) for i in range(n)]
            want = self._brute(segs)
            self.assertEqual(any_segments_intersect(segs), want, f"{segs}")
            if want:
                yes += 1
            else:
                no += 1
        self.assertGreater(yes, 50)
        self.assertGreater(no, 50)

    def test_simple_cases(self) -> None:
        self.assertFalse(any_segments_intersect([]))
        self.assertFalse(any_segments_intersect([((0, 0), (5, 5))]))
        self.assertTrue(any_segments_intersect([((0, 0), (4, 4)), ((0, 4), (4, 0))]))
        self.assertTrue(any_segments_intersect([((0, 0), (4, 0)), ((2, 0), (2, 3))]))


if __name__ == "__main__":
    print(demo(), end="")
