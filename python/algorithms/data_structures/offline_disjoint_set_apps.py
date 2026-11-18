"""Disjoint-Set Applications: Off-line Minimum, Depth, Tarjan's LCA — CLRS Problems 21-1, 21-2, 21-3 (1e–3e) / 19-1…3 (4e) (1,2,3,4).

Three classic union-find applications: OFF-LINE-MINIMUM, FIND-DEPTH/GRAFT with weighted union-find, and off-line least common ancestors.
"""

from __future__ import annotations

from dataclasses import dataclass
import heapq
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


@dataclass(frozen=True)
class Insert:
    val: int


@dataclass(frozen=True)
class Extract:
    pass


Op = Insert | Extract


def _find(p: list[int], x: int) -> int:
    r = x
    while p[r] != r:
        r = p[r]
    c = x
    while p[c] != r:
        p[c], c = r, p[c]
    return r


def offline_minimum(n: int, ops: list[Op]) -> list[int]:
    """Problem 21-1, OFF-LINE-MINIMUM: sequence inserts 1..=n once each; returns extracted[j]."""
    m = sum(1 for o in ops if isinstance(o, Extract))
    set_of = [0] * (n + 1)
    j = 0
    for op in ops:
        if isinstance(op, Insert):
            set_of[op.val] = j
        else:
            j += 1
    parent = list(range(m + 1))
    extracted = [0] * m
    for i in range(1, n + 1):
        s = _find(parent, set_of[i])
        if s < m:
            extracted[s] = i
            parent[s] = s + 1
    return extracted


class DepthForest:
    """Problem 21-2: forest supporting MAKE-TREE, FIND-DEPTH(v) and GRAFT(r, v) in O(α(n)) amortized."""

    def __init__(self) -> None:
        self.p: list[int] = []
        self.d: list[int] = []
        self.rank: list[int] = []

    def make_tree(self) -> int:
        idx = len(self.p)
        self.p.append(idx)
        self.d.append(0)
        self.rank.append(0)
        return idx

    def _root(self, x: int) -> int:
        if self.p[x] == x:
            return x
        px = self.p[x]
        r = self._root(px)
        if px != r:
            self.d[x] += self.d[px]
        self.p[x] = r
        return r

    def find_depth(self, v: int) -> int:
        r = self._root(v)
        return self.d[r] if v == r else self.d[v] + self.d[r]

    def graft(self, r: int, v: int) -> None:
        shift = self.find_depth(v) + 1
        ra, rb = self._root(r), self._root(v)
        if self.rank[ra] <= self.rank[rb]:
            self.p[ra] = rb
            self.d[ra] += shift - self.d[rb]
            if self.rank[ra] == self.rank[rb]:
                self.rank[rb] += 1
        else:
            self.d[ra] += shift
            self.p[rb] = ra
            self.d[rb] -= self.d[ra]


def tarjan_lca(children: list[list[int]], root: int, queries: list[tuple[int, int]]) -> list[int]:
    """Problem 21-3: Tarjan's off-line least-common-ancestors."""
    n = len(children)
    at: list[list[tuple[int, int]]] = [[] for _ in range(n)]
    for i, (u, v) in enumerate(queries):
        at[u].append((v, i))
        at[v].append((u, i))
    parent = list(range(n))
    ancestor = list(range(n))
    black = [False] * n
    ans = [-1] * len(queries)

    stack: list[tuple[str, int, int]] = [("enter", root, 0)]
    while stack:
        kind, u, c = stack.pop()
        if kind == "enter":
            ancestor[u] = u
            stack.append(("finish", u, -1))
            for ch in reversed(children[u]):
                stack.append(("after", u, ch))
                stack.append(("enter", ch, 0))
        elif kind == "after":
            a, b = _find(parent, u), _find(parent, c)
            parent[b] = a
            ancestor[_find(parent, u)] = u
        else:
            black[u] = True
            for v, qi in at[u]:
                if black[v]:
                    ans[qi] = ancestor[_find(parent, v)]
    return ans


def demo() -> str:
    lines: list[str] = []
    seq: list[Op] = [
        Insert(4), Insert(8), Extract(), Insert(3), Extract(), Insert(9),
        Insert(2), Insert(6), Extract(), Extract(), Extract(), Insert(1),
        Insert(7), Extract(), Insert(5),
    ]
    lines.append(f"OFF-LINE-MINIMUM extracted = {offline_minimum(9, seq)}")
    f = DepthForest()
    nodes = [f.make_tree() for _ in range(6)]
    f.graft(nodes[1], nodes[0])
    f.graft(nodes[2], nodes[1])
    f.graft(nodes[4], nodes[3])
    f.graft(nodes[3], nodes[2])
    lines.append(f"depths after grafts: {[f.find_depth(v) for v in nodes]}")
    children = [[1, 2], [3, 4], [5], [], [6], [], []]
    lines.append(f"off-line LCA of (3,6), (5,4), (3,4), (6,6): {tarjan_lca(children, 0, [(3, 6), (5, 4), (3, 4), (6, 6)])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_offline_minimum_matches_simulation(self) -> None:
        r = Rng(326)
        for _ in range(200):
            n = r.range(1, 25)
            perm = list(range(1, n + 1))
            r.shuffle(perm)
            ops: list[Op] = []
            for x in perm:
                ops.append(Insert(x))
                if r.range(0, 2) == 0:
                    ops.append(Extract())
            heap: list[int] = []
            want: list[int] = []
            ok = True
            for op in ops:
                if isinstance(op, Insert):
                    heapq.heappush(heap, op.val)
                else:
                    if heap:
                        want.append(heapq.heappop(heap))
                    else:
                        ok = False
            if ok:
                self.assertEqual(offline_minimum(n, ops), want)

    def test_depth_forest_matches_naive_parent_arrays(self) -> None:
        r = Rng(327)
        for _ in range(100):
            n = r.range(2, 40)
            f = DepthForest()
            ids = [f.make_tree() for _ in range(n)]
            parent: list[int | None] = [None] * n

            def depth(v: int) -> int:
                d = 0
                while parent[v] is not None:
                    v = parent[v]  # type: ignore[assignment]
                    d += 1
                return d

            def tree_root(v: int) -> int:
                while parent[v] is not None:
                    v = parent[v]  # type: ignore[assignment]
                return v

            for _ in range(3 * n):
                rt, v = r.range(0, n - 1), r.range(0, n - 1)
                if parent[rt] is None and tree_root(v) != rt:
                    parent[rt] = v
                    f.graft(ids[rt], ids[v])
                q = r.range(0, n - 1)
                self.assertEqual(f.find_depth(ids[q]), depth(q))
            for q in range(n):
                self.assertEqual(f.find_depth(ids[q]), depth(q))

    def test_tarjan_lca_matches_naive(self) -> None:
        r = Rng(328)
        for _ in range(100):
            n = r.range(1, 40)
            children: list[list[int]] = [[] for _ in range(n)]
            parent = [-1] * n
            for v in range(1, n):
                p = r.range(0, v - 1)
                children[p].append(v)
                parent[v] = p
            qs = [(r.range(0, n - 1), r.range(0, n - 1)) for _ in range(20)]

            def anc(v: int) -> list[int]:
                a = [v]
                while parent[v] != -1:
                    v = parent[v]
                    a.append(v)
                return a

            want = []
            for u, v in qs:
                au = set(anc(u))
                want.append(next(x for x in anc(v) if x in au))
            self.assertEqual(tarjan_lca(children, 0, qs), want)


if __name__ == "__main__":
    print(demo(), end="")
