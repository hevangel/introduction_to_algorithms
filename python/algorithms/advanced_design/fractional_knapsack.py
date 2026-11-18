"""Fractional Knapsack — CLRS 16.2 (1e–3e) / 15.2 (4e) (1e,2e,3e,4e).

Greedy by value-per-pound is optimal when items may be split.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def fractional_knapsack(items: list[tuple[float, float]], cap: float) -> tuple[float, list[float]]:
    """Items are (weight, value); returns (best total value, fraction of each item taken)."""
    idx = sorted(range(len(items)), key=lambda i: items[i][1] / items[i][0], reverse=True)
    left = cap
    total = 0.0
    take = [0.0] * len(items)
    for i in idx:
        if left <= 0.0:
            break
        f = min(left / items[i][0], 1.0)
        take[i] = f
        total += f * items[i][1]
        left -= f * items[i][0]
    return total, take


def _knapsack_01(items: list[tuple[int, int]], cap: int) -> tuple[int, list[int]]:
    """Self-contained 0-1 knapsack DP for comparison with fractional knapsack."""
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


def demo() -> str:
    lines: list[str] = []
    items = [(10.0, 60.0), (20.0, 100.0), (30.0, 120.0)]
    v, take = fractional_knapsack(items, 50.0)
    lines.append(f"items (weight, value) = {items}, capacity 50")
    lines.append(f"fractional optimum = {v:.2f}, fractions taken = {take}")
    zero_one, _ = _knapsack_01([(10, 60), (20, 100), (30, 120)], 50)
    lines.append(f"0-1 optimum        = {zero_one} (never larger than the fractional one)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_thief_example(self) -> None:
        v, take = fractional_knapsack([(10.0, 60.0), (20.0, 100.0), (30.0, 120.0)], 50.0)
        self.assertLess(abs(v - 240.0), 1e-9)
        self.assertEqual(take, [1.0, 1.0, 2.0 / 3.0])

    def test_upper_bounds_the_zero_one_optimum_and_fits(self) -> None:
        r = Rng(113)
        for _ in range(60):
            n = r.range(1, 8)
            ints = [(r.range(1, 9), r.range(1, 30)) for _ in range(n)]
            cap = r.range(1, 30)
            items = [(float(w), float(v)) for w, v in ints]
            f, take = fractional_knapsack(items, float(cap))
            self.assertLessEqual(sum(t * item[0] for t, item in zip(take, items)), cap + 1e-9)
            self.assertGreaterEqual(f + 1e-9, float(_knapsack_01(ints, cap)[0]))


if __name__ == "__main__":
    print(demo(), end="")
