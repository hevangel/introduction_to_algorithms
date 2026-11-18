"""Heapsort — CLRS 6.1–6.4 (1e–4e).

MAX-HEAPIFY, BUILD-MAX-HEAP and in-place Θ(n lg n) heapsort.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def max_heapify(a: list, i: int, heap_size: int) -> None:
    """MAX-HEAPIFY(A, i): float A[i] down so the subtree rooted at i is a max-heap."""
    while True:
        l, r = 2 * i + 1, 2 * i + 2
        largest = i
        if l < heap_size and a[l] > a[largest]:
            largest = l
        if r < heap_size and a[r] > a[largest]:
            largest = r
        if largest == i:
            return
        a[i], a[largest] = a[largest], a[i]
        i = largest


def build_max_heap(a: list) -> None:
    """BUILD-MAX-HEAP(A): Θ(n) — heapify every internal node bottom-up."""
    n = len(a)
    for i in range(n // 2 - 1, -1, -1):
        max_heapify(a, i, n)


def heapsort(a: list) -> None:
    """HEAPSORT(A): build a max-heap, then repeatedly move the maximum to the end."""
    build_max_heap(a)
    size = len(a)
    while size > 1:
        a[0], a[size - 1] = a[size - 1], a[0]
        size -= 1
        max_heapify(a, 0, size)


def is_max_heap(a: list) -> bool:
    return all(a[(i - 1) // 2] >= a[i] for i in range(1, len(a)))


def demo() -> str:
    lines: list[str] = []
    a = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7]
    lines.append(f"input          : {a}")
    build_max_heap(a)
    lines.append(f"BUILD-MAX-HEAP : {a}  (heap? {is_max_heap(a)})")
    heapsort(a)
    lines.append(f"HEAPSORT       : {a}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_sorts_and_builds_heaps(self) -> None:
        r = Rng(20)
        for n in range(80):
            v = r.vec(n, -30, 30)
            h = list(v)
            build_max_heap(h)
            self.assertTrue(is_max_heap(h))
            e = sorted(v)
            s = list(v)
            heapsort(s)
            self.assertEqual(s, e)

    def test_book_figure_6_2(self) -> None:
        a = [16, 4, 10, 14, 7, 9, 3, 2, 8, 1]
        max_heapify(a, 1, 10)
        self.assertEqual(a, [16, 14, 10, 8, 7, 9, 3, 2, 4, 1])


if __name__ == "__main__":
    print(demo(), end="")
