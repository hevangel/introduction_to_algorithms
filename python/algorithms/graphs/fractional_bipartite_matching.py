"""Fractional Bipartite Matching & Vertex Labels — CLRS Problems 25-4 & 25-5 (4e).

Round a fractional bipartite matching to an integral matching of ≥ weight in O(E) time via alternating cycles/paths, and recover optimal Egerváry vertex labels from a matching.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def round_fractional_matching(
    left: int,
    right: int,
    edges: list[tuple[int, int, float]],
    x_init: list[float],
) -> list[int]:
    """Round feasible fractional matching x into an integral matching with total weight >= original fractional weight."""
    eps = 1e-11
    m = len(edges)
    assert m == len(x_init)
    x = list(x_init)
    n_total = left + right
    active = [eps < v < 1.0 - eps for v in x]

    while True:
        deg = [0] * n_total
        adj: list[list[tuple[int, int]]] = [[] for _ in range(n_total)]
        for e, (u, v, _) in enumerate(edges):
            if active[e]:
                ru = u
                rv = left + v
                adj[ru].append((rv, e))
                adj[rv].append((ru, e))
                deg[ru] += 1
                deg[rv] += 1
        if not any(active):
            break

        state = [0] * n_total
        parent_v = [-1] * n_total
        parent_e = [-1] * n_total
        cycle_edges: list[int] | None = None

        def dfs(u: int, pe: int) -> None:
            nonlocal cycle_edges
            state[u] = 1
            for v, e in adj[u]:
                if e == pe or not active[e]:
                    continue
                if state[v] == 1:
                    cyc = [e]
                    cur = u
                    while cur != v:
                        cyc.append(parent_e[cur])
                        cur = parent_v[cur]
                    cycle_edges = cyc
                    return
                if state[v] == 0:
                    parent_v[v] = u
                    parent_e[v] = e
                    dfs(v, e)
                    if cycle_edges is not None:
                        return
            state[u] = 2

        for s in range(n_total):
            if state[s] == 0 and deg[s] > 0:
                dfs(s, -1)
                if cycle_edges is not None:
                    break

        if cycle_edges is not None:
            seq = cycle_edges
        else:
            leaf = next(v for v in range(n_total) if deg[v] == 1)
            seq = []
            cur = leaf
            prev_e = -1
            while True:
                nxt = next(((nv, e) for nv, e in adj[cur] if e != prev_e and active[e]), None)
                if nxt is None:
                    break
                nxt_v, e = nxt
                seq.append(e)
                prev_e = e
                cur = nxt_v

        even_w = sum(edges[e][2] for e in seq[0::2])
        odd_w = sum(edges[e][2] for e in seq[1::2])
        raise_even = even_w >= odd_w
        delta = math.inf
        for pos, e in enumerate(seq):
            inc = (pos % 2 == 0) == raise_even
            delta = min(delta, 1.0 - x[e] if inc else x[e])
        for pos, e in enumerate(seq):
            inc = (pos % 2 == 0) == raise_even
            x[e] = x[e] + delta if inc else x[e] - delta
            if x[e] <= eps:
                x[e] = 0.0
                active[e] = False
            elif x[e] >= 1.0 - eps:
                x[e] = 1.0
                active[e] = False

    return [e for e in range(m) if x[e] >= 1.0 - 1e-6]


def dual_labels_from_matching(
    left: int,
    right: int,
    edges: list[tuple[int, int, int]],
    matched_edges: list[int],
) -> tuple[list[int], list[int]] | None:
    """Compute feasible non-negative dual vertex labels (u_left, v_right) in O(VE) time via Bellman-Ford."""
    n = left + right + 1
    s = left + right
    is_matched_edge = [False] * len(edges)
    l_matched = [False] * left
    r_matched = [False] * right
    for ei in matched_edges:
        is_matched_edge[ei] = True
        a, b, _ = edges[ei]
        l_matched[a] = True
        r_matched[b] = True

    arcs: list[tuple[int, int, int]] = []
    for a in range(left):
        arcs.append((s, a, 0))
        if not l_matched[a]:
            arcs.append((a, s, 0))
    for b in range(right):
        rb = left + b
        arcs.append((rb, s, 0))
        if not r_matched[b]:
            arcs.append((s, rb, 0))
    for ei, (a, b, w) in enumerate(edges):
        rb = left + b
        arcs.append((rb, a, -w))
        if is_matched_edge[ei]:
            arcs.append((a, rb, w))

    inf = 1 << 60
    dist = [inf] * n
    dist[s] = 0
    for _ in range(n):
        changed = False
        for u, v, w in arcs:
            if dist[u] < inf // 2 and dist[u] + w < dist[v]:
                dist[v] = dist[u] + w
                changed = True
        if not changed:
            break
    for u, v, w in arcs:
        if dist[u] < inf // 2 and dist[u] + w < dist[v]:
            return None

    u_labels = [-dist[a] for a in range(left)]
    v_labels = [dist[left + b] for b in range(right)]
    return u_labels, v_labels


def _max_weight_assignment(w: list[list[int]]) -> tuple[int, list[int]]:
    n = len(w)
    if n == 0:
        return 0, []
    inf = 1 << 60
    u = [0] * (n + 1)
    v = [0] * (n + 1)
    p = [0] * (n + 1)
    way = [0] * (n + 1)
    for i in range(1, n + 1):
        p[0] = i
        j0 = 0
        minv = [inf] * (n + 1)
        used = [False] * (n + 1)
        while True:
            used[j0] = True
            i0 = p[j0]
            delta = inf
            j1 = 0
            for j in range(1, n + 1):
                if not used[j]:
                    cur = -w[i0 - 1][j - 1] - u[i0] - v[j]
                    if cur < minv[j]:
                        minv[j] = cur
                        way[j] = j0
                    if minv[j] < delta:
                        delta = minv[j]
                        j1 = j
            for j in range(n + 1):
                if used[j]:
                    u[p[j]] += delta
                    v[j] -= delta
                else:
                    minv[j] -= delta
            j0 = j1
            if p[j0] == 0:
                break
        while True:
            j1 = way[j0]
            p[j0] = p[j1]
            j0 = j1
            if j0 == 0:
                break
    assign = [0] * n
    for j in range(1, n + 1):
        assign[p[j] - 1] = j - 1
    total = sum(w[i][assign[i]] for i in range(n))
    return total, assign


def demo() -> str:
    lines: list[str] = []
    edges = [(0, 0, 8.0), (1, 0, 3.0), (1, 1, 7.0), (2, 1, 2.0), (2, 2, 9.0), (0, 2, 4.0)]
    frac = [0.5] * 6
    frac_wt = sum(e[2] * x for e, x in zip(edges, frac))
    rounded = round_fractional_matching(3, 3, edges, frac)
    round_wt = sum(edges[i][2] for i in rounded)
    lines.append(f"fractional matching weight = {frac_wt:.1f} -> rounded edges {rounded}, integral weight = {round_wt:.1f}")

    w_mat = [[9, 2, 7], [6, 4, 3], [5, 8, 1]]
    opt_w, assign = _max_weight_assignment(w_mat)
    g_edges: list[tuple[int, int, int]] = []
    matched: list[int] = []
    for i in range(3):
        for j in range(3):
            if j == assign[i]:
                matched.append(len(g_edges))
            g_edges.append((i, j, w_mat[i][j]))
    res = dual_labels_from_matching(3, 3, g_edges, matched)
    assert res is not None
    ul, vr = res
    lines.append(f"optimal matching weight = {opt_w}; dual labels u = {ul}, v = {vr} (sum = {sum(ul) + sum(vr)})")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_rounds_arbitrary_fractional_matchings_without_loss(self) -> None:
        r = Rng(2504)
        for _ in range(60):
            l = r.range(2, 6)
            rr = r.range(2, 6)
            edges: list[tuple[int, int, float]] = []
            for a in range(l):
                for b in range(rr):
                    if r.f64() < 0.65:
                        edges.append((a, b, float(r.range(1, 20))))
            if not edges:
                continue
            x = [0.0] * len(edges)
            for alpha in (0.2, 0.3, 0.25, 0.25):
                used_l = [False] * l
                used_r = [False] * rr
                order = list(range(len(edges)))
                r.shuffle(order)
                for ei in order:
                    a, b, _ = edges[ei]
                    if not used_l[a] and not used_r[b]:
                        used_l[a] = True
                        used_r[b] = True
                        x[ei] += alpha
            frac_w = sum(e[2] * v for e, v in zip(edges, x))
            chosen = round_fractional_matching(l, rr, edges, x)
            used_l = [False] * l
            used_r = [False] * rr
            int_w = 0.0
            for ei in chosen:
                a, b, w = edges[ei]
                self.assertFalse(used_l[a])
                self.assertFalse(used_r[b])
                used_l[a] = True
                used_r[b] = True
                int_w += w
            self.assertGreaterEqual(int_w + 1e-9, frac_w, f"{int_w} < {frac_w}")

    def test_recovers_feasible_optimal_dual_labels_for_partial_and_full_matchings(self) -> None:
        r = Rng(2505)
        for _ in range(60):
            l = r.range(2, 5)
            rr = r.range(2, 5)
            sz = max(l, rr)
            mat = [[0] * sz for _ in range(sz)]
            edges: list[tuple[int, int, int]] = []
            for a in range(l):
                for b in range(rr):
                    if r.f64() < 0.7:
                        w = r.range(1, 30)
                        mat[a][b] = w
                        edges.append((a, b, w))
            opt_w, assign = _max_weight_assignment(mat)
            matched = [ei for ei, (a, b, w) in enumerate(edges) if assign[a] == b and w > 0]
            res = dual_labels_from_matching(l, rr, edges, matched)
            self.assertIsNotNone(res)
            ul, vr = res  # type: ignore[misc]
            self.assertTrue(all(x >= 0 for x in ul) and all(x >= 0 for x in vr))
            for a, b, w in edges:
                self.assertGreaterEqual(ul[a] + vr[b], w)
            for ei in matched:
                a, b, w = edges[ei]
                self.assertEqual(ul[a] + vr[b], w)
            self.assertEqual(sum(ul) + sum(vr), opt_w)


if __name__ == "__main__":
    print(demo(), end="")
