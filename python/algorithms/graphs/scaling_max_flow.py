"""Maximum Flow by Capacity Scaling — CLRS Problem 26-5 (1e–4e).

Augment only along paths whose residual capacities are all ≥ Δ, halving Δ each phase: O(E² lg C) regardless of the flow value.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class FlowNetwork:
    """Flow network with paired residual edges (edge e and its reverse e ^ 1)."""

    def __init__(self, n: int) -> None:
        self.n = n
        self.to: list[int] = []
        self.cap: list[int] = []
        self.adj: list[list[int]] = [[] for _ in range(n)]

    def copy(self) -> FlowNetwork:
        g = FlowNetwork(self.n)
        g.to = self.to[:]
        g.cap = self.cap[:]
        g.adj = [row[:] for row in self.adj]
        return g

    def add_edge(self, u: int, v: int, c: int) -> int:
        eid = len(self.to)
        self.to.append(v)
        self.cap.append(c)
        self.adj[u].append(eid)
        self.to.append(u)
        self.cap.append(0)
        self.adj[v].append(eid + 1)
        return eid

    def capacity_of(self, eid: int) -> int:
        return self.cap[eid] + self.cap[eid ^ 1]

    def net_flow(self, u: int) -> int:
        return sum(self.cap[e ^ 1] if e % 2 == 0 else -self.cap[e] for e in self.adj[u])

    def _augment_bfs(self, s: int, t: int, threshold: int) -> int:
        """Augment along a BFS path using only residual edges ≥ threshold; returns bottleneck or 0."""
        pred = [-1] * self.n
        seen = [False] * self.n
        seen[s] = True
        q: deque[int] = deque([s])
        while q:
            u = q.popleft()
            for e in self.adj[u]:
                v = self.to[e]
                if not seen[v] and self.cap[e] >= threshold:
                    seen[v] = True
                    pred[v] = e
                    q.append(v)
        if not seen[t]:
            return 0
        b = 1 << 62
        v = t
        while v != s:
            e = pred[v]
            b = min(b, self.cap[e])
            v = self.to[e ^ 1]
        v = t
        while v != s:
            e = pred[v]
            self.cap[e] -= b
            self.cap[e ^ 1] += b
            v = self.to[e ^ 1]
        return b

    def edmonds_karp(self, s: int, t: int) -> int:
        flow = 0
        while True:
            b = self._augment_bfs(s, t, 1)
            if b == 0:
                return flow
            flow += b

    def scaling_max_flow(self, s: int, t: int) -> tuple[int, int]:
        """MAX-FLOW-BY-SCALING: returns (max flow, number of augmentations)."""
        cmax = max((self.capacity_of(e) for e in range(0, len(self.cap), 2)), default=0)
        delta = 1 << (cmax.bit_length() - 1) if cmax > 0 else 0
        flow = 0
        augmentations = 0
        while delta >= 1:
            while True:
                b = self._augment_bfs(s, t, delta)
                if b == 0:
                    break
                flow += b
                augmentations += 1
            delta //= 2
        return flow, augmentations


def clrs_network() -> FlowNetwork:
    """Figure 26.1: s=0, v1=1, v2=2, v3=3, v4=4, t=5."""
    g = FlowNetwork(6)
    for u, v, c in [(0, 1, 16), (0, 2, 13), (1, 3, 12), (2, 1, 4), (2, 4, 14), (3, 2, 9), (3, 5, 20), (4, 3, 7), (4, 5, 4)]:
        g.add_edge(u, v, c)
    return g


def demo() -> str:
    lines: list[str] = []
    # huge capacities: DFS-based Ford–Fulkerson could take ~10⁹ augmentations; scaling needs only a handful
    g = FlowNetwork(4)
    big = 1_000_000_000
    for u, v, c in [(0, 1, big), (0, 2, big), (1, 2, 1), (1, 3, big), (2, 3, big)]:
        g.add_edge(u, v, c)
    f, aug = g.scaling_max_flow(0, 3)
    lines.append(f"classic bad-case network with capacities 10⁹: max flow {f} found with only {aug} augmentations")
    lines.append(f"CLRS Figure 26.1 network: {clrs_network().scaling_max_flow(0, 5)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_equals_edmonds_karp_on_random_networks(self) -> None:
        r = Rng(367)
        for _ in range(100):
            n = r.range(2, 9)
            g = FlowNetwork(n)
            for _ in range(r.range(1, 25)):
                u, v = r.range(0, n - 1), r.range(0, n - 1)
                if u != v:
                    g.add_edge(u, v, r.range(0, 1000))
            a, b = g.copy(), g.copy()
            f, aug = a.scaling_max_flow(0, n - 1)
            self.assertEqual(f, b.edmonds_karp(0, n - 1))
            for v in range(1, n - 1):
                self.assertEqual(a.net_flow(v), 0)
            self.assertLessEqual(aug, 40 * len(g.cap), f"augmentations {aug}")

    def test_few_augmentations_for_huge_capacities(self) -> None:
        g = FlowNetwork(4)
        for u, v, c in [(0, 1, 1 << 40), (0, 2, 1 << 40), (1, 2, 1), (1, 3, 1 << 40), (2, 3, 1 << 40)]:
            g.add_edge(u, v, c)
        f, aug = g.scaling_max_flow(0, 3)
        self.assertEqual(f, 2 << 40)
        self.assertLess(aug, 10, aug)


if __name__ == "__main__":
    print(demo(), end="")
