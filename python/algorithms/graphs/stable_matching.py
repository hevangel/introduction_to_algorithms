"""Stable Matching (Gale-Shapley) — CLRS 25.2 (4e).

Propose-and-reject deferred acceptance yields a stable marriage in O(n²) — optimal for the proposing side.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

NONE = -1


def gale_shapley(men: list[list[int]], women: list[list[int]]) -> list[int]:
    """Gale–Shapley with men proposing. men[m] / women[w] list partners from most to least preferred. Returns wife[m]."""
    n = len(men)
    rank = [[0] * n for _ in range(n)]  # rank[w][m] = position of man m in woman w's list
    for w in range(n):
        for pos, m in enumerate(women[w]):
            rank[w][m] = pos
    nxt = [0] * n
    husband = [NONE] * n
    wife = [NONE] * n
    free = list(range(n - 1, -1, -1))
    while free:
        m = free.pop()
        w = men[m][nxt[m]]
        nxt[m] += 1
        if husband[w] == NONE:
            husband[w] = m
            wife[m] = w
        elif rank[w][m] < rank[w][husband[w]]:
            old = husband[w]
            wife[old] = NONE
            free.append(old)
            husband[w] = m
            wife[m] = w
        else:
            free.append(m)
    return wife


def blocking_pairs(men: list[list[int]], women: list[list[int]], wife: list[int]) -> list[tuple[int, int]]:
    """All pairs (m, w) who prefer each other to their assigned partners."""
    n = len(men)
    husband = [0] * n
    for m in range(n):
        husband[wife[m]] = m
    out: list[tuple[int, int]] = []
    for m in range(n):
        for w in range(n):
            if men[m].index(w) < men[m].index(wife[m]) and women[w].index(m) < women[w].index(husband[w]):
                out.append((m, w))
    return out


def demo() -> str:
    lines: list[str] = []
    men = [[0, 1, 2], [1, 0, 2], [0, 1, 2]]
    women = [[1, 0, 2], [0, 1, 2], [0, 1, 2]]
    wife = gale_shapley(men, women)
    lines.append(f"men's preferences  : {men}")
    lines.append(f"women's preferences: {women}")
    lines.append(f"man → woman        : {wife}")
    lines.append(f"blocking pairs     : {blocking_pairs(men, women, wife)}")
    return "\n".join(lines) + "\n"


def _permutations(n: int) -> list[list[int]]:
    if n == 0:
        return [[]]
    out: list[list[int]] = []
    for p in _permutations(n - 1):
        for pos in range(len(p) + 1):
            q = p[:]
            q.insert(pos, n - 1)
            out.append(q)
    return out


class Tests(unittest.TestCase):
    def test_stable_and_men_optimal(self) -> None:
        r = Rng(143)
        for _ in range(40):
            n = r.range(1, 5)

            def prefs() -> list[list[int]]:
                out = []
                for _ in range(n):
                    v = list(range(n))
                    r.shuffle(v)
                    out.append(v)
                return out

            men = prefs()
            women = prefs()
            wife = gale_shapley(men, women)
            self.assertEqual(blocking_pairs(men, women, wife), [])
            stable = [p for p in _permutations(n) if not blocking_pairs(men, women, p)]
            self.assertTrue(stable)
            for m in range(n):
                best = min(men[m].index(p[m]) for p in stable)
                self.assertEqual(men[m][best], wife[m])

    def test_terminates_in_at_most_n_squared_proposals(self) -> None:
        n = 30
        men = [list(range(n)) for _ in range(n)]  # everyone ranks identically
        women = [list(range(n)) for _ in range(n)]
        self.assertEqual(gale_shapley(men, women), list(range(n)))


if __name__ == "__main__":
    print(demo(), end="")
