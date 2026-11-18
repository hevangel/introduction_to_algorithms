"""Hungarian Algorithm (Assignment Problem) — CLRS 25.3 (4e).

Minimum-cost perfect matching in a complete bipartite graph in O(n³) using vertex potentials.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def hungarian(cost: list[list[int]]) -> tuple[int, list[int]]:
    """Minimum-cost assignment for an n×n cost matrix (cost[i][j] = cost of giving job j to worker i).

    Returns (total cost, assignment[i] = job of worker i).
    """
    n = len(cost)
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
                    cur = cost[i0 - 1][j - 1] - u[i0] - v[j]
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
    total = sum(cost[i][assign[i]] for i in range(n))
    return total, assign


def max_weight_assignment(w: list[list[int]]) -> tuple[int, list[int]]:
    """Maximum-weight assignment (the 4th edition's formulation) by negating weights."""
    neg = [[-x for x in row] for row in w]
    c, a = hungarian(neg)
    return -c, a


def demo() -> str:
    lines: list[str] = []
    cost = [[9, 2, 7, 8], [6, 4, 3, 7], [5, 8, 1, 8], [7, 6, 9, 4]]
    c, a = hungarian(cost)
    lines.append("cost matrix:")
    for r in cost:
        lines.append(f"  {r}")
    lines.append(f"minimum total cost = {c}; worker → job = {a}")
    w, a2 = max_weight_assignment(cost)
    lines.append(f"maximum-weight assignment = {w}; worker → job = {a2}")
    return "\n".join(lines) + "\n"


def _brute(cost: list[list[int]], row: int, used: list[bool]) -> int:
    if row == len(cost):
        return 0
    best = 1 << 60
    for j in range(len(cost)):
        if not used[j]:
            used[j] = True
            cand = cost[row][j] + _brute(cost, row + 1, used)
            if cand < best:
                best = cand
            used[j] = False
    return best


class Tests(unittest.TestCase):
    def test_book_style_example(self) -> None:
        cost = [[9, 2, 7, 8], [6, 4, 3, 7], [5, 8, 1, 8], [7, 6, 9, 4]]
        self.assertEqual(hungarian(cost)[0], 13)

    def test_matches_brute_force_including_negative_costs(self) -> None:
        r = Rng(142)
        for _ in range(80):
            n = r.range(1, 7)
            cost = [r.vec(n, -10, 30) for _ in range(n)]
            c, a = hungarian(cost)
            self.assertEqual(c, _brute(cost, 0, [False] * n))
            self.assertEqual(sorted(a), list(range(n)))
        self.assertEqual(hungarian([])[0], 0)


if __name__ == "__main__":
    print(demo(), end="")
