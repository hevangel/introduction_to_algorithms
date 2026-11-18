"""Median of Two Sorted Arrays — CLRS Exercise 9.3-8 (1e–4e).

Find the median of 2n elements held in two sorted arrays in O(lg n) by discarding a quarter of the data at each step.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def kth_of_two(a: list[int], b: list[int], k: int) -> int:
    """The k-th smallest (1-based) element of the union of two sorted slices."""
    a_lo, a_hi = 0, len(a)
    b_lo, b_hi = 0, len(b)
    while True:
        len_a = a_hi - a_lo
        len_b = b_hi - b_lo
        if len_a > len_b:
            a, b = b, a
            a_lo, a_hi, b_lo, b_hi = b_lo, b_hi, a_lo, a_hi
            len_a, len_b = len_b, len_a
        if len_a == 0:
            return b[b_lo + k - 1]
        if k == 1:
            return min(a[a_lo], b[b_lo])
        i = min(k // 2, len_a)
        j = k - i
        if a[a_lo + i - 1] <= b[b_lo + j - 1]:
            a_lo += i
            k -= i
        else:
            b_lo += j
            k -= j


def median_of_two(a: list[int], b: list[int]) -> int:
    """Lower median (the n-th smallest) of two sorted arrays of equal length n."""
    assert len(a) == len(b)
    return kth_of_two(a, b, len(a))


def demo() -> str:
    lines: list[str] = []
    a, b = [1, 4, 7, 10, 13], [2, 3, 8, 9, 20]
    lines.append(f"A = {a}, B = {b}")
    lines.append(f"median of the 10 values (5th smallest) = {median_of_two(a, b)}")
    big_a = [2 * x for x in range(1_000_000)]
    big_b = [2 * x + 1 for x in range(1_000_000)]
    lines.append(
        f"two sorted arrays of 10⁶ elements: median = {median_of_two(big_a, big_b)} "
        "found in about lg n = 20 halving steps"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_merging_for_every_rank(self) -> None:
        r = Rng(315)
        for _ in range(300):
            na, nb = r.range(0, 20), r.range(1, 20)
            a = sorted(r.vec(na, -30, 30))
            b = sorted(r.vec(nb, -30, 30))
            m = sorted(a + b)
            for k in range(1, len(m) + 1):
                self.assertEqual(kth_of_two(a, b, k), m[k - 1], f"{a} {b} k={k}")

    def test_equal_length_median(self) -> None:
        r = Rng(316)
        for n in range(1, 60):
            a = sorted(r.vec(n, 0, 100))
            b = sorted(r.vec(n, 0, 100))
            m = sorted(a + b)
            self.assertEqual(median_of_two(a, b), m[n - 1])


if __name__ == "__main__":
    print(demo(), end="")
