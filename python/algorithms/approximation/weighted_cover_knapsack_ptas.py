"""Weighted Set Cover, Maximal Matching & 0-1 Knapsack FPTAS — CLRS Problems 35-3, 35-4, 35-6 & 35-7 (2e–4e) (1,2,3,4).

Classic approximation algorithms from Chapter 35 problems: H(n)-approx weighted greedy set cover, 2-approx maximal matching, 2-approx max spanning tree, and (1−ε) 0-1 knapsack FPTAS.
"""

from __future__ import annotations

from collections import deque
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def weighted_greedy_set_cover(
    universe: int, sets: list[tuple[float, list[int]]]
) -> tuple[float, list[int]] | None:
    """Problem 35-3: Picks the set minimizing weight_i / newly_covered_elements -> H(d)-approx."""
    uncovered = [True] * universe
    left = universe
    total_w = 0.0
    chosen: list[int] = []
    used = [False] * len(sets)

    while left > 0:
        best_idx: int | None = None
        best_ratio = math.inf
        for i, (w, elems) in enumerate(sets):
            if used[i]:
                continue
            gain = sum(1 for e in elems if uncovered[e])
            if gain > 0:
                ratio = w / gain
                if ratio < best_ratio:
                    best_ratio = ratio
                    best_idx = i
        if best_idx is None:
            return None
        used[best_idx] = True
        total_w += sets[best_idx][0]
        for e in sets[best_idx][1]:
            if uncovered[e]:
                uncovered[e] = False
                left -= 1
        chosen.append(best_idx)
    return (total_w, chosen)


def maximal_matching(n: int, edges: list[tuple[int, int]]) -> list[tuple[int, int]]:
    """Problem 35-4: Arbitrary maximal matching (|M| >= |M*| / 2) in linear time."""
    matched = [False] * n
    m: list[tuple[int, int]] = []
    for u, v in edges:
        if u != v and not matched[u] and not matched[v]:
            matched[u] = True
            matched[v] = True
            m.append((u, v))
    return m


def max_spanning_tree(n: int, edges: list[tuple[int, int, int]]) -> tuple[int, list[tuple[int, int, int]]]:
    """Problem 35-6: Maximum spanning tree/forest via Kruskal's algorithm sorted descending."""
    sorted_edges = sorted(edges, key=lambda e: e[2], reverse=True)
    parent = list(range(n))

    def find(x: int) -> int:
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    total = 0
    tree: list[tuple[int, int, int]] = []
    for u, v, w in sorted_edges:
        ru, rv = find(u), find(v)
        if ru != rv:
            parent[ru] = rv
            total += w
            tree.append((u, v, w))
    return (total, tree)


def knapsack_fptas(items: list[tuple[int, int]], cap: int, eps: float) -> tuple[int, list[int]]:
    """Problem 35-7: FPTAS for 0-1 Knapsack guaranteeing value >= (1 - eps) * OPT."""
    assert 0.0 < eps < 1.0
    feasible = [(idx, w, v) for idx, (w, v) in enumerate(items) if w <= cap]
    if not feasible:
        return (0, [])
    n = len(feasible)
    v_max = max(v for _, _, v in feasible)
    if v_max == 0:
        return (0, [])
    scale = (eps * v_max) / n
    scaled = [int(math.floor(v / scale)) for _, _, v in feasible]
    max_v_sum = sum(scaled)

    inf = cap + 10**9
    dp = [inf] * (max_v_sum + 1)
    parent_val = [[False] * (max_v_sum + 1) for _ in range(n)]
    dp[0] = 0

    for i, (_, w, _) in enumerate(feasible):
        sv = scaled[i]
        for v in range(max_v_sum, sv - 1, -1):
            if dp[v - sv] + w < dp[v]:
                dp[v] = dp[v - sv] + w
                parent_val[i][v] = True

    best_sv = 0
    for v in range(max_v_sum, -1, -1):
        if dp[v] <= cap:
            best_sv = v
            break

    chosen: list[int] = []
    cur_v = best_sv
    for i in range(n - 1, -1, -1):
        if parent_val[i][cur_v]:
            chosen.append(feasible[i][0])
            cur_v -= scaled[i]
    chosen.reverse()
    orig_val = sum(items[idx][1] for idx in chosen)
    return (orig_val, chosen)


def _hopcroft_karp_size(l: int, r: int, adj: list[list[int]]) -> int:
    pair_u = [-1] * l
    pair_v = [-1] * r
    dist = [0] * l

    def bfs() -> bool:
        q: deque[int] = deque()
        found = False
        for u in range(l):
            if pair_u[u] == -1:
                dist[u] = 0
                q.append(u)
            else:
                dist[u] = -1
        while q:
            u = q.popleft()
            for v in adj[u]:
                pu = pair_v[v]
                if pu == -1:
                    found = True
                elif dist[pu] == -1:
                    dist[pu] = dist[u] + 1
                    q.append(pu)
        return found

    def dfs(u: int) -> bool:
        for v in adj[u]:
            pu = pair_v[v]
            if pu == -1 or (dist[pu] == dist[u] + 1 and dfs(pu)):
                pair_u[u] = v
                pair_v[v] = u
                return True
        dist[u] = -1
        return False

    matching = 0
    while bfs():
        for u in range(l):
            if pair_u[u] == -1 and dfs(u):
                matching += 1
    return matching


def _knapsack_01_opt(items: list[tuple[int, int]], cap: int) -> int:
    dp = [0] * (cap + 1)
    for w, v in items:
        for c in range(cap, w - 1, -1):
            if dp[c - w] + v > dp[c]:
                dp[c] = dp[c - w] + v
    return dp[cap]


def demo() -> str:
    sets = [(1.0, [0, 1]), (1.5, [1, 2, 3]), (1.2, [2, 3])]
    items = [(10, 60), (20, 100), (30, 120)]
    return "\n".join([
        f"weighted greedy set cover -> {weighted_greedy_set_cover(4, sets)}",
        f"maximal matching          -> {maximal_matching(4, [(0, 1), (1, 2), (2, 3)])}",
        f"max spanning tree         -> {max_spanning_tree(4, [(0, 1, 3), (1, 2, 8), (2, 3, 5), (0, 3, 7)])}",
        f"0-1 knapsack FPTAS (eps=0.1) -> {knapsack_fptas(items, 50, 0.1)}",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_weighted_cover_maximal_matching_and_max_st_bounds(self) -> None:
        r = Rng(3503)
        for _ in range(40):
            u, m = r.range(2, 10), r.range(2, 9)
            sets = [(float(r.range(1, 20)), [e for e in range(u) if r.f64() < 0.4]) for _ in range(m)]
            greedy = weighted_greedy_set_cover(u, sets)
            opt = math.inf
            for mask in range(1, 1 << m):
                cov = [False] * u
                w = 0.0
                for i, (wi, s) in enumerate(sets):
                    if (mask >> i) & 1:
                        w += wi
                        for e in s:
                            cov[e] = True
                if all(cov):
                    opt = min(opt, w)
            if math.isfinite(opt):
                self.assertIsNotNone(greedy)
                assert greedy is not None
                gw, _ = greedy
                d = max([len(s[1]) for s in sets] + [1])
                hd = sum(1.0 / k for k in range(1, d + 1))
                self.assertLessEqual(gw, opt * hd + 1e-9)
            else:
                self.assertIsNone(greedy)

        for _ in range(40):
            l, rr = r.range(1, 8), r.range(1, 8)
            adj: list[list[int]] = [[] for _ in range(l)]
            edges: list[tuple[int, int]] = []
            for a in range(l):
                for b in range(rr):
                    if r.f64() < 0.4:
                        adj[a].append(b)
                        edges.append((a, l + b))
            opt_m = _hopcroft_karp_size(l, rr, adj)
            mm = maximal_matching(l + rr, edges)
            self.assertGreaterEqual(2 * len(mm), opt_m)

        tw, te = max_spanning_tree(4, [(0, 1, 3), (1, 2, 8), (2, 3, 5), (0, 3, 7), (0, 2, 2)])
        self.assertEqual(tw, 8 + 7 + 5)
        self.assertEqual(len(te), 3)

    def test_knapsack_fptas_meets_one_minus_epsilon_guarantee(self) -> None:
        r = Rng(3507)
        for _ in range(40):
            n = r.range(1, 10)
            items = [(r.range(1, 15), r.range(5, 200)) for _ in range(n)]
            cap = r.range(5, 45)
            opt = _knapsack_01_opt(items, cap)
            for eps in [0.5, 0.25, 0.1]:
                approx, chosen = knapsack_fptas(items, cap, eps)
                w_sum = sum(items[i][0] for i in chosen)
                v_sum = sum(items[i][1] for i in chosen)
                self.assertLessEqual(w_sum, cap)
                self.assertEqual(v_sum, approx)
                self.assertLessEqual(approx, opt)
                self.assertGreaterEqual(float(approx) + 1e-6, (1.0 - eps) * float(opt))


if __name__ == "__main__":
    print(demo(), end="")
