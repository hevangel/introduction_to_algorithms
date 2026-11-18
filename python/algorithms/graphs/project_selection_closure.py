"""Project Selection & Maximum-Weight Closure — CLRS Problem 26-3 (1e–3e); Problem 24-3 (4e).

Reduce profit maximization with prerequisite costs (algorithmic consulting / Space Shuttle payloads) to s-t minimum cut in O(VE²) or push-relabel time.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class _FlowNetwork:
    def __init__(self, n: int) -> None:
        self.n = n
        self.to: list[int] = []
        self.cap: list[int] = []
        self.adj: list[list[int]] = [[] for _ in range(n)]

    def add_edge(self, u: int, v: int, c: int) -> None:
        eid = len(self.to)
        self.to.append(v)
        self.cap.append(c)
        self.adj[u].append(eid)
        self.to.append(u)
        self.cap.append(0)
        self.adj[v].append(eid + 1)

    def edmonds_karp(self, s: int, t: int) -> int:
        flow = 0
        while True:
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
                return flow
            b = 1 << 60
            v = t
            while v != s:
                e = pred[v]
                if self.cap[e] < b:
                    b = self.cap[e]
                v = self.to[e ^ 1]
            v = t
            while v != s:
                e = pred[v]
                self.cap[e] -= b
                self.cap[e ^ 1] += b
                v = self.to[e ^ 1]
            flow += b

    def min_cut_source_side(self, s: int) -> list[bool]:
        seen = [False] * self.n
        seen[s] = True
        st = [s]
        while st:
            u = st.pop()
            for e in self.adj[u]:
                v = self.to[e]
                if not seen[v] and self.cap[e] > 0:
                    seen[v] = True
                    st.append(v)
        return seen


def max_weight_closure(weights: list[int], prereqs: list[tuple[int, int]]) -> tuple[int, list[int]]:
    """Maximum-weight closure on a directed graph with arbitrary signed vertex weights and prerequisites."""
    n = len(weights)
    s, t = n, n + 1
    net = _FlowNetwork(n + 2)
    positive_sum = 0
    for i, w in enumerate(weights):
        if w > 0:
            net.add_edge(s, i, w)
            positive_sum += w
        elif w < 0:
            net.add_edge(i, t, -w)
    inf = positive_sum + 1
    for u, v in prereqs:
        net.add_edge(u, v, inf)
    min_cut = net.edmonds_karp(s, t)
    side = net.min_cut_source_side(s)
    chosen = [i for i in range(n) if side[i]]
    return positive_sum - min_cut, chosen


def project_selection(
    rewards: list[int],
    costs: list[int],
    requires: list[tuple[int, int]],
) -> tuple[int, list[int], list[int]]:
    """CLRS Problem 26-3 / 24-3 formulation: jobs rewards >= 0, instruments costs >= 0, bipartite requirements."""
    nj = len(rewards)
    weights = list(rewards) + [-c for c in costs]
    prereqs = [(j, nj + k) for j, k in requires]
    profit, chosen = max_weight_closure(weights, prereqs)
    chosen_jobs = [x for x in chosen if x < nj]
    chosen_inst = [x - nj for x in chosen if x >= nj]
    return profit, chosen_jobs, chosen_inst


def demo() -> str:
    lines: list[str] = []
    rewards = [100, 120, 70]
    costs = [60, 80, 50]
    requires = [(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)]
    profit, jobs, inst = project_selection(rewards, costs, requires)
    lines.append(f"rewards={rewards}, costs={costs}")
    lines.append(f"best net profit = {profit}, chosen jobs = {jobs}, instruments = {inst}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_closure_matches_2_to_n_brute_force(self) -> None:
        r = Rng(2603)
        for _ in range(80):
            n = r.range(1, 12)
            weights = r.vec(n, -25, 25)
            m = r.range(0, 20)
            prereqs = [(r.range(0, n - 1), r.range(0, n - 1)) for _ in range(m)]
            got_profit, chosen = max_weight_closure(weights, prereqs)
            in_s = [False] * n
            for u in chosen:
                in_s[u] = True
            for u, v in prereqs:
                if in_s[u]:
                    self.assertTrue(in_s[v], f"chosen set is not closed under prerequisite ({u}, {v})")
            self.assertEqual(sum(weights[i] for i in chosen), got_profit)

            brute = 0
            for mask in range(1 << n):
                if all(not ((mask >> u) & 1) or ((mask >> v) & 1) for u, v in prereqs):
                    w = sum(weights[i] for i in range(n) if (mask >> i) & 1)
                    if w > brute:
                        brute = w
            self.assertEqual(got_profit, brute)

    def test_space_shuttle_consulting_example(self) -> None:
        rewards = [100, 120, 70]
        costs = [60, 80, 50]
        requires = [(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)]
        p, jobs, inst = project_selection(rewards, costs, requires)
        self.assertEqual(p, 100)
        self.assertEqual(jobs, [0, 1, 2])
        self.assertEqual(inst, [0, 1, 2])


if __name__ == "__main__":
    print(demo(), end="")
