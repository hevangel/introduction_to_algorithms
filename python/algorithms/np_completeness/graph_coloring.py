"""Graph Coloring — CLRS Problem 34-3 (1e–4e) (1,2,3,4).

2-colorability is decidable in O(V+E) (bipartite test), 3-coloring and chromatic number are NP-complete; greedy uses at most Δ+1 colors; exact search by backtracking.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def two_color(adj: list[list[int]]) -> list[int] | None:
    """Problem 34-3(b): is the graph 2-colorable (bipartite)? Returns coloring or None."""
    n = len(adj)
    color = [-1] * n
    for s in range(n):
        if color[s] != -1:
            continue
        color[s] = 0
        q = deque([s])
        while q:
            u = q.popleft()
            for v in adj[u]:
                if color[v] == -1:
                    color[v] = 1 - color[u]
                    q.append(v)
                elif color[v] == color[u]:
                    return None
    return color


def greedy_coloring(adj: list[list[int]], order: list[int]) -> list[int]:
    """Greedy coloring in the given vertex order (uses at most Δ + 1 colors)."""
    color = [-1] * len(adj)
    for u in order:
        used = {color[v] for v in adj[u] if color[v] != -1}
        c = 0
        while c in used:
            c += 1
        color[u] = c
    return color


def k_coloring(adj: list[list[int]], k: int) -> list[int] | None:
    """Exact k-coloring by backtracking."""
    n = len(adj)
    color = [-1] * n

    def go(u: int) -> bool:
        if u == n:
            return True
        max_used = max((c for c in color[:u] if c != -1), default=-1) + 1
        for c in range(min(k, max_used + 1)):
            if all(v >= u or color[v] != c for v in adj[u]):
                color[u] = c
                if go(u + 1):
                    return True
        color[u] = -1
        return False

    return color if go(0) else None


def chromatic_number(adj: list[list[int]]) -> int:
    """The chromatic number χ(G) by trying k = 1, 2, ..."""
    if not adj:
        return 0
    k = 1
    while k_coloring(adj, k) is None:
        k += 1
    return k


def is_proper(adj: list[list[int]], color: list[int]) -> bool:
    return all(color[u] != color[v] for u, vs in enumerate(adj) for v in vs)


def demo() -> str:
    lines: list[str] = []
    cycle = lambda n: [[(i + 1) % n, (i + n - 1) % n] for i in range(n)]
    for n in [4, 5, 6, 7]:
        c = cycle(n)
        lines.append(f"C{n}: 2-colorable {two_color(c) is not None}, χ = {chromatic_number(c)}")
    k4 = [[j for j in range(4) if j != i] for i in range(4)]
    lines.append(f"K4: χ = {chromatic_number(k4)}, greedy uses {max(greedy_coloring(k4, [0, 1, 2, 3])) + 1} colors")
    pet: list[list[int]] = [[] for _ in range(10)]
    for i in range(5):
        for a, b in [(i, (i + 1) % 5), (i, i + 5), (i + 5, (i + 2) % 5 + 5)]:
            pet[a].append(b)
            pet[b].append(a)
    lines.append(f"Petersen graph: χ = {chromatic_number(pet)} (bipartite? {two_color(pet) is not None})")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _random_graph(self, r: Rng, n: int, p: float) -> list[list[int]]:
        adj: list[list[int]] = [[] for _ in range(n)]
        for u in range(n):
            for v in range(u + 1, n):
                if r.f64() < p:
                    adj[u].append(v)
                    adj[v].append(u)
        return adj

    def test_exact_coloring_matches_exhaustive_enumeration(self) -> None:
        r = Rng(412)
        for _ in range(150):
            n = r.range(1, 6)
            adj = self._random_graph(r, n, 0.5)
            chi = next(
                k
                for k in range(1, n + 1)
                if any(
                    is_proper(adj, [(code // (k**i)) % k for i in range(n)])
                    for code in range(k**n)
                )
            )
            self.assertEqual(chromatic_number(adj), chi)
            col = k_coloring(adj, chi)
            assert col is not None
            self.assertTrue(is_proper(adj, col) and all(c < chi for c in col))
            self.assertEqual(two_color(adj) is not None, chi <= 2)

    def test_greedy_uses_at_most_max_degree_plus_one(self) -> None:
        r = Rng(413)
        for _ in range(100):
            n = r.range(1, 20)
            adj = self._random_graph(r, n, 0.3)
            order = list(range(n))
            r.shuffle(order)
            col = greedy_coloring(adj, order)
            delta = max(len(a) for a in adj)
            self.assertTrue(is_proper(adj, col))
            self.assertLessEqual(max(col) + 1, delta + 1)

    def test_known_chromatic_numbers(self) -> None:
        cycle = lambda n: [[(i + 1) % n, (i + n - 1) % n] for i in range(n)]
        self.assertEqual((chromatic_number(cycle(6)), chromatic_number(cycle(7))), (2, 3))
        self.assertEqual(chromatic_number([]), 0)


if __name__ == "__main__":
    print(demo(), end="")
