"""Lower Bounds for Merging — CLRS Problem 8-6 (1e–4e).

Merging two sorted lists of n elements needs 2n − 1 comparisons in the worst case; the information-theoretic bound lg C(2n,n) is only 2n − O(lg n), but an adversary argument shows every adjacent pair must be compared.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng, is_sorted


def merge_counting(a: list[int], b: list[int]) -> tuple[list[int], list[tuple[int, int]]]:
    """Standard linear merge. Returns the merged list and the list of compared value pairs."""
    i, j = 0, 0
    out: list[int] = []
    cmps: list[tuple[int, int]] = []
    while i < len(a) and j < len(b):
        cmps.append((a[i], b[j]))
        if a[i] <= b[j]:
            out.append(a[i])
            i += 1
        else:
            out.append(b[j])
            j += 1
    out.extend(a[i:])
    out.extend(b[j:])
    return out, cmps


def information_bound(n: int) -> float:
    """lg C(2n, n): minimum comparisons to distinguish C(2n, n) possible interleavings."""
    return sum(math.log2(n + i) - math.log2(i) for i in range(1, n + 1))


def adversary(n: int) -> tuple[list[int], list[int]]:
    """The adversary's input: a₁ < b₁ < a₂ < b₂ < …, forcing every adjacent pair to be compared."""
    return [2 * i for i in range(n)], [2 * i + 1 for i in range(n)]


def demo() -> str:
    lines: list[str] = []
    lines.append("   n | lg C(2n,n) | 2n−1 (merge on the adversary input)")
    for n in [1, 2, 4, 8, 16, 64, 256]:
        a, b = adversary(n)
        lines.append(f"{n:>4} | {information_bound(n):>10.2f} | {len(merge_counting(a, b)[1])}")
    lines.append(
        "the information bound falls short of 2n−1 by about ½ lg n, but no algorithm can beat 2n−1: "
        "each adjacent pair of the interleaving must be compared"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_adversary_forces_every_adjacent_comparison(self) -> None:
        for n in range(1, 21):
            a, b = adversary(n)
            out, cmps = merge_counting(a, b)
            self.assertEqual(out, list(range(2 * n)))
            self.assertEqual(len(cmps), 2 * n - 1)
            cmp_set = set(cmps)
            for v in range(2 * n - 1):
                self.assertTrue((v, v + 1) in cmp_set or (v + 1, v) in cmp_set)

    def test_never_more_than_2n_minus_1_and_bound_is_close(self) -> None:
        r = Rng(443)
        for _ in range(200):
            la, lb = r.range(1, 30), r.range(1, 30)
            a = sorted(r.range(0, 100) for _ in range(la))
            b = sorted(r.range(0, 100) for _ in range(lb))
            out, cmps = merge_counting(a, b)
            self.assertTrue(is_sorted(out) and len(cmps) < la + lb)
        for n in range(1, 201):
            lb = information_bound(n)
            self.assertTrue(
                lb <= (2 * n - 1) + 1e-9 and (2 * n) - lb <= 0.5 * math.log2(n) + 2.0,
                f"n={n} bound {lb}",
            )


if __name__ == "__main__":
    print(demo(), end="")
