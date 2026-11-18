"""Hash Functions & Universal Hashing — CLRS 11.3 (1e-4e) (Editions: 1,2,3,4).

Division, multiplication and universal hashing methods; string keys as radix-128 integers.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest
from typing import Callable

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

_MASK64 = (1 << 64) - 1


def division_method(k: int, m: int) -> int:
    """Division method: h(k) = k mod m (pick m prime, not close to a power of 2)."""
    return k % m


def multiplication_method(k: int, m: int) -> int:
    """Multiplication method with Knuth's constant A ~= (sqrt(5)-1)/2."""
    A = 0.618_033_988_749_894_9
    frac = (float(k) * A) % 1.0
    return math.floor(float(m) * frac)


def multiplication_method_word(k: int, p: int) -> int:
    """Word-level multiplication method for m = 2^p."""
    S = 0x9E37_79B9_7F4A_7C15
    return ((k * S) & _MASK64) >> (64 - p)


def string_to_key(s: str) -> int:
    """Interpret a string as an integer in radix 128 (Section 11.3.1), wrapping mod 2^64."""
    acc = 0
    for b in s.encode("utf-8"):
        acc = (acc * 128 + b) & _MASK64
    return acc


class UniversalHash:
    """A member of the universal class H_{pm}: h_{ab}(k) = ((a*k + b) mod p) mod m."""

    def __init__(self, a: int, b: int, p: int, m: int) -> None:
        self.a = a
        self.b = b
        self.p = p
        self.m = m

    @classmethod
    def random(cls, p: int, m: int, rng: Rng) -> UniversalHash:
        """Pick a in {1..p-1} and b in {0..p-1} at random; p prime larger than any key."""
        return cls(1 + (rng.next_u64() % (p - 1)), rng.next_u64() % p, p, m)

    def hash(self, k: int) -> int:
        return ((self.a * k + self.b) % self.p) % self.m


def colliding_pairs(keys: list[int], m: int, h: Callable[[int], int]) -> int:
    """Number of colliding pairs when hashing `keys` into m slots."""
    c = [0] * m
    for k in keys:
        c[h(k)] += 1
    return sum(x * max(x - 1, 0) // 2 for x in c)


def demo() -> str:
    lines: list[str] = []
    lines.append(f"division_method(123456, 701)       = {division_method(123_456, 701)}")
    lines.append(
        f"multiplication_method(123456, 16384) = {multiplication_method(123_456, 16384)}"
    )
    lines.append(
        f"multiplication_method_word(123456, 14) = {multiplication_method_word(123_456, 14)}"
    )
    lines.append(
        f'string_to_key("pt") = {string_to_key("pt")} (p=112, t=116 -> 112*128+116)'
    )
    r = Rng(17)
    keys = [i * 1024 for i in range(1000)]
    lines.append("keys are multiples of 1024; colliding pairs with m=1024:")
    lines.append(
        f"  division by 1024        : {colliding_pairs(keys, 1024, lambda k: division_method(k, 1024))}"
    )
    lines.append(
        f"  division by prime 1009  : {colliding_pairs(keys, 1009, lambda k: division_method(k, 1009))}"
    )
    u = UniversalHash.random(1_000_003, 1024, r)
    lines.append(
        f"  random universal hash   : {colliding_pairs(keys, 1024, u.hash)} "
        f"(expected ~= n^2/2m = {1000 * 999 // 2 // 1024})"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_ranges_are_respected(self) -> None:
        r = Rng(55)
        for _ in range(1000):
            k = r.next_u64() >> 10
            self.assertLess(division_method(k, 97), 97)
            self.assertLess(multiplication_method(k, 1000), 1000)
            self.assertLess(multiplication_method_word(k, 10), 1024)
        self.assertEqual(string_to_key("pt"), 112 * 128 + 116)

    def test_universality_collision_probability_is_at_most_about_one_over_m(self) -> None:
        p, m = 1_000_003, 100
        r = Rng(56)
        trials = 40_000
        hits = 0
        for _ in range(trials):
            h = UniversalHash.random(p, m, r)
            if h.hash(12) == h.hash(345):
                hits += 1
        self.assertLessEqual(hits / trials, 1.5 / m)

    def test_division_by_power_of_two_is_bad_for_aligned_keys(self) -> None:
        keys = [i * 64 for i in range(200)]
        bad = colliding_pairs(keys, 64, lambda k: division_method(k, 64))
        good = colliding_pairs(keys, 61, lambda k: division_method(k, 61))
        self.assertGreater(bad, 10 * good)


if __name__ == "__main__":
    print(demo(), end="")
