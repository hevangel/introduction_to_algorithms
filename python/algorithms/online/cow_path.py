"""The Cow-Path Problem (Online Search) — CLRS Problem 27-1 (4e) (4e).

Doubling spiral search along a 1D line finds a target at unknown signed distance d in <= 9|d| total steps, optimal among deterministic algorithms.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def cow_path_search(target: int) -> tuple[int, int]:
    """Classic discrete doubling cow-path search (factor r = 2) starting at 0.

    Alternates +2^0, -2^1, +2^2, -2^3, ... until reaching integer coordinate target.
    Returns (total_distance_walked, turn_count). Always satisfies total <= 9 * |target|.
    """
    if target == 0:
        return 0, 0
    pos = 0
    dist = 0
    reach = 1
    direction = 1
    turns = 0

    while True:
        endpoint = direction * reach
        if (pos <= target <= endpoint) or (endpoint <= target <= pos):
            dist += abs(target - pos)
            return dist, turns
        dist += abs(endpoint - pos)
        pos = endpoint
        direction = -direction
        reach *= 2
        turns += 1


def cow_path_search_factor(target: float, r: float) -> tuple[float, int]:
    """Generalized geometric cow-path search with multiplier r > 1.0 on continuous coordinate target."""
    if r <= 1.0:
        raise ValueError("r must be > 1.0")
    if target == 0.0:
        return 0.0, 0
    pos = 0.0
    dist = 0.0
    reach = 1.0
    direction = 1.0
    turns = 0

    while True:
        endpoint = direction * reach
        if (pos <= target <= endpoint) or (endpoint <= target <= pos):
            dist += abs(target - pos)
            return dist, turns
        dist += abs(endpoint - pos)
        pos = endpoint
        direction = -direction
        reach *= r
        turns += 1


def theoretical_ratio(r: float) -> float:
    return 1.0 + 2.0 * r * r / (r - 1.0)


def demo() -> str:
    lines: list[str] = []
    for d in (1, -1, 2, -2, 3, -5, 17, -65, 1025):
        walked, turns = cow_path_search(d)
        ratio = walked / abs(d)
        lines.append(f"target = {d:>5}: walked = {walked:>5} in {turns:>2} turns (ratio = {ratio:.3f} ≤ 9)")
    for r in (1.5, 2.0, 3.0, 4.0):
        lines.append(f"factor r = {r:.1f}: worst-case bound = {theoretical_ratio(r):.3f}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_doubling_never_exceeds_nine_times_distance(self) -> None:
        self.assertEqual(cow_path_search(0), (0, 0))
        worst_ratio = 0.0
        for d in range(-10_000, 10_001):
            if d == 0:
                continue
            dist, _ = cow_path_search(d)
            self.assertLessEqual(dist, 9 * abs(d), f"d={d}, dist={dist} > {9 * abs(d)}")
            worst_ratio = max(worst_ratio, dist / abs(d))
        self.assertGreater(worst_ratio, 7.5)
        self.assertLessEqual(worst_ratio, 9.0)

    def test_general_factor_obeys_theoretical_ratio_bound(self) -> None:
        for r in (1.5, 2.0, 2.5, 3.0, 4.0):
            bound = theoretical_ratio(r)
            for k in range(10):
                d = (r**k) + 0.001
                for sign in (1.0, -1.0):
                    dist, _ = cow_path_search_factor(sign * d, r)
                    self.assertLessEqual(dist, bound * d + 1e-6, f"r={r}, d={d}: {dist} > {bound * d}")


if __name__ == "__main__":
    print(demo(), end="")
