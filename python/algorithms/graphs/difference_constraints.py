"""Difference Constraints — CLRS 24.4 (3e) / 22.4 (4e) / 25.5 (1e).

Solve systems xⱼ − xᵢ ≤ w by shortest paths in a constraint graph (Bellman-Ford).
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

INF = (1 << 60)


@dataclass(frozen=True)
class Constraint:
    """A constraint x[j] − x[i] ≤ w."""
    j: int
    i: int
    w: int


def _bellman_ford(n: int, edges: list[tuple[int, int, int]], s: int) -> list[int] | None:
    d = [INF] * n
    d[s] = 0
    for _ in range(1, n):
        changed = False
        for u, v, w in edges:
            if d[u] < INF and d[u] + w < d[v]:
                d[v] = d[u] + w
                changed = True
        if not changed:
            break
    for u, v, w in edges:
        if d[u] < INF and d[u] + w < d[v]:
            return None
    return d


def solve(n: int, cs: list[Constraint]) -> list[int] | None:
    """Returns a feasible solution (x₁ … xₙ) or None if the system is infeasible."""
    edges = [(c.i, c.j, c.w) for c in cs]
    for v in range(n):
        edges.append((n, v, 0))
    d = _bellman_ford(n + 1, edges, n)
    return d[:n] if d is not None else None


def satisfies(x: list[int], cs: list[Constraint]) -> bool:
    return all(x[c.j] - x[c.i] <= c.w for c in cs)


def demo() -> str:
    lines: list[str] = []
    cs = [
        Constraint(0, 1, 0), Constraint(0, 4, -1), Constraint(1, 4, 1), Constraint(2, 0, 5),
        Constraint(3, 0, 4), Constraint(3, 2, -1), Constraint(4, 2, -3), Constraint(4, 3, -3),
    ]
    x = solve(5, cs)
    assert x is not None
    lines.append(f"solution x = {x}")
    lines.append(f"all 8 constraints satisfied: {satisfies(x, cs)}")
    shifted = [v + 10 for v in x]
    lines.append(f"adding a constant to every variable stays feasible: {satisfies(shifted, cs)}")
    bad = [Constraint(0, 1, -1), Constraint(1, 0, 0)]
    lines.append(f"x0−x1 ≤ −1 and x1−x0 ≤ 0 → {solve(2, bad)} (negative cycle)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_system(self) -> None:
        cs = [
            Constraint(0, 1, 0), Constraint(0, 4, -1), Constraint(1, 4, 1), Constraint(2, 0, 5),
            Constraint(3, 0, 4), Constraint(3, 2, -1), Constraint(4, 2, -3), Constraint(4, 3, -3),
        ]
        self.assertEqual(solve(5, cs), [-5, -3, 0, -1, -4])

    def test_solutions_satisfy_and_infeasible_detected(self) -> None:
        r = Rng(133)
        for _ in range(80):
            n = r.range(1, 6)
            cs = [Constraint(r.range(0, n - 1), r.range(0, n - 1), r.range(0, 14) - 4) for _ in range(r.range(0, 10))]
            x = solve(n, cs)
            if x is not None:
                self.assertTrue(satisfies(x, cs))
            elif n <= 3:
                rng_b = range(-12, 13) if n > 1 else [0]
                rng_c = range(-12, 13) if n > 2 else [0]
                found = False
                for a in range(-12, 13):
                    for b in rng_b:
                        for c in rng_c:
                            if satisfies([a, b, c][:n], cs):
                                found = True
                self.assertFalse(found, f"{cs}")


if __name__ == "__main__":
    print(demo(), end="")
