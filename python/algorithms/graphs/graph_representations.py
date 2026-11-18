"""Graph Representations — CLRS 22.1 (3e) / 20.1 (4e) / 23.1 (1e–2e).

Adjacency lists and matrices, transposing a graph, squaring a graph, and the O(V) universal-sink test.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

AdjList = list[list[int]]
AdjMatrix = list[list[bool]]


def list_to_matrix(adj: AdjList) -> AdjMatrix:
    n = len(adj)
    m = [[False] * n for _ in range(n)]
    for u, vs in enumerate(adj):
        for v in vs:
            m[u][v] = True
    return m


def matrix_to_list(m: AdjMatrix) -> AdjList:
    return [[v for v, b in enumerate(row) if b] for row in m]


def transpose(adj: AdjList) -> AdjList:
    """Exercise 22.1-3: G^T in Θ(V+E) with adjacency lists."""
    t: AdjList = [[] for _ in range(len(adj))]
    for u, vs in enumerate(adj):
        for v in vs:
            t[v].append(u)
    return t


def square(adj: AdjList) -> AdjList:
    """Exercise 22.1-5: the square G² (edge (u,w) iff G has a path of length ≤ 2 from u to w), Θ(V·E)."""
    n = len(adj)
    out: AdjList = [[] for _ in range(n)]
    for u in range(n):
        seen = [False] * n
        for v in adj[u]:
            seen[v] = True
            for w in adj[v]:
                seen[w] = True
        out[u] = [w for w in range(n) if seen[w]]
    return out


def universal_sink(m: AdjMatrix) -> int | None:
    """Exercise 22.1-6: find a universal sink (in-degree |V|−1, out-degree 0) in O(V) matrix probes."""
    n = len(m)
    i = j = 0
    while i < n and j < n:
        if m[i][j]:
            i += 1
        else:
            j += 1
    if i >= n:
        return None
    if all(not m[i][k] and (k == i or m[k][i]) for k in range(n)):
        return i
    return None


def degrees(adj: AdjList) -> tuple[list[int], list[int]]:
    n = len(adj)
    ind = [0] * n
    for vs in adj:
        for v in vs:
            ind[v] += 1
    return ind, [len(vs) for vs in adj]


def demo() -> str:
    lines: list[str] = []
    adj: AdjList = [[1, 3], [4], [4, 5], [1], [3], [5]]
    lines.append(f"adjacency list : {adj}")
    m = list_to_matrix(adj)
    for row in m:
        lines.append("  " + "".join("1" if b else "0" for b in row))
    lines.append(f"round trip equal: {matrix_to_list(m) == adj}")
    lines.append(f"transpose      : {transpose(adj)}")
    lines.append(f"square G²      : {square(adj)}")
    sink = [[False, False, False], [True, False, True], [True, False, False]]
    lines.append(f"universal sink of {sink}: {universal_sink(sink)}")
    ind, outd = degrees(adj)
    lines.append(f"in-degrees {ind}, out-degrees {outd}")
    return "\n".join(lines) + "\n"


def _random_graph(n: int, p: float, r: Rng) -> AdjList:
    return [[v for v in range(n) if u != v and r.f64() < p] for u in range(n)]


class Tests(unittest.TestCase):
    def test_conversions_roundtrip_and_transpose_twice(self) -> None:
        r = Rng(120)
        for n in (0, 1, 5, 12):
            g = _random_graph(n, 0.3, r)
            self.assertEqual(matrix_to_list(list_to_matrix(g)), g)
            tt = transpose(transpose(g))
            for vs in tt:
                vs.sort()
            self.assertEqual(tt, g)

    def test_square_matches_matrix_definition(self) -> None:
        r = Rng(121)
        g = _random_graph(10, 0.2, r)
        m = list_to_matrix(g)
        sq = list_to_matrix(square(g))
        for u in range(10):
            for w in range(10):
                want = m[u][w] or any(m[u][v] and m[v][w] for v in range(10))
                self.assertEqual(sq[u][w], want or (u == w and any(m[u][v] and m[v][u] for v in range(10))))

    def test_universal_sink_found_in_linear_probes(self) -> None:
        n = 8
        for sink in range(n):
            m = [[False] * n for _ in range(n)]
            for i in range(n):
                if i != sink:
                    m[i][sink] = True
            m[0][3] = (sink != 0 and sink != 3) or m[0][3]
            self.assertEqual(universal_sink(m), sink)
        none = [[False] * 4 for _ in range(4)]
        self.assertIsNone(universal_sink(none))


if __name__ == "__main__":
    print(demo(), end="")
