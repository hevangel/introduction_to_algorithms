"""Strongly Connected Components — CLRS 22.5 (3e) / 20.5 (4e) / 23.5 (1e–2e) (1,2,3,4).

Kosaraju's two-pass DFS (CLRS) and Tarjan's one-pass algorithm; the component DAG.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _transpose(adj: list[list[int]]) -> list[list[int]]:
    n = len(adj)
    t: list[list[int]] = [[] for _ in range(n)]
    for u, vs in enumerate(adj):
        for v in vs:
            t[v].append(u)
    return t


def strongly_connected_components(adj: list[list[int]]) -> list[list[int]]:
    """STRONGLY-CONNECTED-COMPONENTS(G): DFS(G), then DFS(G^T) in decreasing finish order."""
    n = len(adj)
    color = [0] * n
    finish_order: list[int] = []
    for s in range(n):
        if color[s] != 0:
            continue
        stack: list[tuple[int, int]] = [(s, 0)]
        color[s] = 1
        while stack:
            u, idx = stack[-1]
            if idx < len(adj[u]):
                stack[-1] = (u, idx + 1)
                v = adj[u][idx]
                if color[v] == 0:
                    color[v] = 1
                    stack.append((v, 0))
            else:
                color[u] = 2
                finish_order.append(u)
                stack.pop()

    t = _transpose(adj)
    seen = [False] * n
    comps: list[list[int]] = []
    for s in reversed(finish_order):
        if seen[s]:
            continue
        comp: list[int] = []
        st = [s]
        seen[s] = True
        while st:
            u = st.pop()
            comp.append(u)
            for v in t[u]:
                if not seen[v]:
                    seen[v] = True
                    st.append(v)
        comp.sort()
        comps.append(comp)
    return comps


def tarjan(adj: list[list[int]]) -> list[list[int]]:
    """Tarjan's algorithm (one DFS with low-link values, iterative to avoid recursion depth limits)."""
    n = len(adj)
    idx = [0] * n
    low = [0] * n
    on = [False] * n
    st: list[int] = []
    next_idx = 0
    out: list[list[int]] = []

    for start in range(n):
        if idx[start] != 0:
            continue
        call_stack: list[tuple[int, int]] = [(start, 0)]
        next_idx += 1
        idx[start] = low[start] = next_idx
        st.append(start)
        on[start] = True
        while call_stack:
            u, pos = call_stack[-1]
            if pos < len(adj[u]):
                call_stack[-1] = (u, pos + 1)
                v = adj[u][pos]
                if idx[v] == 0:
                    next_idx += 1
                    idx[v] = low[v] = next_idx
                    st.append(v)
                    on[v] = True
                    call_stack.append((v, 0))
                elif on[v]:
                    low[u] = min(low[u], idx[v])
            else:
                call_stack.pop()
                if call_stack:
                    parent = call_stack[-1][0]
                    low[parent] = min(low[parent], low[u])
                if low[u] == idx[u]:
                    comp: list[int] = []
                    while True:
                        w = st.pop()
                        on[w] = False
                        comp.append(w)
                        if w == u:
                            break
                    comp.sort()
                    out.append(comp)
    return out


def component_graph(adj: list[list[int]], comps: list[list[int]]) -> list[list[int]]:
    """The component graph G^SCC: vertices are component indices, edges connect different components."""
    comp_id = [0] * len(adj)
    for c, vs in enumerate(comps):
        for v in vs:
            comp_id[v] = c
    g: list[list[int]] = [[] for _ in range(len(comps))]
    for u, vs in enumerate(adj):
        for v in vs:
            if comp_id[u] != comp_id[v] and comp_id[v] not in g[comp_id[u]]:
                g[comp_id[u]].append(comp_id[v])
    return g


def demo() -> str:
    lines: list[str] = []
    names = ["a", "b", "c", "d", "e", "f", "g", "h"]
    edges = [
        (0, 1), (1, 2), (1, 4), (1, 5), (2, 3), (2, 6), (3, 2),
        (3, 7), (4, 0), (4, 5), (5, 6), (6, 5), (6, 7), (7, 7),
    ]
    adj: list[list[int]] = [[] for _ in range(8)]
    for a, b in edges:
        adj[a].append(b)
    show = lambda cs: " ".join("{" + ",".join(names[v] for v in c) + "}" for c in cs)
    k = strongly_connected_components(adj)
    lines.append(f"Kosaraju (CLRS): {show(k)}")
    t = tarjan(adj)
    lines.append(f"Tarjan agrees  : {sorted(k) == sorted(t)}")
    lines.append(f"component DAG  : {component_graph(adj, k)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _is_dag(self, adj: list[list[int]]) -> bool:
        n = len(adj)
        indeg = [0] * n
        for vs in adj:
            for v in vs:
                indeg[v] += 1
        q = [u for u in range(n) if indeg[u] == 0]
        visited = 0
        while q:
            u = q.pop()
            visited += 1
            for v in adj[u]:
                indeg[v] -= 1
                if indeg[v] == 0:
                    q.append(v)
        return visited == n

    def test_book_figure(self) -> None:
        edges = [
            (0, 1), (1, 2), (1, 4), (1, 5), (2, 3), (2, 6), (3, 2),
            (3, 7), (4, 0), (4, 5), (5, 6), (6, 5), (6, 7), (7, 7),
        ]
        adj: list[list[int]] = [[] for _ in range(8)]
        for a, b in edges:
            adj[a].append(b)
        c = sorted(strongly_connected_components(adj))
        self.assertEqual(c, [[0, 1, 4], [2, 3], [5, 6], [7]])

    def test_matches_reachability_definition(self) -> None:
        rg = Rng(125)
        for _ in range(60):
            n = rg.range(1, 12)
            adj = [[v for v in range(n) if u != v and rg.f64() < 0.18] for u in range(n)]
            r = [[False] * n for _ in range(n)]
            for u in range(n):
                r[u][u] = True
                for v in adj[u]:
                    r[u][v] = True
            for m in range(n):
                for i in range(n):
                    for j in range(n):
                        if r[i][m] and r[m][j]:
                            r[i][j] = True
            k = sorted(strongly_connected_components(adj))
            t = sorted(tarjan(adj))
            self.assertEqual(k, t)
            label = [0] * n
            for c, vs in enumerate(k):
                for v in vs:
                    label[v] = c
            for u in range(n):
                for v in range(n):
                    self.assertEqual(label[u] == label[v], r[u][v] and r[v][u])
            cg = component_graph(adj, k)
            self.assertTrue(self._is_dag(cg))


if __name__ == "__main__":
    print(demo(), end="")
