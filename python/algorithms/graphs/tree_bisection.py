"""Tree Bisection (Balanced Separator) — CLRS Problem B-3 (1e–4e).

Partition any n-node binary or bounded-degree tree in O(n) DFS time into two vertex sets of size ≤ 3n/4 (and single-edge separator ≤ 2n/3 for binary trees).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def binary_tree_edge_separator(n: int, edges: list[tuple[int, int]]) -> tuple[tuple[int, int], list[bool]]:
    """O(n) single-edge centroid separator for a tree (max degree ≤ 3, n ≥ 2).

    Cuts one edge (u, v) partitioning the vertices into two subtrees (side flags: True = subtree of u).
    """
    assert n >= 2 and len(edges) == n - 1
    adj: list[list[int]] = [[] for _ in range(n)]
    for u, v in edges:
        adj[u].append(v)
        adj[v].append(u)
    size = [0] * n
    parent = [-1] * n
    order: list[int] = []
    stack = [0]
    parent[0] = 0
    while stack:
        u = stack.pop()
        order.append(u)
        for v in adj[u]:
            if v != parent[u]:
                parent[v] = u
                stack.append(v)
    for u in reversed(order):
        size[u] = 1
        for v in adj[u]:
            if v != parent[u]:
                size[u] += size[v]
    best_u, best_min = 1, 0
    for u in range(1, n):
        s = min(size[u], n - size[u])
        if s > best_min:
            best_min, best_u = s, u
    p = parent[best_u]
    side = [False] * n
    st = [best_u]
    side[best_u] = True
    while st:
        u = st.pop()
        for v in adj[u]:
            if v != p and not side[v]:
                side[v] = True
                st.append(v)
    return (best_u, p), side


def tree_exact_bisection(n: int, edges: list[tuple[int, int]]) -> tuple[list[bool], list[tuple[int, int]]]:
    """Recursive centroid-based exact bisection: side A has n // 2 vertices; returns (side mask, cut edges)."""
    assert len(edges) == max(n - 1, 0)
    if n <= 1:
        return [True] * n, []
    adj: list[list[int]] = [[] for _ in range(n)]
    for u, v in edges:
        adj[u].append(v)
        adj[v].append(u)
    side = [False] * n

    def assign(comp: list[int], need: int) -> None:
        if need == 0 or not comp:
            return
        if need == len(comp):
            for u in comp:
                side[u] = True
            return
        in_comp = [False] * n
        for u in comp:
            in_comp[u] = True
        parent = [-1] * n
        order: list[int] = []
        st = [comp[0]]
        parent[comp[0]] = comp[0]
        while st:
            u = st.pop()
            order.append(u)
            for v in adj[u]:
                if in_comp[v] and v != parent[u]:
                    parent[v] = u
                    st.append(v)
        sz = [0] * n
        for u in reversed(order):
            sz[u] = 1
            for v in adj[u]:
                if in_comp[v] and v != parent[u]:
                    sz[u] += sz[v]
        m = len(comp)

        def is_centroid(u: int) -> bool:
            max_branch = m - sz[u]
            for v in adj[u]:
                if in_comp[v] and v != parent[u]:
                    max_branch = max(max_branch, sz[v])
            return max_branch <= m // 2

        centroid = next(u for u in comp if is_centroid(u))
        branches: list[list[int]] = []
        for nbr in adj[centroid]:
            if not in_comp[nbr]:
                continue
            b: list[int] = []
            b_st = [(nbr, centroid)]
            while b_st:
                u, p = b_st.pop()
                b.append(u)
                for v in adj[u]:
                    if in_comp[v] and v != p:
                        b_st.append((v, u))
            branches.append(b)
        branches.sort(key=len)
        rem = need
        for b in branches:
            if rem == 0:
                break
            if len(b) <= rem:
                for u in b:
                    side[u] = True
                rem -= len(b)
            else:
                assign(b, rem)
                rem = 0
        if rem == 1:
            side[centroid] = True

    assign(list(range(n)), n // 2)
    cut = [(u, v) for u, v in edges if side[u] != side[v]]
    return side, cut


def demo() -> str:
    lines: list[str] = []
    edges = [(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6), (3, 7), (3, 8), (5, 9)]
    sep, side = binary_tree_edge_separator(10, edges)
    count_a = sum(side)
    lines.append(f"10-node binary tree single-edge separator {sep} -> split ({count_a}, {10 - count_a})")
    bi_side, cut = tree_exact_bisection(10, edges)
    bi_a = sum(bi_side)
    lines.append(f"exact bisection split ({bi_a}, {10 - bi_a}) cutting {len(cut)} edges: {cut}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_binary_tree_edge_separator_and_exact_bisection_bounds(self) -> None:
        r = Rng(3003)
        for _ in range(80):
            n = r.range(2, 60)
            deg = [0] * n
            edges: list[tuple[int, int]] = []
            for v in range(1, n):
                candidates = [u for u in range(v) if deg[u] < 3]
                u = candidates[r.range(0, len(candidates) - 1)]
                deg[u] += 1
                deg[v] += 1
                edges.append((u, v))
            _, side = binary_tree_edge_separator(n, edges)
            a = sum(side)
            b = n - a
            self.assertLessEqual(max(a, b), (3 * n) // 4, f"n={n}: split ({a}, {b}) exceeds 3n/4")
            bi_side, cut = tree_exact_bisection(n, edges)
            self.assertEqual(sum(bi_side), n // 2)
            self.assertTrue(cut)
            self.assertLessEqual(len(cut), ((n.bit_length() - 1) + 2) * 3)


if __name__ == "__main__":
    print(demo(), end="")
