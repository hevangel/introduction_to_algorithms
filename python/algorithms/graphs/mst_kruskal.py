"""Kruskal's Minimum Spanning Tree — CLRS 23.1–23.2 (3e) / 21 (4e) / 24 (1e–2e); Problem 23-1.

Sort edges by weight and add each that joins two components (union-find); second-best MST.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Edge = tuple[int, int, int]


def _find(p: list[int], x: int) -> int:
    r = x
    while p[r] != r:
        r = p[r]
    c = x
    while p[c] != r:
        p[c], c = r, p[c]
    return r


def kruskal(n: int, edges: list[Edge]) -> tuple[int, list[int]]:
    """MST-KRUSKAL: returns (total weight, indices of the chosen edges)."""
    order = sorted(range(len(edges)), key=lambda i: edges[i][2])
    p = list(range(n))
    total = 0
    chosen: list[int] = []
    for i in order:
        u, v, w = edges[i]
        a, b = _find(p, u), _find(p, v)
        if a != b:
            p[a] = b
            total += w
            chosen.append(i)
    return total, chosen


def second_best_mst(n: int, edges: list[Edge]) -> int | None:
    """Problem 23-1: weight of a second-best spanning tree (different from the MST found by Kruskal), O(V²) after MST."""
    total, tree = kruskal(n, edges)
    if len(tree) + 1 != n:
        return None
    adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
    for i in tree:
        u, v, w = edges[i]
        adj[u].append((v, w))
        adj[v].append((u, w))
    neg_inf = -(1 << 60)
    maxe = [[neg_inf] * n for _ in range(n)]
    for s in range(n):
        stack = [(s, -1, neg_inf)]
        while stack:
            u, par, m = stack.pop()
            maxe[s][u] = m
            for v, w in adj[u]:
                if v != par:
                    stack.append((v, u, w if w > m else m))
    in_tree = set(tree)
    cands = [
        total + edges[i][2] - maxe[edges[i][0]][edges[i][1]]
        for i in range(len(edges))
        if i not in in_tree and edges[i][0] != edges[i][1]
    ]
    return min(cands, default=None)


def demo() -> str:
    lines: list[str] = []
    e: list[Edge] = [
        (0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2),
        (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7),
    ]
    w, tree = kruskal(9, e)
    lines.append(f"MST weight = {w}")
    for i in tree:
        lines.append(f"  edge {e[i][0]}-{e[i][1]} (w={e[i][2]})")
    lines.append(f"second-best spanning tree weight = {second_best_mst(9, e)}")
    return "\n".join(lines) + "\n"


def _spanning_trees(n: int, edges: list[Edge]) -> list[tuple[int, list[int]]]:
    m = len(edges)
    out: list[tuple[int, list[int]]] = []
    for mask in range(1 << m):
        if mask.bit_count() != n - 1:
            continue
        ids = [i for i in range(m) if (mask >> i) & 1]
        p = list(range(n))
        ok = True
        for i in ids:
            a, b = _find(p, edges[i][0]), _find(p, edges[i][1])
            if a == b:
                ok = False
                break
            p[a] = b
        if ok:
            out.append((sum(edges[i][2] for i in ids), ids))
    return out


class Tests(unittest.TestCase):
    def test_book_weight_37(self) -> None:
        e: list[Edge] = [
            (0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2),
            (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7),
        ]
        self.assertEqual(kruskal(9, e)[0], 37)

    def test_mst_and_second_best_match_enumeration(self) -> None:
        r = Rng(128)
        for _ in range(40):
            n = r.range(2, 6)
            edges: list[Edge] = [(r.range(0, v - 1), v, r.range(1, 9)) for v in range(1, n)]
            for _ in range(r.range(0, 5)):
                a, b = r.range(0, n - 1), r.range(0, n - 1)
                if a != b:
                    edges.append((a, b, r.range(1, 9)))
            trees = _spanning_trees(n, edges)
            best = min(t[0] for t in trees)
            w, chosen = kruskal(n, edges)
            chosen_sorted = sorted(chosen)
            self.assertEqual(w, best)
            second = min((t[0] for t in trees if t[1] != chosen_sorted), default=None)
            self.assertEqual(second_best_mst(n, edges), second, f"edges {edges}")

    def test_forest_for_disconnected_graph(self) -> None:
        w, t = kruskal(5, [(0, 1, 3), (3, 4, 1)])
        self.assertEqual((w, len(t)), (4, 2))
        self.assertIsNone(second_best_mst(5, [(0, 1, 3), (3, 4, 1)]))


if __name__ == "__main__":
    print(demo(), end="")
