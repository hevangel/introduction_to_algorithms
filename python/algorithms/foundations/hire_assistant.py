"""Hiring Problem & Indicator Variables — CLRS 5.1–5.2 (2e–4e).

HIRE-ASSISTANT and its expected number of hires (the harmonic number H_n ≈ ln n).
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def hire_assistant(ranks: list[int]) -> int:
    """HIRE-ASSISTANT: returns how many times a better candidate appeared (hires)."""
    best = 0
    hires = 0
    for r in ranks:
        if r > best:
            best = r
            hires += 1
    return hires


def harmonic(n: int) -> float:
    """H_n = 1 + 1/2 + … + 1/n."""
    return sum(1.0 / i for i in range(1, n + 1))


def average_hires(n: int, trials: int, rng: Rng) -> float:
    """Average number of hires over `trials` uniformly random permutations of 1..=n."""
    total = 0
    for _ in range(trials):
        v = list(range(1, n + 1))
        rng.shuffle(v)
        total += hire_assistant(v)
    return total / trials


def demo() -> str:
    lines: list[str] = []
    lines.append(f"best-to-worst order hires once   : {hire_assistant([5, 4, 3, 2, 1])}")
    lines.append(f"worst-to-best order hires n times: {hire_assistant([1, 2, 3, 4, 5])}")
    r = Rng(2024)
    for n in [10, 100, 1000]:
        lines.append(
            f"n={n:>4}: simulated avg hires = {average_hires(n, 2000, r):.3f}   "
            f"H_n = {harmonic(n):.3f}   ln n = {math.log(n):.3f}"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_extremes(self) -> None:
        self.assertEqual(hire_assistant([1, 2, 3, 4, 5, 6]), 6)
        self.assertEqual(hire_assistant([6, 5, 4, 3, 2, 1]), 1)
        self.assertEqual(hire_assistant([]), 0)

    def test_average_matches_harmonic_number(self) -> None:
        r = Rng(11)
        avg = average_hires(50, 20000, r)
        self.assertLess(abs(avg - harmonic(50)), 0.1, f"{avg} vs {harmonic(50)}")


if __name__ == "__main__":
    print(demo(), end="")
