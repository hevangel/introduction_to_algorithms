"""Alternative MST Algorithms (MAYBE-MST-A/B/C) — CLRS Problem 23-4 (1e–4e).

Three candidate algorithms: reverse-delete (correct), add-if-acyclic in arbitrary order (wrong), add-edge-and-drop-heaviest-cycle-edge (correct).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Edge = tuple[int, int, int]


def _connected(n: int, edges: list[Edge], keep: list[bool], skip: int | None) -> bool:
    if n == 0:
        return True
    adj: list[list[int]] = [[] for _ in range(n)]
    for i, (u, v, _) in enumerate(edges):
        if keep[i] and i != skip:
            adj[u].append(v)
            adj[v].append(u)
    seen = [False] * n
    seen[0] = True
    st = [0]
    while st:
        u = st.pop()
        for v in adj[u]:
            if not seen[v]:
                seen[v] = True
                st.append(v)
    return all(seen)


def maybe_mst_a(n: int, edges: list[Edge]) -> list[int]:
    """MAYBE-MST-A: sort edges by weight decreasing; remove an edge if the graph stays connected without it (reverse-delete)."""
    order = sorted(range(len(edges)), key=lambda i: edges[i][2], reverse=True)
    keep = [True] * len(edges)
    for i in order:
        if _connected(n, edges, keep, i):
            keep[i] = False
    return [i for i in range(len(edges)) if keep[i]]


def _find(p: list[int], x: int) -> int:
    r = x
    while p[r] != r:
        r = p[r]
    c = x
    while p[c] != r:
        p[c], c = r, p[c]
    return r


def maybe_mst_b(n: int, edges: list[Edge]) -> list[int]:
    """MAYBE-MST-B: scan edges in the given (arbitrary) order, adding each acyclic edge. Returns a spanning tree, not always minimum."""
    p = list(range(n))
    t: list[int] = []
    for i, (u, v, _) in enumerate(edges):
        a, b = _find(p, u), _find(p, v)
        if a != b:
            p[a] = b
            t.append(i)
    return t


def maybe_mst_c(n: int, edges: list[Edge]) -> list[int]:
    """MAYBE-MST-C: scan edges in arbitrary order, add each to T and, if that creates a cycle, delete a maximum-weight edge on the cycle."""
    t: list[int] = []
    for i in range(len(edges)):
        t.append(i)
        adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
        for j in t:
            if j != i:
                u, v, _ = edges[j]
                adj[u].append((v, j))
                adj[v].append((u, j))
        s, goal = edges[i][0], edges[i][1]
        parent: list[tuple[int, int] | None] = [None] * n
        seen = [False] * n
        seen[s] = True
        st = [s]
        while st:
            u = st.pop()
            for v, j in adj[u]:
                if not seen[v]:
                    seen[v] = True
                    parent[v] = (u, j)
                    st.append(v)
        if seen[goal]:
            cycle = [i]
            v = goal
            while parent[v] is not None:
                u, j = parent[v]  # type: ignore[misc]
                cycle.append(j)
                v = u
            heaviest = max(cycle, key=lambda j: edges[j][2])
            t = [j for j in t if j != heaviest]
    return t


def weight(edges: list[Edge], t: list[int]) -> int:
    return sum(edges[i][2] for i in t)


def _kruskal_weight(n: int, edges: list[Edge]) -> int:
    p = list(range(n))
    total = 0
    for u, v, w in sorted(edges, key=lambda e: e[2]):
        a, b = _find(p, u), _find(p, v)
        if a != b:
            p[a] = b
            total += w
    return total


def demo() -> str:
    lines: list[str] = []
    e: list[Edge] = [(0, 1, 9), (1, 2, 5), (0, 2, 1)]
    k = _kruskal_weight(3, e)
    lines.append(f"triangle with weights 9, 5, 1 in that order; MST weight = {k}")
    lines.append(f"MAYBE-MST-A (reverse delete) → {weight(e, maybe_mst_a(3, e))}")
    lines.append(f"MAYBE-MST-B (arbitrary order, add if acyclic) → {weight(e, maybe_mst_b(3, e))}  ← not minimum!")
    lines.append(f"MAYBE-MST-C (add, drop heaviest on cycle) → {weight(e, maybe_mst_c(3, e))}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_a_and_c_are_correct_and_b_is_not(self) -> None:
        r = Rng(351)
        b_wrong = 0
        for _ in range(150):
            n = r.range(2, 8)
            edges: list[Edge] = [(r.range(0, v - 1), v, r.range(1, 20)) for v in range(1, n)]
            for _ in range(r.range(0, 10)):
                a, b = r.range(0, n - 1), r.range(0, n - 1)
                if a != b:
                    edges.append((a, b, r.range(1, 20)))
            r.shuffle(edges)
            best = _kruskal_weight(n, edges)
            a_t, b_t, c_t = maybe_mst_a(n, edges), maybe_mst_b(n, edges), maybe_mst_c(n, edges)
            self.assertEqual(weight(edges, a_t), best)
            self.assertEqual(weight(edges, c_t), best)
            for t in (a_t, b_t, c_t):
                self.assertEqual(len(t), n - 1)
            self.assertGreaterEqual(weight(edges, b_t), best)
            if weight(edges, b_t) > best:
                b_wrong += 1
        self.assertGreater(b_wrong, 10, f"MAYBE-MST-B should fail on many inputs, failed {b_wrong} times")


if __name__ == "__main__":
    print(demo(), end="")
