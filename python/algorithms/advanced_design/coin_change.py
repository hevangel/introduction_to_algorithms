"""Coin Changing — CLRS Problem 16-1 (1e–4e) (1e,2e,3e,4e).

Greedy change-making is optimal for powers of c, but not for arbitrary denominations; DP always works.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def greedy_change(coins: list[int], amount: int) -> list[int] | None:
    """Greedy: repeatedly take the largest coin that fits."""
    cs = sorted(coins, reverse=True)
    rest = amount
    used: list[int] = []
    for c in cs:
        while rest >= c:
            rest -= c
            used.append(c)
    return used if rest == 0 else None


def dp_change(coins: list[int], amount: int) -> list[int] | None:
    """Dynamic programming: minimum number of coins and one optimal multiset (O(n*amount))."""
    inf = sys.maxsize
    best = [inf] * (amount + 1)
    pick = [0] * (amount + 1)
    best[0] = 0
    for v in range(1, amount + 1):
        for c in coins:
            if c <= v and best[v - c] != inf and best[v - c] + 1 < best[v]:
                best[v] = best[v - c] + 1
                pick[v] = c
    if best[amount] == inf:
        return None
    v = amount
    used: list[int] = []
    while v > 0:
        used.append(pick[v])
        v -= pick[v]
    return used


def power_coins(c: int, k: int) -> list[int]:
    """Problem 16-1(a): for coins c^0, c^1, ..., c^k the greedy algorithm is optimal."""
    return [c**i for i in range(k + 1)]


def demo() -> str:
    lines: list[str] = []
    us = [25, 10, 5, 1]
    lines.append(f"US coins, 67¢: greedy {greedy_change(us, 67)}")
    pw = power_coins(3, 4)
    lines.append(f"coins {pw}: 100 → greedy {greedy_change(pw, 100)}, dp {dp_change(pw, 100)}")
    odd = [1, 3, 4]
    lines.append(
        f"coins {odd}: 6 → greedy {greedy_change(odd, 6)} (3 coins) but dp {dp_change(odd, 6)} (2 coins)"
    )
    lines.append(f"coins [5, 10]: 3 → greedy {greedy_change([5, 10], 3)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_greedy_is_optimal_for_powers_of_c(self) -> None:
        for c in range(2, 6):
            coins = power_coins(c, 4)
            for amount in range(400):
                g = greedy_change(coins, amount)
                d = dp_change(coins, amount)
                assert g is not None and d is not None
                self.assertEqual(len(g), len(d), f"c={c} n={amount}")

    def test_greedy_fails_in_general(self) -> None:
        coins = [1, 3, 4]
        g = greedy_change(coins, 6)
        d = dp_change(coins, 6)
        assert g is not None and d is not None
        self.assertEqual(len(g), 3)
        self.assertEqual(len(d), 2)

    def test_us_coins_are_canonical_and_impossible_amounts(self) -> None:
        us = [25, 10, 5, 1]
        for n in range(300):
            g = greedy_change(us, n)
            d = dp_change(us, n)
            assert g is not None and d is not None
            self.assertEqual(len(g), len(d))
        self.assertIsNone(dp_change([5, 10], 3))
        self.assertIsNone(greedy_change([2], 3))
        self.assertEqual(dp_change([2], 0), [])


if __name__ == "__main__":
    print(demo(), end="")
