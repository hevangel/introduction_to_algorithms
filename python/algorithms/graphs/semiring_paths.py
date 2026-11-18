"""Closed-Semiring All-Pairs Path Problems — CLRS 26.4 (1e).

Unify transitive closure, all-pairs shortest paths, widest bottleneck paths, and highest-reliability paths over an algebraic closed semiring (S, ⊕, ⊙, 0̄, 1̄, *).
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest
from typing import Any

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

INF = (1 << 62) // 4 * 1  # mirrors i64::MAX / 4 ≈ 2**61 (any value far above real path lengths works)
INF = ((1 << 63) - 1) // 4


@dataclass(frozen=True)
class BooleanClosure:
    v: bool = False

    @staticmethod
    def zero() -> BooleanClosure:
        return BooleanClosure(False)

    @staticmethod
    def one() -> BooleanClosure:
        return BooleanClosure(True)

    def add(self, o: BooleanClosure) -> BooleanClosure:
        return BooleanClosure(self.v or o.v)

    def mul(self, o: BooleanClosure) -> BooleanClosure:
        return BooleanClosure(self.v and o.v)

    def closure(self) -> BooleanClosure:
        return BooleanClosure(True)


@dataclass(frozen=True)
class TropicalMinPlus:
    v: int = INF

    @staticmethod
    def zero() -> TropicalMinPlus:
        return TropicalMinPlus(INF)

    @staticmethod
    def one() -> TropicalMinPlus:
        return TropicalMinPlus(0)

    def add(self, o: TropicalMinPlus) -> TropicalMinPlus:
        return TropicalMinPlus(min(self.v, o.v))

    def mul(self, o: TropicalMinPlus) -> TropicalMinPlus:
        if self.v >= INF or o.v >= INF:
            return TropicalMinPlus(INF)
        return TropicalMinPlus(self.v + o.v)

    def closure(self) -> TropicalMinPlus:
        return TropicalMinPlus(0)


@dataclass(frozen=True)
class BottleneckMaxMin:
    v: int = 0

    @staticmethod
    def zero() -> BottleneckMaxMin:
        return BottleneckMaxMin(0)

    @staticmethod
    def one() -> BottleneckMaxMin:
        return BottleneckMaxMin(INF)

    def add(self, o: BottleneckMaxMin) -> BottleneckMaxMin:
        return BottleneckMaxMin(max(self.v, o.v))

    def mul(self, o: BottleneckMaxMin) -> BottleneckMaxMin:
        return BottleneckMaxMin(min(self.v, o.v))

    def closure(self) -> BottleneckMaxMin:
        return BottleneckMaxMin(INF)


@dataclass(frozen=True)
class ReliabilityMaxMul:
    v: float = 0.0

    @staticmethod
    def zero() -> ReliabilityMaxMul:
        return ReliabilityMaxMul(0.0)

    @staticmethod
    def one() -> ReliabilityMaxMul:
        return ReliabilityMaxMul(1.0)

    def add(self, o: ReliabilityMaxMul) -> ReliabilityMaxMul:
        return ReliabilityMaxMul(max(self.v, o.v))

    def mul(self, o: ReliabilityMaxMul) -> ReliabilityMaxMul:
        return ReliabilityMaxMul(self.v * o.v)

    def closure(self) -> ReliabilityMaxMul:
        return ReliabilityMaxMul(1.0)


def semiring_apsp(base: list[list[Any]]) -> list[list[Any]]:
    """Generic closed-semiring all-pairs path solver (Kleene / Floyd-Warshall generalization)."""
    n = len(base)
    cls = type(base[0][0]) if n else None
    m = [[cls.one().add(base[i][j]) if i == j else base[i][j] for j in range(n)] for i in range(n)]
    for k in range(n):
        prev = [row[:] for row in m]
        kk_star = prev[k][k].closure()
        for i in range(n):
            for j in range(n):
                through_k = prev[i][k].mul(kk_star).mul(prev[k][j])
                m[i][j] = prev[i][j].add(through_k)
    return m


def _floyd_warshall(n: int, edges: list[tuple[int, int, int]]) -> list[list[int]]:
    d = [[INF] * n for _ in range(n)]
    for i in range(n):
        d[i][i] = 0
    for u, v, w in edges:
        d[u][v] = min(d[u][v], w)
    for k in range(n):
        for i in range(n):
            for j in range(n):
                if d[i][k] < INF and d[k][j] < INF and d[i][k] + d[k][j] < d[i][j]:
                    d[i][j] = d[i][k] + d[k][j]
    return d


def _transitive_closure(adj: list[list[bool]]) -> list[list[bool]]:
    n = len(adj)
    t = [[i == j or adj[i][j] for j in range(n)] for i in range(n)]
    for k in range(n):
        for i in range(n):
            for j in range(n):
                t[i][j] = t[i][j] or (t[i][k] and t[k][j])
    return t


def demo() -> str:
    lines: list[str] = []
    n = 4
    edges = [(0, 1, 5, 0.9), (0, 2, 3, 0.7), (1, 3, 2, 0.95), (2, 1, 4, 0.8), (2, 3, 6, 0.6)]
    min_w = [[TropicalMinPlus.zero() for _ in range(n)] for _ in range(n)]
    cap_w = [[BottleneckMaxMin.zero() for _ in range(n)] for _ in range(n)]
    rel_w = [[ReliabilityMaxMul.zero() for _ in range(n)] for _ in range(n)]
    for u, v, c, p in edges:
        min_w[u][v] = TropicalMinPlus(c)
        cap_w[u][v] = BottleneckMaxMin(c)
        rel_w[u][v] = ReliabilityMaxMul(p)
    sp = semiring_apsp(min_w)
    bp = semiring_apsp(cap_w)
    rp = semiring_apsp(rel_w)
    lines.append(f"4-vertex DAG edges: {edges}")
    lines.append(f"shortest path 0 -> 3 (min, +):   {sp[0][3].v}")
    lines.append(f"widest path   0 -> 3 (max, min): {bp[0][3].v}")
    lines.append(f"most reliable 0 -> 3 (max, *):   {rp[0][3].v:.4f}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_floyd_warshall_and_transitive_closure(self) -> None:
        edges = [(0, 1, 3), (0, 2, 8), (0, 4, -4), (1, 3, 1), (1, 4, 7), (2, 1, 4), (3, 0, 2), (3, 2, -5), (4, 3, 6)]
        fw_dist = _floyd_warshall(5, edges)
        trop = [[TropicalMinPlus.zero() for _ in range(5)] for _ in range(5)]
        bool_m = [[BooleanClosure.zero() for _ in range(5)] for _ in range(5)]
        adj = [[False] * 5 for _ in range(5)]
        for u, v, c in edges:
            trop[u][v] = TropicalMinPlus(c)
            bool_m[u][v] = BooleanClosure(True)
            adj[u][v] = True
        sp = semiring_apsp(trop)
        for i in range(5):
            for j in range(5):
                self.assertEqual(sp[i][j].v, fw_dist[i][j])
        tc_ref = _transitive_closure(adj)
        tc_semi = semiring_apsp(bool_m)
        for i in range(5):
            for j in range(5):
                self.assertEqual(tc_semi[i][j].v, tc_ref[i][j])

    def test_bottleneck_widest_path_matches_simple_path_brute_force(self) -> None:
        r = Rng(264)
        for _ in range(40):
            n = 5
            cap = [[BottleneckMaxMin.zero() for _ in range(n)] for _ in range(n)]
            raw = [[0] * n for _ in range(n)]
            for i in range(n):
                for j in range(n):
                    if i != j and r.f64() < 0.55:
                        c = r.range(1, 50)
                        cap[i][j] = BottleneckMaxMin(c)
                        raw[i][j] = c
            bp = semiring_apsp(cap)
            for s in range(n):
                best = [0] * n
                best[s] = INF
                for _ in range(n):
                    for u in range(n):
                        for v in range(n):
                            if raw[u][v] > 0:
                                best[v] = max(best[v], min(best[u], raw[u][v]))
                for t in range(n):
                    self.assertEqual(bp[s][t].v, best[t])


if __name__ == "__main__":
    print(demo(), end="")
