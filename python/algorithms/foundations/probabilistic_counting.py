"""Probabilistic Counting — CLRS Problem 5-1 (2e–4e).

Count to n in only lg n bits: increment the stored value i with probability 1/(nᵢ₊₁ − nᵢ); unbiased, Morris-counter variance.
"""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class Counter:
    """A probabilistic counter storing a small index i that represents the count n_i."""

    def __init__(self, values: Callable[[int], int]) -> None:
        self._i = 0
        self._values = values

    @classmethod
    def linear(cls) -> Counter:
        """n_i = 100·i (Problem 5-1(b)): variance of the estimate after n increments is 99n."""
        return cls(lambda i: 100 * i)

    @classmethod
    def morris(cls) -> Counter:
        """n_i = 2^i − 1 (Morris's counter): count up to 2^b with about lg lg n bits; variance ≈ n²/2."""
        return cls(lambda i: (1 << i) - 1)

    def stored(self) -> int:
        return self._i

    def increment(self, rng: Rng) -> None:
        """INCREMENT: advance i with probability 1/(n_{i+1} − n_i)."""
        gap = self._values(self._i + 1) - self._values(self._i)
        if rng.next_u64() % gap == 0:
            self._i += 1

    def estimate(self) -> int:
        """The represented count n_i, an unbiased estimator of the number of increments."""
        return self._values(self._i)


def simulate(mk: Callable[[], Counter], n: int, trials: int, rng: Rng) -> tuple[float, float]:
    """Mean and variance of the estimate after `n` increments over `trials` runs."""
    est: list[float] = []
    for _ in range(trials):
        c = mk()
        for _ in range(n):
            c.increment(rng)
        est.append(float(c.estimate()))
    mean = sum(est) / trials
    var = sum((x - mean) ** 2 for x in est) / trials
    return mean, var


def demo() -> str:
    lines: list[str] = []
    r = Rng(3)
    n = 1000
    m, v = simulate(Counter.linear, n, 4000, r)
    lines.append(f"n_i = 100i  : mean estimate {m:.1f} (true {n}), variance {v:.0f} (theory 99n = {99 * n})")
    m, v = simulate(Counter.morris, n, 4000, r)
    lines.append(
        f"n_i = 2^i−1 : mean estimate {m:.1f} (true {n}), variance {v:.0f} (theory ≈ n²/2 = {n * n // 2})"
    )
    c = Counter.morris()
    for _ in range(1_000_000):
        c.increment(r)
    bits = c.stored().bit_length()
    lines.append(
        f"Morris counter after 10⁶ increments stores i = {c.stored()} (a {bits}-bit register) and estimates {c.estimate()}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_estimators_are_unbiased(self) -> None:
        r = Rng(302)
        m, v = simulate(Counter.linear, 1000, 4000, r)
        self.assertLess(abs(m - 1000.0), 30.0, f"{m}")
        self.assertLess(abs(v / 99_000.0 - 1.0), 0.2, f"{v}")
        m, _ = simulate(Counter.morris, 1000, 6000, r)
        self.assertLess(abs(m - 1000.0), 70.0, f"{m}")

    def test_stored_index_is_tiny(self) -> None:
        r = Rng(303)
        c = Counter.morris()
        for _ in range(100_000):
            c.increment(r)
        self.assertLess(c.stored(), 25)
        self.assertEqual(Counter.morris().estimate(), 0)


if __name__ == "__main__":
    print(demo(), end="")
