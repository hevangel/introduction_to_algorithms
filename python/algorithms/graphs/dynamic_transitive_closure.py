"""Transitive Closure of a Dynamic Graph — CLRS Problem 25-1 (1e–4e).

Maintain the closure under edge insertions in O(V²) per insertion, and O(V³) total for any sequence.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class IncrementalClosure:
    """The transitive closure (reachability matrix) of a growing directed graph."""

    def __init__(self, n: int) -> None:
        self.n = n
        self.reach: list[list[bool]] = [[i == j for j in range(n)] for i in range(n)]
        self.work = 0

    def reachable(self, u: int, v: int) -> bool:
        return self.reach[u][v]

    def matrix(self) -> list[list[bool]]:
        return self.reach

    def add_edge(self, u: int, v: int) -> None:
        """INSERT-EDGE(u, v): O(V³) total across any sequence of insertions."""
        n = self.n
        reach = self.reach
        for i in range(n):
            self.work += 1
            if reach[i][u] and not reach[i][v]:
                ri = reach[i]
                rv = reach[v]
                for j in range(n):
                    self.work += 1
                    if rv[j]:
                        ri[j] = True


def warshall(n: int, edges: list[tuple[int, int]]) -> list[list[bool]]:
    """Recompute the closure from scratch with Warshall's algorithm (for comparison)."""
    t = [[i == j for j in range(n)] for i in range(n)]
    for u, v in edges:
        t[u][v] = True
    for k in range(n):
        for i in range(n):
            if t[i][k]:
                for j in range(n):
                    if t[k][j]:
                        t[i][j] = True
    return t


def demo() -> str:
    lines: list[str] = []
    c = IncrementalClosure(6)
    for u, v in [(0, 1), (1, 2), (3, 4), (2, 3), (5, 0)]:
        c.add_edge(u, v)
        pairs = sum(1 for a in range(6) for b in range(6) if a != b and c.reachable(a, b))
        lines.append(f"after inserting {u}→{v}: {pairs} reachable ordered pairs")
    r = Rng(1)
    n = 60
    big = IncrementalClosure(n)
    for _ in range(n * n):
        big.add_edge(r.range(0, n - 1), r.range(0, n - 1))
    lines.append(f"{n * n} insertions into a {n}-vertex graph cost {big.work} steps (V³ = {n * n * n})")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_recomputation_after_every_insertion(self) -> None:
        r = Rng(365)
        for _ in range(50):
            n = r.range(1, 10)
            c = IncrementalClosure(n)
            edges: list[tuple[int, int]] = []
            for _ in range(r.range(0, 25)):
                u, v = r.range(0, n - 1), r.range(0, n - 1)
                edges.append((u, v))
                c.add_edge(u, v)
                self.assertEqual(c.matrix(), warshall(n, edges))

    def test_total_work_is_cubic_even_for_many_insertions(self) -> None:
        n = 40
        c = IncrementalClosure(n)
        r = Rng(366)
        inserts = 4 * n * n
        for _ in range(inserts):
            c.add_edge(r.range(0, n - 1), r.range(0, n - 1))
        self.assertLessEqual(c.work, inserts * n + n * n * n, f"{c.work}")


if __name__ == "__main__":
    print(demo(), end="")
