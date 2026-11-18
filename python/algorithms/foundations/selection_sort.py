"""Selection Sort — CLRS Exercise 2.2-2 (1e–4e).

Repeatedly select the minimum of the unsorted suffix and swap it forward.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def selection_sort(a: list) -> None:
    """SELECTION-SORT(A): after round i, A[0..=i] holds the i+1 smallest elements in order."""
    n = len(a)
    if n < 2:
        return
    for i in range(n - 1):
        m = i
        for j in range(i + 1, n):
            if a[j] < a[m]:
                m = j
        a[i], a[m] = a[m], a[i]


def demo() -> str:
    lines: list[str] = []
    a = [64, 25, 12, 22, 11]
    lines.append(f"before: {a}")
    selection_sort(a)
    lines.append(f"after : {a}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_std_sort(self) -> None:
        r = Rng(2)
        for n in range(30):
            v = r.vec(n, -9, 9)
            e = sorted(v)
            selection_sort(v)
            self.assertEqual(v, e)


if __name__ == "__main__":
    print(demo(), end="")
