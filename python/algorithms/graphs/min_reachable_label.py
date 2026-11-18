"""Minimum Reachable Label — CLRS Problem 22-4 (1e–4e) / 20-4.

For every vertex v find the smallest-labelled vertex reachable from v, in O(V+E): sweep labels upward and search the transpose graph.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def min_reachable_label(adj: list[list[int]], label: list[int]) -> list[int]:
    """label[v] are distinct vertex labels. Returns min_label[v] = min{ label[u] : u reachable from v }."""
    n = len(adj)
    radj: list[list[int]] = [[] for _ in range(n)]
    for u, vs in enumerate(adj):
        for v in vs:
            radj[v].append(u)
    by_label = sorted(range(n), key=lambda v: label[v])
    unset = -1
    best = [unset] * n
    for s in by_label:
        if best[s] != unset:
            continue
        st = [s]
        best[s] = label[s]
        while st:
            u = st.pop()
            for p in radj[u]:
                if best[p] == unset:
                    best[p] = label[s]
                    st.append(p)
    return best


def demo() -> str:
    lines: list[str] = []
    adj = [[1], [2], [0, 3], [], [3], [4]]
    label = [5, 2, 4, 6, 1, 3]
    lines.append(f"edges {adj}, labels {label}")
    lines.append(f"minimum reachable label per vertex: {min_reachable_label(adj, label)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_search_from_every_vertex(self) -> None:
        r = Rng(354)
        for _ in range(200):
            n = r.range(1, 14)
            adj = [[v for v in range(n) if u != v and r.f64() < 0.15] for u in range(n)]
            label = list(range(n))
            r.shuffle(label)
            got = min_reachable_label(adj, label)
            for s in range(n):
                seen = [False] * n
                seen[s] = True
                st = [s]
                while st:
                    u = st.pop()
                    for v in adj[u]:
                        if not seen[v]:
                            seen[v] = True
                            st.append(v)
                self.assertEqual(got[s], min(label[v] for v in range(n) if seen[v]))


if __name__ == "__main__":
    print(demo(), end="")
