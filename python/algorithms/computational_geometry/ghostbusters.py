"""Ghostbusters and Ghosts — CLRS Problem 33-4 (2e–3e) (2,3).

Match n ghostbusters to n ghosts by non-crossing segments (no three points collinear): split by the lowest point's angular sweep, O(n² lg n) worst case.
"""

from __future__ import annotations

from functools import cmp_to_key
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Pt = tuple[int, int]
Person = tuple[Pt, bool]


def cross(p0: Pt, p1: Pt, p2: Pt) -> int:
    return (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1])


def match_no_crossing(people: list[Person]) -> list[tuple[int, int]]:
    """Perfect matching between busters and ghosts (index pairs) with non-crossing segments."""
    out: list[tuple[int, int]] = []

    def solve(idx: list[int]) -> None:
        if not idx:
            return
        p = min(idx, key=lambda i: (people[i][0][1], people[i][0][0]))
        rest = [i for i in idx if i != p]
        o = people[p][0]

        def cmp_idx(a: int, b: int) -> int:
            c = cross(o, people[a][0], people[b][0])
            return -1 if c > 0 else (1 if c < 0 else 0)

        rest.sort(key=cmp_to_key(cmp_idx))
        balance = 0
        for k, q in enumerate(rest):
            same = people[q][1] == people[p][1]
            if not same and balance == 0:
                out.append((p, q))
                solve(rest[:k])
                solve(rest[k + 1 :])
                return
            balance += 1 if same else -1
        raise RuntimeError("a balanced split always exists when #busters = #ghosts")

    solve(list(range(len(people))))
    return out


def crosses(a: Pt, b: Pt, c: Pt, d: Pt) -> bool:
    """Do two segments (a,b) and (c,d) cross properly? (Valid when no three points are collinear.)"""
    return (cross(a, b, c) > 0) != (cross(a, b, d) > 0) and (cross(c, d, a) > 0) != (
        cross(c, d, b) > 0
    )


def demo() -> str:
    people: list[Person] = [
        ((0, 0), True),
        ((10, 1), True),
        ((4, 9), True),
        ((9, 8), False),
        ((2, 5), False),
        ((6, 3), False),
    ]
    m = match_no_crossing(people)
    lines = [
        f"{people[a][0]} ({'buster' if people[a][1] else 'ghost'}) — "
        f"{people[b][0]} ({'buster' if people[b][1] else 'ghost'})"
        for a, b in m
    ]
    crossing = any(
        crosses(people[m[i][0]][0], people[m[i][1]][0], people[m[j][0]][0], people[m[j][1]][0])
        for i in range(len(m))
        for j in range(i + 1, len(m))
    )
    lines.append(f"any two segments cross? {crossing}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    @staticmethod
    def _no_three_collinear(p: list[Pt]) -> bool:
        return all(
            cross(p[i], p[j], p[k]) != 0
            for i in range(len(p))
            for j in range(i + 1, len(p))
            for k in range(j + 1, len(p))
        )

    def test_matchings_are_perfect_bichromatic_and_planar(self) -> None:
        r = Rng(408)
        checked = 0
        while checked < 200:
            n = r.range(1, 6)
            pts = [(r.range(0, 40), r.range(0, 40)) for _ in range(2 * n)]
            if len(set(pts)) < 2 * n or not self._no_three_collinear(pts):
                continue
            people = [(p, i < n) for i, p in enumerate(pts)]
            m = match_no_crossing(people)
            self.assertEqual(len(m), n)
            used = sorted(x for pair in m for x in pair)
            self.assertEqual(used, list(range(2 * n)))
            self.assertTrue(all(people[a][1] != people[b][1] for a, b in m))
            for i in range(len(m)):
                for j in range(i + 1, len(m)):
                    self.assertFalse(
                        crosses(pts[m[i][0]], pts[m[i][1]], pts[m[j][0]], pts[m[j][1]])
                    )
            checked += 1

    def test_crossing_predicate(self) -> None:
        self.assertTrue(crosses((0, 0), (4, 4), (0, 4), (4, 0)))
        self.assertFalse(crosses((0, 0), (1, 1), (3, 0), (4, 5)))


if __name__ == "__main__":
    print(demo(), end="")
