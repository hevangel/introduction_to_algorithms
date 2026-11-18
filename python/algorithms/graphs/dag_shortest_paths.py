"""Shortest Paths in a DAG — CLRS 24.2 (3e) / 22.2 (4e) / 25.4 (1e).

Relax edges in topological order: Θ(V+E) even with negative weights; longest paths and critical paths.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

INF = (1 << 60)
Edge = tuple[int, int, int]


def _topo(n: int, edges: list[Edge]) -> list[int] | None:
    indeg = [0] * n
    adj: list[list[int]] = [[] for _ in range(n)]
    for u, v, _ in edges:
        adj[u].append(v)
        indeg[v] += 1
    stack = [v for v in range(n) if indeg[v] == 0]
    order: list[int] = []
    while stack:
        u = stack.pop()
        order.append(u)
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                stack.append(v)
    return order if len(order) == n else None


def dag_shortest_paths(n: int, edges: list[Edge], s: int) -> tuple[list[int], list[int | None]] | None:
    """DAG-SHORTEST-PATHS(G, w, s): None if the graph has a cycle."""
    order = _topo(n, edges)
    if order is None:
        return None
    adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
    for u, v, w in edges:
        adj[u].append((v, w))
    d = [INF] * n
    pi: list[int | None] = [None] * n
    d[s] = 0
    for u in order:
        if d[u] == INF:
            continue
        for v, w in adj[u]:
            if d[u] + w < d[v]:
                d[v] = d[u] + w
                pi[v] = u
    return d, pi


def dag_longest_paths(n: int, edges: list[Edge], s: int) -> list[int | None] | None:
    """Longest paths from s (negate the weights): the critical-path method used in PERT charts. Unreachable = None."""
    neg = [(u, v, -w) for u, v, w in edges]
    res = dag_shortest_paths(n, neg, s)
    if res is None:
        return None
    d, _ = res
    return [None if x == INF else -x for x in d]


def _bellman_ford(n: int, edges: list[Edge], s: int) -> list[int] | None:
    d = [INF] * n
    d[s] = 0
    for _ in range(1, n):
        changed = False
        for u, v, w in edges:
            if d[u] < INF and d[u] + w < d[v]:
                d[v] = d[u] + w
                changed = True
        if not changed:
            break
    for u, v, w in edges:
        if d[u] < INF and d[u] + w < d[v]:
            return None
    return d


def demo() -> str:
    lines: list[str] = []
    # Figure 24.5: r=0 s=1 t=2 x=3 y=4 z=5, source s
    e: list[Edge] = [
        (0, 1, 5), (0, 2, 3), (1, 2, 2), (1, 3, 6), (2, 3, 7),
        (2, 4, 4), (2, 5, 2), (3, 4, -1), (3, 5, 1), (4, 5, -2),
    ]
    d, _ = dag_shortest_paths(6, e, 1)  # type: ignore[misc]
    lines.append(f"shortest distances from s: {['∞' if x == INF else str(x) for x in d]}  (INF = unreachable)")
    lines.append(f"longest (critical) path lengths from s: {dag_longest_paths(6, e, 1)}")
    cyc = dag_shortest_paths(2, [(0, 1, 1), (1, 0, 1)], 0)
    lines.append(f"cyclic graph → {cyc[0] if cyc is not None else None}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_figure_24_5(self) -> None:
        e: list[Edge] = [
            (0, 1, 5), (0, 2, 3), (1, 2, 2), (1, 3, 6), (2, 3, 7),
            (2, 4, 4), (2, 5, 2), (3, 4, -1), (3, 5, 1), (4, 5, -2),
        ]
        res = dag_shortest_paths(6, e, 1)
        self.assertIsNotNone(res)
        d, _ = res  # type: ignore[misc]
        self.assertEqual(d, [INF, 0, 2, 6, 5, 3])

    def test_matches_bellman_ford_on_random_dags(self) -> None:
        r = Rng(131)
        for _ in range(60):
            n = r.range(1, 12)
            perm = list(range(n))
            r.shuffle(perm)
            edges: list[Edge] = []
            for i in range(n):
                for j in range(i + 1, n):
                    if r.f64() < 0.35:
                        edges.append((perm[i], perm[j], r.range(0, 20) - 8))
            s = r.range(0, n - 1)
            d, _ = dag_shortest_paths(n, edges, s)  # type: ignore[misc]
            bf = _bellman_ford(n, edges, s)
            self.assertEqual(d, bf)
            lp = dag_longest_paths(n, edges, s)
            assert lp is not None
            for v in range(n):
                self.assertEqual(lp[v] is not None, d[v] != INF)

    def test_cycle_returns_none(self) -> None:
        self.assertIsNone(dag_shortest_paths(3, [(0, 1, 1), (1, 2, 1), (2, 0, 1)], 0))


if __name__ == "__main__":
    print(demo(), end="")
