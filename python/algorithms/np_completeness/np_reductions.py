"""Polynomial-Time Reductions — CLRS 36.3, 36.5 (1e) / 34.3, 34.5 (2e–4e) (1,2,3,4).

3-CNF-SAT ≤ₚ CLIQUE ≤ₚ VERTEX-COVER and HAM-CYCLE ≤ₚ TSP, implemented as instance transformations and tested for equivalence.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Cnf = list[list[int]]
Graph = list[list[bool]]


def sat_to_clique(f: Cnf) -> tuple[Graph, list[tuple[int, int]], int]:
    """3-CNF-SAT <=p CLIQUE (Theorem 34.11)."""
    verts = [(i, l) for i, c in enumerate(f) for l in c]
    n = len(verts)
    g = [[False] * n for _ in range(n)]
    for a in range(n):
        for b in range(n):
            if verts[a][0] != verts[b][0] and verts[a][1] != -verts[b][1]:
                g[a][b] = True
    return (g, verts, len(f))


def clique_to_vertex_cover(g: Graph, k: int) -> tuple[list[tuple[int, int]], int]:
    """CLIQUE <=p VERTEX-COVER (Theorem 34.12)."""
    n = len(g)
    edges = [(u, v) for u in range(n) for v in range(u + 1, n) if not g[u][v]]
    return (edges, n - k)


def hamiltonian_to_tsp(g: Graph) -> list[list[float]]:
    """HAM-CYCLE <=p TSP (Section 34.5.4)."""
    n = len(g)
    return [[0.0 if (i == j or g[i][j]) else 1.0 for j in range(n)] for i in range(n)]


def max_clique(g: Graph) -> int:
    """Maximum clique size by branch-and-bound."""
    best = 0

    def go(cand: list[int], size: int) -> None:
        nonlocal best
        best = max(best, size)
        if size + len(cand) <= best:
            return
        for i, v in enumerate(cand):
            go([u for u in cand[i + 1 :] if g[v][u]], size + 1)

    go(list(range(len(g))), 0)
    return best


def min_vertex_cover(n: int, edges: list[tuple[int, int]]) -> int:
    """Minimum vertex cover size (exhaustive)."""
    return min(
        m.bit_count()
        for m in range(1 << n)
        if all(((m >> u) & 1) or ((m >> v) & 1) for u, v in edges)
    )


def _tsp_min_cost(d: list[list[float]]) -> float:
    n = len(d)
    if n <= 1:
        return 0.0
    full = 1 << (n - 1)
    dp = [[1e18] * (n - 1) for _ in range(full)]
    for j in range(n - 1):
        dp[1 << j][j] = d[0][j + 1]
    for mask in range(1, full):
        for j in range(n - 1):
            if not ((mask >> j) & 1) or dp[mask][j] >= 1e17:
                continue
            for k in range(n - 1):
                if not ((mask >> k) & 1):
                    nm = mask | (1 << k)
                    dp[nm][k] = min(dp[nm][k], dp[mask][j] + d[j + 1][k + 1])
    return min(dp[full - 1][j] + d[j + 1][0] for j in range(n - 1))


def demo() -> str:
    lines: list[str] = []
    f: Cnf = [[1, -2, -3], [-1, 2, 3], [1, 2, 3]]
    g, verts, k = sat_to_clique(f)
    lines.append(f"φ has {k} clauses → graph with {len(verts)} vertices")
    lines.append(f"max clique size = {max_clique(g)} (satisfiable iff it equals k = {k})")
    edges, kc = clique_to_vertex_cover(g, k)
    lines.append(f"complement graph: {len(edges)} edges; vertex-cover size needed = {kc}; true minimum = {min_vertex_cover(len(verts), edges)}")
    ring = [[False] * 5 for _ in range(5)]
    for i in range(5):
        ring[i][(i + 1) % 5] = ring[(i + 1) % 5][i] = True
    d = hamiltonian_to_tsp(ring)
    lines.append(f"5-cycle → TSP instance, optimal tour cost = {_tsp_min_cost(d)} (0 ⇒ Hamiltonian)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _sat(self, n: int, f: Cnf) -> bool:
        return any(
            all(any(((m >> (abs(l) - 1)) & 1 == 1) == (l > 0) for l in c) for c in f)
            for m in range(1 << n)
        )

    def _has_hc(self, g: Graph) -> bool:
        n = len(g)
        used = [False] * n
        used[0] = True

        def go(last: int, depth: int) -> bool:
            if depth == n:
                return g[last][0]
            for v in range(1, n):
                if not used[v] and g[last][v]:
                    used[v] = True
                    if go(v, depth + 1):
                        return True
                    used[v] = False
            return False

        return go(0, 1)

    def test_sat_iff_clique_of_size_k(self) -> None:
        r = Rng(244)
        sat = unsat = 0
        for _ in range(300):
            n, m = r.range(1, 3), r.range(2, 8)
            f = [[r.range(1, n) * (1 if r.range(0, 1) == 0 else -1) for _ in range(3)] for _ in range(m)]
            g, _, k = sat_to_clique(f)
            has_clique = max_clique(g) >= k
            self.assertEqual(has_clique, self._sat(n, f))
            if has_clique:
                sat += 1
            else:
                unsat += 1
        self.assertGreater(sat, 20)
        self.assertGreater(unsat, 20)

    def test_clique_iff_complement_vertex_cover(self) -> None:
        r = Rng(245)
        for _ in range(100):
            n = r.range(2, 9)
            g = [[False] * n for _ in range(n)]
            for i in range(n):
                for j in range(i + 1, n):
                    if r.f64() < 0.5:
                        g[i][j] = g[j][i] = True
            for k in range(n + 1):
                edges, kc = clique_to_vertex_cover(g, k)
                self.assertEqual(max_clique(g) >= k, min_vertex_cover(n, edges) <= kc)

    def test_hamiltonian_iff_zero_cost_tour(self) -> None:
        r = Rng(246)
        for _ in range(100):
            n = r.range(3, 7)
            g = [[False] * n for _ in range(n)]
            for i in range(n):
                for j in range(i + 1, n):
                    if r.f64() < 0.5:
                        g[i][j] = g[j][i] = True
            cost = _tsp_min_cost(hamiltonian_to_tsp(g))
            self.assertEqual(cost == 0.0, self._has_hc(g))


if __name__ == "__main__":
    print(demo(), end="")
