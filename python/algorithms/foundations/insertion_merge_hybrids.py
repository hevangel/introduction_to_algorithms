"""Binary Insertion Sort & Insertion/Merge Hybrid — CLRS Exercise 2.3-6, Problem 2-1 (1e–4e).

Binary-search the insertion point (Θ(n lg n) comparisons); switch merge sort to insertion sort below a cutoff k.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def binary_insertion_sort(a: list) -> int:
    """Insertion sort that binary-searches the insertion point. Returns comparison count."""
    comps = 0
    for j in range(1, len(a)):
        key = a[j]
        lo, hi = 0, j
        while lo < hi:
            mid = (lo + hi) // 2
            comps += 1
            if a[mid] <= key:
                lo = mid + 1
            else:
                hi = mid
        a[lo + 1 : j + 1] = a[lo:j]
        a[lo] = key
    return comps


def _insertion_counting(a: list, lo: int, hi: int) -> int:
    comps = 0
    for j in range(lo + 1, hi):
        key = a[j]
        i = j
        while i > lo:
            comps += 1
            if a[i - 1] > key:
                a[i] = a[i - 1]
                i -= 1
            else:
                break
        a[i] = key
    return comps


def hybrid_merge_sort(a: list, k: int) -> int:
    """Problem 2-1: merge sort that sorts subarrays of length ≤ k by insertion sort. Returns comparisons."""

    def sort_range(lo: int, hi: int) -> int:
        if hi - lo <= max(k, 1):
            return _insertion_counting(a, lo, hi)
        mid = (lo + hi) // 2
        comps = sort_range(lo, mid) + sort_range(mid, hi)
        left = a[lo:mid]
        right = a[mid:hi]
        i, j = 0, 0
        for idx in range(lo, hi):
            if i < len(left) and j < len(right):
                comps += 1
            if j >= len(right) or (i < len(left) and left[i] <= right[j]):
                a[idx] = left[i]
                i += 1
            else:
                a[idx] = right[j]
                j += 1
        return comps

    return sort_range(0, len(a))


def demo() -> str:
    lines: list[str] = []
    r = Rng(1)
    v = r.vec(4096, 0, 1_000_000)
    b = list(v)
    plain = list(v)
    lines.append(
        f"n = 4096 random keys: binary insertion sort used {binary_insertion_sort(b)} comparisons "
        f"(n lg n ≈ {4096 * 12}); plain insertion sort ≈ {_insertion_counting(plain, 0, len(plain))}"
    )
    for k in [1, 4, 16, 64, 256]:
        c = list(v)
        lines.append(f"hybrid merge sort with cutoff k = {k:>3}: {hybrid_merge_sort(c, k)} comparisons")
    return "\n".join(lines) + "\n"


class _Keyed:
    def __init__(self, key: int, val: str) -> None:
        self.key = key
        self.val = val

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, _Keyed):
            return NotImplemented
        return self.key == other.key

    def __lt__(self, other: _Keyed) -> bool:
        return self.key < other.key

    def __le__(self, other: _Keyed) -> bool:
        return self.key <= other.key

    def __gt__(self, other: _Keyed) -> bool:
        return self.key > other.key


class Tests(unittest.TestCase):
    def test_sort_correctly_and_stably(self) -> None:
        r = Rng(300)
        for n in range(80):
            v = r.vec(n, 0, 15)
            e = sorted(v)
            a, b = list(v), list(v)
            binary_insertion_sort(a)
            hybrid_merge_sort(b, 8)
            self.assertEqual(a, e)
            self.assertEqual(b, e)
        ks = [_Keyed(k, c) for k, c in [(2, "a"), (1, "b"), (2, "c"), (1, "d")]]
        binary_insertion_sort(ks)
        self.assertEqual("".join(k.val for k in ks), "bdac")

    def test_comparison_counts(self) -> None:
        r = Rng(301)
        v = r.vec(1024, 0, 100000)
        self.assertLessEqual(binary_insertion_sort(list(v)), 1024 * 11)
        plain = list(v)
        self.assertGreater(_insertion_counting(plain, 0, len(plain)), 200_000)


if __name__ == "__main__":
    print(demo(), end="")
