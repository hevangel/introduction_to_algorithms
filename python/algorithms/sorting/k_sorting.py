"""Average Sorting (k-Sorted Arrays) — CLRS Problem 8-5 (1e–4e).

Permute an array so every k-length window average is nondecreasing in O(n lg(n/k)) time, plus complete sorting of k-sorted arrays in O(n lg k).
"""

from __future__ import annotations

import heapq
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def is_k_sorted(a: list[int], k: int) -> bool:
    """Verify the k-sorted property: for all i, a[i] <= a[i + k]."""
    if k == 0:
        return False
    return all(a[i] <= a[i + k] for i in range(max(len(a) - k, 0)))


def k_sort(a: list[int], k: int) -> None:
    """k-sort `a` in O(n lg(n/k)) time by sorting each residue class modulo k independently."""
    assert k >= 1, "k must be at least 1"
    n = len(a)
    for r in range(min(k, n)):
        stride = sorted(a[r:n:k])
        a[r:n:k] = stride


def merge_k_sorted(a: list[int], k: int) -> list[int]:
    """Sort an already k-sorted array in O(n lg k) time using a k-way min-heap merge."""
    assert k >= 1, "k must be at least 1"
    n = len(a)
    heap = [(a[r], r) for r in range(min(k, n))]
    heapq.heapify(heap)
    out: list[int] = []
    while heap:
        val, idx = heapq.heappop(heap)
        out.append(val)
        nxt = idx + k
        if nxt < n:
            heapq.heappush(heap, (a[nxt], nxt))
    return out


def demo() -> str:
    lines: list[str] = []
    orig = [9, 3, 7, 1, 8, 2, 6, 4, 5, 0]
    a = list(orig)
    k_sort(a, 3)
    lines.append(f"original   = {orig}")
    lines.append(f"3-sorted   = {a} (is_3_sorted = {is_k_sorted(a, 3)})")
    avgs = [round(sum(a[i : i + 3]) / 3.0, 2) for i in range(len(a) - 2)]
    lines.append(f"3-window averages = {avgs}")
    lines.append(f"k-way merged      = {merge_k_sorted(a, 3)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_k_sort_window_averages_and_merge(self) -> None:
        r = Rng(8501)
        for _ in range(100):
            n = r.range(1, 120)
            k = r.range(1, 20)
            orig = r.vec(n, -500, 500)
            a = list(orig)
            k_sort(a, k)
            self.assertTrue(is_k_sorted(a, k))
            if k <= n:
                sums = [sum(a[i : i + k]) for i in range(n - k + 1)]
                self.assertTrue(all(sums[i] <= sums[i + 1] for i in range(len(sums) - 1)))
            merged = merge_k_sorted(a, k)
            self.assertEqual(merged, sorted(orig))

    def test_k_equal_1_is_full_sort(self) -> None:
        a = [5, 2, 9, 1, 5, 6]
        k_sort(a, 1)
        self.assertEqual(a, [1, 2, 5, 5, 6, 9])
        self.assertTrue(is_k_sorted(a, 1))


if __name__ == "__main__":
    print(demo(), end="")
