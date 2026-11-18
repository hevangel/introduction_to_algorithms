"""Inventory Planning — CLRS Problem 15-11 (3e) / 14-11 (4e) (3e,4e).

Meet monthly demand with regular capacity m, costly overtime and a holding cost h(j): DP over (month, inventory) in O(n*D^2).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def plan(demand: list[int], capacity: int, overtime: int, hold: list[int]) -> tuple[int, list[int]]:
    """Minimum total cost and the production plan."""
    total = sum(demand)
    if len(hold) <= total or hold[0] != 0:
        raise ValueError("hold table must cover sum(demand) with hold[0] == 0")
    inf = 1 << 60
    n = len(demand)
    f = [[inf] * (total + 1) for _ in range(n + 1)]
    frm = [[0] * (total + 1) for _ in range(n + 1)]
    f[0][0] = 0
    for i in range(1, n + 1):
        d = demand[i - 1]
        for prev in range(total + 1):
            if f[i - 1][prev] >= inf:
                continue
            for inv in range(total + 1):
                x = inv + d - prev
                if x < 0:
                    continue
                c = f[i - 1][prev] + overtime * max(x - capacity, 0) + hold[inv]
                if c < f[i][inv]:
                    f[i][inv] = c
                    frm[i][inv] = prev
    best_inv = min(range(total + 1), key=lambda inv: f[n][inv])
    best = f[n][best_inv]
    prod = [0] * n
    inv = best_inv
    for i in range(n, 0, -1):
        prev = frm[i][inv]
        prod[i - 1] = inv + demand[i - 1] - prev
        inv = prev
    return best, prod


def evaluate(
    demand: list[int],
    prod: list[int],
    capacity: int,
    overtime: int,
    hold: list[int],
) -> int | None:
    """Cost of a given production plan (None if inventory ever drops below 0)."""
    inv = 0
    cost = 0
    for d, x in zip(demand, prod):
        inv += x - d
        if inv < 0:
            return None
        cost += overtime * max(x - capacity, 0) + hold[inv]
    return cost


def demo() -> str:
    lines: list[str] = []
    demand = [3, 8, 2, 7, 4]
    hold = [2 * j for j in range(25)]
    cost, p = plan(demand, 5, 10, hold)
    lines.append(f"demand {demand}, regular capacity 5/month, overtime $10/machine, holding $2/machine/month")
    lines.append(f"optimal plan {p} costs ${cost}")
    lines.append(f"'produce exactly the demand' costs ${evaluate(demand, demand, 5, 10, hold)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _brute(self, demand: list[int], cap: int, c: int, hold: list[int], i: int, inv: int) -> int:
        if i == len(demand):
            return 0
        total = sum(demand)
        best = 1 << 60
        for x in range(total + 1):
            if inv + x < demand[i]:
                continue
            ninv = inv + x - demand[i]
            if ninv > total:
                continue
            best = min(
                best,
                c * max(x - cap, 0) + hold[ninv] + self._brute(demand, cap, c, hold, i + 1, ninv),
            )
        return best

    def test_matches_exhaustive_search_and_plan_is_consistent(self) -> None:
        r = Rng(342)
        for _ in range(80):
            n = r.range(1, 4)
            demand = [r.range(0, 4) for _ in range(n)]
            total = sum(demand)
            hold = [0]
            for _ in range(total):
                hold.append(hold[-1] + r.range(0, 4))
            cap, c = r.range(0, 4), r.range(1, 9)
            cost, p = plan(demand, cap, c, hold)
            self.assertEqual(cost, self._brute(demand, cap, c, hold, 0, 0), str(demand))
            self.assertEqual(evaluate(demand, p, cap, c, hold), cost)

    def test_free_inventory_levels_production(self) -> None:
        hold = [0] * 21
        cost, p = plan([0, 0, 12], 4, 100, hold)
        self.assertEqual((cost, sum(p)), (0, 12))


if __name__ == "__main__":
    print(demo(), end="")
