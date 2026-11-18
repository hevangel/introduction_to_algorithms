"""k-Means Clustering (Lloyd's Algorithm) — CLRS 33.1 (4e) (4e).

Alternate between assigning points to the nearest center and moving centers to cluster means; cost never increases.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Point = list[float]


def _d2(a: list[float], b: list[float]) -> float:
    return sum((x - y) ** 2 for x, y in zip(a, b))


def init_centers(points: list[Point], k: int, rng: Rng) -> list[Point]:
    """k-means++ seeding: first center uniform, later centers chosen with probability proportional to D^2."""
    centers = [list(points[rng.range(0, len(points) - 1)])]
    while len(centers) < k:
        d = [min(_d2(p, c) for c in centers) for p in points]
        total = sum(d)
        if total == 0.0:
            centers.append(list(points[rng.range(0, len(points) - 1)]))
            continue
        t = rng.f64() * total
        pick = len(d) - 1
        for i, x in enumerate(d):
            if t < x:
                pick = i
                break
            t -= x
        centers.append(list(points[pick]))
    return centers


def cost(points: list[Point], centers: list[Point], assign: list[int]) -> float:
    """Sum of squared distances from each point to its assigned center — the k-means objective."""
    return sum(_d2(p, centers[c]) for p, c in zip(points, assign))


def lloyd(
    points: list[Point],
    k: int,
    rng: Rng,
    max_iter: int,
) -> tuple[list[Point], list[int], list[float]]:
    """LLOYD(points, k): returns (centers, assignment, cost history per iteration)."""
    dim = len(points[0])
    centers = init_centers(points, k, rng)
    assign = [0] * len(points)
    history: list[float] = []
    for _ in range(max_iter):
        changed = False
        for i, p in enumerate(points):
            best = min(range(k), key=lambda c: _d2(p, centers[c]))
            if assign[i] != best:
                assign[i] = best
                changed = True
        history.append(cost(points, centers, assign))
        sums = [[0.0] * dim for _ in range(k)]
        counts = [0] * k
        for p, c in zip(points, assign):
            counts[c] += 1
            for j in range(dim):
                sums[c][j] += p[j]
        for c in range(k):
            if counts[c] > 0:
                centers[c] = [s / counts[c] for s in sums[c]]
        if not changed:
            break
    history.append(cost(points, centers, assign))
    return centers, assign, history


def demo() -> str:
    lines: list[str] = []
    r = Rng(11)
    blobs = [(0.0, 0.0), (10.0, 0.0), (5.0, 9.0)]
    pts: list[Point] = []
    for cx, cy in blobs:
        for _ in range(40):
            pts.append([cx + (r.f64() - 0.5) * 3.0, cy + (r.f64() - 0.5) * 3.0])
    centers, assign, hist = lloyd(pts, 3, r, 50)
    lines.append(f"120 points in 3 blobs; Lloyd's algorithm converged in {len(hist) - 1} iterations")
    rounded = [round(c * 10.0) / 10.0 for c in hist]
    lines.append(f"cost per iteration: {rounded}")
    for i, c in enumerate(centers):
        cnt = sum(1 for a in assign if a == i)
        lines.append(f"center {i}: ({c[0]:.2f}, {c[1]:.2f}) with {cnt} points")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _blobs(self, r: Rng) -> list[Point]:
        pts: list[Point] = []
        for cx, cy in ((0.0, 0.0), (20.0, 0.0), (10.0, 18.0)):
            for _ in range(30):
                pts.append([cx + r.f64() * 2.0, cy + r.f64() * 2.0])
        return pts

    def test_cost_never_increases(self) -> None:
        r = Rng(260)
        for _ in range(20):
            pts = [[r.f64() * 10.0, r.f64() * 10.0] for _ in range(80)]
            _, _, h = lloyd(pts, 4, r, 100)
            self.assertTrue(all(h[i + 1] <= h[i] + 1e-9 for i in range(len(h) - 1)), str(h))

    def test_recovers_well_separated_clusters(self) -> None:
        r = Rng(261)
        for _ in range(10):
            pts = self._blobs(r)
            _, assign, _ = lloyd(pts, 3, r, 100)
            for b in range(3):
                first = assign[b * 30]
                self.assertTrue(all(a == first for a in assign[b * 30 : (b + 1) * 30]), f"blob {b} split")
            ids = sorted({assign[0], assign[30], assign[60]})
            self.assertEqual(len(ids), 3)

    def test_single_cluster_is_the_mean(self) -> None:
        pts = [[0.0], [2.0], [4.0]]
        c, _, _ = lloyd(pts, 1, Rng(1), 10)
        self.assertLess(abs(c[0][0] - 2.0), 1e-12)


if __name__ == "__main__":
    print(demo(), end="")
