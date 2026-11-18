"""Random Permutations & Sampling — CLRS 5.3 (2e–4e).

PERMUTE-BY-SORTING, RANDOMIZE-IN-PLACE, random subsets, and RANDOM(a,b) from fair coin flips.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def permute_by_sorting(a: list, rng: Rng) -> list:
    """PERMUTE-BY-SORTING: give each element a random priority in [1, n³], then sort by priority."""
    n = max(len(a), 1)
    n3 = n * n * n
    keyed = [(rng.next_u64() % n3 + 1, i) for i in range(len(a))]
    keyed.sort()
    return [a[i] for _, i in keyed]


def randomize_in_place(a: list, rng: Rng) -> None:
    """RANDOMIZE-IN-PLACE: Fisher–Yates/Knuth shuffle, Θ(n), uniform."""
    for i in range(len(a)):
        j = rng.range(i, len(a) - 1)
        a[i], a[j] = a[j], a[i]


def random_sample(m: int, n: int, rng: Rng) -> list[int]:
    """Exercise 5.3-7: RANDOM-SAMPLE(m, n) — a uniformly random m-subset of {1..n}."""
    if m == 0:
        return []
    s = random_sample(m - 1, n - 1, rng)
    i = rng.range(1, n)
    if i in s:
        s.append(n)
    else:
        s.append(i)
    return s


def random_ab(a: int, b: int, rng: Rng, coin_flips: list[int] | None = None) -> int:
    """Exercise 5.1-2: RANDOM(a, b) using only fair coin flips (rejection sampling)."""
    span = b - a + 1
    bits = (span - 1).bit_length()
    while True:
        x = 0
        for _ in range(bits):
            if coin_flips is not None:
                coin_flips[0] += 1
            x = (x << 1) | (rng.next_u64() & 1)
        if x < span:
            return a + x


def demo() -> str:
    lines: list[str] = []
    r = Rng(7)
    base = list(range(1, 9))
    lines.append(f"permute_by_sorting : {permute_by_sorting(base, r)}")
    b = list(base)
    randomize_in_place(b, r)
    lines.append(f"randomize_in_place : {b}")
    s = sorted(random_sample(4, 20, r))
    lines.append(f"random_sample(4,20): {s}")
    flips = [0]
    rolls = [random_ab(1, 6, r, flips) for _ in range(10)]
    lines.append(f"RANDOM(1,6) ×10    : {rolls} using {flips[0]} coin flips")
    counts: dict[tuple[int, ...], int] = {}
    for _ in range(6000):
        v = [1, 2, 3]
        randomize_in_place(v, r)
        k = tuple(v)
        counts[k] = counts.get(k, 0) + 1
    freqs = [counts[k] for k in sorted(counts)]
    lines.append(f"frequencies of the 6 permutations of [1,2,3] over 6000 shuffles: {freqs}")
    return "\n".join(lines) + "\n"


def _is_perm(v: list[int], n: int) -> bool:
    return sorted(v) == list(range(1, n + 1))


class Tests(unittest.TestCase):
    def test_outputs_are_permutations(self) -> None:
        r = Rng(12)
        base = list(range(1, 21))
        self.assertTrue(_is_perm(permute_by_sorting(base, r), 20))
        b = list(base)
        randomize_in_place(b, r)
        self.assertTrue(_is_perm(b, 20))

    def test_shuffle_is_uniform_on_three_elements(self) -> None:
        r = Rng(13)
        counts: dict[tuple[int, ...], int] = {}
        for _ in range(12000):
            v = [0, 1, 2]
            randomize_in_place(v, r)
            k = tuple(v)
            counts[k] = counts.get(k, 0) + 1
        self.assertEqual(len(counts), 6)
        for c in counts.values():
            self.assertTrue(1700 <= c < 2300, f"count {c}")

    def test_sample_has_distinct_elements_in_range(self) -> None:
        r = Rng(14)
        for _ in range(200):
            s = sorted(set(random_sample(5, 12, r)))
            self.assertEqual(len(s), 5)
            self.assertTrue(all(1 <= x <= 12 for x in s))

    def test_random_ab_stays_in_range_and_covers_it(self) -> None:
        r = Rng(15)
        seen = [False] * 6
        f = [0]
        for _ in range(500):
            x = random_ab(10, 15, r, f)
            self.assertTrue(10 <= x <= 15)
            seen[x - 10] = True
        self.assertTrue(all(seen))


if __name__ == "__main__":
    print(demo(), end="")
