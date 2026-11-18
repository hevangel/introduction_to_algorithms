"""Waiting for an Elevator (Competitive Analysis) — CLRS 27.1 (4e) (4e).

Wait up to the cost of the stairs, then walk: a deterministic 2-competitive online strategy (ski rental).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def online_cost(a: float, threshold: float, k: float) -> float:
    """Cost of 'wait up to threshold, then take stairs (cost k extra)' when elevator arrives at time a."""
    return a if a <= threshold else threshold + k


def offline_cost(a: float, k: float) -> float:
    """Cost of the optimal offline strategy that knows a in advance: min(a, k)."""
    return min(a, k)


def worst_ratio(threshold: float, k: float) -> float:
    """Worst-case competitive ratio of the threshold strategy over arrival times in a fine grid."""
    worst = 1.0
    a = 0.0
    while a <= 20.0 * k:
        opt = offline_cost(a, k)
        if opt > 0.0:
            worst = max(worst, online_cost(a, threshold, k) / opt)
        a += k / 200.0
    return max(worst, (threshold + k) / max(min(threshold, k), 1e-12))


def demo() -> str:
    lines: list[str] = []
    k = 1.0
    lines.append(f"stairs cost k = {k}; ratio of 'wait θ, then walk' vs the offline optimum:")
    for theta in (0.1, 0.25, 0.5, 1.0, 2.0, 4.0, 10.0):
        lines.append(f"  θ = {theta:>4}: worst-case ratio {worst_ratio(theta, k):.3f}")
    lines.append(
        "best threshold θ = k gives ratio 2: the strategy is 2-competitive, and no deterministic strategy does better"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_costs(self) -> None:
        self.assertEqual(online_cost(0.5, 1.0, 1.0), 0.5)
        self.assertEqual(online_cost(3.0, 1.0, 1.0), 2.0)
        self.assertEqual(offline_cost(3.0, 1.0), 1.0)
        self.assertEqual(offline_cost(0.4, 1.0), 0.4)

    def test_threshold_k_is_two_competitive_and_optimal(self) -> None:
        k = 2.5
        r = worst_ratio(k, k)
        self.assertLess(abs(r - 2.0), 0.02, str(r))
        for theta in (0.2, 0.5, 1.0, 2.0, 5.0, 12.0):
            if abs(theta - k) > 0.2:
                self.assertGreater(worst_ratio(theta, k), r + 0.1, f"θ={theta}")

    def test_never_exceeds_two_for_any_arrival_time(self) -> None:
        k = 3.0
        for i in range(2000):
            a = i * 0.01
            opt = offline_cost(a, k)
            if opt > 0.0:
                self.assertLessEqual(online_cost(a, k, k), 2.0 * opt + 1e-9)


if __name__ == "__main__":
    print(demo(), end="")
