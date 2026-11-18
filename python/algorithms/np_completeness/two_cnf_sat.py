"""2-CNF Satisfiability in Polynomial Time — CLRS Exercise 34.4-7 / 34.4 (1e–4e) (1,2,3,4).

Unlike 3-CNF-SAT, 2-CNF-SAT is in P: build the implication graph, find strongly connected components, and unsatisfiable ⇔ some x and ¬x share a component. O(n + m).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Clause = tuple[int, int]


def _node(lit: int) -> int:
    v = abs(lit) - 1
    return 2 * v + (1 if lit < 0 else 0)


def _scc(adj: list[list[int]]) -> list[int]:
    """Iterative Tarjan's SCC (sinks get smaller component ids)."""
    n = len(adj)
    idx = [0] * n
    low = [0] * n
    on = [False] * n
    stack: list[int] = []
    comp = [0] * n
    counter = 0
    ncomp = 0
    for s in range(n):
        if idx[s] != 0:
            continue
        call = [(s, 0)]
        counter += 1
        idx[s] = low[s] = counter
        stack.append(s)
        on[s] = True
        while call:
            u, pos = call[-1]
            if pos < len(adj[u]):
                call[-1] = (u, pos + 1)
                v = adj[u][pos]
                if idx[v] == 0:
                    counter += 1
                    idx[v] = low[v] = counter
                    stack.append(v)
                    on[v] = True
                    call.append((v, 0))
                elif on[v]:
                    low[u] = min(low[u], idx[v])
            else:
                call.pop()
                if call:
                    low[call[-1][0]] = min(low[call[-1][0]], low[u])
                if low[u] == idx[u]:
                    while True:
                        v = stack.pop()
                        on[v] = False
                        comp[v] = ncomp
                        if v == u:
                            break
                    ncomp += 1
    return comp


def solve_2sat(n: int, clauses: list[Clause]) -> list[bool] | None:
    """Return a satisfying assignment (index 0 <-> x_1) or None."""
    adj: list[list[int]] = [[] for _ in range(2 * n)]
    for a, b in clauses:
        adj[_node(-a)].append(_node(b))
        adj[_node(-b)].append(_node(a))
    comp = _scc(adj)
    val = [False] * n
    for v in range(n):
        if comp[2 * v] == comp[2 * v + 1]:
            return None
        val[v] = comp[2 * v] < comp[2 * v + 1]
    return val


def satisfies(clauses: list[Clause], val: list[bool]) -> bool:
    return all((val[abs(a) - 1] == (a > 0)) or (val[abs(b) - 1] == (b > 0)) for a, b in clauses)


def demo() -> str:
    lines: list[str] = []
    sat = [(1, 2), (-1, 3), (-2, -3), (2, 3)]
    lines.append(f"(x1∨x2)(¬x1∨x3)(¬x2∨¬x3)(x2∨x3): {solve_2sat(3, sat)}")
    unsat = [(1, 1), (-1, -1)]
    lines.append(f"(x1)(¬x1): {solve_2sat(1, unsat)}")
    forced = [(1, 2), (1, -2), (-1, 2), (-1, -2)]
    lines.append(f"all four clauses on x1,x2: {solve_2sat(2, forced)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_agrees_with_brute_force(self) -> None:
        r = Rng(411)
        for _ in range(400):
            n = r.range(1, 7)
            lit = lambda: r.range(1, n) * (1 if r.range(0, 1) == 0 else -1)
            clauses = [(lit(), lit()) for _ in range(r.range(0, 14))]
            brute = any(satisfies(clauses, [bool((m >> i) & 1) for i in range(n)]) for m in range(1 << n))
            sol = solve_2sat(n, clauses)
            if sol is not None:
                self.assertTrue(brute)
                self.assertTrue(satisfies(clauses, sol))
            else:
                self.assertFalse(brute)

    def test_large_chain_of_implications(self) -> None:
        n = 5000
        clauses = [(-i, i + 1) for i in range(1, n)]
        clauses.append((1, 1))
        v = solve_2sat(n, clauses)
        assert v is not None
        self.assertTrue(all(v))
        clauses.append((-n, -n))
        self.assertIsNone(solve_2sat(n, clauses))


if __name__ == "__main__":
    print(demo(), end="")
