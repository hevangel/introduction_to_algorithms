"""Chip Testing — CLRS Problem 4-5 (2e–4e).

Find one good chip among n (more than half good) using pairwise tests in Θ(n): discard pairs, recurse on half.
"""

from __future__ import annotations

from enum import Enum
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class Bad(Enum):
    """How a bad chip answers when it is asked about another chip."""

    RANDOM = "Random"
    ALWAYS_GOOD = "AlwaysGood"
    ALWAYS_BAD = "AlwaysBad"


class Lab:
    """A collection of chips; `good[i]` is the secret truth. Tests are counted."""

    def __init__(self, good: list[bool], bad: Bad, seed: int) -> None:
        self._good = list(good)
        self._bad = bad
        self._rng = Rng(seed)
        self.tests = 0

    def __len__(self) -> int:
        return len(self._good)

    def is_empty(self) -> bool:
        return len(self._good) == 0

    def is_good(self, i: int) -> bool:
        return self._good[i]

    def ask(self, i: int, j: int) -> bool:
        """Chip `i` is asked whether chip `j` is good. Good chips tell the truth; bad ones answer per policy."""
        self.tests += 1
        if self._good[i]:
            return self._good[j]
        if self._bad == Bad.ALWAYS_GOOD:
            return True
        if self._bad == Bad.ALWAYS_BAD:
            return False
        return (self._rng.next_u64() & 1) == 1

    def pair_test(self, i: int, j: int) -> bool:
        """Pairwise test: both chips are asked about each other; True iff both say 'good'."""
        a = self.ask(i, j)
        b = self.ask(j, i)
        return a and b


def find_good_chip(lab: Lab) -> int:
    """Problem 4-5(c): with strictly more than n/2 good chips, find one good chip using Θ(n) pairwise tests."""
    cand = list(range(len(lab)))
    while True:
        if len(cand) <= 2:
            return cand[0]
        if len(cand) % 2 == 1:
            x = cand.pop()
            n = len(cand) + 1
            endorsements = sum(1 for c in cand if lab.ask(c, x))
            if endorsements >= (n - 1) // 2:
                return x
        cand = [cand[i] for i in range(0, len(cand), 2) if lab.pair_test(cand[i], cand[i + 1])]
        if not cand:
            raise AssertionError("strict majority invariant violated")


def identify_all(lab: Lab, good: int) -> list[bool]:
    """Problem 4-5(d): once one good chip is known, n−1 tests identify every chip."""
    return [j == good or lab.ask(good, j) for j in range(len(lab))]


def demo() -> str:
    lines: list[str] = []
    for n, goods in [(9, 5), (16, 9), (101, 51), (1000, 501)]:
        pattern = [i < goods for i in range(n)]
        Rng(n).shuffle(pattern)
        lab = Lab(pattern, Bad.RANDOM, 1)
        g = find_good_chip(lab)
        used = lab.tests
        all_chips = identify_all(lab, g)
        ok = all(b == lab.is_good(i) for i, b in enumerate(all_chips))
        lines.append(
            f"n = {n:>4}, good = {goods:>3}: found good chip #{g:<4} ({lab.is_good(g)}) "
            f"using {used} tests (≤ 3n = {3 * n}); full classification correct: {ok}"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_always_finds_a_good_chip_with_linear_tests(self) -> None:
        r = Rng(308)
        for trial in range(400):
            n = r.range(1, 60)
            g = n // 2 + 1 + r.range(0, n - n // 2 - 1)
            pattern = [i < g for i in range(n)]
            r.shuffle(pattern)
            for bad in (Bad.RANDOM, Bad.ALWAYS_GOOD, Bad.ALWAYS_BAD):
                lab = Lab(pattern, bad, trial)
                c = find_good_chip(lab)
                self.assertTrue(lab.is_good(c), f"n={n} good={g} {bad}")
                self.assertLessEqual(lab.tests, 3 * n, f"{lab.tests} tests for n={n}")
                all_chips = identify_all(lab, c)
                self.assertTrue(all(b == lab.is_good(i) for i, b in enumerate(all_chips)))

    def test_pair_tests_semantics(self) -> None:
        lab = Lab([True, True, False], Bad.ALWAYS_GOOD, 1)
        self.assertTrue(lab.pair_test(0, 1))
        self.assertFalse(lab.pair_test(0, 2))
        self.assertFalse(lab.pair_test(2, 0))
        self.assertEqual(lab.tests, 6)


if __name__ == "__main__":
    print(demo(), end="")
