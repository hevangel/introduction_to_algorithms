"""Longest Monotonically Increasing Subsequence — CLRS Problem 15-4 / Exercises 15.4-5, 15.4-6 (2e–3e), 14.4 (4e) (2e,3e,4e).

O(n^2) dynamic program and O(n lg n) patience-sorting variant using binary search.
"""

from __future__ import annotations

from bisect import bisect_left
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def lis_quadratic(a: list[int]) -> list[int]:
    """Theta(n^2): len[i] = length of the longest increasing subsequence ending at i."""
    n = len(a)
    if n == 0:
        return []
    length = [1] * n
    prev = [-1] * n
    for i in range(n):
        for j in range(i):
            if a[j] < a[i] and length[j] + 1 > length[i]:
                length[i] = length[j] + 1
                prev[i] = j
    end = max(range(n), key=lambda i: (length[i], -i))
    out: list[int] = []
    while end != -1:
        out.append(a[end])
        end = prev[end]
    out.reverse()
    return out


def lis_nlogn(a: list[int]) -> list[int]:
    """Theta(n lg n): tails[k] is the smallest possible last element of an increasing subsequence of length k+1."""
    n = len(a)
    tails: list[int] = []
    tails_idx: list[int] = []
    prev = [-1] * n
    for i in range(n):
        pos = bisect_left(tails, a[i])
        if pos == len(tails):
            tails.append(a[i])
            tails_idx.append(i)
        else:
            tails[pos] = a[i]
            tails_idx[pos] = i
        if pos > 0:
            prev[i] = tails_idx[pos - 1]
    out: list[int] = []
    k = tails_idx[-1] if tails_idx else -1
    while k != -1:
        out.append(a[k])
        k = prev[k]
    out.reverse()
    return out


def demo() -> str:
    lines: list[str] = []
    a = [5, 2, 8, 6, 3, 6, 9, 7]
    lines.append(f"A = {a}")
    lines.append(f"O(n²)      LIS = {lis_quadratic(a)}")
    lines.append(f"O(n lg n)  LIS = {lis_nlogn(a)}")
    b = [10, 22, 9, 33, 21, 50, 41, 60, 80]
    lines.append(f"B = {b}\n  LIS length (n lg n) = {len(lis_nlogn(b))}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _is_inc_subseq(self, s: list[int], a: list[int]) -> bool:
        if any(s[i] >= s[i + 1] for i in range(len(s) - 1)):
            return False
        it = iter(a)
        return all(x in it for x in s)

    def _brute(self, a: list[int]) -> int:
        best = 0
        n = len(a)
        for m in range(1 << n):
            v = [a[i] for i in range(n) if (m >> i) & 1]
            if all(v[i] < v[i + 1] for i in range(len(v) - 1)):
                best = max(best, len(v))
        return best

    def test_examples(self) -> None:
        self.assertEqual(len(lis_nlogn([10, 22, 9, 33, 21, 50, 41, 60, 80])), 6)
        self.assertEqual(lis_quadratic([]), [])
        self.assertEqual(len(lis_nlogn([3, 3, 3])), 1)
        self.assertEqual(lis_nlogn([1, 2, 3, 4]), [1, 2, 3, 4])

    def test_both_agree_with_brute_force(self) -> None:
        r = Rng(106)
        for _ in range(100):
            n = r.range(1, 12)
            a = r.vec(n, 0, 10)
            q, f = lis_quadratic(a), lis_nlogn(a)
            self.assertTrue(self._is_inc_subseq(q, a))
            self.assertTrue(self._is_inc_subseq(f, a))
            self.assertEqual(len(q), self._brute(a))
            self.assertEqual(len(f), len(q))


if __name__ == "__main__":
    print(demo(), end="")
