"""Naive String Matching — CLRS 32.1 (1e–4e) (1,2,3,4).

Slide the pattern over the text one position at a time: Θ((n−m+1)m).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def naive_string_matcher(t: bytes, p: bytes) -> list[int]:
    """NAIVE-STRING-MATCHER(T, P): all valid shifts s with T[s..s+m] = P."""
    n, m = len(t), len(p)
    if m > n:
        return []
    return [s for s in range(n - m + 1) if t[s : s + m] == p]


def comparisons(t: bytes, p: bytes) -> int:
    """Count of character comparisons performed."""
    n, m = len(t), len(p)
    if m > n:
        return 0
    c = 0
    for s in range(n - m + 1):
        for j in range(m):
            c += 1
            if t[s + j] != p[j]:
                break
    return c


def demo() -> str:
    lines: list[str] = []
    t, p = "acaabc", "aab"
    lines.append(f"T = {t}, P = {p}: valid shifts {naive_string_matcher(t.encode(), p.encode())}")
    t2 = "a" * 1000
    p2 = "a" * 99 + "b"
    lines.append(
        f"worst case T = a^1000, P = a^99 b: {comparisons(t2.encode(), p2.encode())} "
        f"comparisons ≈ (n−m+1)·m = {(1000 - 100 + 1) * 100}"
    )
    lines.append(f"all occurrences of 'aa' in 'aaaa': {naive_string_matcher(b'aaaa', b'aa')}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_basic_cases(self) -> None:
        self.assertEqual(naive_string_matcher(b"acaabc", b"aab"), [2])
        self.assertEqual(naive_string_matcher(b"aaaa", b"aa"), [0, 1, 2])
        self.assertEqual(naive_string_matcher(b"abc", b""), [0, 1, 2, 3])
        self.assertEqual(naive_string_matcher(b"ab", b"abc"), [])

    def test_quadratic_worst_case(self) -> None:
        t = b"a" * 200
        p = b"a" * 49 + b"b"
        self.assertEqual(comparisons(t, p), (200 - 50 + 1) * 50)


if __name__ == "__main__":
    print(demo(), end="")
