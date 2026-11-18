"""Online Hiring (Secretary Problem) — CLRS 6.6 (1e) / 5.4.4 (2e–4e).

Observe the first k=n/e candidates, then hire the first one who beats them: success ≈ 1/e.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def online_maximum(scores: list[int], k: int) -> int:
    """ON-LINE-MAXIMUM(k, n): reject the first k, then accept the first score above all of those."""
    n = len(scores)
    best_score = 0
    for i in range(min(k, n)):
        if scores[i] > best_score:
            best_score = scores[i]
    for i in range(k, n):
        if scores[i] > best_score:
            return i
    return n - 1


def success_rate(n: int, k: int, trials: int, rng: Rng) -> float:
    """Estimated probability of hiring the overall best candidate."""
    ok = 0
    for _ in range(trials):
        v = list(range(1, n + 1))
        rng.shuffle(v)
        if v[online_maximum(v, k)] == n:
            ok += 1
    return ok / trials


def demo() -> str:
    lines: list[str] = []
    n = 100
    r = Rng(99)
    lines.append(
        f"n = {n}; theoretical optimum k = n/e ≈ {n / math.e:.1f}, success ≈ 1/e ≈ {1.0 / math.e:.3f}"
    )
    for k in [0, 5, 20, 37, 60, 90]:
        lines.append(f"k = {k:>2}: success rate = {success_rate(n, k, 20000, r):.3f}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_deterministic_cases(self) -> None:
        self.assertEqual(online_maximum([3, 1, 4, 1, 5], 2), 2)
        self.assertEqual(online_maximum([9, 1, 2, 3], 1), 3)
        self.assertEqual(online_maximum([1, 2, 3], 0), 0)

    def test_near_one_over_e_at_optimal_k(self) -> None:
        r = Rng(21)
        p = success_rate(100, 37, 30000, r)
        self.assertTrue(0.33 <= p < 0.42, f"{p}")
        bad = success_rate(100, 0, 30000, r)
        self.assertLess(bad, 0.03)


if __name__ == "__main__":
    print(demo(), end="")
