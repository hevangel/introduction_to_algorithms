"""Longest Simple Path in a DAG — CLRS Problem 15-1 (1e–4e) (1e,2e,3e,4e).

Dynamic programming over a topological order finds the longest path between two vertices of a DAG in Theta(V+E); in general graphs the problem is NP-hard.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def longest_path(
    n: int,
    edges: list[tuple[int, int, int]],
    s: int,
    t: int,
) -> tuple[int, list[int]] | None:
    """Longest (maximum-weight) path from s to t in a DAG. None if cyclic or t unreachable."""
    indeg = [0] * n
    adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
    for u, v, w in edges:
        adj[u].append((v, w))
        indeg[v] += 1
    order: list[int] = []
    stack = [v for v in range(n) if indeg[v] == 0]
    while stack:
        u = stack.pop()
        order.append(u)
        for v, _ in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                stack.append(v)
    if len(order) != n:
        return None
    best: list[int | None] = [None] * n
    pred = [-1] * n
    best[s] = 0
    for u in order:
        bu = best[u]
        if bu is None:
            continue
        for v, w in adj[u]:
            bv = best[v]
            if bv is None or bu + w > bv:
                best[v] = bu + w
                pred[v] = u
    total = best[t]
    if total is None:
        return None
    path = [t]
    v = t
    while v != s:
        v = pred[v]
        path.append(v)
    path.reverse()
    return total, path


def demo() -> str:
    lines: list[str] = []
    e = [(0, 1, 3), (0, 2, 2), (1, 3, 4), (2, 3, 7), (1, 2, -1), (3, 4, 2), (2, 4, 6)]
    lines.append(f"edges {e}")
    lines.append(f"longest path 0 → 4: {longest_path(5, e, 0, 4)}")
    lines.append(f"cyclic graph: {longest_path(2, [(0, 1, 1), (1, 0, 1)], 0, 1)}")
    lines.append(
        "in a general graph the same question decides Hamiltonian path (NP-hard); "
        "the DAG order removes the cycles"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _brute(self, adj: list[list[tuple[int, int]]], u: int, t: int) -> int | None:
        if u == t:
            return 0
        cands: list[int] = []
        for v, w in adj[u]:
            res = self._brute(adj, v, t)
            if res is not None:
                cands.append(res + w)
        return max(cands) if cands else None

    def test_matches_exhaustive_path_enumeration(self) -> None:
        r = Rng(340)
        for _ in range(200):
            n = r.range(2, 9)
            perm = list(range(n))
            r.shuffle(perm)
            edges: list[tuple[int, int, int]] = []
            for i in range(n):
                for j in range(i + 1, n):
                    if r.f64() < 0.4:
                        edges.append((perm[i], perm[j], r.range(0, 12) - 4))
            adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
            for u, v, w in edges:
                adj[u].append((v, w))
            s, t = r.range(0, n - 1), r.range(0, n - 1)
            got = longest_path(n, edges, s, t)
            self.assertEqual(got[0] if got is not None else None, self._brute(adj, s, t))
            if got is not None:
                w, p = got
                walked = sum(
                    max(e[2] for e in edges if e[0] == p[k] and e[1] == p[k + 1])
                    for k in range(len(p) - 1)
                )
                self.assertEqual(walked, w)

    def test_cycle_detected(self) -> None:
        self.assertIsNone(longest_path(3, [(0, 1, 1), (1, 2, 1), (2, 0, 1)], 0, 2))


if __name__ == "__main__":
    print(demo(), end="")
