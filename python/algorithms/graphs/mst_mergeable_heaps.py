"""MST with Mergeable Heaps — CLRS Problem 19-2 (1e–4e).

Keep every component's outgoing edges in a mergeable heap; repeatedly extract a component's lightest edge, skip edges that became internal, union, and merge the heaps: O(E lg V) with leftist heaps.
"""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Edge = tuple[int, int, int]


@dataclass
class _Heap:
    key: tuple[int, int]
    rank: int = 1
    left: _Heap | None = None
    right: _Heap | None = None


def _rank(h: _Heap | None) -> int:
    return 0 if h is None else h.rank


def _merge(a: _Heap | None, b: _Heap | None) -> _Heap | None:
    if a is None:
        return b
    if b is None:
        return a
    if a.key > b.key:
        a, b = b, a
    a.right = _merge(a.right, b)
    if _rank(a.left) < _rank(a.right):
        a.left, a.right = a.right, a.left
    a.rank = _rank(a.right) + 1
    return a


def _pop(h: _Heap | None) -> tuple[tuple[int, int], _Heap | None] | None:
    if h is None:
        return None
    return h.key, _merge(h.left, h.right)


def _find(p: list[int], x: int) -> int:
    r = x
    while p[r] != r:
        r = p[r]
    c = x
    while p[c] != r:
        p[c], c = r, p[c]
    return r


def mst_mergeable_heaps(n: int, edges: list[Edge]) -> tuple[int, list[int]]:
    """MST-MERGEABLE-HEAP: returns (total weight, chosen edge ids). On a disconnected graph returns minimum spanning forest."""
    heaps: list[_Heap | None] = [None] * n
    for i, (u, v, w) in enumerate(edges):
        if u == v:
            continue
        for x in (u, v):
            heaps[x] = _merge(heaps[x], _Heap((w, i)))
    p = list(range(n))
    queue: deque[int] = deque(range(n))
    total = 0
    chosen: list[int] = []
    while queue:
        c = queue.popleft()
        if _find(p, c) != c:
            continue
        h = heaps[c]
        heaps[c] = None
        found: tuple[int, int, int] | None = None
        while True:
            res = _pop(h)
            if res is None:
                h = None
                break
            (w, eid), rest = res
            h = rest
            a, b = _find(p, edges[eid][0]), _find(p, edges[eid][1])
            if a != b:
                found = (w, eid, b if a == c else a)
                break
        if found is not None:
            w, eid, other = found
            total += w
            chosen.append(eid)
            p[other] = c
            heaps[c] = _merge(h, heaps[other])
            heaps[other] = None
            queue.append(c)
    return total, chosen


def _kruskal(n: int, edges: list[Edge]) -> tuple[int, list[int]]:
    p = list(range(n))
    chosen: list[int] = []
    for i in sorted(range(len(edges)), key=lambda idx: edges[idx][2]):
        a, b = _find(p, edges[i][0]), _find(p, edges[i][1])
        if a != b:
            p[a] = b
            chosen.append(i)
    return sum(edges[i][2] for i in chosen), chosen


def demo() -> str:
    lines: list[str] = []
    e: list[Edge] = [
        (0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2),
        (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7),
    ]
    w, ids = mst_mergeable_heaps(9, e)
    lines.append(f"MST weight {w} (Kruskal {_kruskal(9, e)[0]}), edges {[e[i] for i in ids]}")
    r = Rng(2)
    n = 20_000
    big: list[Edge] = [(r.range(0, v - 1), v, r.range(1, 100_000)) for v in range(1, n)]
    for _ in range(60_000):
        a, b = r.range(0, n - 1), r.range(0, n - 1)
        if a != b:
            big.append((a, b, r.range(1, 100_000)))
    lines.append(f"random graph n = {n}, m = {len(big)}: weight {mst_mergeable_heaps(n, big)[0]} = Kruskal {_kruskal(n, big)[0]}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_same_weight_and_forest_size_as_kruskal(self) -> None:
        r = Rng(446)
        for _ in range(200):
            n = r.range(1, 25)
            edges: list[Edge] = []
            for v in range(1, n):
                if r.f64() < 0.9:
                    edges.append((r.range(0, v - 1), v, r.range(1, 20)))
            for _ in range(r.range(0, 40)):
                edges.append((r.range(0, n - 1), r.range(0, n - 1), r.range(1, 20)))
            w, ids = mst_mergeable_heaps(n, edges)
            kw, kt = _kruskal(n, edges)
            self.assertEqual((w, len(ids)), (kw, len(kt)), f"{edges}")

    def test_leftist_heap_sorts(self) -> None:
        r = Rng(447)
        keys = r.vec(500, 0, 1000)
        h: _Heap | None = None
        for i, k in enumerate(keys):
            h = _merge(h, _Heap((k, i)))
        out: list[int] = []
        while True:
            res = _pop(h)
            if res is None:
                break
            k, rest = res
            out.append(k[0])
            h = rest
        self.assertEqual(out, sorted(keys))


if __name__ == "__main__":
    print(demo(), end="")
