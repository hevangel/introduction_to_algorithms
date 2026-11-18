"""APSP by Matrix Multiplication — CLRS 25.1 (3e) / 23.1 (4e) / 26.1 (1e).

Shortest paths via (min,+) matrix products: SLOW-ALL-PAIRS Θ(V⁴), FASTER-ALL-PAIRS Θ(V³ lg V).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

INF = (1 << 60)
Matrix = list[list[int]]


def extend_shortest_paths(l_mat: Matrix, w_mat: Matrix) -> Matrix:
    """EXTEND-SHORTEST-PATHS(L, W): the (min, +) product L ⊗ W — Θ(n³)."""
    n = len(l_mat)
    out = [[INF] * n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            best = INF
            for k in range(n):
                if l_mat[i][k] < INF and w_mat[k][j] < INF:
                    cand = l_mat[i][k] + w_mat[k][j]
                    if cand < best:
                        best = cand
            out[i][j] = best
    return out


def slow_all_pairs(w: Matrix) -> Matrix:
    """SLOW-ALL-PAIRS-SHORTEST-PATHS: L^(m) for m = 1 … n−1, one extension at a time — Θ(n⁴)."""
    n = len(w)
    l_mat = [row[:] for row in w]
    for _ in range(2, n):
        l_mat = extend_shortest_paths(l_mat, w)
    return l_mat


def faster_all_pairs(w: Matrix) -> Matrix:
    """FASTER-ALL-PAIRS-SHORTEST-PATHS: repeated squaring, ⌈lg(n−1)⌉ products — Θ(n³ lg n)."""
    n = len(w)
    l_mat = [row[:] for row in w]
    m = 1
    limit = max(0, n - 1)
    while m < limit:
        l_mat = extend_shortest_paths(l_mat, l_mat)
        m *= 2
    return l_mat


def _build_matrix(n: int, edges: list[tuple[int, int, int]]) -> Matrix:
    w = [[INF] * n for _ in range(n)]
    for i in range(n):
        w[i][i] = 0
    for u, v, weight in edges:
        if weight < w[u][v]:
            w[u][v] = weight
    return w


def _floyd_warshall(w: Matrix) -> Matrix:
    n = len(w)
    d = [row[:] for row in w]
    for k in range(n):
        for i in range(n):
            for j in range(n):
                if d[i][k] < INF and d[k][j] < INF and d[i][k] + d[k][j] < d[i][j]:
                    d[i][j] = d[i][k] + d[k][j]
    return d


def demo() -> str:
    lines: list[str] = []
    e = [(0, 1, 3), (0, 2, 8), (0, 4, -4), (1, 3, 1), (1, 4, 7), (2, 1, 4), (3, 0, 2), (3, 2, -5), (4, 3, 6)]
    w = _build_matrix(5, e)
    slow = slow_all_pairs(w)
    fast = faster_all_pairs(w)
    lines.append("SLOW (4 extensions):")
    for r in slow:
        lines.append(f"  {r}")
    lines.append(f"FASTER (2 squarings) gives the same matrix: {slow == fast}")
    lines.append("min-plus product used 2 times vs 3")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_floyd_warshall(self) -> None:
        r = Rng(137)
        for _ in range(30):
            n = r.range(1, 9)
            edges = [(r.range(0, n - 1), r.range(0, n - 1), r.range(0, 20)) for _ in range(r.range(0, 25))]
            w = _build_matrix(n, edges)
            fw = _floyd_warshall(w)
            self.assertEqual(slow_all_pairs(w), fw)
            self.assertEqual(faster_all_pairs(w), fw)

    def test_identity_for_min_plus(self) -> None:
        ident = [[INF] * 3 for _ in range(3)]
        for i in range(3):
            ident[i][i] = 0
        a = [[0, 2, INF], [INF, 0, 5], [1, INF, 0]]
        self.assertEqual(extend_shortest_paths(a, ident), a)
        self.assertEqual(extend_shortest_paths(ident, a), a)


if __name__ == "__main__":
    print(demo(), end="")
