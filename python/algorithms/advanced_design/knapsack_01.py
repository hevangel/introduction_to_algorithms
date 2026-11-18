"""0-1 Knapsack — CLRS Exercise 16.2-2 (2e–3e), 15.2-2 (4e) (1e,2e,3e,4e).

Pseudo-polynomial dynamic program O(nW) for items that cannot be split.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def knapsack_01(items: list[tuple[int, int]], cap: int) -> tuple[int, list[int]]:
    """Best total value and the chosen item indices for capacity cap; items are (weight, value)."""
    n = len(items)
    dp = [[0] * (cap + 1) for _ in range(n + 1)]
    for i in range(1, n + 1):
        w, v = items[i - 1]
        for c in range(cap + 1):
            dp[i][c] = dp[i - 1][c]
            if c >= w and dp[i - 1][c - w] + v > dp[i][c]:
                dp[i][c] = dp[i - 1][c - w] + v
    c = cap
    chosen: list[int] = []
    for i in range(n, 0, -1):
        if dp[i][c] != dp[i - 1][c]:
            chosen.append(i - 1)
            c -= items[i - 1][0]
    chosen.reverse()
    return dp[n][cap], chosen


def knapsack_unbounded(items: list[tuple[int, int]], cap: int) -> int:
    """Unbounded variant: unlimited copies of each item."""
    dp = [0] * (cap + 1)
    for c in range(1, cap + 1):
        for w, v in items:
            if c >= w:
                dp[c] = max(dp[c], dp[c - w] + v)
    return dp[cap]


def demo() -> str:
    lines: list[str] = []
    items = [(10, 60), (20, 100), (30, 120)]
    best, chosen = knapsack_01(items, 50)
    lines.append(f"items (weight, value) = {items}, capacity 50")
    lines.append(f"0-1 knapsack optimum = {best} using items {chosen}")
    lines.append(f"unbounded optimum    = {knapsack_unbounded(items, 50)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_thief_example(self) -> None:
        v, c = knapsack_01([(10, 60), (20, 100), (30, 120)], 50)
        self.assertEqual(v, 220)
        self.assertEqual(c, [1, 2])

    def test_matches_brute_force_and_respects_capacity(self) -> None:
        r = Rng(107)
        for _ in range(80):
            n = r.range(0, 10)
            items = [(r.range(1, 9), r.range(1, 30)) for _ in range(n)]
            cap = r.range(0, 25)
            best, chosen = knapsack_01(items, cap)
            self.assertLessEqual(sum(items[i][0] for i in chosen), cap)
            self.assertEqual(sum(items[i][1] for i in chosen), best)
            brute = max(
                sum(items[i][1] for i in range(n) if (m >> i) & 1)
                for m in range(1 << n)
                if sum(items[i][0] for i in range(n) if (m >> i) & 1) <= cap
            )
            self.assertEqual(best, brute)
            self.assertGreaterEqual(knapsack_unbounded(items, cap), best)


if __name__ == "__main__":
    print(demo(), end="")
