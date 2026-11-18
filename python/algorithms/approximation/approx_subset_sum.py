"""FPTAS for Subset Sum — CLRS 37.4 (1e) / 35.5 (2e–4e) (1,2,3,4).

Trim the list of reachable sums by a factor (1+ε/2n): a fully polynomial-time approximation scheme.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _merge_lists(a: list[int], b: list[int]) -> list[int]:
    return sorted(set(a) | set(b))


def exact_subset_sum(s: list[int], t: int) -> int:
    """EXACT-SUBSET-SUM(S, t): L_i = L_{i-1} U (L_{i-1} + x_i), keeping sums <= t."""
    l = [0]
    for x in s:
        shifted = [y + x for y in l if y + x <= t]
        l = _merge_lists(l, shifted)
    return l[-1]


def trim(l: list[int], delta: float) -> list[int]:
    """TRIM(L, delta): drop y when a kept z <= y satisfies y / (1 + delta) <= z."""
    out = [l[0]]
    last = l[0]
    for y in l[1:]:
        if float(y) > float(last) * (1.0 + delta):
            out.append(y)
            last = y
    return out


def approx_subset_sum(s: list[int], t: int, eps: float) -> tuple[int, int]:
    """APPROX-SUBSET-SUM(S, t, eps): returns (z, max_list_len) with z <= t and z >= (1 - eps) * OPT."""
    n = max(len(s), 1)
    l = [0]
    max_len = 1
    for x in s:
        shifted = [y + x for y in l if y + x <= t]
        l = trim(_merge_lists(l, shifted), eps / (2.0 * n))
        max_len = max(max_len, len(l))
    return (l[-1], max_len)


def demo() -> str:
    s = [104, 102, 201, 101]
    lines = [
        f"S = {s}, t = 308",
        f"exact optimum        = {exact_subset_sum(s, 308)}",
        f"FPTAS with ε = 0.4   = {approx_subset_sum(s, 308, 0.4)}",
    ]
    r = Rng(1)
    big = [r.range(1_000, 1_000_000) for _ in range(22)]
    t = sum(big) // 2
    lines.append(f"22 numbers, t = {t}: exact {exact_subset_sum(big, t)}")
    for eps in [0.5, 0.1, 0.01]:
        z, length = approx_subset_sum(big, t, eps)
        lines.append(f"  ε = {eps:<4}: z = {z}  (≥ (1−ε)·OPT), max list length {length}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_example(self) -> None:
        s = [104, 102, 201, 101]
        self.assertEqual(exact_subset_sum(s, 308), 307)
        z, _ = approx_subset_sum(s, 308, 0.4)
        self.assertLessEqual(z, 308)
        self.assertGreaterEqual(z, 0.6 * 307.0)

    def test_approximation_guarantee_and_polynomial_list_size(self) -> None:
        r = Rng(253)
        for _ in range(100):
            n = r.range(1, 14)
            s = [r.range(1, 5000) for _ in range(n)]
            t = r.range(1, 40_000)
            opt = exact_subset_sum(s, t)
            for eps in [0.5, 0.2, 0.05]:
                z, length = approx_subset_sum(s, t, eps)
                self.assertLessEqual(z, t)
                self.assertGreaterEqual(float(z), (1.0 - eps) * float(opt) - 1e-9)
                self.assertLessEqual(float(length), math.log(t) * 2.0 * n / eps + 3.0)


if __name__ == "__main__":
    print(demo(), end="")
