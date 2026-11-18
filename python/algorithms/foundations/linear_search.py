"""Linear Search — CLRS Exercise 2.1-3 (1e–4e).

Scan until the value is found; a loop-invariant warm-up exercise.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def linear_search(a: list, v) -> int | None:
    """LINEAR-SEARCH(A, v): index of the first element equal to `v`."""
    for i, x in enumerate(a):
        if x == v:
            return i
    return None


def sentinel_search(a: list[int], v: int) -> int | None:
    """Sentinel version: one comparison per element instead of two."""
    n = len(a)
    a.append(v)
    i = 0
    while a[i] != v:
        i += 1
    a.pop()
    return i if i < n else None


def demo() -> str:
    lines: list[str] = []
    a = [31, 41, 59, 26, 41, 58]
    lines.append(f"A = {a}")
    lines.append(f"linear_search(41)   = {linear_search(a, 41)}")
    lines.append(f"linear_search(99)   = {linear_search(a, 99)}")
    lines.append(f"sentinel_search(26) = {sentinel_search(a, 26)}")
    lines.append(f"sentinel_search(7)  = {sentinel_search(a, 7)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_finds_first_occurrence(self) -> None:
        a = [3, 1, 4, 1, 5]
        self.assertEqual(linear_search(a, 1), 1)
        self.assertIsNone(linear_search(a, 9))
        self.assertIsNone(linear_search([], 1))

    def test_sentinel_matches_linear(self) -> None:
        a = [3, 1, 4, 1, 5]
        for v in range(7):
            e = linear_search(a, v)
            self.assertEqual(sentinel_search(a, v), e)
        self.assertEqual(len(a), 5)


if __name__ == "__main__":
    print(demo(), end="")
