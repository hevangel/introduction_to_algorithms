"""Perfect-Power Test — CLRS Exercise 31.1-8 (2e–4e) (2, 3, 4).

Decide whether n = bᵏ for integers b > 1, k > 1 by trying every exponent k ≤ lg n with an integer k-th root by binary search.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def _cmp_pow(b: int, k: int, n: int) -> int:
    acc = 1
    for _ in range(k):
        acc *= b
        if acc > n:
            return 1
    return -1 if acc < n else 0


def kth_root(n: int, k: int) -> int:
    """floor(n^(1/k)) by binary search on the answer."""
    lo, hi = 1, max(n, 1)
    while lo < hi:
        mid = lo + (hi - lo + 1) // 2
        if _cmp_pow(mid, k, n) > 0:
            hi = mid - 1
        else:
            lo = mid
    return lo


def perfect_power(n: int) -> tuple[int, int] | None:
    """If n = b^k with b >= 2 and k >= 2, return (b, k) with the largest such k; else None."""
    if n < 4:
        return None
    max_k = n.bit_length() - 1
    for k in range(max_k, 1, -1):
        b = kth_root(n, k)
        if b >= 2 and _cmp_pow(b, k, n) == 0:
            return (b, k)
    return None


def demo() -> str:
    lines: list[str] = []
    for n in (1, 2, 4, 36, 64, 100, 243, 1000, 1_000_003, 3_486_784_401, 1 << 62):
        res = perfect_power(n)
        if res is not None:
            b, k = res
            lines.append(f"{n:>20} = {b}^{k}")
        else:
            lines.append(f"{n:>20} is not a perfect power")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_exhaustive_search(self) -> None:
        for n in range(1, 6000):
            brute = False
            b = 2
            while b * b <= n:
                x = b * b
                while x < n:
                    x *= b
                if x == n:
                    brute = True
                    break
                b += 1
            res = perfect_power(n)
            self.assertEqual(res is not None, brute, f"n = {n}")
            if res is not None:
                base, exp = res
                self.assertEqual(base**exp, n)

    def test_roots_and_extremes(self) -> None:
        self.assertEqual(kth_root(1000, 3), 10)
        self.assertEqual(kth_root(999, 3), 9)
        self.assertEqual(perfect_power(3_486_784_401), (3, 20))
        self.assertEqual(perfect_power(1 << 62), (2, 62))
        self.assertIsNone(perfect_power((1 << 64) - 1))


if __name__ == "__main__":
    print(demo(), end="")
