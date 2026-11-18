"""Updating a Maximum Flow — CLRS Problem 26-4 (1e–4e).

After one edge's capacity changes by 1, restore a maximum flow with a single augmenting-path search: O(E) instead of recomputing.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class FlowNetwork:
    """Flow network with paired residual edges (edge e and its reverse e ^ 1)."""

    def __init__(self, n: int) -> None:
        self.n = n
        self.to: list[int] = []
        self.cap: list[int] = []
        self.adj: list[list[int]] = [[] for _ in range(n)]

    def add_edge(self, u: int, v: int, c: int) -> int:
        eid = len(self.to)
        self.to.append(v)
        self.cap.append(c)
        self.adj[u].append(eid)
        self.to.append(u)
        self.cap.append(0)
        self.adj[v].append(eid + 1)
        return eid

    def capacity_of(self, eid: int) -> int:
        return self.cap[eid] + self.cap[eid ^ 1]

    def net_flow(self, u: int) -> int:
        return sum(self.cap[e ^ 1] if e % 2 == 0 else -self.cap[e] for e in self.adj[u])

    def augment_once(self, s: int, t: int) -> int:
        """One BFS augmenting-path search in the residual graph; pushes the bottleneck. Returns the amount (0 if none)."""
        pred = [-1] * self.n
        seen = [False] * self.n
        seen[s] = True
        q: deque[int] = deque([s])
        while q:
            u = q.popleft()
            for e in self.adj[u]:
                v = self.to[e]
                if not seen[v] and self.cap[e] > 0:
                    seen[v] = True
                    pred[v] = e
                    q.append(v)
        if not seen[t]:
            return 0
        b = 1 << 62
        v = t
        while v != s:
            e = pred[v]
            b = min(b, self.cap[e])
            v = self.to[e ^ 1]
        v = t
        while v != s:
            e = pred[v]
            self.cap[e] -= b
            self.cap[e ^ 1] += b
            v = self.to[e ^ 1]
        return b

    def edmonds_karp(self, s: int, t: int) -> int:
        flow = 0
        while True:
            b = self.augment_once(s, t)
            if b == 0:
                return flow
            flow += b

    def flow_path(self, start: int, goal: int) -> list[int] | None:
        """BFS along edges that carry flow; returns the forward edge ids on a path start ⇝ goal."""
        pred = [-1] * self.n
        seen = [False] * self.n
        seen[start] = True
        q: deque[int] = deque([start])
        while q:
            u = q.popleft()
            for e in self.adj[u]:
                if e % 2 == 0 and self.cap[e ^ 1] > 0:
                    v = self.to[e]
                    if not seen[v]:
                        seen[v] = True
                        pred[v] = e
                        q.append(v)
        if not seen[goal]:
            return None
        path: list[int] = []
        v = goal
        while v != start:
            e = pred[v]
            path.append(e)
            v = self.to[e ^ 1]
        return path

    def increase_capacity(self, eid: int, s: int, t: int) -> int:
        """Increase the capacity of edge eid by one and restore maximality with one augmentation: O(E)."""
        self.cap[eid] += 1
        self.augment_once(s, t)
        return self.net_flow(s)

    def decrease_capacity(self, eid: int, s: int, t: int) -> int:
        """Decrease the capacity of edge eid by one (it must be ≥ 1); cancel a unit of flow if saturated, then augment once."""
        assert eid % 2 == 0 and self.capacity_of(eid) >= 1
        if self.cap[eid] == 0:  # saturated: the unit that used to cross it must be rerouted
            u, v = self.to[eid ^ 1], self.to[eid]
            cancel = [eid]
            cycle = self.flow_path(v, u)
            if cycle is not None:  # the unit lies on a flow cycle
                cancel.extend(cycle)
            else:
                into_u = self.flow_path(s, u)
                out_of_v = self.flow_path(v, t)
                assert into_u is not None and out_of_v is not None, "flow conservation guarantees these flow paths"
                cancel.extend(into_u)
                cancel.extend(out_of_v)
            for e in cancel:
                self.cap[e] += 1
                self.cap[e ^ 1] -= 1
        self.cap[eid] -= 1
        self.augment_once(s, t)
        return self.net_flow(s)


def clrs_network() -> FlowNetwork:
    """Figure 26.1: s=0, v1=1, v2=2, v3=3, v4=4, t=5."""
    g = FlowNetwork(6)
    for u, v, c in [(0, 1, 16), (0, 2, 13), (1, 3, 12), (2, 1, 4), (2, 4, 14), (3, 2, 9), (3, 5, 20), (4, 3, 7), (4, 5, 4)]:
        g.add_edge(u, v, c)
    return g


def _rebuild(n: int, caps: list[tuple[int, int, int]]) -> FlowNetwork:
    g = FlowNetwork(n)
    for u, v, c in caps:
        g.add_edge(u, v, c)
    return g


def demo() -> str:
    lines: list[str] = []
    g = clrs_network()
    ids = list(range(0, len(g.cap), 2))
    f0 = g.edmonds_karp(0, 5)
    lines.append(f"initial maximum flow = {f0}")
    lines.append(f"increase the capacity of v4→t (edge {ids[8]}) by 1: flow = {g.increase_capacity(ids[8], 0, 5)}")
    lines.append(f"decrease the capacity of v1→v3 (edge {ids[2]}) by 1: flow = {g.decrease_capacity(ids[2], 0, 5)}")
    fresh = _rebuild(6, [(0, 1, 16), (0, 2, 13), (1, 3, 11), (2, 1, 4), (2, 4, 14), (3, 2, 9), (3, 5, 20), (4, 3, 7), (4, 5, 5)])
    lines.append(f"recomputed from scratch on the updated network: {fresh.edmonds_karp(0, 5)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_single_unit_updates_agree_with_recomputation(self) -> None:
        r = Rng(371)
        for _ in range(100):
            n = r.range(2, 8)
            model: list[list[int]] = []
            for _ in range(r.range(1, 15)):
                u, v = r.range(0, n - 1), r.range(0, n - 1)
                if u != v:
                    model.append([u, v, r.range(1, 6)])
            if not model:
                continue
            g = _rebuild(n, [(u, v, c) for u, v, c in model])
            g.edmonds_karp(0, n - 1)
            for _ in range(12):
                e = r.range(0, len(model) - 1)
                if r.range(0, 1) == 0:
                    model[e][2] += 1
                    got = g.increase_capacity(2 * e, 0, n - 1)
                elif model[e][2] >= 1:
                    model[e][2] -= 1
                    got = g.decrease_capacity(2 * e, 0, n - 1)
                else:
                    continue
                want = _rebuild(n, [(u, v, c) for u, v, c in model]).edmonds_karp(0, n - 1)
                self.assertEqual(got, want, f"model {model}")
                for v in range(1, n - 1):
                    self.assertEqual(g.net_flow(v), 0, "conservation")


if __name__ == "__main__":
    print(demo(), end="")
