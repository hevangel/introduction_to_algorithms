"""Picking Up Sticks — CLRS Problem 35-4 (1e); Problem 33-4 (2e–3e) (1,2,3).

Determine in O(n lg n) time whether n overlapping 3D projection sticks can be lifted sequentially without obstruction, and output a valid removal order.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Pt = tuple[int, int]
Pt3 = tuple[int, int, int]


def _direction(pi: Pt, pj: Pt, pk: Pt) -> int:
    return (pk[0] - pi[0]) * (pj[1] - pi[1]) - (pj[0] - pi[0]) * (pk[1] - pi[1])


def _on_segment(pi: Pt, pj: Pt, pk: Pt) -> bool:
    return (
        min(pi[0], pj[0]) <= pk[0] <= max(pi[0], pj[0])
        and min(pi[1], pj[1]) <= pk[1] <= max(pi[1], pj[1])
    )


def _segments_intersect(p1: Pt, p2: Pt, p3: Pt, p4: Pt) -> bool:
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


def pick_up_sticks(n: int, above_below: list[tuple[int, int]]) -> list[int] | None:
    """Given explicit (above, below) blocking relations, find a top-to-bottom pickup order or None."""
    adj: list[list[int]] = [[] for _ in range(n)]
    indeg = [0] * n
    for above, below in above_below:
        adj[above].append(below)
        indeg[below] += 1
    q = deque(i for i in range(n) if indeg[i] == 0)
    order: list[int] = []
    while q:
        u = q.popleft()
        order.append(u)
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return order if len(order) == n else None


def above_at_crossing(a: tuple[Pt3, Pt3], b: tuple[Pt3, Pt3]) -> bool | None:
    """Compare heights z_a and z_b at the 2D crossing of segments a and b."""
    p1, p2 = a
    p3, p4 = b
    if not _segments_intersect((p1[0], p1[1]), (p2[0], p2[1]), (p3[0], p3[1]), (p4[0], p4[1])):
        return None
    rx, ry = p2[0] - p1[0], p2[1] - p1[1]
    sx, sy = p4[0] - p3[0], p4[1] - p3[1]
    qx, qy = p3[0] - p1[0], p3[1] - p1[1]
    den = rx * sy - ry * sx
    if den == 0:
        return None
    t_num = qx * sy - qy * sx
    u_num = qx * ry - qy * rx
    sign = 1 if den > 0 else -1
    abs_den = abs(den)
    za_scaled = p1[2] * abs_den + (t_num * sign) * (p2[2] - p1[2])
    zb_scaled = p3[2] * abs_den + (u_num * sign) * (p4[2] - p3[2])
    if za_scaled > zb_scaled:
        return True
    if zb_scaled > za_scaled:
        return False
    return None


def pick_up_3d_sticks(sticks: list[tuple[Pt3, Pt3]]) -> list[int] | None:
    """3D stick pile solver using x-interval sweep pruning + topological sort."""
    n = len(sticks)
    order_x = sorted((min(a[0], b[0]), max(a[0], b[0]), i) for i, (a, b) in enumerate(sticks))
    edges: list[tuple[int, int]] = []
    for idx_i in range(n):
        _, x1_max, i = order_x[idx_i]
        for x2_min, _, j in order_x[idx_i + 1 :]:
            if x2_min > x1_max:
                break
            a_top = above_at_crossing(sticks[i], sticks[j])
            if a_top is True:
                edges.append((i, j))
            elif a_top is False:
                edges.append((j, i))
    return pick_up_sticks(n, edges)


def demo() -> str:
    acyc = [
        ((0, 0, 1), (4, 4, 1)),
        ((0, 4, 2), (4, 0, 2)),
        ((2, 0, 3), (2, 4, 3)),
    ]
    cyc_edges = [(0, 1), (1, 2), (2, 0)]
    return "\n".join([
        f"acyclic 3D pile removal order = {pick_up_3d_sticks(acyc)}",
        f"cyclic Escher pile removal order = {pick_up_sticks(3, cyc_edges)}",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_valid_3d_layers_lift_in_order_and_pinwheel_cycle_is_rejected(self) -> None:
        r = Rng(3304)
        for _ in range(50):
            n = r.range(1, 20)
            sticks = [
                (
                    (r.range(0, 20), r.range(0, 20), k * 2),
                    (r.range(0, 20) + 1, r.range(0, 20), k * 2),
                )
                for k in range(n)
            ]
            order = pick_up_3d_sticks(sticks)
            self.assertIsNotNone(order)
            assert order is not None
            self.assertEqual(len(order), n)
            pos = [0] * n
            for step, sid in enumerate(order):
                pos[sid] = step
            for i in range(n):
                for j in range(i + 1, n):
                    a_top = above_at_crossing(sticks[i], sticks[j])
                    if a_top is True:
                        self.assertLess(pos[i], pos[j])
                    elif a_top is False:
                        self.assertLess(pos[j], pos[i])

        pinwheel = [
            ((0, 1, 12), (6, 1, 0)),
            ((5, 0, 10), (1, 4, 0)),
            ((1, 0, 0), (1, 5, 10)),
        ]
        self.assertIsNone(pick_up_3d_sticks(pinwheel))


if __name__ == "__main__":
    print(demo(), end="")
