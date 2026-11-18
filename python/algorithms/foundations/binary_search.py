"""Binary Search — CLRS Exercise 2.3-5 / 2.3-7 (1e–4e).

Halve a sorted search interval each step: Θ(lg n). Includes the two-sum-in-sorted-order idea.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def binary_search(a: list, v) -> int | None:
    """Iterative binary search on a sorted list; returns an index of `v` if present."""
    lo, hi = 0, len(a)
    while lo < hi:
        mid = lo + (hi - lo) // 2
        if a[mid] == v:
            return mid
        if a[mid] < v:
            lo = mid + 1
        else:
            hi = mid
    return None


def binary_search_rec(a: list, v, lo: int, hi: int) -> int | None:
    """Recursive version over `a[lo:hi]`."""
    if lo >= hi:
        return None
    mid = lo + (hi - lo) // 2
    if a[mid] == v:
        return mid
    if a[mid] < v:
        return binary_search_rec(a, v, mid + 1, hi)
    return binary_search_rec(a, v, lo, mid)


def has_pair_with_sum(s: list[int], x: int) -> bool:
    """Exercise 2.3-7: do two distinct elements of `s` sum to `x`? Θ(n lg n) — sort, then sweep."""
    a = sorted(s)
    if len(a) < 2:
        return False
    i, j = 0, len(a) - 1
    while i < j:
        cur = a[i] + a[j]
        if cur == x:
            return True
        if cur < x:
            i += 1
        else:
            j -= 1
    return False


def demo() -> str:
    lines: list[str] = []
    a = [2, 3, 5, 7, 11, 13, 17, 19]
    lines.append(f"A = {a}")
    for v in [11, 4, 19, 2]:
        lines.append(
            f"search {v:>2}: iterative={binary_search(a, v)} recursive={binary_search_rec(a, v, 0, len(a))}"
        )
    lines.append(f"pair with sum 24 exists? {has_pair_with_sum(a, 24)}")
    lines.append(f"pair with sum 100 exists? {has_pair_with_sum(a, 100)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_finds_every_element_and_rejects_absent(self) -> None:
        a = [x * 2 for x in range(50)]
        for i, x in enumerate(a):
            self.assertEqual(binary_search(a, x), i)
            self.assertEqual(binary_search_rec(a, x, 0, len(a)), i)
            self.assertIsNone(binary_search(a, x + 1))
        self.assertIsNone(binary_search([], 3))

    def test_two_sum(self) -> None:
        self.assertTrue(has_pair_with_sum([1, 5, 9, 14], 15))
        self.assertFalse(has_pair_with_sum([1, 5, 9, 14], 16))
        self.assertFalse(has_pair_with_sum([7], 14))


if __name__ == "__main__":
    print(demo(), end="")
