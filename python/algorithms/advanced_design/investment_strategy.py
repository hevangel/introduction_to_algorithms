"""Planning an Investment Strategy — CLRS Problem 15-10 (3e) / 14-10 (4e) (3e,4e).

Maximise the money after y years when each year all money sits in one investment: stay (fee f1) or switch (fee f2). O(n*y) DP with the two best predecessors.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def best_strategy(start: float, rates: list[list[float]], f1: float, f2: float) -> tuple[float, list[int]]:
    """rates[j][i] is the return factor of investment i in year j.

    Staying in the same investment costs fee f1, switching costs f2.
    Returns (final money, chosen investment per year).
    """
    years = len(rates)
    n = len(rates[0])
    best = [[-math.inf] * n for _ in range(years)]
    frm = [[-1] * n for _ in range(years)]
    for i in range(n):
        best[0][i] = (start - f2) * rates[0][i]
    for j in range(1, years):
        m1 = (-math.inf, -1)
        m2 = (-math.inf, -1)
        for k in range(n):
            v = (best[j - 1][k], k)
            if v[0] > m1[0]:
                m2 = m1
                m1 = v
            elif v[0] > m2[0]:
                m2 = v
        for i in range(n):
            stay = (best[j - 1][i] - f1) * rates[j][i]
            other = m1 if m1[1] != i else m2
            switch = -math.inf if other[1] == -1 else (other[0] - f2) * rates[j][i]
            if stay >= switch:
                best[j][i] = stay
                frm[j][i] = i
            else:
                best[j][i] = switch
                frm[j][i] = other[1]
    i = max(range(n), key=lambda idx: best[years - 1][idx])
    final_money = best[years - 1][i]
    plan = [0] * years
    for j in range(years - 1, -1, -1):
        plan[j] = i
        if j > 0:
            i = frm[j][i]
    return final_money, plan


def simulate(start: float, rates: list[list[float]], plan: list[int], f1: float, f2: float) -> float:
    """Money at the end of a given plan (one investment per year)."""
    money = start
    for j, i in enumerate(plan):
        fee = f1 if (j > 0 and plan[j - 1] == i) else f2
        money = (money - fee) * rates[j][i]
    return money


def demo() -> str:
    lines: list[str] = []
    rates = [
        [1.10, 1.02, 1.05],
        [0.95, 1.20, 1.01],
        [1.15, 0.98, 1.04],
        [1.00, 1.12, 1.30],
        [1.08, 1.01, 0.90],
    ]
    money, plan = best_strategy(10_000.0, rates, 20.0, 150.0)
    lines.append("5 years, 3 investments, fees: stay $20 / switch $150")
    lines.append(f"optimal plan (investment per year) = {plan}, final money ${money:.2f}")
    lines.append(f"always investment 1: ${simulate(10_000.0, rates, [0] * 5, 20.0, 150.0):.2f}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _brute(self, start: float, rates: list[list[float]], f1: float, f2: float) -> float:
        years, n = len(rates), len(rates[0])
        best = -math.inf
        for code in range(n**years):
            plan = [(code // (n**j)) % n for j in range(years)]
            best = max(best, simulate(start, rates, plan, f1, f2))
        return best

    def test_matches_exhaustive_search(self) -> None:
        r = Rng(343)
        for _ in range(100):
            years, n = r.range(1, 5), r.range(1, 4)
            rates = [[0.9 + r.f64() * 0.4 for _ in range(n)] for _ in range(years)]
            f1, f2 = r.f64() * 50.0, 50.0 + r.f64() * 200.0
            m, plan = best_strategy(10_000.0, rates, f1, f2)
            self.assertLess(abs(m - self._brute(10_000.0, rates, f1, f2)), 1e-6)
            self.assertLess(abs(simulate(10_000.0, rates, plan, f1, f2) - m), 1e-6)

    def test_single_investment_and_fee_effects(self) -> None:
        rates = [[1.1], [1.1], [1.1]]
        m, plan = best_strategy(1000.0, rates, 10.0, 50.0)
        self.assertEqual(plan, [0, 0, 0])
        expected = (((1000.0 - 50.0) * 1.1 - 10.0) * 1.1 - 10.0) * 1.1
        self.assertLess(abs(m - expected), 1e-9)


if __name__ == "__main__":
    print(demo(), end="")
