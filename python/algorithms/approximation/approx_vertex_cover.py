"""Approximate Vertex Cover — CLRS 37.1 (1e) / 35.1 (2e–4e) (1,2,3,4).

Take both endpoints of a maximal matching: a polynomial-time 2-approximation.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def approx_vertex_cover(n: int, edges: list[tuple[int, int]]) -> list[int]:
    """APPROX-VERTEX-COVER(G): pick an uncovered edge, add both endpoints, repeat."""
    in_cover = [False] * n
    for u, v in edges:
        if not in_cover[u] and not in_cover[v]:
            in_cover[u] = True
            in_cover[v] = True
    return [v for v in range(n) if in_cover[v]]


def matching_chosen(n: int, edges: list[tuple[int, int]]) -> list[tuple[int, int]]:
    """The matching A chosen by the algorithm (|cover| = 2|A|)."""
    used = [False] * n
    m: list[tuple[int, int]] = []
    for u, v in edges:
        if not used[u] and not used[v]:
            used[u] = True
            used[v] = True
            m.append((u, v))
    return m


def _min_vertex_cover(n: int, edges: list[tuple[int, int]]) -> int:
    return min(
        mask.bit_count()
        for mask in range(1 << n)
        if all(((mask >> u) & 1) or ((mask >> v) & 1) for u, v in edges)
    )


def demo() -> str:
    edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (1, 5)]
    c = approx_vertex_cover(7, edges)
    opt = _min_vertex_cover(7, edges)
    return "\n".join([
        f"edges: {edges}",
        f"matching: {matching_chosen(7, edges)}",
        f"approximate cover {c} (size {len(c)}), optimal size {opt} → ratio {len(c) / opt:.2f} ≤ 2",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_valid_cover_within_factor_two(self) -> None:
        r = Rng(250)
        for _ in range(300):
            n = r.range(2, 11)
            edges = [
                (r.range(0, n - 1), r.range(0, n - 1))
                for _ in range(r.range(1, 20))
            ]
            edges = [(u, v) for u, v in edges if u != v]
            if not edges:
                continue
            c = approx_vertex_cover(n, edges)
            c_set = set(c)
            self.assertTrue(all(u in c_set or v in c_set for u, v in edges))
            opt = _min_vertex_cover(n, edges)
            self.assertLessEqual(len(c), 2 * opt)
            self.assertEqual(len(c), 2 * len(matching_chosen(n, edges)))

    def test_worst_case_is_tight_on_complete_bipartite_graphs(self) -> None:
        n = 5
        edges = [(i, n + j) for i in range(n) for j in range(n)]
        self.assertEqual(len(approx_vertex_cover(2 * n, edges)), 2 * n)


if __name__ == "__main__":
    print(demo(), end="")
