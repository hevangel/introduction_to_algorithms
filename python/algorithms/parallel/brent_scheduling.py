"""Greedy Scheduling and Brent's Theorem — CLRS 27.1 (3e) / 26.1 (4e) (Editions: 3,4).

A greedy scheduler runs min(P, #ready) strands each step, achieving
T_P <= T_1/P + T_inf (Theorem 27.1), within a factor 2 of optimal.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def analyse(deps: list[list[int]], p: int) -> tuple[int, int, int]:
    """Computation DAG of unit-time strands: `deps[v]` lists strands finishing before v (indices < v).

    Returns (T_1 = work, T_inf = span, T_P of a greedy schedule).
    """
    n = len(deps)
    work = n
    depth = [1] * n
    for v in range(n):
        for u in deps[v]:
            depth[v] = max(depth[v], depth[u] + 1)
    span = max(depth, default=0)

    remaining = [len(d) for d in deps]
    succ: list[list[int]] = [[] for _ in range(n)]
    for v in range(n):
        for u in deps[v]:
            succ[u].append(v)
    ready: deque[int] = deque(v for v in range(n) if remaining[v] == 0)
    done = 0
    steps = 0
    while done < n:
        batch = [ready.popleft() for _ in range(min(p, len(ready)))]
        steps += 1
        done += len(batch)
        for u in batch:
            for v in succ[u]:
                remaining[v] -= 1
                if remaining[v] == 0:
                    ready.append(v)
    return work, span, steps


def demo() -> str:
    lines: list[str] = []
    deps: list[list[int]] = [[]]
    for _ in range(2):
        root = len(deps) - 1
        first = len(deps)
        for _ in range(4):
            deps.append([root])
        deps.append(list(range(first, first + 4)))
    lines.append(f"fork-join DAG with {len(deps)} strands")
    for p in [1, 2, 3, 4, 8]:
        t1, tinf, tp = analyse(deps, p)
        lb = max(t1 / p, float(tinf))
        ub = t1 / p + tinf
        lines.append(
            f"P = {p}: T_P = {tp:>2}  (lower bound max(T1/P, T_inf) = {lb:.1f}, upper bound T1/P + T_inf = {ub:.1f})"
        )
    return "\n".join(lines) + "\n"


def _div_ceil(a: int, b: int) -> int:
    return (a + b - 1) // b


class Tests(unittest.TestCase):
    def test_greedy_bound_holds_on_random_dags(self) -> None:
        r = Rng(433)
        for _ in range(300):
            n = r.range(1, 40)
            deps = [[u for u in range(v) if r.f64() < 0.15] for v in range(n)]
            for p in [1, 2, 3, 5, 100]:
                t1, tinf, tp = analyse(deps, p)
                self.assertTrue(tp >= _div_ceil(t1, p) and tp >= tinf)
                self.assertLessEqual(tp * p, t1 + tinf * p)
                self.assertLessEqual(tp, 2 * max(_div_ceil(t1, p), tinf))
            t1, tinf, t1p = analyse(deps, 1)
            self.assertEqual(t1p, t1)
            self.assertLessEqual(tinf, t1)

    def test_infinite_processors_reach_the_span(self) -> None:
        deps = [[] if v == 0 else [v - 1] for v in range(10)]
        self.assertEqual(analyse(deps, 64), (10, 10, 10))


if __name__ == "__main__":
    print(demo(), end="")
