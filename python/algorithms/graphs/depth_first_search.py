"""Depth-First Search — CLRS 22.3 (3e) / 20.3 (4e) / 23.3 (1e–2e).

Discovery/finish timestamps, parenthesis structure and tree/back/forward/cross edge classification.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class EdgeKind(Enum):
    TREE = "Tree"
    BACK = "Back"
    FORWARD = "Forward"
    CROSS = "Cross"


@dataclass
class Dfs:
    d: list[int]
    f: list[int]
    parent: list[int | None]
    edges: list[tuple[int, int, EdgeKind]] = field(default_factory=list)
    finish_order: list[int] = field(default_factory=list)


def dfs(adj: list[list[int]]) -> Dfs:
    """DFS(G) with an explicit stack (no recursion, so deep graphs are safe)."""
    n = len(adj)
    color = [0] * n  # 0 white, 1 gray, 2 black
    r = Dfs(d=[0] * n, f=[0] * n, parent=[None] * n)
    time = 0
    for s in range(n):
        if color[s] != 0:
            continue
        time += 1
        r.d[s] = time
        color[s] = 1
        stack = [[s, 0]]
        while stack:
            top = stack[-1]
            u = top[0]
            if top[1] < len(adj[u]):
                v = adj[u][top[1]]
                top[1] += 1
                if color[v] == 0:
                    r.edges.append((u, v, EdgeKind.TREE))
                    r.parent[v] = u
                    time += 1
                    r.d[v] = time
                    color[v] = 1
                    stack.append([v, 0])
                elif color[v] == 1:
                    r.edges.append((u, v, EdgeKind.BACK))
                else:
                    kind = EdgeKind.FORWARD if r.d[u] < r.d[v] else EdgeKind.CROSS
                    r.edges.append((u, v, kind))
            else:
                color[u] = 2
                time += 1
                r.f[u] = time
                r.finish_order.append(u)
                stack.pop()
    return r


def has_cycle(adj: list[list[int]]) -> bool:
    """A directed graph is acyclic iff DFS finds no back edge (Lemma 22.11)."""
    return any(k == EdgeKind.BACK for _, _, k in dfs(adj).edges)


def demo() -> str:
    lines: list[str] = []
    adj = [[1, 3], [4], [4, 5], [1], [3], [5]]
    names = ["u", "v", "w", "x", "y", "z"]
    r = dfs(adj)
    for v in range(6):
        lines.append(f"{names[v]}: discovered {r.d[v]:>2}, finished {r.f[v]:>2}")
    for u, v, k in r.edges:
        lines.append(f"edge {names[u]}→{names[v]}: {k.value}")
    lines.append(f"has a cycle? {has_cycle(adj)}")
    lines.append(f"finish order: {[names[v] for v in r.finish_order]}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_figure_22_4(self) -> None:
        adj = [[1, 3], [4], [4, 5], [1], [3], [5]]
        r = dfs(adj)
        self.assertEqual((r.d, r.f), ([1, 2, 9, 4, 3, 10], [8, 7, 12, 5, 6, 11]))
        count = lambda k: sum(1 for _, _, kind in r.edges if kind == k)
        self.assertEqual(
            (count(EdgeKind.TREE), count(EdgeKind.BACK), count(EdgeKind.FORWARD), count(EdgeKind.CROSS)),
            (4, 2, 1, 1),
        )

    def test_parenthesis_theorem_and_edge_kinds(self) -> None:
        rg = Rng(123)
        for _ in range(50):
            n = rg.range(1, 12)
            adj = [[v for v in range(n) if rg.f64() < 0.2 and (u != v or rg.f64() < 0.2)] for u in range(n)]
            r = dfs(adj)
            for u in range(n):
                for v in range(n):
                    if u == v:
                        continue
                    disjoint = r.f[u] < r.d[v] or r.f[v] < r.d[u]
                    nested = (r.d[u] < r.d[v] and r.f[v] < r.f[u]) or (r.d[v] < r.d[u] and r.f[u] < r.f[v])
                    self.assertTrue(disjoint or nested)
            for u, v, k in r.edges:
                if k in (EdgeKind.TREE, EdgeKind.FORWARD):
                    ok = r.d[u] < r.d[v] and r.f[v] < r.f[u]
                elif k == EdgeKind.BACK:
                    ok = r.d[v] <= r.d[u] and r.f[u] <= r.f[v]
                else:
                    ok = r.f[v] < r.d[u]
                self.assertTrue(ok, f"{u}->{v} {k}")

    def test_cycle_detection(self) -> None:
        self.assertTrue(has_cycle([[1], [2], [0]]))
        self.assertFalse(has_cycle([[1, 2], [2], []]))
        self.assertTrue(has_cycle([[0]]))
        self.assertFalse(has_cycle([]))


if __name__ == "__main__":
    print(demo(), end="")
