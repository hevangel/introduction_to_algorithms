"""Currency Arbitrage — CLRS Problem 24-3 (1e–4e) / 22-3.

An arbitrage cycle is a cycle with product of exchange rates > 1 — a negative cycle for weights −ln R. Detect and print one with Bellman-Ford.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def find_arbitrage(r: list[list[float]]) -> tuple[list[int], float] | None:
    """Find a cycle of currencies whose exchange-rate product exceeds 1, if any.

    r[i][j] = units of currency j obtained for one unit of currency i. Returns (cycle, profit factor).
    """
    n = len(r)
    w = [[-math.log(r[i][j]) for j in range(n)] for i in range(n)]
    d = [0.0] * n
    pi = [-1] * n
    last: int | None = None
    for _ in range(n):
        last = None
        for i in range(n):
            for j in range(n):
                if i != j and d[i] + w[i][j] < d[j] - 1e-12:
                    d[j] = d[i] + w[i][j]
                    pi[j] = i
                    last = j
        if last is None:
            return None
    if last is None:
        return None
    x = last
    for _ in range(n):
        x = pi[x]
    cycle = [x]
    y = pi[x]
    while y != x:
        cycle.append(y)
        y = pi[y]
    cycle.reverse()
    profit = 1.0
    for k in range(len(cycle)):
        profit *= r[cycle[k]][cycle[(k + 1) % len(cycle)]]
    return cycle, profit


def demo() -> str:
    lines: list[str] = []
    # 0 = USD, 1 = EUR, 2 = GBP, 3 = JPY
    r = [
        [1.0, 0.90, 0.78, 110.0],
        [1.12, 1.0, 0.87, 123.0],
        [1.30, 1.15, 1.0, 142.0],
        [0.0090, 0.0081, 0.0070, 1.0],
    ]
    res = find_arbitrage(r)
    if res is not None:
        c, p = res
        lines.append(f"arbitrage cycle {c} multiplies money by {p:.4f}")
    else:
        lines.append("no arbitrage")
    base = [1.0, 0.9, 0.8, 110.0]
    consistent = [[base[j] / base[i] for j in range(4)] for i in range(4)]
    cons_res = find_arbitrage(consistent)
    lines.append(f"rates derived from a single price vector: {cons_res[0] if cons_res is not None else None}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_detects_planted_arbitrage_and_returns_a_profitable_cycle(self) -> None:
        rng = Rng(358)
        for _ in range(100):
            n = rng.range(2, 7)
            price = [0.5 + rng.f64() * 100.0 for _ in range(n)]
            r = [[price[j] / price[i] for j in range(n)] for i in range(n)]
            self.assertIsNone(find_arbitrage(r), "consistent rates have no arbitrage")
            a, b = rng.range(0, n - 1), rng.range(0, n - 1)
            if a == b:
                continue
            r[a][b] *= 1.2
            res = find_arbitrage(r)
            self.assertIsNotNone(res, "arbitrage must be found")
            cycle, profit = res  # type: ignore[misc]
            self.assertGreater(profit, 1.0 + 1e-9, f"profit {profit}")
            self.assertGreaterEqual(len(cycle), 2)
            self.assertTrue(all(cycle[k] != cycle[(k + 1) % len(cycle)] for k in range(len(cycle))))

    def test_matches_brute_force_on_random_tables(self) -> None:
        rng = Rng(359)
        for _ in range(200):
            n = rng.range(2, 5)
            r = [[1.0 if i == j else 0.5 + rng.f64() * 1.2 for j in range(n)] for i in range(n)]

            def go(start: int, u: int, prod: float, vis: list[bool]) -> bool:
                for v in range(n):
                    if v == u:
                        continue
                    if v == start:
                        if prod * r[u][v] > 1.0 + 1e-9:
                            return True
                    elif v > start and not vis[v]:
                        vis[v] = True
                        f = go(start, v, prod * r[u][v], vis)
                        vis[v] = False
                        if f:
                            return True
                return False

            exists = False
            for s in range(n):
                vis = [False] * n
                vis[s] = True
                if go(s, s, 1.0, vis):
                    exists = True
                    break
            self.assertEqual(find_arbitrage(r) is not None, exists)


if __name__ == "__main__":
    print(demo(), end="")
