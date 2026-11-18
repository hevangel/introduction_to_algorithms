"""Euler Tours — CLRS Problem 22-3 (3e) / 20-3 (4e) / 23-3 (1e–2e).

Closed walks using every edge exactly once (Hierholzer), for directed and undirected graphs.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def euler_circuit_undirected(n: int, edges: list[tuple[int, int]]) -> list[int] | None:
    """Euler circuit of an undirected multigraph: a vertex sequence (first == last) using each edge once."""
    if not edges:
        return []
    deg = [0] * n
    adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
    for eid, (a, b) in enumerate(edges):
        adj[a].append((b, eid))
        adj[b].append((a, eid))
        deg[a] += 1
        deg[b] += 1
    if any(d % 2 == 1 for d in deg):
        return None
    start = edges[0][0]
    used = [False] * len(edges)
    ptr = [0] * n
    stack = [start]
    circuit: list[int] = []
    while stack:
        u = stack[-1]
        while ptr[u] < len(adj[u]) and used[adj[u][ptr[u]][1]]:
            ptr[u] += 1
        if ptr[u] < len(adj[u]):
            v, eid = adj[u][ptr[u]]
            used[eid] = True
            stack.append(v)
        else:
            circuit.append(u)
            stack.pop()
    if len(circuit) == len(edges) + 1:
        circuit.reverse()
        return circuit
    return None


def euler_circuit_directed(n: int, edges: list[tuple[int, int]]) -> list[int] | None:
    """Euler circuit of a directed multigraph (in-degree = out-degree everywhere, all edges connected)."""
    if not edges:
        return []
    indeg = [0] * n
    outdeg = [0] * n
    adj: list[list[int]] = [[] for _ in range(n)]
    for a, b in edges:
        adj[a].append(b)
        outdeg[a] += 1
        indeg[b] += 1
    if indeg != outdeg:
        return None
    ptr = [0] * n
    stack = [edges[0][0]]
    circuit: list[int] = []
    while stack:
        u = stack[-1]
        if ptr[u] < len(adj[u]):
            v = adj[u][ptr[u]]
            ptr[u] += 1
            stack.append(v)
        else:
            circuit.append(u)
            stack.pop()
    if len(circuit) == len(edges) + 1:
        circuit.reverse()
        return circuit
    return None


def demo() -> str:
    lines: list[str] = []
    bowtie = [(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 2)]
    lines.append(f"undirected bowtie : {euler_circuit_undirected(5, bowtie)}")
    lines.append(f"path graph (odd degrees): {euler_circuit_undirected(3, [(0, 1), (1, 2)])}")
    lines.append(f"two disjoint triangles  : {euler_circuit_undirected(6, [(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)])}")
    digraph = [(0, 1), (1, 2), (2, 0), (0, 3), (3, 0)]
    lines.append(f"directed circuit  : {euler_circuit_directed(4, digraph)}")
    lines.append(f"unbalanced digraph: {euler_circuit_directed(3, [(0, 1), (1, 2)])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _check_undirected(self, n: int, edges: list[tuple[int, int]], c: list[int]) -> None:
        self.assertEqual(len(c), len(edges) + 1)
        self.assertEqual(c[0], c[-1])
        want = sorted((min(a, b), max(a, b)) for a, b in edges)
        got = sorted((min(c[i], c[i + 1]), max(c[i], c[i + 1])) for i in range(len(c) - 1))
        self.assertEqual(got, want)
        _ = n

    def test_undirected_cases(self) -> None:
        bow = [(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 2)]
        res = euler_circuit_undirected(5, bow)
        assert res is not None
        self._check_undirected(5, bow, res)
        self.assertIsNone(euler_circuit_undirected(3, [(0, 1), (1, 2)]))
        self.assertIsNone(euler_circuit_undirected(6, [(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]))
        self.assertEqual(euler_circuit_undirected(3, []), [])

    def test_random_eulerian_graphs(self) -> None:
        r = Rng(127)
        for _ in range(40):
            n = r.range(3, 9)
            edges: list[tuple[int, int]] = []
            for _ in range(r.range(1, 4)):
                cyc = [0] + [r.range(1, n - 1) for _ in range(r.range(2, 5))]
                for w in range(len(cyc)):
                    edges.append((cyc[w], cyc[(w + 1) % len(cyc)]))
            edges = [e for e in edges if e[0] != e[1]]
            if not edges:
                continue
            uc = euler_circuit_undirected(n, edges)
            assert uc is not None
            self._check_undirected(n, edges, uc)
            dc = euler_circuit_directed(n, edges)
            assert dc is not None
            self.assertEqual(len(dc), len(edges) + 1)
            want = sorted(edges)
            got = sorted((dc[i], dc[i + 1]) for i in range(len(dc) - 1))
            self.assertEqual(got, want)

    def test_directed_requires_balance(self) -> None:
        self.assertIsNone(euler_circuit_directed(3, [(0, 1), (1, 2)]))
        self.assertIsNotNone(euler_circuit_directed(2, [(0, 1), (1, 0), (0, 1), (1, 0)]))


if __name__ == "__main__":
    print(demo(), end="")
