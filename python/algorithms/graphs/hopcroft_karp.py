"""Hopcroft-Karp Matching — CLRS 25.1 (4e); Problem 26-6 (3e).

Find many shortest vertex-disjoint augmenting paths per phase: O(E√V) maximum bipartite matching.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

NIL = -1


def hopcroft_karp(left: int, right: int, adj: list[list[int]]) -> tuple[int, list[int], list[int]]:
    """Returns (matching size, partner of each left vertex, partner of each right vertex). adj[l] lists right neighbours."""
    ml = [NIL] * left
    mr = [NIL] * right
    size = 0
    inf = 1 << 60

    def dfs(l: int, dist: list[int]) -> bool:
        for r in adj[l]:
            l2 = mr[r]
            if l2 == NIL or (dist[l2] == dist[l] + 1 and dfs(l2, dist)):
                ml[l] = r
                mr[r] = l
                return True
        dist[l] = inf
        return False

    while True:
        dist = [inf] * left
        q: deque[int] = deque()
        for l in range(left):
            if ml[l] == NIL:
                dist[l] = 0
                q.append(l)
        found = False
        while q:
            l = q.popleft()
            for r in adj[l]:
                l2 = mr[r]
                if l2 == NIL:
                    found = True
                elif dist[l2] == inf:
                    dist[l2] = dist[l] + 1
                    q.append(l2)
        if not found:
            break
        for l in range(left):
            if ml[l] == NIL and dfs(l, dist):
                size += 1
    return size, ml, mr


def _flow_matching_size(left: int, right: int, edges: list[tuple[int, int]]) -> int:
    n = left + right + 2
    s, t = left + right, left + right + 1
    to: list[int] = []
    cap: list[int] = []
    adj: list[list[int]] = [[] for _ in range(n)]

    def add(u: int, v: int, c: int) -> None:
        eid = len(to)
        to.extend((v, u))
        cap.extend((c, 0))
        adj[u].append(eid)
        adj[v].append(eid + 1)

    for l in range(left):
        add(s, l, 1)
    for r in range(right):
        add(left + r, t, 1)
    for l, r in edges:
        add(l, left + r, 1)

    flow = 0
    while True:
        pred = [-1] * n
        seen = [False] * n
        seen[s] = True
        q: deque[int] = deque([s])
        while q:
            u = q.popleft()
            for e in adj[u]:
                v = to[e]
                if not seen[v] and cap[e] > 0:
                    seen[v] = True
                    pred[v] = e
                    q.append(v)
        if not seen[t]:
            return flow
        v = t
        while v != s:
            e = pred[v]
            cap[e] -= 1
            cap[e ^ 1] += 1
            v = to[e ^ 1]
        flow += 1


def demo() -> str:
    lines: list[str] = []
    adj = [[0, 1], [0], [1, 2], [2, 3], [3]]
    size, ml, _ = hopcroft_karp(5, 4, adj)
    lines.append(f"adjacency (left → right): {adj}")
    lines.append(f"maximum matching size = {size}")
    for l, r in enumerate(ml):
        if r != NIL:
            lines.append(f"  left {l} — right {r}")
        else:
            lines.append(f"  left {l} unmatched")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_agrees_with_flow_based_matching(self) -> None:
        r = Rng(141)
        for _ in range(100):
            l, rr = r.range(1, 12), r.range(1, 12)
            edges = sorted({(r.range(0, l - 1), r.range(0, rr - 1)) for _ in range(r.range(0, 40))})
            adj: list[list[int]] = [[] for _ in range(l)]
            for a, b in edges:
                adj[a].append(b)
            size, ml, mr = hopcroft_karp(l, rr, adj)
            self.assertEqual(size, _flow_matching_size(l, rr, edges))
            for a in range(l):
                if ml[a] != NIL:
                    self.assertEqual(mr[ml[a]], a)
                    self.assertIn(ml[a], adj[a])
            self.assertEqual(sum(1 for x in ml if x != NIL), size)

    def test_perfect_matching_found(self) -> None:
        n = 50
        adj = [[i, (i + 1) % n] for i in range(n)]
        self.assertEqual(hopcroft_karp(n, n, adj)[0], n)


if __name__ == "__main__":
    print(demo(), end="")
