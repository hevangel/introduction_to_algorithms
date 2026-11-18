"""Bubble Sort — CLRS Problem 2-2 (1e–4e).

Swap adjacent out-of-order pairs; simple, correct, and Θ(n²).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def bubble_sort(a: list) -> None:
    """BUBBLESORT(A) from Problem 2-2."""
    n = len(a)
    for i in range(n):
        for j in range(n - 1, i, -1):
            if a[j] < a[j - 1]:
                a[j], a[j - 1] = a[j - 1], a[j]


def bubble_sort_early_exit(a: list) -> int:
    """Early-exit variant: stops when a full pass makes no swap."""
    passes = 0
    while True:
        passes += 1
        swapped = False
        for j in range(1, len(a)):
            if a[j] < a[j - 1]:
                a[j], a[j - 1] = a[j - 1], a[j]
                swapped = True
        if not swapped:
            return passes


def demo() -> str:
    lines: list[str] = []
    a = [5, 1, 4, 2, 8]
    lines.append(f"before: {a}")
    bubble_sort(a)
    lines.append(f"after : {a}")
    s = [1, 2, 3, 4]
    lines.append(f"early exit on sorted input used {bubble_sort_early_exit(s)} pass(es)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_sorts(self) -> None:
        r = Rng(4)
        for n in range(30):
            v = r.vec(n, 0, 30)
            e = sorted(v)
            a, b = list(v), list(v)
            bubble_sort(a)
            bubble_sort_early_exit(b)
            self.assertEqual(a, e)
            self.assertEqual(b, e)

    def test_early_exit_is_one_pass_when_sorted(self) -> None:
        self.assertEqual(bubble_sort_early_exit([1, 2, 3]), 1)


if __name__ == "__main__":
    print(demo(), end="")
