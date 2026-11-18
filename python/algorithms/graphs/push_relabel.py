"""Push-Relabel & Relabel-to-Front — CLRS 26.4–26.5 (1e–3e).

Preflow-push maximum flow: generic O(V²E) and relabel-to-front O(V³).
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class _State:
    def __init__(self, n: int, edges: list[tuple[int, int, int]], s: int) -> None:
        self.n = n
        self.c = [[0] * n for _ in range(n)]
        for u, v, w in edges:
            self.c[u][v] += w
        self.f = [[0] * n for _ in range(n)]
        self.h = [0] * n
        self.e = [0] * n
        self.pushes = 0
        self.relabels = 0
        self.h[s] = n
        for v in range(n):
            if self.c[s][v] > 0:
                cap = self.c[s][v]
                self.f[s][v] = cap
                self.f[v][s] = -cap
                self.e[v] += cap
                self.e[s] -= cap

    def cf(self, u: int, v: int) -> int:
        return self.c[u][v] - self.f[u][v]

    def push(self, u: int, v: int) -> None:
        d = min(self.e[u], self.cf(u, v))
        self.f[u][v] += d
        self.f[v][u] -= d
        self.e[u] -= d
        self.e[v] += d
        self.pushes += 1

    def relabel(self, u: int) -> None:
        m = min(self.h[v] for v in range(self.n) if self.cf(u, v) > 0)
        self.h[u] = 1 + m
        self.relabels += 1


def generic_with_stats(n: int, edges: list[tuple[int, int, int]], s: int, t: int) -> tuple[int, int, int]:
    """Returns (max flow, pushes, relabels)."""
    st = _State(n, edges, s)
    while True:
        progressed = False
        for u in range(n):
            if u == s or u == t or st.e[u] <= 0:
                continue
            target = next((v for v in range(n) if st.cf(u, v) > 0 and st.h[u] == st.h[v] + 1), None)
            if target is not None:
                st.push(u, target)
            else:
                st.relabel(u)
            progressed = True
            break
        if not progressed:
            return st.e[t], st.pushes, st.relabels


def generic_push_relabel(n: int, edges: list[tuple[int, int, int]], s: int, t: int) -> int:
    """GENERIC-PUSH-RELABEL: apply any applicable push or relabel until no vertex (other than s, t) overflows."""
    return generic_with_stats(n, edges, s, t)[0]


def relabel_to_front(n: int, edges: list[tuple[int, int, int]], s: int, t: int) -> int:
    """RELABEL-TO-FRONT: discharge vertices from a list, moving relabelled vertices to the front. O(V³)."""
    st = _State(n, edges, s)
    lst = [v for v in range(n) if v != s and v != t]
    cur = [0] * n
    i = 0
    while i < len(lst):
        u = lst[i]
        old = st.h[u]
        while st.e[u] > 0:
            if cur[u] == n:
                st.relabel(u)
                cur[u] = 0
            else:
                v = cur[u]
                if st.cf(u, v) > 0 and st.h[u] == st.h[v] + 1:
                    st.push(u, v)
                else:
                    cur[u] += 1
        if st.h[u] > old:
            lst.pop(i)
            lst.insert(0, u)
            i = 0
        i += 1
    return st.e[t]


def _edmonds_karp(n: int, edges: list[tuple[int, int, int]], s: int, t: int) -> int:
    to: list[int] = []
    cap: list[int] = []
    adj: list[list[int]] = [[] for _ in range(n)]
    for u, v, c in edges:
        eid = len(to)
        to.extend((v, u))
        cap.extend((c, 0))
        adj[u].append(eid)
        adj[v].append(eid + 1)
    flow = 0
    while True:
        pred = [-1] * n
        seen = [False] * n
        seen[s] = True
        q: deque[int] = deque([s])
        while q:
            u = q.popleft()
            for e in adj[u]:
                v = to[e]
                if not seen[v] and cap[e] > 0:
                    seen[v] = True
                    pred[v] = e
                    q.append(v)
        if not seen[t]:
            return flow
        b = 1 << 60
        v = t
        while v != s:
            e = pred[v]
            if cap[e] < b:
                b = cap[e]
            v = to[e ^ 1]
        v = t
        while v != s:
            e = pred[v]
            cap[e] -= b
            cap[e ^ 1] += b
            v = to[e ^ 1]
        flow += b


def demo() -> str:
    lines: list[str] = []
    edges = [(0, 1, 16), (0, 2, 13), (1, 3, 12), (2, 1, 4), (2, 4, 14), (3, 2, 9), (3, 5, 20), (4, 3, 7), (4, 5, 4)]
    f, pushes, relabels = generic_with_stats(6, edges, 0, 5)
    lines.append(f"generic push-relabel: max flow {f} using {pushes} pushes and {relabels} relabels")
    lines.append(f"relabel-to-front    : max flow {relabel_to_front(6, edges, 0, 5)}")
    lines.append(f"Edmonds-Karp check  : {_edmonds_karp(6, edges, 0, 5)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_network(self) -> None:
        edges = [(0, 1, 16), (0, 2, 13), (1, 3, 12), (2, 1, 4), (2, 4, 14), (3, 2, 9), (3, 5, 20), (4, 3, 7), (4, 5, 4)]
        self.assertEqual(generic_push_relabel(6, edges, 0, 5), 23)
        self.assertEqual(relabel_to_front(6, edges, 0, 5), 23)

    def test_agrees_with_edmonds_karp_on_random_networks(self) -> None:
        r = Rng(139)
        for _ in range(80):
            n = r.range(2, 9)
            edges = [
                (u, v, r.range(1, 20))
                for u, v in ((r.range(0, n - 1), r.range(0, n - 1)) for _ in range(r.range(1, 25)))
                if u != v
            ]
            want = _edmonds_karp(n, edges, 0, n - 1)
            self.assertEqual(generic_push_relabel(n, edges, 0, n - 1), want, f"{edges}")
            self.assertEqual(relabel_to_front(n, edges, 0, n - 1), want, f"{edges}")


if __name__ == "__main__":
    print(demo(), end="")
