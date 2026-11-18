"""Widest Augmenting Path Max Flow — CLRS Problem 24-6 (4e).

Choose the maximum-bottleneck s-t augmenting path per step via modified Dijkstra in O(E lg C) augmentations.
"""

from __future__ import annotations

import heapq
from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

BIG = (1 << 63) - 1


class FlowNetwork:
    """Flow network with paired residual edges (edge e and its reverse e ^ 1)."""

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

    def net_flow(self, u: int) -> int:
        return sum(self.cap[e ^ 1] if e % 2 == 0 else -self.cap[e] for e in self.adj[u])

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
            b = BIG
            v = t
            while v != s:
                b = min(b, self.cap[pred[v]])
                v = self.to[pred[v] ^ 1]
            v = t
            while v != s:
                e = pred[v]
                self.cap[e] -= b
                self.cap[e ^ 1] += b
                v = self.to[e ^ 1]
            flow += b


def widest_augmenting_path_flow(n: int, edges: list[tuple[int, int, int]], s: int, t: int) -> tuple[int, int, FlowNetwork]:
    """Max s-t flow by repeatedly augmenting along a residual path of maximum bottleneck.

    Returns (max_flow, augmentation_count, final_network).
    """
    net = FlowNetwork(n)
    for u, v, c in edges:
        net.add_edge(u, v, c)
    total_flow = 0
    augmentations = 0
    while True:
        best_bot = [0] * n
        pred_edge = [-1] * n
        best_bot[s] = BIG
        pq: list[tuple[int, int]] = [(-BIG, -s)]  # max-heap via negation
        while pq:
            nb, nu = heapq.heappop(pq)
            b, u = -nb, -nu
            if b < best_bot[u]:
                continue
            if u == t:
                break
            for e in net.adj[u]:
                cap = net.cap[e]
                if cap <= 0:
                    continue
                v = net.to[e]
                cand = min(b, cap)
                if cand > best_bot[v]:
                    best_bot[v] = cand
                    pred_edge[v] = e
                    heapq.heappush(pq, (-cand, -v))
        b = best_bot[t]
        if b == 0:
            break
        v = t
        while v != s:
            e = pred_edge[v]
            net.cap[e] -= b
            net.cap[e ^ 1] += b
            v = net.to[e ^ 1]
        total_flow += b
        augmentations += 1
    return total_flow, augmentations, net


def demo() -> str:
    lines: list[str] = []
    # Large-capacity vs small-capacity bypass to highlight O(E lg C) augmentations.
    edges = [(0, 1, 1000), (1, 2, 1), (0, 2, 1000), (1, 3, 1000), (2, 3, 1000)]
    flow, augs, _ = widest_augmenting_path_flow(4, edges, 0, 3)
    lines.append(f"widest augmenting path: max_flow = {flow} in {augs} augmentations")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_edmonds_karp_on_random_capacitated_graphs(self) -> None:
        r = Rng(2406)
        for _ in range(80):
            n = r.range(2, 14)
            m = r.range(1, 36)
            edges = [(r.range(0, n - 1), r.range(0, n - 1), r.range(1, 10_000)) for _ in range(m)]
            s, t = 0, n - 1
            wap_flow, augs, net_w = widest_augmenting_path_flow(n, edges, s, t)
            ek = FlowNetwork(n)
            for u, v, c in edges:
                ek.add_edge(u, v, c)
            self.assertEqual(wap_flow, ek.edmonds_karp(s, t))
            for u in range(1, n - 1):
                self.assertEqual(net_w.net_flow(u), 0)
            if wap_flow > 0:
                self.assertGreaterEqual(augs, 1)

    def test_avoids_bad_unit_shortcut_diamond(self) -> None:
        edges = [(0, 1, 1_000_000), (0, 2, 1_000_000), (1, 2, 1), (1, 3, 1_000_000), (2, 3, 1_000_000)]
        flow, augs, _ = widest_augmenting_path_flow(4, edges, 0, 3)
        self.assertEqual(flow, 2_000_000)
        self.assertEqual(augs, 2)


if __name__ == "__main__":
    print(demo(), end="")
