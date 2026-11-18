"""Maximum Bipartite Matching via Flow — CLRS 26.3 (3e) / 24.3 (4e) / 27.3 (1e–2e: 26.3).

Reduce matching to max flow with unit capacities; integrality gives a maximum matching in O(VE).
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

    def add_edge(self, u: int, v: int, c: int) -> int:
        eid = len(self.to)
        self.to.append(v)
        self.cap.append(c)
        self.adj[u].append(eid)
        self.to.append(u)
        self.cap.append(0)
        self.adj[v].append(eid + 1)
        return eid

    def flow_on(self, eid: int) -> int:
        return self.cap[eid ^ 1]

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


def maximum_matching(left: int, right: int, edges: list[tuple[int, int]]) -> list[tuple[int, int]]:
    """Maximum matching in a bipartite graph with left and right vertices; edges are (l, r) pairs."""
    s, t = left + right, left + right + 1
    g = _FlowNetwork(left + right + 2)
    for l in range(left):
        g.add_edge(s, l, 1)
    for r in range(right):
        g.add_edge(left + r, t, 1)
    ids = [g.add_edge(l, left + r, 1) for l, r in edges]
    g.edmonds_karp(s, t)
    return sorted(e for e, eid in zip(edges, ids) if g.flow_on(eid) == 1)


def is_matching(edges: list[tuple[int, int]], m: list[tuple[int, int]], left: int, right: int) -> bool:
    edge_set = set(edges)
    l_used = [False] * left
    r_used = [False] * right
    for a, b in m:
        if (a, b) not in edge_set or l_used[a] or r_used[b]:
            return False
        l_used[a] = True
        r_used[b] = True
    return True


def demo() -> str:
    lines: list[str] = []
    edges = [(0, 0), (0, 1), (1, 0), (1, 2), (2, 1), (2, 2), (2, 3), (3, 3), (3, 4), (4, 3)]
    m = maximum_matching(5, 5, edges)
    lines.append(f"edges: {edges}")
    lines.append(f"maximum matching of size {len(m)}: {m}")
    lines.append(f"valid matching? {is_matching(edges, m, 5, 5)}")
    return "\n".join(lines) + "\n"


def _brute(l: int, edges: list[tuple[int, int]], i: int, used: list[bool]) -> int:
    if i == l:
        return 0
    best = _brute(l, edges, i + 1, used)
    for a, b in edges:
        if a == i and not used[b]:
            used[b] = True
            cand = 1 + _brute(l, edges, i + 1, used)
            if cand > best:
                best = cand
            used[b] = False
    return best


class Tests(unittest.TestCase):
    def test_matches_brute_force(self) -> None:
        r = Rng(140)
        for _ in range(80):
            l, rr = r.range(1, 6), r.range(1, 6)
            edges = sorted({(r.range(0, l - 1), r.range(0, rr - 1)) for _ in range(r.range(0, 14))})
            m = maximum_matching(l, rr, edges)
            self.assertTrue(is_matching(edges, m, l, rr))
            self.assertEqual(len(m), _brute(l, edges, 0, [False] * rr))

    def test_hall_violation_limits_size(self) -> None:
        self.assertEqual(len(maximum_matching(3, 2, [(0, 0), (1, 0), (2, 0)])), 1)
        self.assertEqual(maximum_matching(0, 0, []), [])


if __name__ == "__main__":
    print(demo(), end="")
