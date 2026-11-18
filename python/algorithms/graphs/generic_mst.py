"""The Generic MST Method (Safe Edges) — CLRS 23.1 (1e–3e) / 21.1 (4e).

GENERIC-MST: repeatedly add a light edge crossing a cut that respects the current forest A — Kruskal and Prim are special cases.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Edge = tuple[int, int, int]


def generic_mst(n: int, edges: list[Edge]) -> tuple[int, list[int], list[int]]:
    """GENERIC-MST on a connected graph. Returns (total weight, chosen edge indices, sequence of |S| cut sizes)."""
    a: list[int] = []
    cut_sizes: list[int] = []
    while len(a) + 1 < n:
        in_s = [False] * n
        stack = [0]
        in_s[0] = True
        while stack:
            u = stack.pop()
            for i in a:
                x, y, _ = edges[i]
                if x == u:
                    other = y
                elif y == u:
                    other = x
                else:
                    continue
                if not in_s[other]:
                    in_s[other] = True
                    stack.append(other)
        cut_sizes.append(sum(in_s))
        light = min(
            (i for i in range(len(edges)) if in_s[edges[i][0]] != in_s[edges[i][1]]),
            key=lambda i: edges[i][2],
            default=None,
        )
        if light is None:
            break
        a.append(light)
    return sum(edges[i][2] for i in a), a, cut_sizes


def _kruskal_weight(n: int, edges: list[Edge]) -> int:
    p = list(range(n))

    def find(x: int) -> int:
        while p[x] != x:
            p[x] = p[p[x]]
            x = p[x]
        return x

    total = 0
    for u, v, w in sorted(edges, key=lambda e: e[2]):
        a, b = find(u), find(v)
        if a != b:
            p[a] = b
            total += w
    return total


def demo() -> str:
    lines: list[str] = []
    e: list[Edge] = [
        (0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2),
        (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7),
    ]
    w, a, cuts = generic_mst(9, e)
    lines.append(f"GENERIC-MST total weight {w} (Kruskal: {_kruskal_weight(9, e)})")
    lines.append(f"edges added in order: {[e[i] for i in a]}")
    lines.append(f"size of the vertex set S on the source side of each cut: {cuts}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_equals_kruskal_weight_on_random_connected_graphs(self) -> None:
        r = Rng(350)
        for _ in range(100):
            n = r.range(1, 12)
            edges: list[Edge] = [(r.range(0, v - 1), v, r.range(1, 30)) for v in range(1, n)]
            for _ in range(r.range(0, 20)):
                a, b = r.range(0, n - 1), r.range(0, n - 1)
                if a != b:
                    edges.append((a, b, r.range(1, 30)))
            w, a, _ = generic_mst(n, edges)
            self.assertEqual(w, _kruskal_weight(n, edges))
            self.assertEqual(len(a), max(0, n - 1))

    def test_disconnected_graph_stops_early(self) -> None:
        _, a, _ = generic_mst(4, [(0, 1, 1), (2, 3, 1)])
        self.assertEqual(len(a), 1)


if __name__ == "__main__":
    print(demo(), end="")
