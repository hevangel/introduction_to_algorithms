"""Longest Palindromic Subsequence — CLRS Problem 15-2 (2e–3e), 14-2 (4e) (2e,3e,4e).

The longest subsequence that reads the same forwards and backwards: Theta(n^2).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def longest_palindromic_subsequence(s: str) -> str:
    """Return one longest palindromic subsequence of s."""
    n = len(s)
    if n == 0:
        return ""
    l = [[0] * n for _ in range(n)]
    for i in range(n - 1, -1, -1):
        l[i][i] = 1
        for j in range(i + 1, n):
            if s[i] == s[j]:
                l[i][j] = l[i + 1][j - 1] + 2
            else:
                l[i][j] = max(l[i + 1][j], l[i][j - 1])
    i, j = 0, n - 1
    front: list[str] = []
    back: list[str] = []
    while i <= j:
        if i == j:
            front.append(s[i])
            break
        if s[i] == s[j]:
            front.append(s[i])
            back.append(s[j])
            if j == i + 1:
                break
            i += 1
            j -= 1
        elif l[i + 1][j] >= l[i][j - 1]:
            i += 1
        else:
            j -= 1
    back.reverse()
    return "".join(front + back)


def min_insertions_to_palindrome(s: str) -> int:
    """Minimum number of characters to insert to make s a palindrome = n - |LPS|."""
    return len(s) - len(longest_palindromic_subsequence(s))


def demo() -> str:
    lines: list[str] = []
    for s in ("character", "underqualified", "bananas", "racecar", "abcd"):
        l = longest_palindromic_subsequence(s)
        lines.append(
            f"{s:<15} -> {l:<10} (length {len(l)}, insertions needed {min_insertions_to_palindrome(s)})"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _is_pal(self, s: str) -> bool:
        return s == s[::-1]

    def _is_subseq(self, s: str, t: str) -> bool:
        it = iter(t)
        return all(c in it for c in s)

    def _brute(self, s: str) -> int:
        n = len(s)
        best = 0
        for m in range(1 << n):
            v = [s[i] for i in range(n) if (m >> i) & 1]
            if v == v[::-1]:
                best = max(best, len(v))
        return best

    def test_book_examples(self) -> None:
        self.assertEqual(len(longest_palindromic_subsequence("character")), 5)
        self.assertEqual(
            len(longest_palindromic_subsequence("underqualified")),
            self._brute("underqualified"),
        )
        self.assertEqual(longest_palindromic_subsequence("racecar"), "racecar")
        self.assertEqual(longest_palindromic_subsequence(""), "")
        self.assertEqual(longest_palindromic_subsequence("z"), "z")

    def test_random_strings(self) -> None:
        r = Rng(104)
        for _ in range(80):
            n = r.range(0, 12)
            s = "".join(chr(ord("a") + r.range(0, 2)) for _ in range(n))
            l = longest_palindromic_subsequence(s)
            self.assertTrue(self._is_pal(l) and self._is_subseq(l, s), f"{s} -> {l}")
            self.assertEqual(len(l), 0 if n == 0 else self._brute(s), s)


if __name__ == "__main__":
    print(demo(), end="")
