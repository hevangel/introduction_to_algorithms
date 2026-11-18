"""Undirected Global Minimum Cut — CLRS Problem 24-7 (4e).

Find a minimum-weight nonempty vertex bipartition via both the V−1 s-t max-flow reduction and Stoer–Wagner minimum-cut phase contractions.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class _FlowNetwork:
    def __init__(self, n: int) -> None:
        self.n = n
        self.to: list[int] = []
        self.cap: list[int] = []
        self.adj: list[list[int]] = [[] for _ in range(n)]

    def add_edge(self, u: int, v: int, c: int) -> None:
        eid = len(self.to)
        self.to.append(v)
        self.cap.append(c)
        self.adj[u].append(eid)
        self.to.append(u)
        self.cap.append(0)
        self.adj[v].append(eid + 1)

    def edmonds_karp(self, s: int, t: int) -> int:
        flow = 0
        while True:
            pred = [-1] * self.n
            seen = [False] * self.n
            seen[s] = True
            q: deque[int] = deque([s])
            while q:
                u = q.popleft()
                for e in self.adj[u]:
                    v = self.to[e]
                    if not seen[v] and self.cap[e] > 0:
                        seen[v] = True
                        pred[v] = e
                        q.append(v)
            if not seen[t]:
                return flow
            b = 1 << 60
            v = t
            while v != s:
                e = pred[v]
                if self.cap[e] < b:
                    b = self.cap[e]
                v = self.to[e ^ 1]
            v = t
            while v != s:
                e = pred[v]
                self.cap[e] -= b
                self.cap[e ^ 1] += b
                v = self.to[e ^ 1]
            flow += b

    def min_cut_source_side(self, s: int) -> list[bool]:
        seen = [False] * self.n
        seen[s] = True
        st = [s]
        while st:
            u = st.pop()
            for e in self.adj[u]:
                v = self.to[e]
                if not seen[v] and self.cap[e] > 0:
                    seen[v] = True
                    st.append(v)
        return seen


def global_min_cut_flows(n: int, edges: list[tuple[int, int, int]]) -> tuple[int, list[bool]]:
    """Find the global minimum cut of an undirected weighted graph via |V|-1 directed s-t max-flow queries."""
    assert n >= 2
    best_cut = 1 << 60
    best_side = [False] * n
    for t in range(1, n):
        net = _FlowNetwork(n)
        for u, v, w in edges:
            if u != v and w > 0:
                net.add_edge(u, v, w)
                net.add_edge(v, u, w)
        flow = net.edmonds_karp(0, t)
        if flow < best_cut:
            best_cut = flow
            best_side = net.min_cut_source_side(0)
    return best_cut, best_side


def stoer_wagner(n: int, edges: list[tuple[int, int, int]]) -> tuple[int, list[bool]]:
    """Stoer–Wagner global minimum cut via V - 1 maximum-adjacency ordering contraction phases."""
    assert n >= 2
    adj = [[0] * n for _ in range(n)]
    for u, v, w in edges:
        if u != v:
            adj[u][v] += w
            adj[v][u] += w
    alive = [True] * n
    group = [[i] for i in range(n)]
    best_weight = 1 << 60
    best_side = [False] * n

    for phase in range(n - 1):
        in_a = [False] * n
        key = [0] * n
        prev = -1
        last = -1
        remaining = n - phase
        for _ in range(remaining):
            best_u = -1
            best_k = -(1 << 60)
            for u in range(n):
                if alive[u] and not in_a[u] and key[u] > best_k:
                    best_k = key[u]
                    best_u = u
            in_a[best_u] = True
            prev = last
            last = best_u
            for v in range(n):
                if alive[v] and not in_a[v]:
                    key[v] += adj[best_u][v]

        cut_of_phase = key[last]
        if cut_of_phase < best_weight:
            best_weight = cut_of_phase
            best_side = [False] * n
            for orig in group[last]:
                best_side[orig] = True

        alive[last] = False
        group[prev].extend(group[last])
        group[last] = []
        for v in range(n):
            if alive[v] and v != prev:
                adj[prev][v] += adj[last][v]
                adj[v][prev] = adj[prev][v]

    return best_weight, best_side


def demo() -> str:
    lines: list[str] = []
    edges: list[tuple[int, int, int]] = []
    for u in range(4):
        for v in range(u + 1, 4):
            edges.append((u, v, 5))
            edges.append((u + 4, v + 4, 5))
    edges.append((1, 4, 1))
    edges.append((2, 6, 2))
    c_flow, side_f = global_min_cut_flows(8, edges)
    c_sw, side_sw = stoer_wagner(8, edges)
    lines.append(f"V-1 flows global min-cut = {c_flow}, partition = {side_f}")
    lines.append(f"Stoer-Wagner min-cut     = {c_sw}, partition = {side_sw}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_both_algorithms_match_2_to_n_cut_enumeration(self) -> None:
        r = Rng(2407)
        for _ in range(50):
            n = r.range(2, 10)
            m = r.range(0, 22)
            edges = [(r.range(0, n - 1), r.range(0, n - 1), r.range(1, 20)) for _ in range(m)]
            c1, s1 = global_min_cut_flows(n, edges)
            c2, s2 = stoer_wagner(n, edges)

            def eval_cut(side: list[bool]) -> int:
                self.assertTrue(any(side) and any(not b for b in side))
                return sum(w for u, v, w in edges if side[u] != side[v])

            self.assertEqual(eval_cut(s1), c1)
            self.assertEqual(eval_cut(s2), c2)

            brute = 1 << 60
            for mask in range(1, (1 << n) - 1):
                side = [bool((mask >> i) & 1) for i in range(n)]
                w = sum(w for u, v, w in edges if side[u] != side[v])
                if w < brute:
                    brute = w
            self.assertEqual(c1, brute)
            self.assertEqual(c2, brute)


if __name__ == "__main__":
    print(demo(), end="")
