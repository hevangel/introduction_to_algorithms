"""Signing Free-Agent Players — CLRS Problem 15-12 (3e) / 14-12 (4e) (3e,4e).

Fill every position with exactly one free agent, maximising total VORP under a salary budget: multiple-choice knapsack in O(N*X*P).
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


@dataclass(frozen=True)
class Player:
    salary: int
    vorp: int


def sign_players(positions: list[list[Player]], budget: int) -> tuple[int, list[int]] | None:
    """Choose one candidate per position with total salary <= budget, maximising total VORP."""
    n = len(positions)
    neg = -(1 << 60)
    v = [[neg] * (budget + 1) for _ in range(n + 1)]
    pick = [[-1] * (budget + 1) for _ in range(n + 1)]
    v[0] = [0] * (budget + 1)
    for i in range(1, n + 1):
        for x in range(budget + 1):
            for c, p in enumerate(positions[i - 1]):
                if p.salary <= x and v[i - 1][x - p.salary] > neg:
                    cand = v[i - 1][x - p.salary] + p.vorp
                    if cand > v[i][x]:
                        v[i][x] = cand
                        pick[i][x] = c
    if v[n][budget] <= neg:
        return None
    x = budget
    chosen = [0] * n
    for i in range(n, 0, -1):
        c = pick[i][x]
        chosen[i - 1] = c
        x -= positions[i - 1][c].salary
    return v[n][budget], chosen


def demo() -> str:
    lines: list[str] = []
    pos = [
        [Player(salary=5, vorp=6), Player(salary=2, vorp=2), Player(salary=8, vorp=9)],
        [Player(salary=4, vorp=5), Player(salary=1, vorp=1)],
        [Player(salary=6, vorp=7), Player(salary=3, vorp=3), Player(salary=2, vorp=1)],
    ]
    for budget in (6, 9, 12, 20):
        lines.append(f"budget {budget:>2}: {sign_players(pos, budget)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _brute(self, pos: list[list[Player]], i: int, left: int) -> int | None:
        if i == len(pos):
            return 0
        cands: list[int] = []
        for p in pos[i]:
            if p.salary <= left:
                r = self._brute(pos, i + 1, left - p.salary)
                if r is not None:
                    cands.append(r + p.vorp)
        return max(cands) if cands else None

    def test_matches_enumeration(self) -> None:
        r = Rng(344)
        for _ in range(200):
            positions = [
                [Player(salary=r.range(0, 8), vorp=r.range(0, 20)) for _ in range(r.range(1, 4))]
                for _ in range(r.range(1, 5))
            ]
            budget = r.range(0, 25)
            got = sign_players(positions, budget)
            self.assertEqual(got[0] if got is not None else None, self._brute(positions, 0, budget))
            if got is not None:
                v, ch = got
                self.assertEqual(sum(positions[i][c].vorp for i, c in enumerate(ch)), v)
                self.assertLessEqual(sum(positions[i][c].salary for i, c in enumerate(ch)), budget)

    def test_infeasible_budget(self) -> None:
        self.assertIsNone(sign_players([[Player(salary=5, vorp=1)]], 4))


if __name__ == "__main__":
    print(demo(), end="")
