"""Insertion Sort — CLRS 2.1 (1e–4e).

Builds a sorted prefix by inserting each element into place; the book's first algorithm.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def insertion_sort(a: list) -> None:
    """INSERTION-SORT(A): sorts in place into non-decreasing order."""
    for j in range(1, len(a)):
        key = a[j]
        i = j
        while i > 0 and a[i - 1] > key:
            a[i] = a[i - 1]
            i -= 1
        a[i] = key


def insertion_sort_desc(a: list) -> None:
    """Exercise 2.1-2: non-increasing order."""
    for j in range(1, len(a)):
        key = a[j]
        i = j
        while i > 0 and a[i - 1] < key:
            a[i] = a[i - 1]
            i -= 1
        a[i] = key


def demo() -> str:
    lines: list[str] = []
    a = [5, 2, 4, 6, 1, 3]
    lines.append(f"input      : {a}")
    insertion_sort(a)
    lines.append(f"ascending  : {a}")
    insertion_sort_desc(a)
    lines.append(f"descending : {a}")
    return "\n".join(lines) + "\n"


class _Keyed:
    def __init__(self, key: int, val: str) -> None:
        self.key = key
        self.val = val

    def __lt__(self, other: _Keyed) -> bool:
        return self.key < other.key

    def __gt__(self, other: _Keyed) -> bool:
        return self.key > other.key


class Tests(unittest.TestCase):
    def test_sorts_random_inputs(self) -> None:
        r = Rng(1)
        for n in range(40):
            v = r.vec(n, -50, 50)
            e = sorted(v)
            insertion_sort(v)
            self.assertEqual(v, e)
            insertion_sort_desc(v)
            e.reverse()
            self.assertEqual(v, e)

    def test_stable_on_equal_keys(self) -> None:
        w = [_Keyed(k, c) for k, c in [(1, "a"), (0, "b"), (1, "c"), (0, "d")]]
        insertion_sort(w)
        self.assertEqual("".join(k.val for k in w), "bdac")


if __name__ == "__main__":
    print(demo(), end="")
