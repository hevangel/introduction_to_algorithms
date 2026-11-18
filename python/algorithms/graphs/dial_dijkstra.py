"""Dijkstra with a Bucket Queue (Dial's Algorithm) — CLRS Exercises 24.3-7, 24.3-8 (1e–4e).

For integer weights ≤ W the priority queue can be an array of buckets: O(E + W·V) — O(E) for constant W.
"""

from __future__ import annotations

import heapq
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

INF = (1 << 60)


def dial(adj: list[list[tuple[int, int]]], s: int) -> list[int]:
    """Dijkstra with a bucket queue indexed by distance; weights must be non-negative integers. Unreachable vertices get INF."""
    n = len(adj)
    d = [INF] * n
    d[s] = 0
    buckets: list[list[int]] = [[s]]
    done = [False] * n
    cur = 0
    while cur < len(buckets):
        bucket = buckets[cur]
        while bucket:
            u = bucket.pop()
            if done[u] or d[u] != cur:
                continue
            done[u] = True
            for v, w in adj[u]:
                nd = cur + w
                if nd < d[v]:
                    d[v] = nd
                    if len(buckets) <= nd:
                        buckets.extend([] for _ in range(nd + 1 - len(buckets)))
                    buckets[nd].append(v)
        cur += 1
    return d


def _dijkstra(adj: list[list[tuple[int, int]]], s: int) -> list[int]:
    n = len(adj)
    d = [INF] * n
    d[s] = 0
    pq: list[tuple[int, int]] = [(0, s)]
    while pq:
        du, u = heapq.heappop(pq)
        if du > d[u]:
            continue
        for v, w in adj[u]:
            if du + w < d[v]:
                d[v] = du + w
                heapq.heappush(pq, (d[v], v))
    return d


def demo() -> str:
    lines: list[str] = []
    e = [(0, 1, 2), (0, 2, 5), (1, 2, 1), (1, 3, 4), (2, 3, 1), (3, 4, 3)]
    adj: list[list[tuple[int, int]]] = [[] for _ in range(5)]
    for u, v, w in e:
        adj[u].append((v, w))
    lines.append(f"Dial distances from 0: {dial(adj, 0)}")
    r = Rng(1)
    n = 20_000
    big: list[list[tuple[int, int]]] = [[] for _ in range(n)]
    for u in range(n):
        for _ in range(4):
            big[u].append((r.range(0, n - 1), r.range(0, 10)))
    d = dial(big, 0)
    dd = _dijkstra(big, 0)
    lines.append(f"n = {n}, E = {4 * n}, W = 10: bucket queue equals binary-heap Dijkstra: {d == dd}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_equals_heap_dijkstra_including_zero_weights_and_unreachable_vertices(self) -> None:
        r = Rng(362)
        for _ in range(200):
            n = r.range(1, 20)
            maxw = r.range(0, 12)
            adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
            for _ in range(r.range(0, 50)):
                adj[r.range(0, n - 1)].append((r.range(0, n - 1), r.range(0, maxw)))
            s = r.range(0, n - 1)
            self.assertEqual(dial(adj, s), _dijkstra(adj, s))


if __name__ == "__main__":
    print(demo(), end="")
