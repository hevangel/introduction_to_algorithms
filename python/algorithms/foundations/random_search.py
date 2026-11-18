"""Searching an Unsorted Array (Randomized) — CLRS Problem 5-2 (2e–4e).

Random probing with replacement vs. a random permutation vs. a deterministic scan: expected probes n, (n+1)/2, n/(k+1).
"""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def random_probe(a: list[int], x: int, rng: Rng) -> tuple[bool, int]:
    """RANDOM-SEARCH: probe uniformly random indices (with replacement) until `x` is found or all probed."""
    n = len(a)
    seen = [False] * n
    left = n
    probes = 0
    while left > 0:
        i = rng.range(0, n - 1)
        probes += 1
        if a[i] == x:
            return True, probes
        if not seen[i]:
            seen[i] = True
            left -= 1
    return False, probes


def permutation_probe(a: list[int], x: int, rng: Rng) -> tuple[bool, int]:
    """Probe indices in a random permutation order (without replacement)."""
    order = list(range(len(a)))
    rng.shuffle(order)
    for k, i in enumerate(order):
        if a[i] == x:
            return True, k + 1
    return False, len(a)


def deterministic_scan(a: list[int], x: int) -> tuple[bool, int]:
    """DETERMINISTIC-SEARCH: left-to-right scan."""
    for i, v in enumerate(a):
        if v == x:
            return True, i + 1
    return False, len(a)


def _mean_probes(f: Callable[[Rng], int], trials: int, rng: Rng) -> float:
    return sum(f(rng) for _ in range(trials)) / trials


def demo() -> str:
    lines: list[str] = []
    r = Rng(9)
    n = 100
    for k in [1, 4, 10]:
        a = [7 if i < k else 0 for i in range(n)]
        trials = 4000
        m1 = _mean_probes(lambda rng, a=a: random_probe(a, 7, rng)[1], trials, r)
        m2 = _mean_probes(lambda rng, a=a: permutation_probe(a, 7, rng)[1], trials, r)
        lines.append(
            f"k = {k:>2} matches: with replacement {m1:>6.1f} probes (theory n/k = {n / k:.1f}); "
            f"permutation {m2:>5.1f} (theory (n+1)/(k+1) = {(n + 1) / (k + 1):.1f})"
        )
    none = [0] * n
    m_none = _mean_probes(lambda rng: random_probe(none, 7, rng)[1], 2000, r)
    hn = sum(1.0 / i for i in range(1, n + 1))
    lines.append(
        f"no match: with replacement {m_none:.0f} probes (≈ n·H_n = {n * hn:.0f}); "
        f"permutation {permutation_probe(none, 7, r)[1]}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_expected_probe_counts(self) -> None:
        r = Rng(304)
        n = 50
        a = [0] * n
        a[17] = 7
        with_repl = _mean_probes(lambda rng: random_probe(a, 7, rng)[1], 6000, r)
        without = _mean_probes(lambda rng: permutation_probe(a, 7, rng)[1], 6000, r)
        self.assertLess(abs(with_repl - 50.0), 3.0, f"{with_repl}")
        self.assertLess(abs(without - 25.5), 1.0, f"{without}")
        self.assertEqual(deterministic_scan(a, 7), (True, 18))

    def test_absent_element_costs(self) -> None:
        r = Rng(305)
        a = [0] * 30
        self.assertEqual(permutation_probe(a, 1, r), (False, 30))
        h30 = sum(1.0 / i for i in range(1, 31))
        m = _mean_probes(lambda rng: random_probe(a, 1, rng)[1], 4000, r)
        self.assertLess(abs(m - 30.0 * h30), 5.0, f"{m}")


if __name__ == "__main__":
    print(demo(), end="")
