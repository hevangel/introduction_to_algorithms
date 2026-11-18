"""Articulation Points, Bridges & Biconnected Components — CLRS Problem 22-2 (3e) / 20-2 (4e) / 23-2 (1e–2e).

Low-link DFS finds cut vertices, bridges and the biconnected components of an undirected graph in Θ(V+E).
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


@dataclass
class AnalysisResult:
    articulation_points: list[int]
    bridges: list[int]
    components: list[list[int]]


def analyze(n: int, edges: list[tuple[int, int]]) -> AnalysisResult:
    """Analyse an undirected multigraph given as an edge list (self-loops are ignored)."""
    adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
    for eid, (a, b) in enumerate(edges):
        if a != b:
            adj[a].append((b, eid))
            adj[b].append((a, eid))

    disc = [0] * n
    low = [0] * n
    t = 0
    estack: list[int] = []
    ap = [False] * n
    bridges: list[int] = []
    comps: list[list[int]] = []

    def go(u: int, pe: int) -> None:
        nonlocal t
        t += 1
        disc[u] = low[u] = t
        children = 0
        for v, eid in adj[u]:
            if eid == pe:
                continue
            if disc[v] == 0:
                children += 1
                estack.append(eid)
                go(v, eid)
                if low[v] < low[u]:
                    low[u] = low[v]
                if low[v] >= disc[u]:
                    if pe != -1:
                        ap[u] = True
                    comp: list[int] = []
                    while True:
                        e = estack.pop()
                        comp.append(e)
                        if e == eid:
                            break
                    comp.sort()
                    comps.append(comp)
                if low[v] > disc[u]:
                    bridges.append(eid)
            elif disc[v] < disc[u]:
                estack.append(eid)
                if disc[v] < low[u]:
                    low[u] = disc[v]
        if pe == -1 and children > 1:
            ap[u] = True

    for s in range(n):
        if disc[s] == 0:
            go(s, -1)

    bridges.sort()
    comps.sort()
    return AnalysisResult(
        articulation_points=[v for v in range(n) if ap[v]],
        bridges=bridges,
        components=comps,
    )


def demo() -> str:
    lines: list[str] = []
    edges = [(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 5), (5, 3), (5, 6)]
    r = analyze(7, edges)
    lines.append(f"edges: {edges}")
    lines.append(f"articulation points: {r.articulation_points}")
    lines.append(f"bridges            : {[edges[b] for b in r.bridges]}")
    for c in r.components:
        lines.append(f"biconnected component: {[edges[e] for e in c]}")
    return "\n".join(lines) + "\n"


def _components_count(n: int, edges: list[tuple[int, int]], skip_v: int | None, skip_e: int | None) -> int:
    p = list(range(n))

    def f(x: int) -> int:
        while p[x] != x:
            p[x] = p[p[x]]
            x = p[x]
        return x

    for i, (a, b) in enumerate(edges):
        if i == skip_e or a == skip_v or b == skip_v:
            continue
        x, y = f(a), f(b)
        p[x] = y
    return sum(1 for v in range(n) if v != skip_v and f(v) == v)


class Tests(unittest.TestCase):
    def test_book_style_example(self) -> None:
        r = analyze(7, [(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 5), (5, 3), (5, 6)])
        self.assertEqual(r.articulation_points, [2, 3, 5])
        self.assertEqual(r.bridges, [3, 7])
        self.assertEqual(len(r.components), 4)

    def test_matches_removal_definition_on_random_graphs(self) -> None:
        rg = Rng(126)
        for _ in range(80):
            n = rg.range(1, 9)
            edges = [(rg.range(0, n - 1), rg.range(0, n - 1)) for _ in range(rg.range(0, 14))]
            r = analyze(n, edges)
            base = _components_count(n, edges, None, None)
            for v in range(n):
                is_ap = _components_count(n, edges, v, None) > base
                self.assertEqual(v in r.articulation_points, is_ap, f"vertex {v} in {edges}")
            for e in range(len(edges)):
                if edges[e][0] == edges[e][1]:
                    continue
                is_bridge = _components_count(n, edges, None, e) > base
                self.assertEqual(e in r.bridges, is_bridge, f"edge {e} in {edges}")
            seen = sorted(e for c in r.components for e in c)
            want = [e for e in range(len(edges)) if edges[e][0] != edges[e][1]]
            self.assertEqual(seen, want)


if __name__ == "__main__":
    print(demo(), end="")
