"""Euler-Tour Technique on Trees — CLRS 30.1-30.2 (1e) application (Editions: 1).

Compute every vertex's depth in O(lg n) PRAM time by listing the Euler tour and
running a parallel prefix over +-1.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def depths(children: list[list[int]], root: int) -> tuple[list[int], int]:
    """Depth of every vertex of the rooted tree given by `children`, computed with Euler-tour technique.

    Returns (depths, pointer-jumping rounds).
    """
    n = len(children)
    if n == 1:
        return ([0], 0)
    parent = [-1] * n
    for p, cs in enumerate(children):
        for c in cs:
            parent[c] = p
    adj: list[list[int]] = []
    for v in range(n):
        a: list[int] = []
        if parent[v] != -1:
            a.append(parent[v])
        a.extend(children[v])
        adj.append(a)

    def edge(a: int, b: int) -> int:
        return 2 * b if parent[b] == a else 2 * a + 1

    def ends(e: int) -> tuple[int, int]:
        v = e // 2
        return (parent[v], v) if e % 2 == 0 else (v, parent[v])

    nxt: list[int | None] = [None] * (2 * n)
    valid = [False] * (2 * n)
    for v in range(n):
        if v != root:
            valid[2 * v] = True
            valid[2 * v + 1] = True
    for e in range(2 * n):
        if not valid[e]:
            continue
        u, v = ends(e)
        pos = adj[v].index(u)
        w = adj[v][(pos + 1) % len(adj[v])]
        nxt[e] = edge(v, w)

    start = edge(root, children[root][0])
    last = next(e for e in range(2 * n) if nxt[e] == start)
    nxt[last] = None
    weight = [0 if not valid[e] else (1 if e % 2 == 0 else -1) for e in range(2 * n)]

    suf = list(weight)
    rounds = 0
    while any(x is not None for x in nxt):
        s_old, n_old = list(suf), list(nxt)
        for i in range(2 * n):
            j = n_old[i]
            if j is not None:
                suf[i] = s_old[i] + s_old[j]
                nxt[i] = n_old[j]
        rounds += 1

    total = sum(weight)
    depth = [0] * n
    for v in range(n):
        if v != root:
            e = 2 * v
            depth[v] = total - suf[e] + weight[e]
    return depth, rounds


def demo() -> str:
    lines: list[str] = []
    children = [[1, 2, 3], [4, 5], [], [6], [], [7], [], []]
    d, rounds = depths(children, 0)
    lines.append(f"children lists: {children}")
    lines.append(
        f"depths via Euler tour + list scan: {d} ({rounds} pointer-jumping rounds over {2 * 7} tour edges)"
    )
    return "\n".join(lines) + "\n"


def _sequential_depths(children: list[list[int]], root: int) -> list[int]:
    d = [0] * len(children)
    st = [root]
    while st:
        u = st.pop()
        for c in children[u]:
            d[c] = d[u] + 1
            st.append(c)
    return d


class Tests(unittest.TestCase):
    def test_matches_sequential_depths_on_random_trees(self) -> None:
        r = Rng(163)
        for n in [1, 2, 3, 8, 30, 100]:
            children: list[list[int]] = [[] for _ in range(n)]
            for v in range(1, n):
                children[r.range(0, v - 1)].append(v)
            d, rounds = depths(children, 0)
            self.assertEqual(d, _sequential_depths(children, 0), f"n={n}")
            if n > 1:
                self.assertLessEqual(rounds, math.ceil(math.log2(2 * n)) + 1)

    def test_path_and_star(self) -> None:
        path = [[i + 1] if i < 5 else [] for i in range(6)]
        self.assertEqual(depths(path, 0)[0], [0, 1, 2, 3, 4, 5])
        star = [[1, 2, 3, 4], [], [], [], []]
        self.assertEqual(depths(star, 0)[0], [0, 1, 1, 1, 1])


if __name__ == "__main__":
    print(demo(), end="")
