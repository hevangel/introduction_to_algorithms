"""Bellman-Ford Single-Source Shortest Paths — CLRS 24.1 (3e) / 22.1 (4e) / 25.3 (1e).

Relax every edge V−1 times; detects reachable negative-weight cycles. O(VE).
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


def bellman_ford(n: int, edges: list[Edge], s: int) -> tuple[list[int], list[int | None]] | None:
    """BELLMAN-FORD(G, w, s): (dist, pred) or None if a negative-weight cycle is reachable from s."""
    d = [INF] * n
    pi: list[int | None] = [None] * n
    d[s] = 0
    for _ in range(1, n):
        changed = False
        for u, v, w in edges:
            if d[u] < INF and d[u] + w < d[v]:
                d[v] = d[u] + w
                pi[v] = u
                changed = True
        if not changed:
            break
    for u, v, w in edges:
        if d[u] < INF and d[u] + w < d[v]:
            return None
    return d, pi


def path(pi: list[int | None], s: int, v: int) -> list[int] | None:
    """Shortest path s ⇝ v from the predecessor array."""
    p = [v]
    cur = v
    while cur != s:
        pred = pi[cur]
        if pred is None:
            return None
        cur = pred
        p.append(cur)
        if len(p) > len(pi) + 1:
            return None
    p.reverse()
    return p


def find_negative_cycle(n: int, edges: list[Edge]) -> list[int] | None:
    """Exercise 24.1-6: find a negative-weight cycle anywhere in the graph (as a vertex list), or None."""
    d = [0] * n
    pi = [-1] * n
    last: int | None = None
    for _ in range(n):
        last = None
        for u, v, w in edges:
            if d[u] + w < d[v]:
                d[v] = d[u] + w
                pi[v] = u
                last = v
        if last is None:
            return None
    if last is None:
        return None
    x = last
    for _ in range(n):
        x = pi[x]
    cycle = [x]
    y = pi[x]
    while y != x:
        cycle.append(y)
        y = pi[y]
    cycle.reverse()
    return cycle


def demo() -> str:
    lines: list[str] = []
    # Figure 24.4: s=0 t=1 x=2 y=3 z=4
    e: list[Edge] = [
        (0, 1, 6), (0, 3, 7), (1, 2, 5), (1, 3, 8), (1, 4, -4),
        (2, 1, -2), (3, 2, -3), (3, 4, 9), (4, 0, 2), (4, 2, 7),
    ]
    d, pi = bellman_ford(5, e, 0)  # type: ignore[misc]
    lines.append(f"distances from s: {d}")
    lines.append(f"path s → x: {path(pi, 0, 2)}")
    neg = list(e)
    neg.append((2, 4, -2))
    res = bellman_ford(5, neg, 0)
    lines.append(f"after adding edge x→z of weight −2: {res[0] if res is not None else None}")
    lines.append(f"negative cycle: {find_negative_cycle(5, neg)}")
    return "\n".join(lines) + "\n"


def _cycle_weight(c: list[int], edges: list[Edge]) -> int:
    total = 0
    for i in range(len(c)):
        u, v = c[i], c[(i + 1) % len(c)]
        total += min(w for a, b, w in edges if a == u and b == v)
    return total


class Tests(unittest.TestCase):
    def test_book_figure_24_4(self) -> None:
        e: list[Edge] = [
            (0, 1, 6), (0, 3, 7), (1, 2, 5), (1, 3, 8), (1, 4, -4),
            (2, 1, -2), (3, 2, -3), (3, 4, 9), (4, 0, 2), (4, 2, 7),
        ]
        res = bellman_ford(5, e, 0)
        self.assertIsNotNone(res)
        d, _ = res  # type: ignore[misc]
        self.assertEqual(d, [0, 2, 4, 7, -2])

    def test_negative_cycles_detected_consistently_with_floyd_warshall(self) -> None:
        r = Rng(130)
        for _ in range(80):
            n = r.range(1, 7)
            edges: list[Edge] = [(r.range(0, n - 1), r.range(0, n - 1), r.range(0, 12) - 3) for _ in range(r.range(0, 14))]
            d = [[INF] * n for _ in range(n)]
            for i in range(n):
                d[i][i] = 0
            for u, v, w in edges:
                if w < d[u][v]:
                    d[u][v] = w
            for k in range(n):
                for i in range(n):
                    for j in range(n):
                        if d[i][k] < INF and d[k][j] < INF and d[i][k] + d[k][j] < d[i][j]:
                            d[i][j] = d[i][k] + d[k][j]
            any_neg = any(d[i][i] < 0 for i in range(n))
            cyc = find_negative_cycle(n, edges)
            self.assertEqual(cyc is not None, any_neg, f"{edges}")
            if cyc is not None:
                self.assertLess(_cycle_weight(cyc, edges), 0)
            reach_neg = any(d[0][k] < INF and d[k][k] < 0 for k in range(n))
            res = bellman_ford(n, edges, 0)
            if res is None:
                self.assertTrue(reach_neg)
            else:
                self.assertFalse(reach_neg)
                dist, _ = res
                for v in range(n):
                    self.assertEqual(dist[v], d[0][v])


if __name__ == "__main__":
    print(demo(), end="")
