"""Gabow's Scaling Algorithm — CLRS Problem 24-4 (1e–4e).

Single-source shortest paths with integer weights ≤ W in O(E lg W) by solving the problem on the high-order bits first and reweighting with the result.
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


def _dial(adj: list[list[tuple[int, int]]], s: int) -> list[int]:
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


def gabow(n: int, edges: list[tuple[int, int, int]], s: int) -> list[int | None]:
    """Gabow scaling. edges = (u, v, w) with integer w ≥ 0. Returns distances (None = unreachable)."""
    max_w = max((e[2] for e in edges), default=0)
    k = max(max_w, 1).bit_length()
    reach = [False] * n
    adj0: list[list[int]] = [[] for _ in range(n)]
    for u, v, _ in edges:
        adj0[u].append(v)
    st = [s]
    reach[s] = True
    while st:
        u = st.pop()
        for v in adj0[u]:
            if not reach[v]:
                reach[v] = True
                st.append(v)
    delta = [0] * n
    for i in range(1, k + 1):
        shift = k - i
        adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
        for u, v, w in edges:
            if reach[u]:
                rw = (w >> shift) + 2 * delta[u] - 2 * delta[v]
                assert rw >= 0
                adj[u].append((v, rw))
        dhat = _dial(adj, s)
        for v in range(n):
            if reach[v]:
                delta[v] = 2 * delta[v] + dhat[v]
    return [delta[v] if reach[v] else None for v in range(n)]


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
    e = [(0, 1, 7), (0, 2, 12), (1, 2, 2), (1, 3, 9), (2, 4, 10), (3, 4, 4), (3, 5, 1), (4, 5, 5)]
    lines.append(f"edges {e}")
    lines.append(f"Gabow scaling from 0: {gabow(6, e, 0)}")
    adj: list[list[tuple[int, int]]] = [[] for _ in range(6)]
    for u, v, w in e:
        adj[u].append((v, w))
    lines.append(f"binary-heap Dijkstra  : {_dijkstra(adj, 0)}")
    lines.append("4 scaling phases (one per bit of the largest weight 12), each solved with small integer keys")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_dijkstra(self) -> None:
        r = Rng(363)
        for _ in range(200):
            n = r.range(1, 15)
            maxw = [1, 7, 100, 5000][r.range(0, 3)]
            edges = [(r.range(0, n - 1), r.range(0, n - 1), r.range(0, maxw)) for _ in range(r.range(0, 40))]
            s = r.range(0, n - 1)
            adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
            for u, v, w in edges:
                adj[u].append((v, w))
            dj = _dijkstra(adj, s)
            want = [None if d >= INF else d for d in dj]
            self.assertEqual(gabow(n, edges, s), want, f"{edges} s={s}")

    def test_single_vertex_and_zero_weights(self) -> None:
        self.assertEqual(gabow(1, [], 0), [0])
        self.assertEqual(gabow(3, [(0, 1, 0), (1, 2, 0)], 0), [0, 0, 0])


if __name__ == "__main__":
    print(demo(), end="")
