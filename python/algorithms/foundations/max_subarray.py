"""Maximum Subarray — CLRS 4.1, Exercise 4.1-5 (3e).

Divide-and-conquer (and Kadane's linear scan) for the contiguous subarray of maximum sum.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def find_max_crossing_subarray(a: list[int], low: int, mid: int, high: int) -> tuple[int, int, int]:
    """FIND-MAX-CROSSING-SUBARRAY: best subarray containing the midpoint. Returns (low, high, sum), inclusive."""
    left_sum = -math.inf
    cur = 0
    max_left = mid
    for i in range(mid, low - 1, -1):
        cur += a[i]
        if cur > left_sum:
            left_sum = cur
            max_left = i

    right_sum = -math.inf
    cur = 0
    max_right = mid + 1
    for j in range(mid + 1, high + 1):
        cur += a[j]
        if cur > right_sum:
            right_sum = cur
            max_right = j

    return max_left, max_right, int(left_sum + right_sum)


def _rec(a: list[int], low: int, high: int) -> tuple[int, int, int]:
    if low == high:
        return low, high, a[low]
    mid = (low + high) // 2
    left = _rec(a, low, mid)
    right = _rec(a, mid + 1, high)
    cross = find_max_crossing_subarray(a, low, mid, high)
    if left[2] >= right[2] and left[2] >= cross[2]:
        return left
    if right[2] >= left[2] and right[2] >= cross[2]:
        return right
    return cross


def find_maximum_subarray(a: list[int]) -> tuple[int, int, int]:
    """FIND-MAXIMUM-SUBARRAY: Θ(n lg n) divide and conquer. Raises ValueError on empty input."""
    if not a:
        raise ValueError("input must be non-empty")
    return _rec(a, 0, len(a) - 1)


def brute_force(a: list[int]) -> tuple[int, int, int]:
    """Θ(n²) brute force."""
    best = (0, 0, a[0])
    for i in range(len(a)):
        s = 0
        for j in range(i, len(a)):
            s += a[j]
            if s > best[2]:
                best = (i, j, s)
    return best


def kadane(a: list[int]) -> tuple[int, int, int]:
    """Θ(n) scan (Exercise 4.1-5): extend the best subarray ending at each index."""
    best = (0, 0, a[0])
    cur_start = 0
    cur = a[0]
    for j in range(1, len(a)):
        if cur < 0:
            cur = a[j]
            cur_start = j
        else:
            cur += a[j]
        if cur > best[2]:
            best = (cur_start, j, cur)
    return best


def demo() -> str:
    lines: list[str] = []
    a = [13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7]
    lines.append(f"A = {a}")
    lines.append(f"divide & conquer : {find_maximum_subarray(a)}")
    lines.append(f"kadane           : {kadane(a)}")
    lines.append(f"brute force      : {brute_force(a)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_example(self) -> None:
        a = [13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7]
        self.assertEqual(find_maximum_subarray(a), (7, 10, 43))

    def test_all_methods_agree_on_sum(self) -> None:
        r = Rng(6)
        for n in range(1, 60):
            v = r.vec(n, -20, 20)
            s = brute_force(v)[2]
            self.assertEqual(find_maximum_subarray(v)[2], s)
            self.assertEqual(kadane(v)[2], s)

    def test_all_negative_picks_largest(self) -> None:
        self.assertEqual(kadane([-5, -2, -9]), (1, 1, -2))
        self.assertEqual(find_maximum_subarray([-5, -2, -9]), (1, 1, -2))


if __name__ == "__main__":
    print(demo(), end="")
