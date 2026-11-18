"""Birthday Paradox, Balls & Bins, Streaks — CLRS 5.4 (1e–4e).

The probabilistic-analysis examples: birthday collisions, coupon collector, longest streak of heads.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def birthday_collision_probability(k: int, n: int) -> float:
    """Exact probability that among k people in a year of n days at least two share a birthday."""
    no_collision = 1.0
    for i in range(k):
        no_collision *= (n - min(i, n)) / n
    return 1.0 - no_collision


def smallest_group_for_even_odds(n: int) -> int:
    """Smallest k for which the collision probability is at least 1/2."""
    for k in range(1, n + 2):
        if birthday_collision_probability(k, n) >= 0.5:
            return k
    raise AssertionError("unreachable")


def expected_colliding_pairs(k: int, n: int) -> float:
    """Expected number of colliding pairs, C(k,2)/n (Section 5.4.1, indicator variables)."""
    return (k * (k - 1)) / (2.0 * n)


def balls_until_all_bins_nonempty(n: int, rng: Rng) -> int:
    """Balls and bins: throw balls into n bins until every bin is non-empty. Returns tosses."""
    seen = [False] * n
    left = n
    tosses = 0
    while left > 0:
        b = rng.range(0, n - 1)
        tosses += 1
        if not seen[b]:
            seen[b] = True
            left -= 1
    return tosses


def longest_streak(n: int, rng: Rng) -> int:
    """Length of the longest run of heads in n fair coin flips (Section 5.4.3)."""
    best = 0
    cur = 0
    for _ in range(n):
        if rng.next_u64() & 1 == 1:
            cur += 1
            if cur > best:
                best = cur
        else:
            cur = 0
    return best


def demo() -> str:
    lines: list[str] = []
    lines.append(
        f"smallest group with ≥ 50% shared-birthday chance (365 days): {smallest_group_for_even_odds(365)}"
    )
    for k in [10, 23, 40, 57]:
        lines.append(f"  k={k:>2}: P(collision) = {birthday_collision_probability(k, 365):.4f}")
    r = Rng(1)
    n = 100
    avg = sum(balls_until_all_bins_nonempty(n, r) for _ in range(2000)) / 2000.0
    h = sum(1.0 / i for i in range(1, n + 1))
    lines.append(f"coupon collector n={n}: simulated {avg:.1f} tosses, theory n·H_n = {n * h:.1f}")
    s = sum(longest_streak(1000, r) for _ in range(500)) / 500.0
    lines.append(f"longest head streak in 1000 flips: simulated {s:.2f}, lg 1000 = {math.log2(1000):.2f}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_birthday_threshold_is_23(self) -> None:
        self.assertEqual(smallest_group_for_even_odds(365), 23)
        self.assertGreater(birthday_collision_probability(23, 365), 0.507)
        self.assertAlmostEqual(birthday_collision_probability(366, 365), 1.0, delta=1e-12)
        self.assertEqual(birthday_collision_probability(1, 365), 0.0)

    def test_coupon_collector_expectation(self) -> None:
        r = Rng(31)
        n = 50
        avg = sum(balls_until_all_bins_nonempty(n, r) for _ in range(4000)) / 4000.0
        h = sum(1.0 / i for i in range(1, n + 1))
        self.assertLess(abs(avg - n * h), 6.0, f"{avg}")

    def test_streak_is_logarithmic(self) -> None:
        r = Rng(32)
        avg = sum(longest_streak(1024, r) for _ in range(300)) / 300.0
        self.assertTrue(7.0 <= avg < 11.5, f"{avg}")


if __name__ == "__main__":
    print(demo(), end="")
