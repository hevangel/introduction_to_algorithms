"""Subset Sum — CLRS 36.5.5 (1e) / 34.5.5 (2e–4e) (1,2,3,4).

NP-complete in the input length yet solvable in O(nt) pseudo-polynomial time; meet-in-the-middle O(2^{n/2}).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def exhaustive(nums: list[int], target: int) -> list[int] | None:
    """Exhaustive search over subsets (Θ(2^n))."""
    n = len(nums)
    for m in range(1 << n):
        if sum(nums[i] for i in range(n) if (m >> i) & 1) == target:
            return [i for i in range(n) if (m >> i) & 1]
    return None


def dp(nums: list[int], target: int) -> list[int] | None:
    """Pseudo-polynomial dynamic program for non-negative numbers: O(n·t)."""
    if target < 0:
        return None
    t = target
    reach = [[False] * (t + 1) for _ in range(len(nums) + 1)]
    reach[0][0] = True
    for i, x in enumerate(nums):
        for s in range(t + 1):
            reach[i + 1][s] = reach[i][s] or (0 <= x <= s and reach[i][s - x])
    if not reach[len(nums)][t]:
        return None
    s = t
    idx: list[int] = []
    for i in range(len(nums) - 1, -1, -1):
        if not reach[i][s]:
            idx.append(i)
            s -= nums[i]
    idx.reverse()
    return idx


def meet_in_the_middle(nums: list[int], target: int) -> list[int] | None:
    """Meet in the middle: O(2^{n/2}·n)."""
    n = len(nums)
    h = n // 2

    def sums(lo: int, hi: int) -> list[tuple[int, int]]:
        k = hi - lo
        v = [(sum(nums[lo + i] for i in range(k) if (m >> i) & 1), m) for m in range(1 << k)]
        v.sort()
        return v

    a, b = sums(0, h), sums(h, n)
    i, j = 0, len(b) - 1
    while i < len(a) and j >= 0:
        s = a[i][0] + b[j][0]
        if s == target:
            idx = [k for k in range(h) if (a[i][1] >> k) & 1]
            idx.extend(h + k for k in range(n - h) if (b[j][1] >> k) & 1)
            return idx
        if s < target:
            i += 1
        else:
            j -= 1
    return None


def demo() -> str:
    lines: list[str] = []
    nums = [3, 34, 4, 12, 5, 2]
    for t in [9, 30, 58, 1]:
        lines.append(
            f"target {t:>2}: exhaustive {exhaustive(nums, t)}  dp {dp(nums, t)}  "
            f"meet-in-the-middle {meet_in_the_middle(nums, t)}"
        )
    lines.append("dp table size is n·t — exponential in the *bit length* of t, hence pseudo-polynomial")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _verify(self, nums: list[int], target: int, idx: list[int]) -> None:
        self.assertEqual(len(idx), len(set(idx)))
        self.assertTrue(all(0 <= i < len(nums) for i in idx))
        self.assertEqual(sum(nums[i] for i in idx), target)

    def test_three_solvers_agree(self) -> None:
        r = Rng(247)
        for _ in range(300):
            n = r.range(0, 12)
            nums = r.vec(n, 1, 30)
            t = r.range(0, 120)
            e, d, m = exhaustive(nums, t), dp(nums, t), meet_in_the_middle(nums, t)
            self.assertEqual(e is not None, d is not None)
            self.assertEqual(e is not None, m is not None)
            for sol in (e, d, m):
                if sol is not None:
                    self._verify(nums, t, sol)

    def test_edge_cases(self) -> None:
        self.assertEqual(dp([], 0), [])
        self.assertEqual(dp([5], 0), [])
        self.assertIsNone(dp([5], -1))
        self.assertIsNone(meet_in_the_middle([1, 2, 3], 7))


if __name__ == "__main__":
    print(demo(), end="")
