"""Hashing Bounds: Longest Chain & Probe Tails — CLRS Problems 11-1, 11-2 (1e-4e) (Editions: 1,2,3,4).

Experiments behind the probabilistic bounds: the longest probe sequence is O(lg n)
w.h.p. and the longest chain is Theta(lg n / lg lg n).
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def max_chain_length(n: int, m: int, rng: Rng) -> int:
    """Problem 11-2: throw n keys into m slots uniformly; return longest chain length."""
    c = [0] * m
    for _ in range(n):
        c[rng.range(0, m - 1)] += 1
    return max(c)


def chain_scale(n: int) -> float:
    """The expected-maximum scale lg n / lg lg n."""
    lg = math.log2(n)
    return lg / math.log2(lg)


def probe_tail(m: int, alpha: float, kmax: int, trials: int, rng: Rng) -> list[float]:
    """Problem 11-1: empirical P(probes > k) for k = 0..kmax under uniform open addressing."""
    exceed = [0] * (kmax + 1)
    filled = int(alpha * m)
    for _ in range(trials):
        occupied = [False] * m
        fill = 0
        while fill < filled:
            s = rng.range(0, m - 1)
            if not occupied[s]:
                occupied[s] = True
                fill += 1
        probes = 0
        while True:
            probes += 1
            if not occupied[rng.range(0, m - 1)]:
                break
        for k in range(kmax + 1):
            if probes > k:
                exceed[k] += 1
    return [e / trials for e in exceed]


def demo() -> str:
    lines: list[str] = []
    r = Rng(7)
    lines.append("chaining with n = m keys: average longest chain vs lg n / lg lg n")
    for k in [8, 10, 12, 14, 16]:
        n = 1 << k
        avg = sum(max_chain_length(n, n, r) for _ in range(40)) / 40.0
        sc = chain_scale(n)
        lines.append(
            f"  n = 2^{k:<2}: longest chain {avg:>5.2f}   lg n/lg lg n = {sc:.2f}   ratio {avg / sc:.2f}"
        )
    tail = probe_tail(512, 0.5, 6, 20_000, r)
    rounded_tail = [round(x * 1000.0) / 1000.0 for x in tail]
    expected_tail = [0.5**k for k in range(7)]
    lines.append(f"open addressing at alpha = 1/2: P(probes > k) = {rounded_tail}  vs alpha^k = {expected_tail}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_longest_chain_grows_like_lg_n_over_lg_lg_n(self) -> None:
        r = Rng(331)

        def avg(n: int) -> float:
            return sum(max_chain_length(n, n, r) for _ in range(60)) / 60.0

        for n in [1 << 8, 1 << 12, 1 << 15]:
            a = avg(n)
            s = chain_scale(n)
            self.assertTrue(s <= a <= 3.5 * s, f"n={n}: avg {a}, scale {s}")
        self.assertGreater(avg(1 << 15), avg(1 << 8))

    def test_probe_tail_decays_geometrically(self) -> None:
        r = Rng(332)
        t = probe_tail(256, 0.5, 4, 20_000, r)
        for k, p in enumerate(t):
            self.assertAlmostEqual(p, 0.5**k, delta=0.02)
        t9 = probe_tail(256, 0.9, 3, 8_000, r)
        self.assertAlmostEqual(t9[3], 0.9**3, delta=0.03)


if __name__ == "__main__":
    print(demo(), end="")
