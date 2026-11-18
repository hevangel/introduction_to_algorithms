"""Matroids & Weighted Greedy — CLRS 16.4–16.5 (1e–3e) (1e,2e,3e).

The matroid greedy algorithm and its instances: maximum spanning tree and unit-time task scheduling with penalties.
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class Matroid(ABC):
    """An independence system over the ground set {0, ..., size - 1}."""

    @abstractmethod
    def size(self) -> int: ...

    @abstractmethod
    def independent(self, subset: list[int]) -> bool: ...


def greedy(m: Matroid, weights: list[int]) -> list[int]:
    """GREEDY(M, w): consider elements by decreasing weight and keep each that preserves independence."""
    order = sorted(range(m.size()), key=lambda i: weights[i], reverse=True)
    a: list[int] = []
    for x in order:
        a.append(x)
        if not m.independent(a):
            a.pop()
    a.sort()
    return a


class GraphicMatroid(Matroid):
    """Graphic matroid M_G: independent sets are acyclic edge sets (forests)."""

    def __init__(self, vertices: int, edges: list[tuple[int, int]]) -> None:
        self.vertices = vertices
        self.edges = edges

    def size(self) -> int:
        return len(self.edges)

    def independent(self, subset: list[int]) -> bool:
        p = list(range(self.vertices))

        def find(x: int) -> int:
            while p[x] != x:
                p[x] = p[p[x]]
                x = p[x]
            return x

        for e in subset:
            a, b = find(self.edges[e][0]), find(self.edges[e][1])
            if a == b:
                return False
            p[a] = b
        return True


class UniformMatroid(Matroid):
    """Uniform matroid U_{k,n}: any set of at most k elements is independent."""

    def __init__(self, n: int, k: int) -> None:
        self.n = n
        self.k = k

    def size(self) -> int:
        return self.n

    def independent(self, subset: list[int]) -> bool:
        return len(subset) <= self.k


class SchedulingMatroid(Matroid):
    """Section 16.5: sets of unit-time tasks that can all finish by their deadlines."""

    def __init__(self, deadlines: list[int]) -> None:
        self.deadlines = deadlines

    def size(self) -> int:
        return len(self.deadlines)

    def independent(self, subset: list[int]) -> bool:
        maxd = max(self.deadlines, default=0)
        return all(sum(1 for a in subset if self.deadlines[a] <= t) <= t for t in range(1, maxd + 1))


def schedule_unit_tasks(deadlines: list[int], penalties: list[int]) -> tuple[list[int], int]:
    """Schedule unit tasks with deadlines and late-penalties."""
    m = SchedulingMatroid(deadlines)
    early = greedy(m, penalties)
    sched = sorted(early, key=lambda a: deadlines[a])
    early_set = set(early)
    late = sum(penalties[a] for a in range(len(deadlines)) if a not in early_set)
    return sched, late


def demo() -> str:
    lines: list[str] = []
    g = GraphicMatroid(5, [(0, 1), (1, 2), (0, 2), (2, 3), (3, 4), (2, 4)])
    w = [4, 8, 3, 7, 2, 9]
    lines.append(f"maximum spanning tree edges (greedy on the graphic matroid): {greedy(g, w)}")
    sched, penalty = schedule_unit_tasks([4, 2, 4, 3, 1, 4, 6], [70, 60, 50, 40, 30, 20, 10])
    lines.append(f"unit-task scheduling: early tasks {sched} (0-based), total penalty {penalty}")
    lines.append(
        f"uniform matroid U(2,5) with weights [5,1,9,3,7] picks {greedy(UniformMatroid(5, 2), [5, 1, 9, 3, 7])}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_scheduling_example(self) -> None:
        deadlines = [4, 2, 4, 3, 1, 4, 6]
        sched, penalty = schedule_unit_tasks(deadlines, [70, 60, 50, 40, 30, 20, 10])
        self.assertEqual(penalty, 50)
        self.assertEqual(len(sched), 5)
        for slot, t in enumerate(sched):
            self.assertLessEqual(slot + 1, deadlines[t])

    def _brute_best(self, m: Matroid, w: list[int]) -> int:
        best = 0
        sz = m.size()
        for mask in range(1 << sz):
            s = [i for i in range(sz) if (mask >> i) & 1]
            if m.independent(s):
                best = max(best, sum(w[i] for i in s))
        return best

    def test_greedy_is_optimal_on_matroids(self) -> None:
        r = Rng(115)
        for _ in range(40):
            n = r.range(2, 6)
            m_edges = r.range(1, 9)
            edges = [(u, v) for _ in range(m_edges) if (u := r.range(0, n - 1)) != (v := r.range(0, n - 1))]
            g = GraphicMatroid(n, edges)
            w = r.vec(g.size(), 1, 20)
            sel = greedy(g, w)
            self.assertEqual(sum(w[i] for i in sel), self._brute_best(g, w))
            u = UniformMatroid(6, r.range(0, 6))
            w2 = r.vec(6, 1, 20)
            self.assertEqual(sum(w2[i] for i in greedy(u, w2)), self._brute_best(u, w2))
            dl = [r.range(1, 5) for _ in range(6)]
            s = SchedulingMatroid(dl)
            self.assertEqual(sum(w2[i] for i in greedy(s, w2)), self._brute_best(s, w2))


if __name__ == "__main__":
    print(demo(), end="")
