"""Randomized Approximation: MAX-3-CNF & MAX-CUT — CLRS 35.4 (2e–4e) (2,3,4).

A random assignment satisfies 7/8 of clauses in expectation; derandomise with conditional expectations.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Cnf = list[list[int]]


def satisfied(f: Cnf, a: list[bool]) -> int:
    return sum(1 for c in f if any(a[abs(l) - 1] == (l > 0) for l in c))


def random_assignment(n: int, rng: Rng) -> list[bool]:
    """RANDOMIZED MAX-3-CNF: fair coin for each variable."""
    return [(rng.next_u64() & 1) == 1 for _ in range(n)]


def derandomized(n: int, f: Cnf) -> list[bool]:
    """Method of conditional expectations: deterministically achieves >= 7m/8."""
    def expect(asg: list[bool | None]) -> float:
        total = 0.0
        for c in f:
            if any(asg[abs(l) - 1] == (l > 0) for l in c):
                total += 1.0
            else:
                free = sum(1 for l in c if asg[abs(l) - 1] is None)
                total += 1.0 - (0.5 ** free)
        return total

    asg: list[bool | None] = [None] * n
    for v in range(n):
        asg[v] = True
        e_true = expect(asg)
        asg[v] = False
        e_false = expect(asg)
        asg[v] = e_true >= e_false
    return [bool(x) for x in asg]


def cut_size(edges: list[tuple[int, int]], side: list[bool]) -> int:
    return sum(1 for u, v in edges if side[u] != side[v])


def local_search_cut(n: int, edges: list[tuple[int, int]]) -> list[bool]:
    """Greedy local search for MAX-CUT: ends with a cut >= |E|/2."""
    side = [False] * n
    while True:
        improved = False
        for v in range(n):
            same = diff = 0
            for a, b in edges:
                if a == v or b == v:
                    other = b if a == v else a
                    if side[other] == side[v]:
                        same += 1
                    else:
                        diff += 1
            if same > diff:
                side[v] = not side[v]
                improved = True
        if not improved:
            return side


def _random_3cnf(r: Rng, n: int, m: int) -> Cnf:
    clauses: Cnf = []
    for _ in range(m):
        vs: list[int] = []
        while len(vs) < 3:
            v = r.range(1, n)
            if v not in vs:
                vs.append(v)
        clauses.append([v if r.range(0, 1) == 0 else -v for v in vs])
    return clauses


def demo() -> str:
    r = Rng(8)
    n, m = 20, 200
    f = _random_3cnf(r, n, m)
    trials = 2000
    avg = sum(satisfied(f, random_assignment(n, r)) for _ in range(trials)) / trials
    edges = [(a, b) for _ in range(60) for a, b in [(r.range(0, 14), r.range(0, 14))] if a != b]
    return "\n".join([
        f"MAX-3-CNF with n={n}, m={m}: random assignment satisfies {avg:.1f} on average (7m/8 = {7 * m // 8})",
        f"derandomised assignment satisfies {satisfied(f, derandomized(n, f))} (≥ 7m/8 = {7 * m // 8})",
        f"MAX-CUT on {len(edges)} edges: local search cut {cut_size(edges, local_search_cut(15, edges))} ≥ |E|/2 = {len(edges) // 2}",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_expectation_is_seven_eighths(self) -> None:
        r = Rng(255)
        f = _random_3cnf(r, 15, 120)
        avg = sum(satisfied(f, random_assignment(15, r)) for _ in range(4000)) / 4000.0
        self.assertLess(abs(avg - 105.0), 2.0)

    def test_derandomization_meets_the_guarantee(self) -> None:
        r = Rng(256)
        for _ in range(100):
            n, m = r.range(3, 12), r.range(1, 60)
            f = _random_3cnf(r, n, m)
            s = satisfied(f, derandomized(n, f))
            self.assertGreaterEqual(8 * s, 7 * m)

    def test_local_search_cuts_at_least_half_the_edges(self) -> None:
        r = Rng(257)
        for _ in range(100):
            n = r.range(2, 12)
            edges = [(a, b) for _ in range(r.range(1, 30)) for a, b in [(r.range(0, n - 1), r.range(0, n - 1))] if a != b]
            c = cut_size(edges, local_search_cut(n, edges))
            self.assertGreaterEqual(2 * c, len(edges))


if __name__ == "__main__":
    print(demo(), end="")
