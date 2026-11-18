"""Acyclic Subgraphs — CLRS Problem 17-2 (1e); Problem 16-3 (2e–3e) (1e,2e,3e).

Undirected: a maximum acyclic subgraph is a spanning forest (|V| - #components edges). Directed: choose forward or backward edges of any vertex order to keep >= |E|/2 edges acyclic.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def undirected_max_acyclic(n: int, edges: list[tuple[int, int]]) -> list[int]:
    """Undirected graphs: edge set of a spanning forest (maximum acyclic subgraph), via union-find."""
    p = list(range(n))

    def find(x: int) -> int:
        r = x
        while p[r] != r:
            r = p[r]
        c = x
        while p[c] != r:
            nx = p[c]
            p[c] = r
            c = nx
        return r

    keep: list[int] = []
    for i, (u, v) in enumerate(edges):
        a, b = find(u), find(v)
        if a != b:
            p[a] = b
            keep.append(i)
    return keep


def directed_half_acyclic(edges: list[tuple[int, int]]) -> list[int]:
    """Directed graphs: keep either all forward edges (u < v) or all backward edges (u > v)."""
    fwd = [i for i, (u, v) in enumerate(edges) if u < v]
    bwd = [i for i, (u, v) in enumerate(edges) if u > v]
    return fwd if len(fwd) >= len(bwd) else bwd


def directed_optimum(n: int, edges: list[tuple[int, int]]) -> int:
    """Exact maximum acyclic subgraph of a small digraph: best over all vertex orders."""
    best = 0

    def go(order: list[int], used: int) -> None:
        nonlocal best
        if len(order) == n:
            pos = [0] * n
            for i, v in enumerate(order):
                pos[v] = i
            best = max(best, sum(1 for u, v in edges if pos[u] < pos[v]))
            return
        for v in range(n):
            if not (used >> v & 1):
                order.append(v)
                go(order, used | (1 << v))
                order.pop()

    go([], 0)
    return best


def is_acyclic(n: int, edges: list[tuple[int, int]], keep: list[int]) -> bool:
    """Check if keeping the given edge indices yields a DAG (via Kahn's algorithm)."""
    adj: list[list[int]] = [[] for _ in range(n)]
    indeg = [0] * n
    for i in keep:
        u, v = edges[i]
        adj[u].append(v)
        indeg[v] += 1
    q = deque(v for v in range(n) if indeg[v] == 0)
    visited = 0
    while q:
        u = q.popleft()
        visited += 1
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return visited == n


def _connected_components_count(n: int, edges: list[tuple[int, int]]) -> int:
    p = list(range(n))

    def find(x: int) -> int:
        while p[x] != x:
            p[x] = p[p[x]]
            x = p[x]
        return x

    comps = n
    for u, v in edges:
        a, b = find(u), find(v)
        if a != b:
            p[a] = b
            comps -= 1
    return comps


def demo() -> str:
    lines: list[str] = []
    edges = [(0, 1), (1, 2), (2, 0), (2, 3), (3, 1), (3, 4), (4, 0)]
    keep = directed_half_acyclic(edges)
    lines.append(f"digraph {edges}")
    lines.append(
        f"half-acyclic subgraph keeps {len(keep)} of {len(edges)} edges (>= |E|/2), "
        f"acyclic = {is_acyclic(5, edges, keep)}"
    )
    lines.append(f"true maximum acyclic subgraph has {directed_optimum(5, edges)} edges")
    und = [(0, 1), (1, 2), (2, 0), (3, 4)]
    lines.append(
        f"undirected {und}: spanning forest keeps edges {undirected_max_acyclic(5, und)} "
        f"(n - components = {5 - 2})"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_directed_guarantee_and_acyclicity(self) -> None:
        r = Rng(347)
        for _ in range(150):
            n = r.range(2, 6)
            m = r.range(1, 14)
            edges = [(u, v) for _ in range(m) if (u := r.range(0, n - 1)) != (v := r.range(0, n - 1))]
            if not edges:
                continue
            keep = directed_half_acyclic(edges)
            self.assertTrue(is_acyclic(n, edges, keep))
            self.assertGreaterEqual(2 * len(keep), len(edges))
            opt = directed_optimum(n, edges)
            self.assertLessEqual(len(keep), opt)
            self.assertGreaterEqual(2 * len(keep), opt, f"{len(keep)} vs optimum {opt}")

    def test_undirected_forest_has_n_minus_components_edges(self) -> None:
        r = Rng(348)
        for _ in range(100):
            n = r.range(1, 9)
            m = r.range(0, 15)
            edges = [(u, v) for _ in range(m) if (u := r.range(0, n - 1)) != (v := r.range(0, n - 1))]
            keep = undirected_max_acyclic(n, edges)
            comps = _connected_components_count(n, edges)
            self.assertEqual(len(keep), n - comps)
            for i in range(len(edges)):
                if i not in keep:
                    k2 = keep + [i]
                    sub = [edges[j] for j in k2]
                    self.assertLess(len(undirected_max_acyclic(n, sub)), len(k2))


if __name__ == "__main__":
    print(demo(), end="")
