"""Satisfiability: Brute Force, DPLL and 2-SAT — CLRS 36.4, Exercise 36.4-7 (1e) / 34.4 (2e–4e) (1,2,3,4).

SAT is NP-complete; compare exhaustive search, DPLL backtracking with unit propagation, and linear-time 2-SAT via SCCs.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Cnf = list[list[int]]


def _verify_cnf(f: Cnf, a: list[bool]) -> bool:
    return all(any(a[abs(l) - 1] == (l > 0) for l in c) for c in f)


def brute_force_sat(n_vars: int, f: Cnf) -> list[bool] | None:
    """Try all 2^n assignments."""
    for m in range(1 << n_vars):
        a = [bool((m >> i) & 1) for i in range(n_vars)]
        if _verify_cnf(f, a):
            return a
    return None


def dpll(n_vars: int, f: Cnf) -> list[bool] | None:
    """DPLL: unit propagation + branching."""
    def solve(cur_f: Cnf, asg: list[bool | None]) -> bool:
        while True:
            nxt: Cnf = []
            unit: int | None = None
            for c in cur_f:
                if any(asg[abs(l) - 1] == (l > 0) for l in c):
                    continue
                rest = [l for l in c if asg[abs(l) - 1] is None]
                if not rest:
                    return False
                if len(rest) == 1:
                    unit = rest[0]
                nxt.append(rest)
            cur_f = nxt
            if not cur_f:
                return True
            if unit is not None:
                asg[abs(unit) - 1] = unit > 0
            else:
                break
        v = abs(cur_f[0][0]) - 1
        for val in (True, False):
            a2 = list(asg)
            a2[v] = val
            if solve(cur_f, a2):
                asg[:] = a2
                return True
        return False

    asg: list[bool | None] = [None] * n_vars
    if solve(f, asg):
        return [bool(x) if x is not None else False for x in asg]
    return None


def _tarjan_scc_ids(adj: list[list[int]]) -> list[int]:
    n = len(adj)
    idx = [0] * n
    low = [0] * n
    on = [False] * n
    st: list[int] = []
    comp = [0] * n
    counter = 0
    ncomp = 0
    for s in range(n):
        if idx[s] != 0:
            continue
        call = [(s, 0)]
        counter += 1
        idx[s] = low[s] = counter
        st.append(s)
        on[s] = True
        while call:
            u, pos = call[-1]
            if pos < len(adj[u]):
                call[-1] = (u, pos + 1)
                v = adj[u][pos]
                if idx[v] == 0:
                    counter += 1
                    idx[v] = low[v] = counter
                    st.append(v)
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
                        w = st.pop()
                        on[w] = False
                        comp[w] = ncomp
                        if w == u:
                            break
                    ncomp += 1
    return comp


def two_sat(n_vars: int, f: Cnf) -> list[bool] | None:
    """2-SAT in O(n + m): implication graph (¬a -> b, ¬b -> a) and Tarjan SCCs."""
    def node(l: int) -> int:
        return 2 * (abs(l) - 1) + (1 if l < 0 else 0)

    adj: list[list[int]] = [[] for _ in range(2 * n_vars)]
    for c in f:
        assert len(c) <= 2, "2-SAT requires clauses with at most two literals"
        a = c[0]
        b = c[1] if len(c) > 1 else c[0]
        adj[node(-a)].append(node(b))
        adj[node(-b)].append(node(a))
    cid = _tarjan_scc_ids(adj)
    out: list[bool] = []
    for v in range(n_vars):
        if cid[2 * v] == cid[2 * v + 1]:
            return None
        out.append(cid[2 * v] < cid[2 * v + 1])
    return out


def demo() -> str:
    lines: list[str] = []
    f: Cnf = [[1, 2, -3], [-1, 3], [-2, 3], [1, -2, -3]]
    lines.append("formula (x1∨x2∨¬x3)(¬x1∨x3)(¬x2∨x3)(x1∨¬x2∨¬x3)")
    lines.append(f"brute force: {brute_force_sat(3, f)}")
    lines.append(f"DPLL       : {dpll(3, f)}")
    unsat: Cnf = [[1, 2], [-1, 2], [1, -2], [-1, -2]]
    lines.append(f"unsatisfiable (all four 2-clauses over x1,x2): DPLL = {dpll(2, unsat)}, 2-SAT = {two_sat(2, unsat)}")
    two: Cnf = [[1, 2], [-1, 3], [-2, -3], [3, 4], [-4, 1]]
    lines.append(f"2-SAT instance → {two_sat(4, two)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _random_cnf(self, r: Rng, n: int, m: int, width: int) -> Cnf:
        out: Cnf = []
        for _ in range(m):
            c = []
            for _ in range(r.range(1, width)):
                v = r.range(1, n)
                c.append(v if r.range(0, 1) == 0 else -v)
            out.append(c)
        return out

    def test_dpll_agrees_with_brute_force(self) -> None:
        r = Rng(242)
        sat = unsat = 0
        for _ in range(400):
            n, m = r.range(1, 7), r.range(1, 14)
            f = self._random_cnf(r, n, m, 3)
            b, d = brute_force_sat(n, f), dpll(n, f)
            self.assertEqual(b is not None, d is not None)
            if d is not None:
                self.assertTrue(_verify_cnf(f, d))
                sat += 1
            else:
                unsat += 1
        self.assertGreater(sat, 40)
        self.assertGreater(unsat, 40)

    def test_two_sat_matches_brute_force(self) -> None:
        r = Rng(243)
        sat = unsat = 0
        for _ in range(400):
            n, m = r.range(1, 8), r.range(1, 12)
            f = self._random_cnf(r, n, m, 2)
            t = two_sat(n, f)
            self.assertEqual(t is not None, brute_force_sat(n, f) is not None)
            if t is not None:
                self.assertTrue(_verify_cnf(f, t))
                sat += 1
            else:
                unsat += 1
        self.assertGreater(sat, 40)
        self.assertGreater(unsat, 40)


if __name__ == "__main__":
    print(demo(), end="")
